use anyhow::{bail, Context, Result};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::TempDir;

use crate::atomic;

#[derive(Debug, Clone)]
pub struct VideoTool {
    ffmpeg: PathBuf,
    ffprobe: PathBuf,
}

impl VideoTool {
    pub fn discover(beside: Option<&Path>) -> Option<Self> {
        let executable_dir = beside.map(Path::to_path_buf).or_else(|| {
            std::env::current_exe()
                .ok()?
                .parent()
                .map(Path::to_path_buf)
        });
        if let Some(directory) = executable_dir {
            for tools_dir in [directory.clone(), directory.join("bin")] {
                let ffmpeg = tools_dir.join("ffmpeg.exe");
                let ffprobe = tools_dir.join("ffprobe.exe");
                if command_works(&ffmpeg, "-version") && command_works(&ffprobe, "-version") {
                    return Some(Self { ffmpeg, ffprobe });
                }
            }
        }

        let ffmpeg = PathBuf::from("ffmpeg");
        let ffprobe = PathBuf::from("ffprobe");
        if command_works(&ffmpeg, "-version") && command_works(&ffprobe, "-version") {
            Some(Self { ffmpeg, ffprobe })
        } else {
            None
        }
    }

    pub fn capture_evenly(
        &self,
        video: &Path,
        output_dir: &Path,
        count: usize,
    ) -> Result<Vec<PathBuf>> {
        if count == 0 || count > 50 {
            bail!("截图数量必须在 1 到 50 之间");
        }
        let duration = self.duration(video)?;
        let mut timestamps = Vec::with_capacity(count);
        for index in 0..count {
            let fraction = (index + 1) as f64 / (count + 1) as f64;
            timestamps.push(duration * fraction);
        }
        self.capture_many(video, output_dir, &timestamps)
    }

    pub fn capture_at(&self, video: &Path, seconds: f64, output: &Path) -> Result<()> {
        self.capture_one(video, output, seconds).map(|_| ())
    }

    pub fn duration_seconds(&self, video: &Path) -> Result<Option<f64>> {
        self.duration(video).map(Some)
    }

    pub fn capture_keyframes(
        &self,
        video: &Path,
        output_dir: &Path,
        max_frames: usize,
    ) -> Result<Vec<PathBuf>> {
        if max_frames == 0 || max_frames > 50 {
            bail!("关键画面数量必须在 1 到 50 之间");
        }
        ensure_video(video)?;
        fs::create_dir_all(output_dir)?;
        let temporary = TempDir::new_in(output_dir)
            .with_context(|| format!("无法在 {} 创建临时目录", output_dir.display()))?;
        let pattern = temporary.path().join("scene-%03d.png");
        let filter = "select=gt(scene\\,0.32),scale='min(1920,iw)':-2".to_string();
        let args = vec![
            OsString::from("-hide_banner"),
            OsString::from("-loglevel"),
            OsString::from("error"),
            OsString::from("-i"),
            video.as_os_str().to_os_string(),
            OsString::from("-vf"),
            OsString::from(filter),
            OsString::from("-vsync"),
            OsString::from("vfr"),
            OsString::from("-frames:v"),
            OsString::from(max_frames.to_string()),
            pattern.as_os_str().to_os_string(),
        ];
        run(&self.ffmpeg, &args, "自动识别关键画面失败")?;

        let mut captured: Vec<PathBuf> = fs::read_dir(temporary.path())?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("png"))
            .collect();
        captured.sort();
        if captured.is_empty() {
            return self.capture_evenly(video, output_dir, max_frames.min(5));
        }
        move_frames(captured, output_dir, "key")
    }

    pub fn capture_one(&self, video: &Path, output: &Path, seconds: f64) -> Result<PathBuf> {
        if seconds < 0.0 {
            bail!("截图时间不能小于 0 秒");
        }
        ensure_video(video)?;
        let parent = output.parent().unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent)?;
        let temporary = TempDir::new_in(parent)?;
        let frame = temporary.path().join("frame.png");
        let args = vec![
            OsString::from("-hide_banner"),
            OsString::from("-loglevel"),
            OsString::from("error"),
            OsString::from("-ss"),
            OsString::from(format!("{seconds:.3}")),
            OsString::from("-i"),
            video.as_os_str().to_os_string(),
            OsString::from("-frames:v"),
            OsString::from("1"),
            OsString::from("-vf"),
            OsString::from("scale='min(1920,iw)':-2"),
            frame.as_os_str().to_os_string(),
        ];
        run(&self.ffmpeg, &args, "指定时间截图失败")?;
        atomic::copy_atomic(&frame, output)?;
        Ok(output.to_path_buf())
    }

    pub fn duration(&self, video: &Path) -> Result<f64> {
        ensure_video(video)?;
        let args = [
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
        ];
        let output = Command::new(&self.ffprobe)
            .args(args)
            .arg(video)
            .creation_flags_if_windows()
            .output()
            .context("无法启动 ffprobe")?;
        if !output.status.success() {
            bail!(
                "无法读取视频时长：{}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<f64>()
            .context("ffprobe 返回了无效的视频时长")
    }

    fn capture_many(
        &self,
        video: &Path,
        output_dir: &Path,
        timestamps: &[f64],
    ) -> Result<Vec<PathBuf>> {
        ensure_video(video)?;
        fs::create_dir_all(output_dir)?;
        let temporary = TempDir::new_in(output_dir)?;
        let mut frames = Vec::with_capacity(timestamps.len());
        for (index, timestamp) in timestamps.iter().enumerate() {
            let frame = temporary.path().join(format!("frame-{:03}.png", index + 1));
            let args = vec![
                OsString::from("-hide_banner"),
                OsString::from("-loglevel"),
                OsString::from("error"),
                OsString::from("-ss"),
                OsString::from(format!("{timestamp:.3}")),
                OsString::from("-i"),
                video.as_os_str().to_os_string(),
                OsString::from("-frames:v"),
                OsString::from("1"),
                OsString::from("-vf"),
                OsString::from("scale='min(1920,iw)':-2"),
                frame.as_os_str().to_os_string(),
            ];
            run(&self.ffmpeg, &args, "视频截图失败")?;
            frames.push(frame);
        }
        move_frames(frames, output_dir, "frame")
    }
}

fn move_frames(frames: Vec<PathBuf>, output_dir: &Path, prefix: &str) -> Result<Vec<PathBuf>> {
    let mut outputs = Vec::with_capacity(frames.len());
    for (index, frame) in frames.into_iter().enumerate() {
        let output = unique_path(output_dir, &format!("{prefix}-{:03}", index + 1), "png");
        atomic::copy_atomic(&frame, &output)?;
        outputs.push(output);
    }
    Ok(outputs)
}

fn unique_path(directory: &Path, stem: &str, extension: &str) -> PathBuf {
    let first = directory.join(format!("{stem}.{extension}"));
    if !first.exists() {
        return first;
    }
    for suffix in 2..10_000 {
        let candidate = directory.join(format!("{stem}-{suffix}.{extension}"));
        if !candidate.exists() {
            return candidate;
        }
    }
    directory.join(format!("{stem}-{}.{}", std::process::id(), extension))
}

fn ensure_video(video: &Path) -> Result<()> {
    if !video.is_file() {
        bail!("找不到视频文件：{}", video.display());
    }
    Ok(())
}

fn run(executable: &Path, args: &[OsString], message: &str) -> Result<()> {
    let output = Command::new(executable)
        .args(args)
        .stdin(Stdio::null())
        .creation_flags_if_windows()
        .output()
        .with_context(|| format!("无法启动 {}", executable.display()))?;
    if !output.status.success() {
        bail!(
            "{}：{}",
            message,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}

fn command_works(executable: &Path, argument: &str) -> bool {
    Command::new(executable)
        .arg(argument)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags_if_windows()
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

trait WindowsCommandExt {
    fn creation_flags_if_windows(&mut self) -> &mut Self;
}

impl WindowsCommandExt for Command {
    fn creation_flags_if_windows(&mut self) -> &mut Self {
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            self.creation_flags(0x08000000);
        }
        self
    }
}
