use anyhow::{bail, Context, Result};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .filter(|candidate| !candidate.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).with_context(|| format!("cannot create {}", parent.display()))?;

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("target path must have a valid file name")?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let temp_path = parent.join(format!(".{file_name}.{stamp}.{}.tmp", std::process::id()));

    let result = (|| -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
            .with_context(|| format!("cannot create temporary file {}", temp_path.display()))?;
        file.write_all(bytes)?;
        file.flush()?;
        file.sync_all()?;
        replace_file(&temp_path, path)?;
        sync_directory(parent)?;
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }
    result
}

pub fn copy_atomic(source: &Path, destination: &Path) -> Result<()> {
    let bytes = fs::read(source).with_context(|| format!("cannot read {}", source.display()))?;
    atomic_write(destination, &bytes)
}

pub fn ensure_inside(root: &Path, candidate: &Path) -> Result<PathBuf> {
    let root = root
        .canonicalize()
        .with_context(|| format!("cannot resolve site root {}", root.display()))?;
    let parent = candidate
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let parent = parent
        .canonicalize()
        .with_context(|| format!("cannot resolve parent {}", parent.display()))?;
    let candidate = parent.join(
        candidate
            .file_name()
            .context("candidate path must have a file name")?,
    );
    if !candidate.starts_with(&root) {
        bail!(
            "refusing to write outside site root: {}",
            candidate.display()
        );
    }
    Ok(candidate)
}

fn replace_file(source: &Path, destination: &Path) -> Result<()> {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::Storage::FileSystem::{
            MoveFileExW, MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH,
        };

        let source_wide: Vec<u16> = source
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let destination_wide: Vec<u16> = destination
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let result = unsafe {
            MoveFileExW(
                source_wide.as_ptr(),
                destination_wide.as_ptr(),
                MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
            )
        };
        if result == 0 {
            return Err(std::io::Error::last_os_error()).with_context(|| {
                format!(
                    "cannot atomically replace {} with {}",
                    destination.display(),
                    source.display()
                )
            });
        }
        Ok(())
    }

    #[cfg(not(windows))]
    {
        fs::rename(source, destination).with_context(|| {
            format!(
                "cannot atomically replace {} with {}",
                destination.display(),
                source.display()
            )
        })
    }
}

fn sync_directory(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        std::fs::File::open(path)
            .with_context(|| format!("cannot open directory {}", path.display()))?
            .sync_all()?;
    }
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_existing_file_without_leaving_temporary_file() {
        let directory = tempfile::tempdir().expect("创建测试目录");
        let target = directory.path().join("document.md");
        atomic_write(&target, b"first").expect("首次写入");
        atomic_write(&target, b"second").expect("原子覆盖");
        assert_eq!(fs::read(&target).expect("读取结果"), b"second");
        assert_eq!(
            fs::read_dir(directory.path())
                .expect("读取目录")
                .filter_map(std::result::Result::ok)
                .count(),
            1
        );
    }
}
