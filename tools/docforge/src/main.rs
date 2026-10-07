#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod annotation;
mod app;
mod atomic;
mod markdown;
mod pdf;
mod site;
mod translation;
mod tutorial;
mod video;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "docforge", version, about = "AUTO-MAS 文档站绿色编辑工具")]
struct Cli {
    #[arg(long, help = "启动界面时自动打开这个 VitePress 文档站")]
    site: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "扫描并输出文档站结构")]
    Scan {
        #[arg(value_name = "SITE_ROOT")]
        site: PathBuf,
    },
    #[command(about = "把 Markdown 文档导出为 PDF")]
    ExportPdf {
        #[arg(value_name = "MARKDOWN")]
        markdown: PathBuf,
        #[arg(value_name = "PDF")]
        pdf: PathBuf,
        #[arg(long)]
        font: Option<PathBuf>,
    },
    #[command(about = "把 PDF 可复制文字转换为 Markdown")]
    ImportPdf {
        #[arg(value_name = "PDF")]
        pdf: PathBuf,
        #[arg(value_name = "MARKDOWN")]
        markdown: PathBuf,
    },
    #[command(about = "从视频中截取画面")]
    Capture {
        #[arg(value_name = "VIDEO")]
        video: PathBuf,
        #[arg(value_name = "OUTPUT_DIR")]
        output_dir: PathBuf,
        #[arg(long, default_value_t = 5)]
        count: usize,
        #[arg(long, help = "使用 FFmpeg 场景识别，而不是按时间均匀截图")]
        key_frames: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Scan { site }) => {
            let profile = site::SiteProfile::discover(&site)?;
            println!("{}", serde_json::to_string_pretty(&profile)?);
            Ok(())
        }
        Some(Command::ExportPdf {
            markdown,
            pdf,
            font,
        }) => {
            let source = std::fs::read_to_string(&markdown)
                .with_context(|| format!("cannot read {}", markdown.display()))?;
            let info = pdf::export_markdown_to_pdf(
                &source,
                &pdf,
                pdf::PdfExportOptions {
                    custom_font: font.as_deref(),
                    document_path: Some(&markdown),
                    site_root: markdown.parent(),
                },
            )?;
            println!("created {} ({} pages)", info.path.display(), info.pages);
            if let Some(warning) = info.warning {
                eprintln!("warning: {warning}");
            }
            Ok(())
        }
        Some(Command::ImportPdf { pdf, markdown }) => {
            let source = pdf::pdf_to_markdown(&pdf)?;
            atomic::atomic_write(&markdown, source.as_bytes())?;
            println!("created {}", markdown.display());
            Ok(())
        }
        Some(Command::Capture {
            video,
            output_dir,
            count,
            key_frames,
        }) => {
            let tool = video::VideoTool::discover(None)
                .context("ffmpeg was not found; put ffmpeg.exe beside docforge.exe or on PATH")?;
            let frames = if key_frames {
                tool.capture_keyframes(&video, &output_dir, count)?
            } else {
                tool.capture_evenly(&video, &output_dir, count)?
            };
            for frame in frames {
                println!("{}", frame.display());
            }
            Ok(())
        }
        None => {
            let initial_site = cli.site;
            let native_options = eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default()
                    .with_title("DocForge - AUTO-MAS 文档工坊")
                    .with_inner_size([1600.0, 960.0])
                    .with_min_inner_size([1280.0, 760.0]),
                ..Default::default()
            };
            eframe::run_native(
                "DocForge - AUTO-MAS 文档工坊",
                native_options,
                Box::new(move |cc| Ok(Box::new(app::DocforgeApp::new(cc, initial_site)))),
            )
            .map_err(|error| anyhow::anyhow!("GUI failed: {error}"))
        }
    }
}
