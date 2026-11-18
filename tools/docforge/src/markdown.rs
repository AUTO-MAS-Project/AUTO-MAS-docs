use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct CheckItem {
    pub level: CheckLevel,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreviewBlock {
    Heading { level: u8, text: String },
    Paragraph(String),
    Code(String),
    Image { alt: String, source: String },
    Callout { kind: String, text: String },
    Rule,
}

pub fn check_document(source: &str, document_path: &Path, site_root: &Path) -> Vec<CheckItem> {
    let mut checks = Vec::new();
    if !source
        .lines()
        .any(|line| line.trim_start().starts_with("# "))
    {
        checks.push(CheckItem {
            level: CheckLevel::Warning,
            message: "没有找到一级标题（# 标题），建议补充页面标题。".to_string(),
        });
    }

    let mut fence_open = false;
    for line in source.lines() {
        if line.trim_start().starts_with("```") {
            fence_open = !fence_open;
        }
    }
    if fence_open {
        checks.push(CheckItem {
            level: CheckLevel::Error,
            message: "代码块没有闭合，请补上末尾的 ```。".to_string(),
        });
    }

    let unclosed_containers = unclosed_container_count(source);
    if unclosed_containers > 0 {
        checks.push(CheckItem {
            level: CheckLevel::Error,
            message: format!(
                "有 {unclosed_containers} 个 VitePress 容器没有闭合，请补上对应的 ::: 或 ::::。"
            ),
        });
    }

    for (alt, reference) in image_references(source) {
        if reference.starts_with("http://")
            || reference.starts_with("https://")
            || reference.starts_with("data:")
        {
            continue;
        }
        let clean_reference = clean_image_reference(&reference);
        let path = if clean_reference.starts_with('/') {
            site_root.join(clean_reference.trim_start_matches('/'))
        } else {
            document_path
                .parent()
                .unwrap_or(site_root)
                .join(clean_reference)
        };
        if !path.is_file() {
            checks.push(CheckItem {
                level: CheckLevel::Error,
                message: format!("图片不存在：{}（替代文字：{}）", reference, alt),
            });
        }
    }

    if checks.is_empty() {
        checks.push(CheckItem {
            level: CheckLevel::Info,
            message: "基础检查通过：标题、代码块和本地图片引用均正常。".to_string(),
        });
    }
    checks
}

pub fn image_references(source: &str) -> Vec<(String, String)> {
    let mut images = Vec::new();
    let parser = Parser::new_ext(source, Options::all());
    let mut current: Option<(String, String)> = None;
    for event in parser {
        match event {
            Event::Start(Tag::Image { dest_url, .. }) => {
                current = Some((String::new(), dest_url.to_string()));
            }
            Event::Text(text) if current.is_some() => {
                if let Some((alt, _)) = current.as_mut() {
                    alt.push_str(&text);
                }
            }
            Event::End(TagEnd::Image) => {
                if let Some(image) = current.take() {
                    images.push(image);
                }
            }
            _ => {}
        }
    }
    images
}

pub fn resolve_image_path(reference: &str, document_path: &Path, site_root: &Path) -> PathBuf {
    let clean = clean_image_reference(reference);
    if clean.starts_with('/') {
        site_root.join(clean.trim_start_matches('/'))
    } else {
        document_path.parent().unwrap_or(site_root).join(clean)
    }
}

pub fn replace_image_reference(source: &str, current: &str, replacement: &str) -> String {
    source.replace(
        &format!("]({})", current.trim()),
        &format!("]({})", replacement.trim()),
    )
}

pub fn inline_preview_text(source: &str) -> String {
    let source = source.replace("**", "").replace("__", "").replace('`', "");
    let mut output = String::new();
    let mut remaining = source.as_str();
    while let Some(open) = remaining.find('[') {
        output.push_str(&remaining[..open]);
        let after_open = &remaining[open + 1..];
        let Some(close_label) = after_open.find("](") else {
            output.push_str(&remaining[open..]);
            return output;
        };
        let after_target = &after_open[close_label + 2..];
        let Some(close_target) = after_target.find(')') else {
            output.push_str(&remaining[open..]);
            return output;
        };
        output.push_str(&after_open[..close_label]);
        remaining = &after_target[close_target + 1..];
    }
    output.push_str(remaining);
    output
}

#[cfg(test)]
pub fn append_image(source: &mut String, alt: &str, public_path: &str) {
    if !source.ends_with('\n') {
        source.push('\n');
    }
    source.push('\n');
    source.push_str(&format!("![{}]({})\n", alt.trim(), public_path));
}

pub fn append_media(source: &str, heading: &str, media: &[(String, String)]) -> String {
    let mut output = source.to_string();
    if !output.ends_with('\n') {
        output.push('\n');
    }
    output.push_str(&format!("\n## {}\n\n", heading.trim()));
    for (alt, public_path) in media {
        output.push_str(&format!("![{}]({})\n\n", alt.trim(), public_path.trim()));
    }
    output
}

pub fn preview_blocks(source: &str) -> Vec<PreviewBlock> {
    let mut blocks = Vec::new();
    let mut paragraph = Vec::new();
    let mut code = Vec::new();
    let mut in_code = false;
    let mut container: Option<(String, Vec<String>, usize)> = None;

    let flush = |blocks: &mut Vec<PreviewBlock>, paragraph: &mut Vec<String>| {
        if !paragraph.is_empty() {
            blocks.push(PreviewBlock::Paragraph(paragraph.join("\n")));
            paragraph.clear();
        }
    };

    for raw_line in source.lines() {
        let line = raw_line.trim_end();
        if in_code {
            if line.trim_start().starts_with("```") {
                blocks.push(PreviewBlock::Code(code.join("\n")));
                code.clear();
                in_code = false;
            } else {
                code.push(line.to_string());
            }
            continue;
        }
        if line.trim_start().starts_with("```") {
            flush(&mut blocks, &mut paragraph);
            in_code = true;
            continue;
        }

        if container.is_some() {
            let closes_container = container_marker(line).is_some_and(|(marker, declaration)| {
                declaration.is_empty()
                    && container
                        .as_ref()
                        .is_some_and(|(_, _, opened)| *opened == marker)
            });
            if closes_container {
                let (kind, content, _) = container.take().expect("容器应当存在");
                blocks.push(PreviewBlock::Callout {
                    kind,
                    text: content.join("\n"),
                });
            } else if let Some((_, content, _)) = container.as_mut() {
                content.push(line.to_string());
            }
            continue;
        }
        if let Some((marker, declaration)) = container_marker(line) {
            if !declaration.is_empty() {
                flush(&mut blocks, &mut paragraph);
                container = Some((callout_label(declaration), Vec::new(), marker));
                continue;
            }
        }

        if line.trim() == "---" {
            flush(&mut blocks, &mut paragraph);
            blocks.push(PreviewBlock::Rule);
            continue;
        }
        if let Some((level, text)) = preview_heading(line) {
            flush(&mut blocks, &mut paragraph);
            blocks.push(PreviewBlock::Heading {
                level,
                text: text.to_string(),
            });
            continue;
        }
        let images = image_references(line);
        if images.len() == 1 && line.trim_start().starts_with("![") {
            flush(&mut blocks, &mut paragraph);
            let (alt, source) = images.into_iter().next().expect("应当只有一张图片");
            blocks.push(PreviewBlock::Image { alt, source });
            continue;
        }
        if line.trim().is_empty() {
            flush(&mut blocks, &mut paragraph);
        } else {
            paragraph.push(line.to_string());
        }
    }
    flush(&mut blocks, &mut paragraph);
    if !code.is_empty() {
        blocks.push(PreviewBlock::Code(code.join("\n")));
    }
    if let Some((kind, content, _)) = container {
        blocks.push(PreviewBlock::Callout {
            kind,
            text: content.join("\n"),
        });
    }
    blocks
}

fn container_marker(line: &str) -> Option<(usize, &str)> {
    let trimmed = line.trim();
    let marker = trimmed
        .chars()
        .take_while(|character| *character == ':')
        .count();
    (marker >= 3).then(|| (marker, trimmed[marker..].trim()))
}

fn callout_label(declaration: &str) -> String {
    let mut parts = declaration.splitn(2, char::is_whitespace);
    let kind = parts.next().unwrap_or_default().to_ascii_lowercase();
    let label = match kind.as_str() {
        "tip" => "提示",
        "warning" => "警告",
        "danger" => "危险",
        "info" => "信息",
        "details" => "详情",
        "code-group" => "代码组",
        _ => "VitePress 容器",
    };
    let title = parts.next().unwrap_or_default().trim();
    if title.is_empty() {
        label.to_string()
    } else {
        format!("{label} · {title}")
    }
}

fn unclosed_container_count(source: &str) -> usize {
    let mut stack = Vec::new();
    let mut in_code = false;
    for line in source.lines() {
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if in_code {
            continue;
        }
        let Some((marker, declaration)) = container_marker(line) else {
            continue;
        };
        if declaration.is_empty() {
            if stack.last() == Some(&marker) {
                stack.pop();
            }
        } else {
            stack.push(marker);
        }
    }
    stack.len()
}

fn clean_image_reference(reference: &str) -> &str {
    reference.split(['?', '#']).next().unwrap_or(reference)
}
fn preview_heading(line: &str) -> Option<(u8, &str)> {
    let trimmed = line.trim_start();
    let level = trimmed
        .chars()
        .take_while(|character| *character == '#')
        .count();
    if !(1..=6).contains(&level) || !trimmed[level..].starts_with(' ') {
        return None;
    }
    Some((level as u8, trimmed[level + 1..].trim()))
}

pub fn title(source: &str) -> String {
    let mut in_frontmatter = false;
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if index == 0 && trimmed == "---" {
            in_frontmatter = true;
            continue;
        }
        if in_frontmatter {
            if trimmed == "---" {
                in_frontmatter = false;
            }
            continue;
        }
        if let Some(title) = line.trim_start().strip_prefix("# ") {
            let title = title.trim();
            if !title.is_empty() {
                return title.to_string();
            }
        }
    }
    "AUTO-MAS 文档".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_markdown_images() {
        let images = image_references("![步骤一](img/one.png)\n![two](/docs/img/two.png)");
        assert_eq!(images.len(), 2);
        assert_eq!(images[0].1, "img/one.png");
    }

    #[test]
    fn appends_image_on_its_own_line() {
        let mut source = "# 标题".to_string();
        append_image(&mut source, "关键步骤", "/docs/img/step.png");
        assert!(source.ends_with("![关键步骤](/docs/img/step.png)\n"));
    }

    #[test]
    fn previews_vitepress_container() {
        let blocks = preview_blocks(":::: warning 注意\n请先备份。\n::::");
        assert!(blocks.iter().any(|block| {
            matches!(
                block,
                PreviewBlock::Callout { kind, text }
                    if kind == "警告 · 注意" && text == "请先备份。"
            )
        }));
    }

    #[test]
    fn reports_unclosed_vitepress_container() {
        assert_eq!(unclosed_container_count("::: tip\n内容"), 1);
        assert_eq!(unclosed_container_count("::: tip\n内容\n:::"), 0);
    }

    #[test]
    fn removes_query_and_anchor_from_image_path() {
        assert_eq!(
            clean_image_reference("img/step.png?v=2#focus"),
            "img/step.png"
        );
    }

    #[test]
    fn title_ignores_frontmatter_comments() {
        assert_eq!(
            title("---\n# source comment\n---\n# 页面标题\n"),
            "页面标题"
        );
    }

    #[test]
    fn replaces_only_matching_image_destination() {
        let source = "![步骤](img/step.png)\n![保留](img/other.png)";
        let replaced = replace_image_reference(source, "img/step.png", "img/step-edited.png");
        assert!(replaced.contains("![步骤](img/step-edited.png)"));
        assert!(replaced.contains("![保留](img/other.png)"));
    }

    #[test]
    fn cleans_common_inline_markdown_for_preview() {
        assert_eq!(
            inline_preview_text("**下载** [绿色包](/download/) 并运行 `AUTO-MAS.exe`"),
            "下载 绿色包 并运行 AUTO-MAS.exe"
        );
    }
}
