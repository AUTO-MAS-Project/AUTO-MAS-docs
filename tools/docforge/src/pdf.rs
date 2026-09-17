use anyhow::{bail, Context, Result};
use lopdf::Document;
use printpdf::{
    BuiltinFont, ColorBits, ColorSpace, Image, ImageFilter, ImageTransform, ImageXObject,
    IndirectFontRef, Mm, PdfDocument, PdfDocumentReference, Px,
};
use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;
use unicode_width::UnicodeWidthChar;

use crate::{atomic, markdown};

#[derive(Debug, Clone)]
pub struct PdfInfo {
    pub path: PathBuf,
    pub pages: usize,
    pub warning: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PdfPreview {
    pub path: PathBuf,
    pub pages: usize,
    pub text: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PdfExportOptions<'a> {
    pub custom_font: Option<&'a Path>,
    pub document_path: Option<&'a Path>,
    pub site_root: Option<&'a Path>,
}

pub fn preview_pdf(input: &Path) -> Result<PdfPreview> {
    let info = inspect_pdf(input)?;
    Ok(PdfPreview {
        path: input.to_path_buf(),
        pages: info.pages,
        text: preview_text(input)?,
    })
}

pub fn render_first_page(input: &Path, pdftoppm: Option<&Path>) -> Result<Vec<u8>> {
    let executable = pdftoppm
        .map(Path::to_path_buf)
        .or_else(find_pdftoppm)
        .context(
            "未找到 pdftoppm.exe，仍可使用文字预览；如需页面预览，请把 Poppler 放到绿色包 bin 目录",
        )?;
    let temporary = tempfile::tempdir().context("无法创建 PDF 页面预览临时目录")?;
    let prefix = temporary.path().join("page");
    let output = Command::new(&executable)
        .arg("-f")
        .arg("1")
        .arg("-singlefile")
        .arg("-png")
        .arg(input)
        .arg(&prefix)
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("无法启动 {}", executable.display()))?;
    if !output.status.success() {
        bail!(
            "PDF 页面预览失败：{}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    std::fs::read(prefix.with_extension("png")).context("无法读取渲染后的 PDF 首页")
}

fn find_pdftoppm() -> Option<PathBuf> {
    let executable_dir = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf));
    if let Some(directory) = executable_dir {
        for candidate in [
            directory.join("pdftoppm.exe"),
            directory.join("bin").join("pdftoppm.exe"),
        ] {
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    let candidate = PathBuf::from("pdftoppm");
    Command::new(&candidate)
        .arg("-v")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()
        .filter(|status| status.success())
        .map(|_| candidate)
}

pub fn export_markdown_to_pdf(
    source: &str,
    output: &Path,
    options: PdfExportOptions<'_>,
) -> Result<PdfInfo> {
    if source.trim().is_empty() {
        bail!("当前文档没有内容，无法导出 PDF");
    }
    let title = markdown::title(source);
    let (document, first_page, first_layer) =
        PdfDocument::new(&title, Mm(210.0), Mm(297.0), "正文");
    let (font, font_warning) = load_font(&document, options.custom_font, source)?;
    let mut warnings = font_warning.into_iter().collect::<Vec<_>>();
    let mut page = first_page;
    let mut layer = first_layer;
    let mut y = 278.0f32;
    let mut pages = 1usize;

    for raw_line in source.lines() {
        let trimmed = raw_line.trim_end();
        if trimmed == "---" || trimmed.starts_with(":::") {
            continue;
        }

        let image_references = markdown::image_references(trimmed);
        if trimmed.trim_start().starts_with("![") && image_references.len() == 1 {
            let (_, reference) = &image_references[0];
            let local_image = options
                .document_path
                .zip(options.site_root)
                .filter(|_| !reference.starts_with("http://") && !reference.starts_with("https://"))
                .map(|(document_path, site_root)| {
                    markdown::resolve_image_path(reference, document_path, site_root)
                });
            if let Some(image_path) = local_image {
                match image::open(&image_path) {
                    Ok(dynamic_image) => {
                        let rgb_image = dynamic_image.to_rgb8();
                        let pixel_width = rgb_image.width();
                        let pixel_height = rgb_image.height();
                        let mut jpeg_data = Vec::new();
                        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_data, 92)
                            .encode(
                                rgb_image.as_raw(),
                                pixel_width,
                                pixel_height,
                                image::ExtendedColorType::Rgb8,
                            )
                            .with_context(|| {
                                format!("无法压缩 PDF 图片 {}", image_path.display())
                            })?;
                        let width_at_96_dpi = pixel_width as f32 * 25.4 / 96.0;
                        let height_at_96_dpi = pixel_height as f32 * 25.4 / 96.0;
                        let scale = (178.0 / width_at_96_dpi)
                            .min(118.0 / height_at_96_dpi)
                            .min(1.0);
                        let display_height = height_at_96_dpi * scale;
                        if y - display_height < 18.0 {
                            let (next_page, next_layer) =
                                document.add_page(Mm(210.0), Mm(297.0), "正文");
                            page = next_page;
                            layer = next_layer;
                            y = 278.0;
                            pages += 1;
                        }
                        Image::from(ImageXObject {
                            width: Px(pixel_width as usize),
                            height: Px(pixel_height as usize),
                            color_space: ColorSpace::Rgb,
                            bits_per_component: ColorBits::Bit8,
                            interpolate: true,
                            image_data: jpeg_data,
                            image_filter: Some(ImageFilter::DCT),
                            smask: None,
                            clipping_bbox: None,
                        })
                        .add_to_layer(
                            document.get_page(page).get_layer(layer),
                            ImageTransform {
                                translate_x: Some(Mm(16.0)),
                                translate_y: Some(Mm(y - display_height)),
                                scale_x: Some(scale),
                                scale_y: Some(scale),
                                dpi: Some(96.0),
                                ..Default::default()
                            },
                        );
                        y -= display_height + 8.0;
                        continue;
                    }
                    Err(error) => warnings.push(format!(
                        "图片未写入 PDF：{}（{}）",
                        image_path.display(),
                        error
                    )),
                }
            }
        }

        let (text, size, spacing, indent) = pdf_line_style(trimmed);
        if text.is_empty() {
            y -= 3.0;
            continue;
        }

        let max_units = ((178.0 - indent) / (size * 0.37)).max(12.0) as usize;
        let wrapped = wrap_text(&text, max_units);
        for line in wrapped {
            if y < 18.0 {
                let (next_page, next_layer) = document.add_page(Mm(210.0), Mm(297.0), "正文");
                page = next_page;
                layer = next_layer;
                y = 278.0;
                pages += 1;
            }
            let current_layer = document.get_page(page).get_layer(layer);
            current_layer.use_text(line, size, Mm(16.0 + indent), Mm(y), &font);
            y -= spacing;
        }
    }

    let parent = output.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)?;
    let mut temporary = NamedTempFile::new_in(parent)?;
    document
        .save(&mut BufWriter::new(temporary.as_file_mut()))
        .context("生成 PDF 失败")?;
    temporary.as_file_mut().sync_all()?;
    atomic::copy_atomic(temporary.path(), output)?;

    let actual_pages = inspect_pdf(output)?.pages;
    Ok(PdfInfo {
        path: output.to_path_buf(),
        pages: actual_pages.max(pages),
        warning: (!warnings.is_empty()).then(|| warnings.join("；")),
    })
}

pub fn pdf_to_markdown(input: &Path) -> Result<String> {
    if !input.is_file() {
        bail!("找不到 PDF：{}", input.display());
    }
    let text = pdf_extract::extract_text(input)
        .with_context(|| format!("无法读取 PDF {}", input.display()))?;
    let mut output = String::from("# 从 PDF 导入的文档\n\n");
    for block in text.split("\n\n") {
        let compact = block
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        if !compact.is_empty() {
            output.push_str(&compact);
            output.push_str("\n\n");
        }
    }
    output.push_str("> 提示：PDF 导入会提取可复制文字。复杂排版、表格和图片请在预览后手动整理。\n");
    Ok(output)
}

pub fn inspect_pdf(input: &Path) -> Result<PdfInfo> {
    let document =
        Document::load(input).with_context(|| format!("无法检查 PDF {}", input.display()))?;
    Ok(PdfInfo {
        path: input.to_path_buf(),
        pages: document.get_pages().len(),
        warning: None,
    })
}

pub fn preview_text(input: &Path) -> Result<String> {
    let mut bytes = Vec::new();
    File::open(input)?.read_to_end(&mut bytes)?;
    let text = pdf_extract::extract_text_from_mem(&bytes).context("无法生成 PDF 文字预览")?;
    Ok(text)
}

fn load_font(
    document: &PdfDocumentReference,
    custom_font: Option<&Path>,
    source: &str,
) -> Result<(IndirectFontRef, Option<String>)> {
    let candidate = custom_font
        .filter(|path| path.is_file())
        .map(Path::to_path_buf)
        .or_else(find_windows_cjk_font);
    if let Some(path) = candidate {
        let file =
            File::open(&path).with_context(|| format!("无法打开 PDF 字体 {}", path.display()))?;
        let font = document
            .add_external_font(file)
            .with_context(|| format!("无法加载 PDF 字体 {}", path.display()))?;
        return Ok((font, None));
    }
    if !source.is_ascii() {
        bail!("未找到中文 TrueType 字体。请在“PDF 字体”中选择 .ttf 文件后重试。");
    }
    Ok((
        document.add_builtin_font(BuiltinFont::Helvetica)?,
        Some("未找到外部字体，已使用 Helvetica；仅适合英文内容。".to_string()),
    ))
}

pub fn find_windows_cjk_font() -> Option<PathBuf> {
    let windows = std::env::var_os("WINDIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\Windows"));
    ["simhei.ttf", "Deng.ttf", "msyh.ttf"]
        .into_iter()
        .map(|name| windows.join("Fonts").join(name))
        .find(|path| path.is_file())
}

fn pdf_line_style(line: &str) -> (String, f32, f32, f32) {
    if let Some(text) = line.strip_prefix("# ") {
        return (text.trim().to_string(), 22.0, 11.0, 0.0);
    }
    if let Some(text) = line.strip_prefix("## ") {
        return (text.trim().to_string(), 16.0, 8.5, 0.0);
    }
    if let Some(text) = line.strip_prefix("### ") {
        return (text.trim().to_string(), 13.0, 7.0, 0.0);
    }
    if line.starts_with("![") {
        return (format!("[图片] {}", strip_markdown(line)), 9.0, 5.5, 5.0);
    }
    if let Some(text) = line.strip_prefix("> ") {
        return (strip_markdown(text), 9.5, 5.5, 6.0);
    }
    if let Some(text) = line.strip_prefix("- ") {
        return (format!("- {}", strip_markdown(text)), 10.5, 6.0, 5.0);
    }
    let numbered = line
        .split_once(". ")
        .filter(|(prefix, _)| prefix.chars().all(|character| character.is_ascii_digit()));
    if let Some((prefix, text)) = numbered {
        return (
            format!("{}. {}", prefix, strip_markdown(text)),
            10.5,
            6.0,
            5.0,
        );
    }
    (strip_markdown(line), 10.5, 6.0, 0.0)
}

fn strip_markdown(value: &str) -> String {
    value
        .replace("**", "")
        .replace("__", "")
        .replace('`', "")
        .replace("<br>", " ")
}

fn wrap_text(value: &str, max_units: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut units = 0usize;
    for character in value.chars() {
        let width = UnicodeWidthChar::width(character).unwrap_or(1).max(1);
        if units + width > max_units && !current.is_empty() {
            lines.push(current.trim_end().to_string());
            current.clear();
            units = 0;
        }
        current.push(character);
        units += width;
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_cjk_by_display_width() {
        let lines = wrap_text("一二三四五", 4);
        assert_eq!(lines, vec!["一二", "三四", "五"]);
    }

    #[test]
    fn exports_local_markdown_image_into_pdf() {
        let directory = tempfile::tempdir().expect("创建测试目录");
        let markdown_path = directory.path().join("guide.md");
        let image_path = directory.path().join("step.png");
        let pdf_path = directory.path().join("guide.pdf");
        std::fs::write(&markdown_path, "# Guide\n\n![Step](step.png)\n").expect("写入 Markdown");
        image::DynamicImage::new_rgb8(320, 180)
            .save(&image_path)
            .expect("保存图片");

        let info = export_markdown_to_pdf(
            "# Guide\n\n![Step](step.png)\n",
            &pdf_path,
            PdfExportOptions {
                document_path: Some(&markdown_path),
                site_root: Some(directory.path()),
                ..Default::default()
            },
        )
        .expect("导出 PDF");
        assert!(info.path.is_file());
        assert_eq!(info.pages, 1);
        let pdf = Document::load(&pdf_path).expect("读取 PDF");
        let contains_image = pdf.objects.values().any(|object| {
            object.as_stream().ok().is_some_and(|stream| {
                matches!(
                    stream.dict.get(b"Subtype"),
                    Ok(lopdf::Object::Name(name)) if name == b"Image"
                )
            })
        });
        assert!(contains_image, "PDF 应包含 Markdown 引用的本地图片");
    }
}
