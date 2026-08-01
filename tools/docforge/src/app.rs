use anyhow::{Context, Result};
use eframe::egui::{
    self, Color32, ColorImage, FontData, FontDefinitions, FontFamily, Pos2, Rect, RichText, Sense,
    Stroke, TextureOptions, Vec2,
};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;

use crate::annotation::{self, Annotation, AnnotationKind};
use crate::atomic;
use crate::markdown::{self, PreviewBlock};
use crate::pdf;
use crate::site::{DocumentRef, Language, SiteProfile, SECTIONS};
use crate::translation::{self, TranslationConfig};
use crate::tutorial::{self, TutorialMedia, TutorialStep};
use crate::video::VideoTool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkspaceTab {
    Edit,
    Tutorial,
    Translate,
    Export,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LanguageFilter {
    All,
    Zh,
    En,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TutorialTarget {
    NewDocument,
    CurrentDocument,
}

#[derive(Debug, Clone)]
struct DocumentImageEdit {
    reference: String,
    path: PathBuf,
}

pub struct DocforgeApp {
    site: Option<SiteProfile>,
    selected_document: Option<usize>,
    search: String,
    language_filter: LanguageFilter,
    markdown: String,
    saved_markdown: String,
    editor_line: usize,
    editor_column: usize,
    confirm_close: bool,
    workspace_tab: WorkspaceTab,
    status: String,
    translation: TranslationConfig,
    translation_result: Option<String>,
    translation_receiver: Option<Receiver<Result<String, String>>>,
    pdf_preview: Option<pdf::PdfPreview>,
    pdf_image: Option<egui::TextureHandle>,
    pdf_font: Option<PathBuf>,
    preview_textures: HashMap<String, egui::TextureHandle>,
    video_path: Option<PathBuf>,
    video_duration: Option<f64>,
    capture_seconds: f64,
    capture_count: usize,
    tutorial_steps: Vec<TutorialStep>,
    selected_frame: Option<usize>,
    frame_path: Option<PathBuf>,
    frame_texture: Option<egui::TextureHandle>,
    frame_size: [usize; 2],
    annotations: Vec<Annotation>,
    active_annotation: Option<Annotation>,
    annotation_tool: AnnotationKind,
    annotation_color: Color32,
    annotation_width: f32,
    video_temp: Option<tempfile::TempDir>,
    document_image_edit: Option<DocumentImageEdit>,
    tutorial_target: TutorialTarget,
    tutorial_title: String,
    tutorial_summary: String,
    tutorial_prerequisites: String,
    tutorial_file_stem: String,
    tutorial_section: usize,
    tutorial_language: Language,
}

impl DocforgeApp {
    pub fn new(
        creation_context: &eframe::CreationContext<'_>,
        initial_site: Option<PathBuf>,
    ) -> Self {
        install_chinese_font(&creation_context.egui_ctx);
        install_mas_theme(&creation_context.egui_ctx);
        let mut app = Self {
            site: None,
            selected_document: None,
            search: String::new(),
            language_filter: LanguageFilter::All,
            markdown: String::new(),
            saved_markdown: String::new(),
            editor_line: 1,
            editor_column: 1,
            confirm_close: false,
            workspace_tab: WorkspaceTab::Edit,
            status: "第一步：点击左上角“打开文档站”，选择 AUTO-MAS-docs 仓库。".to_string(),
            translation: TranslationConfig::default(),
            translation_result: None,
            translation_receiver: None,
            pdf_preview: None,
            pdf_image: None,
            pdf_font: None,
            preview_textures: HashMap::new(),
            video_path: None,
            video_duration: None,
            capture_seconds: 0.0,
            capture_count: 8,
            tutorial_steps: Vec::new(),
            selected_frame: None,
            frame_path: None,
            frame_texture: None,
            frame_size: [0, 0],
            annotations: Vec::new(),
            active_annotation: None,
            annotation_tool: AnnotationKind::Rectangle,
            annotation_color: Color32::from_rgb(232, 72, 85),
            annotation_width: 4.0,
            video_temp: None,
            document_image_edit: None,
            tutorial_target: TutorialTarget::NewDocument,
            tutorial_title: "视频操作教程".to_string(),
            tutorial_summary: "跟随下面的关键画面完成操作。".to_string(),
            tutorial_prerequisites: "准备好 AUTO-MAS，并确认相关功能可以正常打开。".to_string(),
            tutorial_file_stem: "video-tutorial".to_string(),
            tutorial_section: 0,
            tutorial_language: Language::Zh,
        };
        if let Some(root) = initial_site {
            app.open_site(root);
        }
        app
    }

    fn open_site(&mut self, root: PathBuf) {
        if self.is_dirty() {
            self.status = "当前文档还有未保存修改，请先保存或放弃修改。".to_string();
            return;
        }
        match SiteProfile::discover(&root) {
            Ok(site) => {
                let count = site.documents.len();
                let initial_document = site
                    .documents
                    .iter()
                    .position(|document| document.relative_path == "docs/user-guide.md")
                    .or_else(|| (!site.documents.is_empty()).then_some(0));
                self.status = format!(
                    "已打开 {}，找到 {count} 篇 Markdown 文档。",
                    site.root.display()
                );
                self.site = Some(site);
                self.selected_document = None;
                self.markdown.clear();
                self.saved_markdown.clear();
                self.translation_result = None;
                self.preview_textures.clear();
                self.document_image_edit = None;
                if let Some(index) = initial_document {
                    self.select_document(index);
                }
            }
            Err(error) => self.status = format!("无法打开文档站：{error:#}"),
        }
    }

    fn select_document(&mut self, index: usize) {
        if self.selected_document != Some(index) && self.is_dirty() {
            self.status =
                "当前文档还有未保存修改：请先保存，或点击“放弃修改”后再切换。".to_string();
            return;
        }
        let Some(site) = &self.site else {
            return;
        };
        let Some(document) = site.documents.get(index) else {
            return;
        };
        match site.document_path(document).and_then(|path| {
            fs::read_to_string(&path).with_context(|| format!("无法读取 {}", path.display()))
        }) {
            Ok(markdown) => {
                self.selected_document = Some(index);
                self.saved_markdown = markdown.clone();
                self.markdown = markdown;
                self.translation_result = None;
                self.pdf_preview = None;
                self.preview_textures.clear();
                self.document_image_edit = None;
                self.editor_line = 1;
                self.editor_column = 1;
                self.tutorial_language = document.language;
                if let Some(section) = SECTIONS
                    .iter()
                    .position(|section| *section == document.section)
                {
                    self.tutorial_section = section;
                }
                self.status = format!("正在编辑：{}", document.relative_path);
            }
            Err(error) => self.status = format!("无法打开文档：{error:#}"),
        }
    }

    fn selected_document_ref(&self) -> Option<&DocumentRef> {
        self.site.as_ref().and_then(|site| {
            self.selected_document
                .and_then(|index| site.documents.get(index))
        })
    }

    fn is_dirty(&self) -> bool {
        self.selected_document.is_some() && self.markdown != self.saved_markdown
    }

    fn discard_changes(&mut self) {
        self.markdown = self.saved_markdown.clone();
        self.translation_result = None;
        self.status = "已放弃尚未保存的修改。".to_string();
    }

    fn save_document(&mut self) {
        let Some(site) = &self.site else {
            self.status = "请先打开文档站。".to_string();
            return;
        };
        let Some(document) = self.selected_document_ref() else {
            self.status = "请先从左侧选择一篇 Markdown 文档。".to_string();
            return;
        };
        let mut did_save = false;
        match site
            .document_path(document)
            .and_then(|path| atomic::atomic_write(&path, self.markdown.as_bytes()))
        {
            Ok(()) => {
                did_save = true;
                self.status = format!("已原子保存：{}", document.relative_path);
            }
            Err(error) => self.status = format!("保存失败：{error:#}"),
        }
        if did_save {
            self.saved_markdown = self.markdown.clone();
        }
    }

    fn start_translation(&mut self) {
        if self.translation_receiver.is_some() {
            return;
        }
        let source = self.markdown.clone();
        let config = self.translation.clone();
        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            let result = translation::translate_markdown(&source, &config)
                .map_err(|error| format!("{error:#}"));
            let _ = sender.send(result);
        });
        self.translation_receiver = Some(receiver);
        self.status = "正在翻译，并保护 VitePress 语法、代码、链接和图片路径……".to_string();
    }

    fn poll_translation(&mut self) {
        let Some(receiver) = &self.translation_receiver else {
            return;
        };
        match receiver.try_recv() {
            Ok(Ok(markdown)) => {
                self.translation_result = Some(markdown);
                self.translation_receiver = None;
                self.status = "翻译完成：请先在校对区检查，当前尚未写入任何文件。".to_string();
            }
            Ok(Err(error)) => {
                self.translation_receiver = None;
                self.status = format!("翻译失败：{error}");
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                self.translation_receiver = None;
                self.status = "翻译任务意外停止，请检查接口配置后重试。".to_string();
            }
            Err(mpsc::TryRecvError::Empty) => {}
        }
    }

    fn save_translation_to_counterpart(&mut self) {
        let Some(translated) = &self.translation_result else {
            self.status = "请先翻译当前文档并检查译文。".to_string();
            return;
        };
        let Some(site) = &self.site else {
            return;
        };
        let Some(document) = self.selected_document_ref() else {
            return;
        };
        let path = match site.translation_target_path(document) {
            Ok(path) => path,
            Err(error) => {
                self.status = format!("无法确定翻译目标：{error:#}");
                return;
            }
        };
        let relative = path
            .strip_prefix(&site.root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        match atomic::ensure_inside(&site.root, &path)
            .and_then(|path| atomic::atomic_write(&path, translated.as_bytes()))
        {
            Ok(()) => self.status = format!("已原子保存对应语言文档：{relative}"),
            Err(error) => self.status = format!("无法保存对应语言文档：{error:#}"),
        }
    }

    fn export_pdf(&mut self, ctx: &egui::Context) {
        let suggested = self
            .selected_document_ref()
            .map(|document| safe_stem(&document.title))
            .unwrap_or_else(|| "auto-mas-document".to_string());
        let Some(path) = rfd::FileDialog::new()
            .set_file_name(format!("{suggested}.pdf"))
            .add_filter("PDF", &["pdf"])
            .save_file()
        else {
            return;
        };
        let document_context = self.site.as_ref().and_then(|site| {
            self.selected_document_ref()
                .and_then(|document| site.document_path(document).ok())
                .map(|document_path| (document_path, site.root.clone()))
        });
        let options = pdf::PdfExportOptions {
            custom_font: self.pdf_font.as_deref(),
            document_path: document_context.as_ref().map(|(path, _)| path.as_path()),
            site_root: document_context.as_ref().map(|(_, root)| root.as_path()),
        };
        match pdf::export_markdown_to_pdf(&self.markdown, &path, options).and_then(|info| {
            let preview = pdf::preview_pdf(&path)?;
            Ok((info, preview))
        }) {
            Ok((info, preview)) => {
                self.pdf_image = pdf::render_first_page(&info.path, None)
                    .ok()
                    .and_then(|bytes| load_texture_from_bytes(ctx, "pdf-preview", &bytes).ok());
                self.pdf_preview = Some(preview);
                self.status = match info.warning {
                    Some(warning) => format!("已导出 {}。{warning}", info.path.display()),
                    None => format!("已导出 {}（{} 页）。", info.path.display(), info.pages),
                };
            }
            Err(error) => self.status = format!("PDF 导出失败：{error:#}"),
        }
    }

    fn import_pdf(&mut self, ctx: &egui::Context) {
        if self.selected_document.is_none() {
            self.status = "请先选择要接收 PDF 内容的 Markdown 文档。".to_string();
            return;
        }
        if self.is_dirty() {
            self.status = "当前文档有未保存修改，请先保存或放弃修改，再导入 PDF。".to_string();
            return;
        }
        let Some(path) = rfd::FileDialog::new()
            .add_filter("PDF", &["pdf"])
            .pick_file()
        else {
            return;
        };
        match pdf::preview_pdf(&path).and_then(|preview| {
            let markdown = pdf::pdf_to_markdown(&path)?;
            Ok((preview, markdown))
        }) {
            Ok((preview, markdown)) => {
                self.markdown = markdown;
                self.pdf_preview = Some(preview);
                self.pdf_image = match pdf::render_first_page(&path, None) {
                    Ok(bytes) => load_texture_from_bytes(ctx, "pdf-preview", &bytes).ok(),
                    Err(_) => None,
                };
                self.status = format!(
                    "已把 {} 转为 Markdown 草稿。请检查排版后再原子保存。",
                    path.display()
                );
            }
            Err(error) => self.status = format!("PDF 导入失败：{error:#}"),
        }
    }

    fn choose_video(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("视频", &["mp4", "mkv", "mov", "webm", "avi"])
            .pick_file()
        else {
            return;
        };
        self.video_duration =
            VideoTool::discover(None).and_then(|tool| tool.duration_seconds(&path).ok().flatten());
        self.capture_seconds = 0.0;
        self.tutorial_steps.clear();
        self.selected_frame = None;
        self.frame_path = None;
        self.frame_texture = None;
        self.annotations.clear();
        self.document_image_edit = None;
        let video_name = path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("视频操作")
            .replace(['-', '_'], " ");
        self.tutorial_title = format!("{}教程", video_name.trim());
        self.tutorial_file_stem = safe_stem(&video_name);
        self.video_path = Some(path.clone());
        self.status = format!("已选择视频：{}", path.display());
    }

    fn temp_video_dir(&mut self) -> Result<PathBuf> {
        if self.video_temp.is_none() {
            self.video_temp = Some(tempfile::tempdir().context("无法创建视频截图临时目录")?);
        }
        Ok(self
            .video_temp
            .as_ref()
            .expect("视频临时目录应当已经创建")
            .path()
            .to_path_buf())
    }

    fn capture_manual_frame(&mut self, ctx: &egui::Context) {
        let Some(video) = self.video_path.clone() else {
            self.status = "请先选择一个视频。".to_string();
            return;
        };
        self.document_image_edit = None;
        let Some(tool) = VideoTool::discover(None) else {
            self.status =
                "未找到 FFmpeg：请把 ffmpeg.exe 和 ffprobe.exe 放入绿色包的 bin 目录。".to_string();
            return;
        };
        let output_dir = match self.temp_video_dir() {
            Ok(path) => path,
            Err(error) => {
                self.status = format!("无法准备截图临时目录：{error:#}");
                return;
            }
        };
        let output = output_dir.join(format!("manual-{:03}.png", self.tutorial_steps.len() + 1));
        match tool.capture_at(&video, self.capture_seconds, &output) {
            Ok(()) => {
                let index = self.tutorial_steps.len();
                self.tutorial_steps.push(TutorialStep::new(
                    output.clone(),
                    index,
                    Some(self.capture_seconds),
                ));
                self.open_frame(ctx, output);
                self.status = "截图完成：现在可以标注画面，或把它插入文档草稿。".to_string();
            }
            Err(error) => self.status = format!("截图失败：{error:#}"),
        }
    }

    fn capture_key_frames(&mut self, ctx: &egui::Context) {
        let Some(video) = self.video_path.clone() else {
            self.status = "请先选择一个视频。".to_string();
            return;
        };
        self.document_image_edit = None;
        let Some(tool) = VideoTool::discover(None) else {
            self.status =
                "未找到 FFmpeg：请把 ffmpeg.exe 和 ffprobe.exe 放入绿色包的 bin 目录。".to_string();
            return;
        };
        let output_dir = match self.temp_video_dir() {
            Ok(path) => path.join("keyframes"),
            Err(error) => {
                self.status = format!("无法准备截图临时目录：{error:#}");
                return;
            }
        };
        match tool.capture_keyframes(&video, &output_dir, self.capture_count) {
            Ok(frames) => {
                self.tutorial_steps = frames
                    .into_iter()
                    .enumerate()
                    .map(|(index, frame)| TutorialStep::new(frame, index, None))
                    .collect();
                if let Some(frame) = self.tutorial_steps.first().map(|step| step.frame.clone()) {
                    self.open_frame(ctx, frame);
                }
                self.status = format!(
                    "已自动截取 {} 张关键画面，可逐张编辑或一键生成文档段落。",
                    self.tutorial_steps.len()
                );
            }
            Err(error) => self.status = format!("关键画面识别失败：{error:#}"),
        }
    }

    fn capture_even_frames(&mut self, ctx: &egui::Context) {
        let Some(video) = self.video_path.clone() else {
            self.status = "请先选择视频。".to_string();
            return;
        };
        self.document_image_edit = None;
        let Some(tool) = VideoTool::discover(None) else {
            self.status =
                "未找到 FFmpeg：请把 ffmpeg.exe 和 ffprobe.exe 放在 DocForge.exe 同目录。"
                    .to_string();
            return;
        };
        let output_dir = match self.temp_video_dir() {
            Ok(path) => path.join("even-frames"),
            Err(error) => {
                self.status = format!("无法准备截图临时目录：{error:#}");
                return;
            }
        };
        match tool.capture_evenly(&video, &output_dir, self.capture_count) {
            Ok(frames) => {
                let duration = self.video_duration.unwrap_or_default();
                let frame_count = frames.len();
                self.tutorial_steps = frames
                    .into_iter()
                    .enumerate()
                    .map(|(index, frame)| {
                        let timestamp = (duration > 0.0)
                            .then(|| duration * (index + 1) as f64 / (frame_count + 1) as f64);
                        TutorialStep::new(frame, index, timestamp)
                    })
                    .collect();
                if let Some(frame) = self.tutorial_steps.first().map(|step| step.frame.clone()) {
                    self.open_frame(ctx, frame);
                }
                self.status = format!("已按时间均匀截取 {} 张画面。", self.tutorial_steps.len());
            }
            Err(error) => self.status = format!("均匀截图失败：{error:#}"),
        }
    }
    fn one_click_generate_from_video(&mut self, ctx: &egui::Context) {
        self.tutorial_steps.clear();
        self.capture_key_frames(ctx);
        if self.tutorial_steps.is_empty() {
            return;
        }
        self.generate_tutorial_document();
    }

    fn open_frame(&mut self, ctx: &egui::Context, path: PathBuf) {
        match load_texture_from_path(ctx, &path) {
            Ok((texture, size)) => {
                self.frame_path = Some(path.clone());
                self.frame_texture = Some(texture);
                self.frame_size = size;
                self.annotations.clear();
                self.active_annotation = None;
                self.selected_frame = self
                    .tutorial_steps
                    .iter()
                    .position(|step| step.frame == path);
            }
            Err(error) => self.status = format!("无法显示画面：{error:#}"),
        }
    }

    fn save_annotated_frame(&mut self, ctx: &egui::Context) {
        if self.document_image_edit.is_some() {
            self.save_document_image_edit(ctx);
            return;
        }
        let Some(frame) = self.frame_path.clone() else {
            self.status = "请先选择一张已经截取的画面。".to_string();
            return;
        };
        if self.annotations.is_empty() {
            self.status = "请先在画面上添加框选、箭头、涂画、遮挡或裁剪。".to_string();
            return;
        }
        let stem = frame
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("frame");
        let output = frame.with_file_name(format!("{stem}-annotated.png"));
        match annotation::save_annotated(&frame, &output, &self.annotations) {
            Ok(()) => {
                if let Some(index) = self.selected_frame {
                    if let Some(step) = self.tutorial_steps.get_mut(index) {
                        step.frame = output.clone();
                    }
                } else {
                    let index = self.tutorial_steps.len();
                    self.tutorial_steps
                        .push(TutorialStep::new(output.clone(), index, None));
                }
                self.open_frame(ctx, output);
                self.status = "已在临时目录中保存编辑后的画面。".to_string();
            }
            Err(error) => self.status = format!("无法保存编辑后的画面：{error:#}"),
        }
    }

    fn open_document_image(&mut self, ctx: &egui::Context, reference: String, path: PathBuf) {
        if !path.is_file() {
            self.status = format!("找不到文档图片：{}", path.display());
            return;
        }
        self.document_image_edit = Some(DocumentImageEdit {
            reference,
            path: path.clone(),
        });
        self.open_frame(ctx, path);
        self.workspace_tab = WorkspaceTab::Tutorial;
        self.status = "已从当前文档打开图片。标注后点击“保存副本并替换文档引用”。".to_string();
    }

    fn save_document_image_edit(&mut self, ctx: &egui::Context) {
        let Some(edit) = self.document_image_edit.clone() else {
            return;
        };
        if self.annotations.is_empty() {
            self.status = "请先添加框选、箭头、涂画、遮挡或裁剪。".to_string();
            return;
        }
        let Some(parent) = edit.path.parent() else {
            self.status = "无法确定图片保存目录。".to_string();
            return;
        };
        let output = parent.join(unique_media_name(&edit.path));
        match annotation::save_annotated(&edit.path, &output, &self.annotations) {
            Ok(()) => {
                let file_name = output
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or("edited-image.png");
                let replacement = replace_reference_file_name(&edit.reference, file_name);
                self.markdown = markdown::replace_image_reference(
                    &self.markdown,
                    &edit.reference,
                    &replacement,
                );
                self.preview_textures.clear();
                self.document_image_edit = Some(DocumentImageEdit {
                    reference: replacement,
                    path: output.clone(),
                });
                self.open_frame(ctx, output);
                self.status =
                    "图片副本已生成，当前 Markdown 引用已替换；点击顶部保存即可写入文档。"
                        .to_string();
            }
            Err(error) => self.status = format!("无法保存编辑后的文档图片：{error:#}"),
        }
    }

    fn insert_current_frame(&mut self, save_after: bool) {
        let Some(frame) = self.frame_path.clone() else {
            self.status = "请先选择一张已经截取的画面。".to_string();
            return;
        };
        let Some(site) = &self.site else {
            self.status = "插入画面前，请先打开文档站。".to_string();
            return;
        };
        let Some(document) = self.selected_document_ref() else {
            self.status = "插入画面前，请先选择一篇 Markdown 文档。".to_string();
            return;
        };
        let media_dir = match site.media_dir(document) {
            Ok(path) => path,
            Err(error) => {
                self.status = format!("无法确定文档图片目录：{error:#}");
                return;
            }
        };
        let destination = media_dir.join(unique_media_name(&frame));
        match atomic::copy_atomic(&frame, &destination) {
            Ok(()) => {
                let relative = destination
                    .strip_prefix(&site.root)
                    .unwrap_or(&destination)
                    .to_string_lossy()
                    .replace('\\', "/");
                let public_path = format!("/{relative}");
                self.markdown = markdown::append_media(
                    &self.markdown,
                    "视频操作记录",
                    &[("视频关键画面".to_string(), public_path)],
                );
                self.status = format!("已把画面插入当前草稿：{}", destination.display());
                if save_after {
                    self.save_document();
                }
            }
            Err(error) => self.status = format!("无法把画面复制到文档站：{error:#}"),
        }
    }

    fn insert_all_frames_and_save(&mut self) {
        if self.tutorial_steps.is_empty() {
            self.status = "请先从视频截取关键画面。".to_string();
            return;
        }
        let Some(site) = &self.site else {
            self.status = "生成文档段落前，请先打开文档站。".to_string();
            return;
        };
        let Some(document) = self.selected_document_ref() else {
            self.status = "生成文档段落前，请先选择一篇 Markdown 文档。".to_string();
            return;
        };
        let media_dir = match site.media_dir(document) {
            Ok(path) => path,
            Err(error) => {
                self.status = format!("无法确定文档图片目录：{error:#}");
                return;
            }
        };
        let mut media = Vec::new();
        for step in &self.tutorial_steps {
            let destination = media_dir.join(unique_media_name(&step.frame));
            if let Err(error) = atomic::copy_atomic(&step.frame, &destination) {
                self.status = format!("无法复制画面 {}：{error:#}", step.frame.display());
                return;
            }
            let relative = destination
                .strip_prefix(&site.root)
                .unwrap_or(&destination)
                .to_string_lossy()
                .replace('\\', "/");
            media.push(TutorialMedia {
                title: step.title.clone(),
                description: step.description.clone(),
                public_path: format!("/{relative}"),
                timestamp: step.timestamp,
            });
        }
        self.markdown.push_str(&tutorial::render_section(
            &self.tutorial_title,
            &self.tutorial_summary,
            &media,
        ));
        self.save_document();
    }

    fn generate_tutorial_document(&mut self) {
        if self.tutorial_target == TutorialTarget::CurrentDocument {
            self.insert_all_frames_and_save();
            return;
        }
        if self.is_dirty() {
            self.status = "当前文档还有未保存修改，请先保存，再生成新的教程文件。".to_string();
            return;
        }
        if self.tutorial_steps.is_empty() {
            self.status = "请先自动识别关键画面，或使用“一键生成教程”。".to_string();
            return;
        }
        let Some(site) = &self.site else {
            self.status = "请先打开 AUTO-MAS 文档站。".to_string();
            return;
        };
        let section = SECTIONS
            .get(self.tutorial_section)
            .copied()
            .unwrap_or("docs");
        let target = match site.tutorial_document_path(
            self.tutorial_language,
            section,
            &self.tutorial_file_stem,
        ) {
            Ok(path) => path,
            Err(error) => {
                self.status = format!("无法确定教程文件位置：{error:#}");
                return;
            }
        };
        if target.exists() {
            self.status = format!(
                "教程文件已经存在：{}。请修改文件名，或选择“追加到当前文档”。",
                target.display()
            );
            return;
        }
        let media_dir = match site.media_dir_for_section(section) {
            Ok(path) => path,
            Err(error) => {
                self.status = format!("无法准备教程图片目录：{error:#}");
                return;
            }
        };
        let mut media = Vec::new();
        for step in &self.tutorial_steps {
            let destination = media_dir.join(unique_media_name(&step.frame));
            if let Err(error) = atomic::copy_atomic(&step.frame, &destination) {
                self.status = format!("无法复制教程画面 {}：{error:#}", step.frame.display());
                return;
            }
            let relative = destination
                .strip_prefix(&site.root)
                .unwrap_or(&destination)
                .to_string_lossy()
                .replace('\\', "/");
            media.push(TutorialMedia {
                title: step.title.clone(),
                description: step.description.clone(),
                public_path: format!("/{relative}"),
                timestamp: step.timestamp,
            });
        }
        let source = tutorial::render_document(
            &self.tutorial_title,
            &self.tutorial_summary,
            &self.tutorial_prerequisites,
            &media,
        );
        if let Err(error) = atomic::atomic_write(&target, source.as_bytes()) {
            self.status = format!("无法原子保存教程文档：{error:#}");
            return;
        }
        let root = site.root.clone();
        let relative = target
            .strip_prefix(&root)
            .unwrap_or(&target)
            .to_string_lossy()
            .replace('\\', "/");
        match SiteProfile::discover(&root) {
            Ok(profile) => {
                self.site = Some(profile);
                self.selected_document = self.site.as_ref().and_then(|profile| {
                    profile
                        .documents
                        .iter()
                        .position(|document| document.relative_path == relative)
                });
                self.saved_markdown = source.clone();
                self.markdown = source;
                self.pdf_preview = None;
                self.pdf_image = None;
                self.preview_textures.clear();
                self.document_image_edit = None;
                self.editor_line = 1;
                self.editor_column = 1;
                self.workspace_tab = WorkspaceTab::Edit;
                self.status = format!("教程已生成并打开：{relative}");
            }
            Err(error) => {
                self.status = format!("教程已保存，但重新扫描文档站失败：{error:#}");
            }
        }
    }

    fn draw_annotation_canvas(&mut self, ui: &mut egui::Ui) {
        let Some(texture) = &self.frame_texture else {
            ui.label("请先从左侧选择一张画面，然后在这里进行编辑。");
            return;
        };
        let source_size = texture.size();
        if source_size[0] == 0 || source_size[1] == 0 {
            return;
        }
        let available = ui.available_size_before_wrap();
        let aspect = source_size[0] as f32 / source_size[1] as f32;
        let mut width = available.x.clamp(220.0, 760.0);
        let mut height = width / aspect;
        if height > available.y.max(220.0) {
            height = available.y.max(220.0);
            width = height * aspect;
        }
        let (response, painter) = ui.allocate_painter(Vec2::new(width, height), Sense::drag());
        let image_rect = response.rect;
        painter.image(
            texture.id(),
            image_rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
        for annotation in self.annotations.iter().chain(self.active_annotation.iter()) {
            draw_annotation_preview(&painter, image_rect, annotation);
        }

        let pointer = response.interact_pointer_pos();
        if response.drag_started() {
            if let Some(pointer) = pointer {
                let mut annotation =
                    Annotation::new(self.annotation_tool, normalize_pointer(pointer, image_rect));
                annotation.color = self.annotation_color.to_array();
                annotation.width = self.annotation_width;
                self.active_annotation = Some(annotation);
            }
        }
        if response.dragged() {
            if let (Some(pointer), Some(annotation)) = (pointer, self.active_annotation.as_mut()) {
                let point = normalize_pointer(pointer, image_rect);
                if matches!(
                    annotation.kind,
                    AnnotationKind::Brush | AnnotationKind::Mosaic
                ) || annotation.points.len() == 1
                {
                    annotation.points.push(point);
                } else {
                    annotation.points[1] = point;
                }
            }
        }
        if response.drag_stopped() {
            if let Some(annotation) = self.active_annotation.take() {
                if annotation.points.len() >= 2 {
                    self.annotations.push(annotation);
                }
            }
        }
    }

    fn ui_document_list(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("资源管理器").size(16.0).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let count = self
                    .site
                    .as_ref()
                    .map(|site| site.documents.len())
                    .unwrap_or_default();
                ui.label(RichText::new(format!("{count} 篇")).color(MUTED_TEXT));
            });
        });
        ui.add_space(8.0);
        ui.add(
            egui::TextEdit::singleline(&mut self.search)
                .hint_text("搜索标题或路径…")
                .desired_width(f32::INFINITY),
        );
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.language_filter, LanguageFilter::All, "全部");
            ui.selectable_value(&mut self.language_filter, LanguageFilter::Zh, "中文");
            ui.selectable_value(&mut self.language_filter, LanguageFilter::En, "EN");
        });
        ui.add_space(8.0);

        let documents = self
            .site
            .as_ref()
            .map(|site| site.documents.clone())
            .unwrap_or_default();
        egui::ScrollArea::vertical()
            .id_salt("document-explorer")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for section in [
                    "index",
                    "docs",
                    "developer",
                    "plugin",
                    "download",
                    "disclosure",
                ] {
                    let visible = documents
                        .iter()
                        .enumerate()
                        .filter(|(_, document)| {
                            document.section == section
                                && matches_language(document.language, self.language_filter)
                                && matches_search(document, &self.search)
                        })
                        .collect::<Vec<_>>();
                    if visible.is_empty() {
                        continue;
                    }
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(section_label(section))
                            .size(11.0)
                            .strong()
                            .color(MUTED_TEXT),
                    );
                    ui.add_space(3.0);
                    for (index, document) in visible {
                        let selected = self.selected_document == Some(index);
                        let fill = if selected {
                            SELECTED_BG
                        } else {
                            Color32::TRANSPARENT
                        };
                        let response = egui::Frame::new()
                            .fill(fill)
                            .corner_radius(6)
                            .inner_margin(egui::Margin::symmetric(8, 6))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let language_color = if document.language == Language::Zh {
                                        BRAND_CYAN
                                    } else {
                                        BRAND_PURPLE
                                    };
                                    ui.colored_label(
                                        language_color,
                                        RichText::new(if document.language == Language::Zh {
                                            "中"
                                        } else {
                                            "EN"
                                        })
                                        .size(10.0)
                                        .strong(),
                                    );
                                    ui.vertical(|ui| {
                                        ui.label(RichText::new(&document.title).strong());
                                        ui.label(
                                            RichText::new(&document.relative_path)
                                                .size(10.0)
                                                .color(MUTED_TEXT),
                                        );
                                    });
                                });
                            })
                            .response
                            .interact(Sense::click());
                        if response.clicked() {
                            self.select_document(index);
                        }
                    }
                }
            });
    }

    fn ui_workspace(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if self.site.is_none() {
            self.ui_welcome(ui);
            return;
        }
        match self.workspace_tab {
            WorkspaceTab::Edit => self.ui_editor_workspace(ctx, ui),
            WorkspaceTab::Tutorial => self.ui_tutorial_workspace(ctx, ui),
            WorkspaceTab::Translate => self.ui_translation_workspace(ui),
            WorkspaceTab::Export => self.ui_export_workspace(ctx, ui),
        }
    }

    fn ui_welcome(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            egui::Frame::new()
                .fill(PANEL_BG)
                .stroke(Stroke::new(1.0, BORDER_COLOR))
                .corner_radius(14)
                .inner_margin(egui::Margin::same(28))
                .show(ui, |ui| {
                    ui.set_max_width(620.0);
                    ui.label(RichText::new("AUTO-MAS DocForge").size(30.0).strong());
                    ui.label(
                        RichText::new("把视频、截图和 Markdown 变成可直接发布的图文教程")
                            .size(16.0)
                            .color(MUTED_TEXT),
                    );
                    ui.add_space(18.0);
                    for (number, title, description) in [
                        ("1", "打开文档站", "选择 AUTO-MAS-docs 仓库根目录"),
                        ("2", "编辑或导入视频", "同屏预览，逐张整理关键步骤"),
                        ("3", "原子保存与导出", "生成 Markdown 教程和带图片的 PDF"),
                    ] {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(number).size(18.0).strong().color(BRAND_CYAN));
                            ui.vertical(|ui| {
                                ui.label(RichText::new(title).strong());
                                ui.label(RichText::new(description).color(MUTED_TEXT));
                            });
                        });
                        ui.add_space(10.0);
                    }
                    if ui.add(primary_button("选择 AUTO-MAS-docs 目录")).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.open_site(path);
                        }
                    }
                });
        });
    }

    fn ui_editor_workspace(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.ui_document_header(ui, "Markdown 编辑器", "编辑与预览始终保持同步");
        ui.add_space(8.0);
        ui.columns(2, |columns| {
            panel_frame().show(&mut columns[0], |ui| {
                self.ui_code_editor(ui);
            });
            panel_frame().show(&mut columns[1], |ui| {
                self.ui_markdown_preview(ctx, ui);
            });
        });
    }

    fn ui_document_header(&mut self, ui: &mut egui::Ui, title: &str, subtitle: &str) {
        let document = self.selected_document_ref().cloned();
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(title).size(20.0).strong());
                if let Some(document) = &document {
                    ui.label(
                        RichText::new(format!("{}  /  {}", document.title, document.relative_path))
                            .color(MUTED_TEXT),
                    );
                } else {
                    ui.label(RichText::new(subtitle).color(MUTED_TEXT));
                }
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add(primary_button("保存文档  Ctrl+S")).clicked() {
                    self.save_document();
                }
                if ui
                    .add_enabled(self.is_dirty(), egui::Button::new("撤销未保存更改"))
                    .clicked()
                {
                    self.discard_changes();
                }
                if self.is_dirty() {
                    ui.label(RichText::new("● 未保存").color(WARNING_COLOR));
                } else if self.selected_document.is_some() {
                    ui.label(RichText::new("✓ 已保存").color(SUCCESS_COLOR));
                }
            });
        });
    }

    fn ui_code_editor(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("编辑").strong().color(BRAND_CYAN));
            ui.label(RichText::new("MARKDOWN").size(10.0).color(MUTED_TEXT));
        });
        ui.separator();

        let line_count = self.markdown.lines().count().max(1);
        let line_numbers = (1..=line_count)
            .map(|line| format!("{line:>4}"))
            .collect::<Vec<_>>()
            .join("\n");
        let editor_height = (ui.available_height() - 28.0).max(260.0);
        egui::Frame::new()
            .fill(EDITOR_BG)
            .corner_radius(6)
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("markdown-code-editor")
                    .max_height(editor_height)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.horizontal_top(|ui| {
                            ui.add(
                                egui::Label::new(
                                    RichText::new(line_numbers)
                                        .monospace()
                                        .size(13.0)
                                        .color(LINE_NUMBER_COLOR),
                                )
                                .selectable(false),
                            );
                            ui.separator();
                            let output = egui::TextEdit::multiline(&mut self.markdown)
                                .code_editor()
                                .font(egui::TextStyle::Monospace)
                                .desired_rows(line_count.max(34))
                                .desired_width((ui.available_width() - 8.0).max(260.0))
                                .lock_focus(true)
                                .frame(false)
                                .show(ui);
                            if let Some(cursor) = output.cursor_range {
                                (self.editor_line, self.editor_column) =
                                    cursor_position(&self.markdown, cursor.primary.index);
                            }
                        });
                    });
            });
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!(
                    "Ln {}, Col {}    UTF-8    Markdown",
                    self.editor_line, self.editor_column
                ))
                .size(10.0)
                .color(MUTED_TEXT),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("{} 行", line_count))
                        .size(10.0)
                        .color(MUTED_TEXT),
                );
            });
        });
    }

    fn ui_markdown_preview(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("实时预览").strong().color(BRAND_PURPLE));
            ui.label(RichText::new("VITEPRESS").size(10.0).color(MUTED_TEXT));
        });
        ui.separator();

        let preview_paths = self.site.as_ref().and_then(|site| {
            self.selected_document
                .and_then(|index| site.documents.get(index))
                .and_then(|document| site.document_path(document).ok())
                .map(|document_path| (site.root.clone(), document_path))
        });
        if let Some((site_root, document_path)) = &preview_paths {
            ui.horizontal_wrapped(|ui| {
                for check in markdown::check_document(&self.markdown, document_path, site_root) {
                    let (symbol, color) = match check.level {
                        markdown::CheckLevel::Info => ("✓", SUCCESS_COLOR),
                        markdown::CheckLevel::Warning => ("!", WARNING_COLOR),
                        markdown::CheckLevel::Error => ("×", ERROR_COLOR),
                    };
                    ui.colored_label(color, format!("{symbol} {}", check.message));
                }
            });
            ui.separator();
        }

        let blocks = markdown::preview_blocks(&self.markdown);
        let mut edit_request: Option<(String, PathBuf)> = None;
        egui::ScrollArea::vertical()
            .id_salt("live-preview")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_max_width(820.0);
                for block in blocks {
                    match block {
                        PreviewBlock::Heading { level, text } => {
                            let size = match level {
                                1 => 28.0,
                                2 => 22.0,
                                3 => 18.0,
                                _ => 16.0,
                            };
                            ui.add_space(if level <= 2 { 14.0 } else { 8.0 });
                            ui.label(
                                RichText::new(markdown::inline_preview_text(&text))
                                    .size(size)
                                    .strong(),
                            );
                        }
                        PreviewBlock::Paragraph(text) => {
                            ui.add_space(5.0);
                            ui.label(
                                RichText::new(markdown::inline_preview_text(&text)).size(14.0),
                            );
                        }
                        PreviewBlock::Code(text) => {
                            egui::Frame::new()
                                .fill(CODE_BLOCK_BG)
                                .stroke(Stroke::new(1.0, BORDER_COLOR))
                                .corner_radius(6)
                                .inner_margin(egui::Margin::same(10))
                                .show(ui, |ui| {
                                    ui.label(RichText::new(text).monospace().color(CODE_TEXT));
                                });
                        }
                        PreviewBlock::Image { alt, source } => {
                            ui.add_space(8.0);
                            egui::Frame::new()
                                .fill(CARD_BG)
                                .stroke(Stroke::new(1.0, BORDER_COLOR))
                                .corner_radius(8)
                                .inner_margin(egui::Margin::same(10))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(&alt).strong());
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                if let Some((site_root, document_path)) =
                                                    &preview_paths
                                                {
                                                    let path = markdown::resolve_image_path(
                                                        &source,
                                                        document_path,
                                                        site_root,
                                                    );
                                                    if path.is_file()
                                                        && ui
                                                            .add(
                                                                egui::Button::new(
                                                                    "框选 / 箭头 / 涂抹",
                                                                )
                                                                .fill(SELECTED_BG),
                                                            )
                                                            .clicked()
                                                    {
                                                        edit_request = Some((source.clone(), path));
                                                    }
                                                }
                                            },
                                        );
                                    });
                                    if let Some((site_root, document_path)) = &preview_paths {
                                        let path = markdown::resolve_image_path(
                                            &source,
                                            document_path,
                                            site_root,
                                        );
                                        let key = path.to_string_lossy().to_string();
                                        if !self.preview_textures.contains_key(&key) {
                                            if let Ok((texture, _)) =
                                                load_texture_from_path(ctx, &path)
                                            {
                                                self.preview_textures.insert(key.clone(), texture);
                                            }
                                        }
                                        if let Some(texture) = self.preview_textures.get(&key) {
                                            let size = texture.size_vec2();
                                            let max_width = ui.available_width().min(760.0);
                                            let scale =
                                                (max_width / size.x).min(480.0 / size.y).min(1.0);
                                            ui.image((texture.id(), size * scale));
                                        } else {
                                            ui.colored_label(
                                                ERROR_COLOR,
                                                format!("无法显示：{}", path.display()),
                                            );
                                        }
                                    }
                                    ui.label(RichText::new(&source).size(10.0).color(MUTED_TEXT));
                                });
                        }
                        PreviewBlock::Callout { kind, text } => {
                            egui::Frame::new()
                                .fill(CALLOUT_BG)
                                .stroke(Stroke::new(1.0, BRAND_PURPLE))
                                .corner_radius(7)
                                .inner_margin(egui::Margin::same(10))
                                .show(ui, |ui| {
                                    ui.label(RichText::new(kind).strong().color(BRAND_CYAN));
                                    ui.label(markdown::inline_preview_text(&text));
                                });
                        }
                        PreviewBlock::Rule => {
                            ui.add_space(6.0);
                            ui.separator();
                        }
                    }
                }
            });
        if let Some((reference, path)) = edit_request {
            self.open_document_image(ctx, reference, path);
        }
    }

    fn ui_translation_workspace(&mut self, ui: &mut egui::Ui) {
        self.ui_document_header(ui, "多语言翻译", "生成草稿、人工校对、再原子保存");
        ui.add_space(8.0);
        ui.columns(2, |columns| {
            panel_frame().show(&mut columns[0], |ui| {
                ui.label(RichText::new("翻译设置").size(17.0).strong());
                ui.label(
                    RichText::new("兼容 OpenAI /chat/completions，密钥只保存在本次运行内存。")
                        .color(MUTED_TEXT),
                );
                ui.add_space(10.0);
                field_label(ui, "接口地址");
                ui.add(
                    egui::TextEdit::singleline(&mut self.translation.endpoint)
                        .hint_text("https://api.example.com/v1")
                        .desired_width(f32::INFINITY),
                );
                field_label(ui, "模型");
                ui.add(
                    egui::TextEdit::singleline(&mut self.translation.model)
                        .hint_text("模型名称")
                        .desired_width(f32::INFINITY),
                );
                field_label(ui, "API Key");
                ui.add(
                    egui::TextEdit::singleline(&mut self.translation.api_key)
                        .password(true)
                        .hint_text("不会写入磁盘")
                        .desired_width(f32::INFINITY),
                );
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        field_label(ui, "源语言");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.translation.source_language)
                                .desired_width(120.0),
                        );
                    });
                    ui.vertical(|ui| {
                        field_label(ui, "目标语言");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.translation.target_language)
                                .desired_width(120.0),
                        );
                    });
                });
                ui.add_space(14.0);
                let translating = self.translation_receiver.is_some();
                if ui
                    .add_enabled(
                        !translating && self.selected_document.is_some(),
                        primary_button(if translating {
                            "正在翻译…"
                        } else {
                            "生成翻译草稿"
                        }),
                    )
                    .clicked()
                {
                    self.start_translation();
                }
            });

            panel_frame().show(&mut columns[1], |ui| {
                ui.label(RichText::new("译文校对").size(17.0).strong());
                ui.label(
                    RichText::new("只有点击保存后，译文才会写入对应语言文件。").color(MUTED_TEXT),
                );
                ui.add_space(10.0);
                let mut use_result = false;
                let mut save_counterpart = false;
                if let Some(result) = &mut self.translation_result {
                    ui.add(
                        egui::TextEdit::multiline(result)
                            .code_editor()
                            .desired_rows(28)
                            .desired_width(f32::INFINITY),
                    );
                    ui.horizontal(|ui| {
                        use_result = ui.button("替换当前编辑草稿").clicked();
                        save_counterpart = ui.add(primary_button("保存到对应语言文件")).clicked();
                    });
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new("译文会显示在这里\n\n先在左侧完成接口配置")
                                .color(MUTED_TEXT),
                        );
                    });
                }
                if use_result {
                    if let Some(result) = &self.translation_result {
                        self.markdown = result.clone();
                        self.status = "译文已复制到当前编辑草稿，尚未保存。".to_string();
                    }
                }
                if save_counterpart {
                    self.save_translation_to_counterpart();
                }
            });
        });
    }

    fn ui_export_workspace(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.ui_document_header(
            ui,
            "导出当前文档",
            "导出内容始终取自左侧正在编辑的 Markdown",
        );
        ui.add_space(8.0);
        ui.columns(2, |columns| {
            panel_frame().show(&mut columns[0], |ui| {
                ui.label(RichText::new("PDF 导出").size(20.0).strong());
                ui.label(
                    RichText::new("标题、正文和当前文档中的本地图片都会写入 PDF。")
                        .color(MUTED_TEXT),
                );
                ui.add_space(14.0);
                if let Some(document) = self.selected_document_ref() {
                    info_row(ui, "当前文档", &document.title);
                    info_row(ui, "文件位置", &document.relative_path);
                    info_row(
                        ui,
                        "内容状态",
                        if self.is_dirty() {
                            "包含尚未保存的编辑内容"
                        } else {
                            "与磁盘文件一致"
                        },
                    );
                } else {
                    ui.colored_label(WARNING_COLOR, "请先从资源管理器选择一篇文档。");
                }
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.button("选择 PDF 字体").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("字体", &["ttf", "otf"])
                            .pick_file()
                        {
                            self.pdf_font = Some(path);
                        }
                    }
                    ui.label(
                        RichText::new(
                            self.pdf_font
                                .as_ref()
                                .and_then(|path| path.file_name())
                                .and_then(|name| name.to_str())
                                .unwrap_or("自动使用 Windows 中文字体"),
                        )
                        .color(MUTED_TEXT),
                    );
                });
                ui.add_space(14.0);
                if ui
                    .add_enabled(
                        self.selected_document.is_some(),
                        primary_button("导出当前编辑内容为 PDF"),
                    )
                    .clicked()
                {
                    self.export_pdf(ctx);
                }
                ui.add_space(8.0);
                if ui.button("从 PDF 导入为当前 Markdown 草稿").clicked() {
                    self.import_pdf(ctx);
                }
            });
            panel_frame().show(&mut columns[1], |ui| {
                self.ui_pdf_preview(ui);
            });
        });
    }

    fn ui_pdf_preview(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("PDF 页面预览").size(17.0).strong());
        let Some(preview) = &self.pdf_preview else {
            ui.centered_and_justified(|ui| {
                ui.label(
                    RichText::new("导出后会在这里显示 PDF 首页\n并标出页数与保存位置")
                        .color(MUTED_TEXT),
                );
            });
            return;
        };
        ui.label(
            RichText::new(format!(
                "{} 页  ·  {}",
                preview.pages,
                preview.path.display()
            ))
            .color(MUTED_TEXT),
        );
        ui.separator();
        egui::ScrollArea::vertical()
            .id_salt("pdf-preview")
            .show(ui, |ui| {
                if let Some(texture) = &self.pdf_image {
                    let size = texture.size_vec2();
                    let scale = (ui.available_width() / size.x).min(680.0 / size.y).min(1.0);
                    egui::Frame::new()
                        .fill(Color32::WHITE)
                        .inner_margin(egui::Margin::same(8))
                        .show(ui, |ui| {
                            ui.image((texture.id(), size * scale));
                        });
                }
                ui.add_space(8.0);
                ui.collapsing("查看 PDF 提取文字", |ui| {
                    ui.label(&preview.text);
                });
            });
    }

    fn ui_tutorial_workspace(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("视频教程工坊").size(20.0).strong());
                ui.label(
                    RichText::new("导入视频 → 自动截取 → 编辑步骤 → 生成站点教程")
                        .color(MUTED_TEXT),
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_enabled(
                        self.video_path.is_some(),
                        primary_button("一键截图并生成教程"),
                    )
                    .clicked()
                {
                    self.one_click_generate_from_video(ctx);
                }
            });
        });
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            step_chip(ui, "1", "导入视频", self.video_path.is_some());
            step_chip(ui, "2", "关键画面", !self.tutorial_steps.is_empty());
            step_chip(
                ui,
                "3",
                "编辑与说明",
                self.tutorial_steps
                    .iter()
                    .any(|step| !step.description.trim().is_empty()),
            );
            step_chip(ui, "4", "生成文档", false);
        });
        ui.add_space(8.0);

        ui.columns(2, |columns| {
            egui::ScrollArea::vertical()
                .id_salt("tutorial-workflow")
                .auto_shrink([false, false])
                .show(&mut columns[0], |ui| {
                    panel_frame().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("① 视频来源").size(16.0).strong());
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("选择视频…").clicked() {
                                        self.choose_video();
                                    }
                                },
                            );
                        });
                        ui.label(
                            RichText::new(
                                self.video_path
                                    .as_ref()
                                    .map(|path| path.display().to_string())
                                    .unwrap_or_else(|| "尚未选择视频".to_string()),
                            )
                            .color(MUTED_TEXT),
                        );
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                self.video_duration
                                    .map(|seconds| format!("时长 {seconds:.1} 秒"))
                                    .unwrap_or_else(|| "等待读取时长".to_string()),
                            );
                            ui.separator();
                            ui.label("画面数量");
                            ui.add(egui::DragValue::new(&mut self.capture_count).range(1..=50));
                        });
                        if let Some(duration) = self.video_duration {
                            ui.add(
                                egui::Slider::new(
                                    &mut self.capture_seconds,
                                    0.0..=duration.max(0.1),
                                )
                                .text("自由截图位置"),
                            );
                        }
                        ui.horizontal_wrapped(|ui| {
                            if ui
                                .add_enabled(
                                    self.video_path.is_some(),
                                    egui::Button::new("自动识别关键画面"),
                                )
                                .clicked()
                            {
                                self.capture_key_frames(ctx);
                            }
                            if ui
                                .add_enabled(
                                    self.video_path.is_some(),
                                    egui::Button::new("均匀截取"),
                                )
                                .clicked()
                            {
                                self.capture_even_frames(ctx);
                            }
                            if ui
                                .add_enabled(
                                    self.video_path.is_some(),
                                    egui::Button::new("截取当前位置"),
                                )
                                .clicked()
                            {
                                self.capture_manual_frame(ctx);
                            }
                        });
                    });
                    ui.add_space(8.0);

                    panel_frame().show(ui, |ui| {
                        ui.label(RichText::new("② 教程信息与位置").size(16.0).strong());
                        ui.horizontal(|ui| {
                            ui.selectable_value(
                                &mut self.tutorial_target,
                                TutorialTarget::NewDocument,
                                "生成新教程文件",
                            );
                            ui.selectable_value(
                                &mut self.tutorial_target,
                                TutorialTarget::CurrentDocument,
                                "追加到当前文档",
                            );
                        });
                        field_label(ui, "教程标题");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.tutorial_title)
                                .desired_width(f32::INFINITY),
                        );
                        field_label(ui, "教程简介");
                        ui.add(
                            egui::TextEdit::multiline(&mut self.tutorial_summary)
                                .desired_rows(2)
                                .desired_width(f32::INFINITY),
                        );
                        if self.tutorial_target == TutorialTarget::NewDocument {
                            field_label(ui, "开始前准备");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.tutorial_prerequisites)
                                    .desired_width(f32::INFINITY),
                            );
                            ui.horizontal(|ui| {
                                egui::ComboBox::from_label("语言")
                                    .selected_text(self.tutorial_language.label())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut self.tutorial_language,
                                            Language::Zh,
                                            "中文",
                                        );
                                        ui.selectable_value(
                                            &mut self.tutorial_language,
                                            Language::En,
                                            "English",
                                        );
                                    });
                                egui::ComboBox::from_label("栏目")
                                    .selected_text(
                                        SECTIONS
                                            .get(self.tutorial_section)
                                            .copied()
                                            .unwrap_or("docs"),
                                    )
                                    .show_ui(ui, |ui| {
                                        for (index, section) in SECTIONS.iter().enumerate() {
                                            ui.selectable_value(
                                                &mut self.tutorial_section,
                                                index,
                                                section_label(section),
                                            );
                                        }
                                    });
                            });
                            field_label(ui, "文件名");
                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.tutorial_file_stem)
                                        .desired_width(240.0),
                                );
                                ui.label(".md");
                            });
                        }
                        ui.add_space(10.0);
                        if ui
                            .add_enabled(
                                !self.tutorial_steps.is_empty(),
                                primary_button(
                                    if self.tutorial_target == TutorialTarget::NewDocument {
                                        "生成并打开教程文档"
                                    } else {
                                        "追加教程并原子保存"
                                    },
                                ),
                            )
                            .clicked()
                        {
                            self.generate_tutorial_document();
                        }
                    });
                    ui.add_space(8.0);

                    ui.label(
                        RichText::new(format!(
                            "③ 教程步骤（{} 张画面）",
                            self.tutorial_steps.len()
                        ))
                        .size(16.0)
                        .strong(),
                    );
                    let mut frame_to_open = None;
                    let mut remove_step = None;
                    for index in 0..self.tutorial_steps.len() {
                        let frame = self.tutorial_steps[index].frame.clone();
                        let key = format!("tutorial-thumbnail-{}", frame.display());
                        if !self.preview_textures.contains_key(&key) {
                            if let Ok((texture, _)) = load_texture_from_path(ctx, &frame) {
                                self.preview_textures.insert(key.clone(), texture);
                            }
                        }
                        let texture = self.preview_textures.get(&key).cloned();
                        let selected = self.selected_frame == Some(index)
                            && self.document_image_edit.is_none();
                        egui::Frame::new()
                            .fill(if selected { SELECTED_BG } else { CARD_BG })
                            .stroke(Stroke::new(1.0, BORDER_COLOR))
                            .corner_radius(8)
                            .inner_margin(egui::Margin::same(9))
                            .show(ui, |ui| {
                                ui.horizontal_top(|ui| {
                                    if let Some(texture) = texture {
                                        let size = texture.size_vec2();
                                        let scale = (116.0 / size.x).min(78.0 / size.y).min(1.0);
                                        if ui
                                            .add(egui::Button::image((texture.id(), size * scale)))
                                            .clicked()
                                        {
                                            frame_to_open = Some(frame.clone());
                                        }
                                    }
                                    let step = &mut self.tutorial_steps[index];
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                RichText::new(format!("步骤 {}", index + 1))
                                                    .strong()
                                                    .color(BRAND_CYAN),
                                            );
                                            if let Some(seconds) = step.timestamp {
                                                ui.label(
                                                    RichText::new(format_time(seconds))
                                                        .size(10.0)
                                                        .color(MUTED_TEXT),
                                                );
                                            }
                                            if ui.small_button("移除").clicked() {
                                                remove_step = Some(index);
                                            }
                                        });
                                        ui.add(
                                            egui::TextEdit::singleline(&mut step.title)
                                                .hint_text("步骤标题")
                                                .desired_width(f32::INFINITY),
                                        );
                                        ui.add(
                                            egui::TextEdit::multiline(&mut step.description)
                                                .hint_text("说明用户在这一步要做什么")
                                                .desired_rows(2)
                                                .desired_width(f32::INFINITY),
                                        );
                                    });
                                });
                            });
                        ui.add_space(6.0);
                    }
                    if let Some(index) = remove_step {
                        self.tutorial_steps.remove(index);
                        self.selected_frame = None;
                    }
                    if let Some(frame) = frame_to_open {
                        self.document_image_edit = None;
                        self.open_frame(ctx, frame);
                    }
                });

            panel_frame().show(&mut columns[1], |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("画面编辑器").size(17.0).strong());
                        let source = self
                            .document_image_edit
                            .as_ref()
                            .map(|edit| format!("当前文档图片：{}", edit.reference))
                            .unwrap_or_else(|| "视频关键画面".to_string());
                        ui.label(RichText::new(source).color(MUTED_TEXT));
                    });
                });
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    for tool in [
                        AnnotationKind::Rectangle,
                        AnnotationKind::Arrow,
                        AnnotationKind::Brush,
                        AnnotationKind::Mosaic,
                        AnnotationKind::Crop,
                    ] {
                        ui.selectable_value(&mut self.annotation_tool, tool, tool.label());
                    }
                });
                ui.horizontal_wrapped(|ui| {
                    ui.label("颜色");
                    ui.color_edit_button_srgba(&mut self.annotation_color);
                    ui.add(
                        egui::DragValue::new(&mut self.annotation_width)
                            .range(1.0..=16.0)
                            .prefix("粗细 "),
                    );
                    if ui.button("撤销").clicked() {
                        self.annotations.pop();
                    }
                    if ui.button("清空").clicked() {
                        self.annotations.clear();
                        self.active_annotation = None;
                    }
                });
                ui.separator();
                self.draw_annotation_canvas(ui);
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    let save_label = if self.document_image_edit.is_some() {
                        "保存副本并替换文档引用"
                    } else {
                        "保存标注后的关键画面"
                    };
                    if ui.add(primary_button(save_label)).clicked() {
                        self.save_annotated_frame(ctx);
                    }
                    if self.document_image_edit.is_none() {
                        if ui.button("插入当前文档草稿").clicked() {
                            self.insert_current_frame(false);
                        }
                        if ui.button("插入并原子保存").clicked() {
                            self.insert_current_frame(true);
                        }
                    }
                });
            });
        });
    }

    fn ui_top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            egui::Frame::new()
                .fill(BRAND_PURPLE)
                .corner_radius(8)
                .inner_margin(egui::Margin::symmetric(9, 5))
                .show(ui, |ui| {
                    ui.label(RichText::new("◆").size(18.0).strong().color(Color32::WHITE));
                });
            ui.vertical(|ui| {
                ui.label(RichText::new("AUTO-MAS DocForge").size(16.0).strong());
                ui.label(
                    RichText::new("文档与视频教程工作台")
                        .size(10.0)
                        .color(MUTED_TEXT),
                );
            });
            ui.separator();
            if let Some(site) = &self.site {
                ui.label(RichText::new(display_path(&site.root)).color(MUTED_TEXT));
            } else {
                ui.label(RichText::new("尚未打开文档站").color(MUTED_TEXT));
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add(primary_button("保存")).clicked() {
                    self.save_document();
                }
                if ui.button("导出 PDF").clicked() {
                    self.workspace_tab = WorkspaceTab::Export;
                }
                if ui.button("打开文档站").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.open_site(path);
                    }
                }
            });
        });
    }

    fn ui_activity_bar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        for (tab, icon, label) in [
            (WorkspaceTab::Edit, "</>", "编辑"),
            (WorkspaceTab::Tutorial, "▶", "视频教程"),
            (WorkspaceTab::Translate, "译", "翻译"),
            (WorkspaceTab::Export, "PDF", "导出"),
        ] {
            if workspace_nav_button(ui, self.workspace_tab == tab, icon, label).clicked() {
                self.workspace_tab = tab;
            }
            ui.add_space(6.0);
        }
    }
}

impl eframe::App for DocforgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_translation();
        if ctx.input_mut(|input| input.consume_key(egui::Modifiers::CTRL, egui::Key::S)) {
            self.save_document();
        }
        if ctx.input(|input| input.viewport().close_requested()) && self.is_dirty() {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.confirm_close = true;
        }
        if self.confirm_close {
            let mut continue_editing = false;
            let mut exit_without_saving = false;
            egui::Window::new("文档尚未保存")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label("当前文档还有修改。");
                    ui.horizontal(|ui| {
                        continue_editing = ui.button("继续编辑").clicked();
                        exit_without_saving = ui.button("不保存并退出").clicked();
                    });
                });
            if continue_editing {
                self.confirm_close = false;
            }
            if exit_without_saving {
                self.saved_markdown = self.markdown.clone();
                self.confirm_close = false;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        egui::TopBottomPanel::top("toolbar")
            .exact_height(56.0)
            .frame(
                egui::Frame::new()
                    .fill(TOP_BAR_BG)
                    .inner_margin(egui::Margin::symmetric(12, 8)),
            )
            .show(ctx, |ui| self.ui_top_bar(ui));
        egui::SidePanel::left("activity")
            .exact_width(86.0)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(ACTIVITY_BG)
                    .inner_margin(egui::Margin::symmetric(6, 4)),
            )
            .show(ctx, |ui| self.ui_activity_bar(ui));
        egui::SidePanel::left("documents")
            .resizable(true)
            .default_width(286.0)
            .width_range(230.0..=420.0)
            .frame(
                egui::Frame::new()
                    .fill(SIDEBAR_BG)
                    .inner_margin(egui::Margin::symmetric(10, 4)),
            )
            .show(ctx, |ui| self.ui_document_list(ui));
        egui::TopBottomPanel::bottom("status")
            .exact_height(28.0)
            .frame(
                egui::Frame::new()
                    .fill(STATUS_BG)
                    .inner_margin(egui::Margin::symmetric(10, 5)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("●").color(BRAND_CYAN));
                    ui.label(RichText::new(&self.status).size(11.0));
                });
            });
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(APP_BG)
                    .inner_margin(egui::Margin::same(12)),
            )
            .show(ctx, |ui| self.ui_workspace(ctx, ui));
    }
}

const APP_BG: Color32 = Color32::from_rgb(13, 15, 23);
const TOP_BAR_BG: Color32 = Color32::from_rgb(20, 22, 33);
const ACTIVITY_BG: Color32 = Color32::from_rgb(17, 19, 29);
const SIDEBAR_BG: Color32 = Color32::from_rgb(23, 25, 37);
const PANEL_BG: Color32 = Color32::from_rgb(26, 29, 43);
const CARD_BG: Color32 = Color32::from_rgb(30, 33, 48);
const EDITOR_BG: Color32 = Color32::from_rgb(15, 17, 26);
const CODE_BLOCK_BG: Color32 = Color32::from_rgb(12, 14, 22);
const CALLOUT_BG: Color32 = Color32::from_rgb(34, 29, 58);
const STATUS_BG: Color32 = Color32::from_rgb(68, 58, 148);
const SELECTED_BG: Color32 = Color32::from_rgb(45, 43, 78);
const BORDER_COLOR: Color32 = Color32::from_rgb(50, 54, 75);
const LINE_NUMBER_COLOR: Color32 = Color32::from_rgb(92, 98, 122);
const MUTED_TEXT: Color32 = Color32::from_rgb(155, 161, 184);
const BRAND_PURPLE: Color32 = Color32::from_rgb(157, 100, 255);
const BRAND_CYAN: Color32 = Color32::from_rgb(65, 209, 255);
const SUCCESS_COLOR: Color32 = Color32::from_rgb(72, 199, 142);
const WARNING_COLOR: Color32 = Color32::from_rgb(245, 181, 71);
const ERROR_COLOR: Color32 = Color32::from_rgb(244, 100, 112);
const CODE_TEXT: Color32 = Color32::from_rgb(199, 214, 255);

fn install_mas_theme(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);
    let mut style = (*ctx.style()).clone();
    style.visuals = egui::Visuals::dark();
    style.visuals.panel_fill = APP_BG;
    style.visuals.window_fill = PANEL_BG;
    style.visuals.extreme_bg_color = EDITOR_BG;
    style.visuals.faint_bg_color = CARD_BG;
    style.visuals.selection.bg_fill = Color32::from_rgb(76, 65, 148);
    style.visuals.selection.stroke = Stroke::new(1.0, BRAND_CYAN);
    style.visuals.widgets.inactive.bg_fill = CARD_BG;
    style.visuals.widgets.inactive.weak_bg_fill = CARD_BG;
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER_COLOR);
    style.visuals.widgets.hovered.bg_fill = SELECTED_BG;
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, BRAND_PURPLE);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(78, 64, 150);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.0, BRAND_CYAN);
    style.spacing.item_spacing = Vec2::new(8.0, 7.0);
    style.spacing.button_padding = Vec2::new(10.0, 6.0);
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(14.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(13.5, FontFamily::Monospace),
    );
    ctx.set_style(style);
}

fn panel_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(PANEL_BG)
        .stroke(Stroke::new(1.0, BORDER_COLOR))
        .corner_radius(9)
        .inner_margin(egui::Margin::same(12))
}

fn primary_button(text: &'static str) -> egui::Button<'static> {
    egui::Button::new(RichText::new(text).strong().color(Color32::WHITE))
        .fill(Color32::from_rgb(87, 70, 176))
        .stroke(Stroke::new(1.0, BRAND_PURPLE))
}

fn workspace_nav_button(
    ui: &mut egui::Ui,
    selected: bool,
    icon: &'static str,
    label: &'static str,
) -> egui::Response {
    let text = RichText::new(format!("{icon}\n{label}"))
        .size(12.0)
        .strong()
        .color(if selected { Color32::WHITE } else { MUTED_TEXT });
    ui.add_sized(
        [72.0, 54.0],
        egui::Button::new(text)
            .fill(if selected {
                SELECTED_BG
            } else {
                Color32::TRANSPARENT
            })
            .stroke(if selected {
                Stroke::new(1.0, BRAND_PURPLE)
            } else {
                Stroke::NONE
            }),
    )
}

fn step_chip(ui: &mut egui::Ui, number: &str, label: &str, complete: bool) {
    let color = if complete { SUCCESS_COLOR } else { MUTED_TEXT };
    egui::Frame::new()
        .fill(if complete {
            Color32::from_rgb(24, 55, 48)
        } else {
            CARD_BG
        })
        .stroke(Stroke::new(1.0, color))
        .corner_radius(12)
        .inner_margin(egui::Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.label(
                RichText::new(format!("{number}  {label}"))
                    .strong()
                    .color(color),
            );
        });
}

fn field_label(ui: &mut egui::Ui, label: &str) {
    ui.add_space(8.0);
    ui.label(RichText::new(label).size(11.0).strong().color(MUTED_TEXT));
}

fn info_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(label).color(MUTED_TEXT));
        ui.label(RichText::new(value).strong());
    });
}

fn section_label(section: &str) -> &'static str {
    match section {
        "index" => "站点首页",
        "docs" => "使用文档",
        "developer" => "开发者文档",
        "plugin" => "插件文档",
        "download" => "下载说明",
        "disclosure" => "信息公开",
        _ => "其他文档",
    }
}

fn cursor_position(source: &str, character_index: usize) -> (usize, usize) {
    let before = source.chars().take(character_index).collect::<String>();
    let line = before
        .chars()
        .filter(|character| *character == '\n')
        .count()
        + 1;
    let column = before
        .rsplit_once('\n')
        .map(|(_, line)| line.chars().count() + 1)
        .unwrap_or_else(|| before.chars().count() + 1);
    (line, column)
}

fn format_time(seconds: f64) -> String {
    let total = seconds.max(0.0).round() as u64;
    format!("{:02}:{:02}", total / 60, total % 60)
}

fn replace_reference_file_name(reference: &str, file_name: &str) -> String {
    let clean = reference.split(['?', '#']).next().unwrap_or(reference);
    clean
        .rsplit_once('/')
        .map(|(directory, _)| format!("{directory}/{file_name}"))
        .unwrap_or_else(|| file_name.to_string())
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy()
        .trim_start_matches(r"\\?\")
        .to_string()
}

fn install_chinese_font(ctx: &egui::Context) {
    let mut candidates = Vec::new();
    if let Ok(executable) = std::env::current_exe() {
        if let Some(directory) = executable.parent() {
            for name in [
                "NotoSansCJKsc-Regular.otf",
                "SourceHanSansCN-Regular.otf",
                "simhei.ttf",
                "Deng.ttf",
            ] {
                candidates.push(directory.join("fonts").join(name));
            }
        }
    }
    let windows = std::env::var_os("WINDIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\Windows"));
    for name in ["simhei.ttf", "Deng.ttf", "msyh.ttc"] {
        candidates.push(windows.join("Fonts").join(name));
    }

    for path in candidates {
        let Ok(bytes) = fs::read(&path) else {
            continue;
        };
        let mut fonts = FontDefinitions::default();
        let name = "docforge-cjk".to_string();
        fonts
            .font_data
            .insert(name.clone(), Arc::new(FontData::from_owned(bytes)));
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            fonts.families.entry(family).or_default().push(name.clone());
        }
        ctx.set_fonts(fonts);
        return;
    }
}
fn load_texture_from_path(
    ctx: &egui::Context,
    path: &Path,
) -> Result<(egui::TextureHandle, [usize; 2])> {
    let bytes = fs::read(path).with_context(|| format!("cannot read {}", path.display()))?;
    let texture = load_texture_from_bytes(ctx, &format!("frame-{}", path.display()), &bytes)?;
    let size = texture.size();
    Ok((texture, size))
}

fn load_texture_from_bytes(
    ctx: &egui::Context,
    name: &str,
    bytes: &[u8],
) -> Result<egui::TextureHandle> {
    let image = image::load_from_memory(bytes)
        .context("cannot decode image")?
        .to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    Ok(ctx.load_texture(
        name,
        ColorImage::from_rgba_unmultiplied(size, image.as_raw()),
        TextureOptions::LINEAR,
    ))
}

fn draw_annotation_preview(painter: &egui::Painter, rect: Rect, annotation: &Annotation) {
    let color = Color32::from_rgba_unmultiplied(
        annotation.color[0],
        annotation.color[1],
        annotation.color[2],
        annotation.color[3],
    );
    let stroke = Stroke::new(annotation.width, color);
    let points = annotation
        .points
        .iter()
        .map(|point| {
            Pos2::new(
                rect.left() + rect.width() * point[0],
                rect.top() + rect.height() * point[1],
            )
        })
        .collect::<Vec<_>>();
    match annotation.kind {
        AnnotationKind::Rectangle if points.len() >= 2 => {
            painter.rect_stroke(
                Rect::from_two_pos(points[0], points[1]),
                0.0,
                stroke,
                egui::StrokeKind::Outside,
            );
        }
        AnnotationKind::Arrow if points.len() >= 2 => {
            painter.line_segment([points[0], points[1]], stroke);
            let angle = (points[1].y - points[0].y).atan2(points[1].x - points[0].x);
            let head = (annotation.width * 4.0).max(14.0);
            for delta in [2.6_f32, -2.6_f32] {
                let point = Pos2::new(
                    points[1].x + head * (angle + delta).cos(),
                    points[1].y + head * (angle + delta).sin(),
                );
                painter.line_segment([points[1], point], stroke);
            }
        }
        AnnotationKind::Brush if points.len() >= 2 => {
            for pair in points.windows(2) {
                painter.line_segment([pair[0], pair[1]], stroke);
            }
        }
        AnnotationKind::Mosaic if points.len() >= 2 => {
            let mosaic_stroke = Stroke::new(
                (annotation.width * 6.0).max(18.0),
                Color32::from_rgba_unmultiplied(90, 90, 90, 150),
            );
            for pair in points.windows(2) {
                painter.line_segment([pair[0], pair[1]], mosaic_stroke);
            }
        }
        AnnotationKind::Crop if points.len() >= 2 => {
            painter.rect_stroke(
                Rect::from_two_pos(points[0], points[1]),
                0.0,
                Stroke::new(2.0, Color32::from_rgb(35, 180, 210)),
                egui::StrokeKind::Outside,
            );
        }
        _ => {}
    }
}

fn normalize_pointer(pointer: Pos2, rect: Rect) -> [f32; 2] {
    [
        ((pointer.x - rect.left()) / rect.width()).clamp(0.0, 1.0),
        ((pointer.y - rect.top()) / rect.height()).clamp(0.0, 1.0),
    ]
}

fn matches_language(language: Language, filter: LanguageFilter) -> bool {
    matches!(
        (language, filter),
        (_, LanguageFilter::All)
            | (Language::Zh, LanguageFilter::Zh)
            | (Language::En, LanguageFilter::En)
    )
}

fn matches_search(document: &DocumentRef, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty()
        || document.title.to_lowercase().contains(&query)
        || document.relative_path.to_lowercase().contains(&query)
}

fn safe_stem(value: &str) -> String {
    let result = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    if result.is_empty() {
        "auto-mas-document".to_string()
    } else {
        result
    }
}

fn unique_media_name(source: &Path) -> String {
    let stem = source
        .file_stem()
        .and_then(|value| value.to_str())
        .map(safe_stem)
        .unwrap_or_else(|| "capture".to_string());
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("{stem}-{stamp}.png")
}
