use std::{path::Path, process::Command};

use crate::eaf::errors::EafError;

/// Returns media duration in milliseconds.
/// TODO find a way to remove ffprobe depenency.
pub fn get_duration(media_file: &Path, ffprobe_path: Option<&Path>) -> Result<u64, EafError> {
    let ffprobe = if let Some(path) = ffprobe_path {
        path
    } else if cfg!(windows) {
        Path::new("ffprobe.exe")
    } else {
        Path::new("ffprobe")
    };

    // ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 FILE
    let args = [
        "-v",
        "error",
        "-show_entries",
        "format=duration",
        "-of",
        "default=noprint_wrappers=1:nokey=1",
        &media_file.display().to_string(),
    ];

    let output = Command::new(ffprobe)
        .args(&args)
        .output()?.stdout; // or ::new().spawn() ?
    let duration: f64 = std::str::from_utf8(&output)?
        .trim()
        .parse()?;

    Ok((duration * 1000.0) as u64)
}