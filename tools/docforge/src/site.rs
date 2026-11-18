use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::atomic;

pub const SECTIONS: [&str; 5] = ["docs", "developer", "plugin", "download", "disclosure"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Language {
    Zh,
    En,
}

impl Language {
    pub fn label(self) -> &'static str {
        match self {
            Self::Zh => "中文",
            Self::En => "English",
        }
    }

    pub fn prefix(self) -> &'static str {
        match self {
            Self::Zh => "",
            Self::En => "en",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DocumentRef {
    pub language: Language,
    pub relative_path: String,
    pub public_path: String,
    pub title: String,
    pub section: String,
    pub counterpart: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SiteProfile {
    pub root: PathBuf,
    pub languages: Vec<String>,
    pub sections: Vec<String>,
    pub documents: Vec<DocumentRef>,
}

impl SiteProfile {
    pub fn discover(root: &Path) -> Result<Self> {
        let root = root
            .canonicalize()
            .with_context(|| format!("cannot resolve {}", root.display()))?;
        if !root.join(".vitepress").join("config.mts").is_file() {
            bail!(
                "{} is not a VitePress site with .vitepress/config.mts",
                root.display()
            );
        }
        if !root.join("index.md").is_file() {
            bail!("{} does not contain index.md", root.display());
        }

        let mut documents = Vec::new();
        for language in [Language::Zh, Language::En] {
            let language_root = if language == Language::Zh {
                root.clone()
            } else {
                root.join(language.prefix())
            };
            if !language_root.is_dir() {
                continue;
            }

            let root_index = language_root.join("index.md");
            if root_index.is_file() {
                documents.push(document_ref(&root, &root_index, language, "index"));
            }
            for section in SECTIONS {
                let section_root = language_root.join(section);
                if !section_root.is_dir() {
                    continue;
                }
                for entry in WalkDir::new(&section_root)
                    .follow_links(false)
                    .into_iter()
                    .filter_map(std::result::Result::ok)
                {
                    let path = entry.path();
                    if !entry.file_type().is_file()
                        || path.extension().and_then(|ext| ext.to_str()) != Some("md")
                    {
                        continue;
                    }
                    documents.push(document_ref(&root, path, language, section));
                }
            }
        }

        documents.sort_by(|left, right| {
            left.language
                .prefix()
                .cmp(right.language.prefix())
                .then_with(|| left.relative_path.cmp(&right.relative_path))
        });
        let relative_paths: std::collections::HashSet<String> = documents
            .iter()
            .map(|document| document.relative_path.clone())
            .collect();
        for document in &mut documents {
            let key = pair_key(&document.relative_path);
            let counterpart_language = if document.language == Language::Zh {
                Language::En
            } else {
                Language::Zh
            };
            let counterpart_relative = if counterpart_language == Language::En {
                format!("en/{key}")
            } else {
                key.clone()
            };
            let counterpart = relative_paths
                .contains(&counterpart_relative)
                .then_some(counterpart_relative);
            document.counterpart = counterpart;
        }

        Ok(Self {
            root,
            languages: vec!["zh-CN".to_string(), "en-US".to_string()],
            sections: SECTIONS.iter().map(|value| (*value).to_string()).collect(),
            documents,
        })
    }

    pub fn document_path(&self, document: &DocumentRef) -> Result<PathBuf> {
        let path = self.root.join(&document.relative_path);
        atomic::ensure_inside(&self.root, &path)
    }

    pub fn translation_target_path(&self, document: &DocumentRef) -> Result<PathBuf> {
        let key = pair_key(&document.relative_path);
        let relative = if document.language == Language::Zh {
            format!("en/{key}")
        } else {
            key
        };
        let candidate = self.root.join(relative);
        let parent = candidate.parent().context("翻译目标文件没有父目录")?;
        if !parent.starts_with(&self.root) {
            bail!("拒绝在文档站目录之外创建翻译文件：{}", candidate.display());
        }
        fs::create_dir_all(parent)
            .with_context(|| format!("无法创建翻译目标目录 {}", parent.display()))?;
        atomic::ensure_inside(&self.root, &candidate)
    }

    pub fn media_dir(&self, document: &DocumentRef) -> Result<PathBuf> {
        let section = if document.section == "index" {
            "docs"
        } else {
            document.section.as_str()
        };
        self.media_dir_for_section(section)
    }

    pub fn media_dir_for_section(&self, section: &str) -> Result<PathBuf> {
        if !SECTIONS.contains(&section) {
            bail!("文档栏目不受支持：{section}");
        }
        let candidate = self.root.join(section).join("img").join("generated");
        fs::create_dir_all(&candidate)?;
        atomic::ensure_inside(&self.root, &candidate)
    }

    pub fn tutorial_document_path(
        &self,
        language: Language,
        section: &str,
        file_stem: &str,
    ) -> Result<PathBuf> {
        if !SECTIONS.contains(&section) {
            bail!("文档栏目不受支持：{section}");
        }
        let file_stem = file_stem.trim().trim_end_matches(".md");
        if file_stem.is_empty()
            || file_stem == "."
            || file_stem == ".."
            || file_stem
                .chars()
                .any(|character| r#"<>:"/\|?*"#.contains(character))
        {
            bail!("教程文件名无效，请只填写文件名，不要填写文件夹路径");
        }
        let language_root = if language == Language::Zh {
            self.root.clone()
        } else {
            self.root.join(language.prefix())
        };
        let candidate = language_root.join(section).join(format!("{file_stem}.md"));
        let parent = candidate.parent().context("教程文件没有父目录")?;
        fs::create_dir_all(parent)
            .with_context(|| format!("无法创建教程目录 {}", parent.display()))?;
        atomic::ensure_inside(&self.root, &candidate)
    }
}

fn document_ref(root: &Path, path: &Path, language: Language, section: &str) -> DocumentRef {
    let relative = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    let title = fs::read_to_string(path)
        .ok()
        .and_then(|content| first_markdown_heading(&content))
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| {
            path.file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or("Untitled")
                .to_string()
        });
    let public_path = relative
        .strip_suffix(".md")
        .unwrap_or(&relative)
        .to_string();
    DocumentRef {
        language,
        relative_path: relative,
        public_path,
        title,
        section: section.to_string(),
        counterpart: None,
    }
}

fn pair_key(relative: &str) -> String {
    relative.strip_prefix("en/").unwrap_or(relative).to_string()
}

fn first_markdown_heading(content: &str) -> Option<String> {
    let mut in_frontmatter = false;
    for (index, line) in content.lines().enumerate() {
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
                return Some(title.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_frontmatter_comments_when_reading_title() {
        let source = "---\n# https://example.test\nlayout: home\n---\n# 真正标题\n";
        assert_eq!(first_markdown_heading(source).as_deref(), Some("真正标题"));
    }

    #[test]
    fn rejects_tutorial_file_name_with_path_separator() {
        let root = tempfile::tempdir().expect("创建测试目录");
        std::fs::create_dir_all(root.path().join(".vitepress")).expect("创建配置目录");
        std::fs::write(
            root.path().join(".vitepress/config.mts"),
            "export default {}",
        )
        .expect("写入配置");
        std::fs::write(root.path().join("index.md"), "# 首页").expect("写入首页");
        let site = SiteProfile::discover(root.path()).expect("识别站点");
        assert!(site
            .tutorial_document_path(Language::Zh, "docs", "folder/tutorial")
            .is_err());
    }
}
