//! EAF media descriptor.

use std::{path::{Path, PathBuf}, ffi::OsStr};
use serde::{Serialize, Deserialize};

use crate::ffmpeg;

use super::{annotation_document::path_to_string, EafError};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "MEDIA_DESCRIPTOR")]
/// EAF media descriptor. Part of EAF header, contains paths to linked media files.
pub struct MediaDescriptor {
    /// Absolute media file path.
    pub media_url: String, // is abs path really required? or...
    /// Media mime type.
    pub mime_type: String,
    /// Path to e.g. the video which a wav-file was extracted from.
    pub extracted_from: Option<String>,
    /// Relative media file path.
    pub relative_media_url: Option<String>, // ...is it rel path that is required?
    /// Time offset in milliseconds used when synchronising multiple media files.
    pub time_origin: Option<String>
}

impl Default for MediaDescriptor {
    fn default() -> Self {
        Self {
            media_url: "".to_owned(),
            mime_type: "".to_owned(),
            extracted_from: None,
            relative_media_url: None,
            time_origin: None
        }
    }
}

impl MediaDescriptor {
    /// Create new `MediaDescriptor`. Relative media path is set to
    /// filename only, e.g. `./VIDEO.MP4`.
    pub fn new(path: &Path, extracted_from: Option<&str>) -> Self {
        let mut mdsc = MediaDescriptor::default();
        mdsc.media_url = path_to_string(path, Some("file:///"), false); // need prefix "file:///"
        mdsc.relative_media_url = Some(path_to_string(path, Some("./"), true)); // need prefix "file:///"
        // mdsc.relative_media_url = path.file_name()
        //     .map(|f| format!("./{}", f.to_string_lossy()));
        mdsc.mime_type = MimeType::from_path(path).to_string();
        mdsc.extracted_from = extracted_from.map(String::from);
        mdsc
    }

    /// Returns filename as `&OsStr`. Prioritises absolute media url.
    pub fn file_name<'a>(&'a self) -> Option<&'a OsStr> {
        if &self.media_url.replace("file:///", "") != "" {
            Path::new(&self.media_url).file_name()
        } else if let Some(p) = &self.relative_media_url { 
            Path::new(p).file_name()
        } else {
            None
        }
    }

    /// Set absolute media path, and optional relative path.
    pub fn set_path(&mut self, path: &Path, rel_path: Option<&Path>) {
        self.media_url = path_to_string(path, Some("file:///"), false);
        if let Some(rel) = rel_path {
            self.relative_media_url = Some(format!("./{}", rel.to_string_lossy()));
        }
        self.mime_type = MimeType::from_path(path).to_string();
    }

    /// Set relative media path.
    pub fn set_rel_path(&mut self, rel_path: &Path, filename_only: bool) {
        self.relative_media_url = Some(path_to_string(rel_path, Some("./"), filename_only));
        self.mime_type = MimeType::from_path(rel_path).to_string();
    }

    /// Matches file names, not full path, to check if media descriptor contains path.
    pub fn contains(&self, path: &Path) -> bool {
        if let (Some(fn_self), Some(fn_in)) = (self.file_name(), path.file_name()) {
            return fn_self == fn_in
        }
        false
    }

    /// Extract specified time span for the media path and sets
    /// the path to the new media file.
    /// 
    /// Relative media url is prioritised if `media_dir` is specified, since the absolute
    /// media url may refer to invalid paths.
    pub fn timespan(&mut self, start: i64, end: i64, media_dir: Option<&Path>, ffmpeg_path: Option<&Path>) -> Result<(), EafError> {
    // pub fn timespan(&mut self, start: u64, end: u64, media_dir: Option<&Path>, ffmpeg_path: Option<&Path>) -> Result<(), EafError> {
        if start < 0 || end < 0 {
            return Err(EafError::ValueTooSmall(start))
        }

        // First try relative media path + media dir (e.g. eaf-dir containing eaf + media-file)...
        let media_path = if let (Some(dir), Some(rel_path)) = (media_dir, &self.relative_media_url) {
            let mut dir = dir.to_owned();
            dir.push(rel_path);
            dir
        } else {
        // ...or default to media url.
            PathBuf::from(&self.media_url)
        };

        self.media_url = ffmpeg::process::extract_timespan(&media_path, start as u64, end as u64, None, ffmpeg_path)?
            .display()
            .to_string()
            .trim_start_matches("file://") // media url/absolute path starts with "file://"
            .to_owned();

        Ok(())
    }
}

enum MimeType {
    Wav,
    Mp4,
    Mpeg,
    Other(String) // file extension
    // Mpeg2 // video
}

impl MimeType {
    /// Returns mime type for linked media files.
    /// 
    /// This is only intended for determining mime type for
    /// ELAN-compatible multimedia files.
    pub fn from_path(path: &Path) -> Self {
        let ext = path
            .extension()
            .map(|o| o.to_string_lossy().to_string())
            .unwrap_or(String::from("none"))
            .to_lowercase();
        
        match ext.as_ref() {
            "mp4" => MimeType::Mp4,
            "wav" => MimeType::Wav,
            "mpg" | "mpeg" => MimeType::Mpeg,
            _ => MimeType::Other(path.to_string_lossy().to_string()),
        }
    }

    /// Returns a mime type string
    pub fn to_string(&self) -> String {
        match self {
            MimeType::Wav => "audio/x-wav".to_owned(),
            MimeType::Mp4 => "video/mp4".to_owned(),
            MimeType::Mpeg => "video/mpeg2".to_owned(), // presumably not mpeg1...
            MimeType::Other(s) => format!("application/{}", s.to_owned())
        }
    }
}
