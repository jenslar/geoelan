use std::path::{Path, PathBuf};

use mp4iter::Mp4;

use crate::{FitError, files::match_extension, Fit};

use super::meta::VirbMeta;

#[derive(Debug, Clone, Default)]
pub struct VirbFile {
    /// High resolution MP4
    pub mp4: Option<PathBuf>,
    /// Low resolution MP4
    pub glv: Option<PathBuf>,
    /// FIT path
    pub fit: Option<PathBuf>,
    /// UUID
    pub uuid: String
}

impl VirbFile {
    pub fn new(path: &Path, uuid: Option<&str>) -> Result<Self, FitError> {
        let mut virbfile = VirbFile::default();
        virbfile.set_path(path);
        virbfile.set_uuid(uuid)?;

        Ok(virbfile)
    }

    pub fn meta(&self) -> Result<VirbMeta, FitError> {
        let path = match (self.mp4(), self.glv()) {
            (Some(mp4), _) => mp4.to_owned(),
            (_, Some(glv)) => glv.to_owned(),
            (None, None) => return Err(FitError::MissingVideo)
        };
        
        VirbMeta::new(&path)
    }

    /// Sets path by checking extention.
    /// Does nothing if not a `.mp4`, `.glv`, or `.fit`.
    /// Case agnostic.
    pub fn set_path(&mut self, path: &Path) {
        if match_extension(path, "mp4") {
            // self.uuid = Self::uuid_mp4(path).map_err(|_| FitError::InvalidVirbMp4)?;
            self.mp4 = Some(path.to_owned());
        }
        if match_extension(path, "glv") {
            // self.uuid = Self::uuid_mp4(path).map_err(|_| FitError::InvalidVirbMp4)?;
            self.glv = Some(path.to_owned());
        }
        if match_extension(path, "fit") {
            self.fit = Some(path.to_owned());
        }
    }

    /// Sets UUID. If `uuid` is `None`, UUID is extracted from GLV or MP4-file if set.
    pub fn set_uuid(&mut self, uuid: Option<&str>) -> Result<(), FitError> {
        match uuid {
            Some(u) => self.uuid = u.to_owned(),
            None => {
                if let Some(path) = self.mp4() {
                    self.uuid = Self::uuid_mp4(path).map_err(|_| FitError::InvalidVirbMp4)?
                } else if let Some(path) = self.glv() {
                    self.uuid = Self::uuid_mp4(path).map_err(|_| FitError::InvalidVirbMp4)?
                }
            }
        }
        Ok(())
    }

    /// Get MP4 path if set.
    pub fn mp4(&self) -> Option<&Path> {
        self.mp4.as_deref()
    }

    /// Get GLV path if set.
    pub fn glv(&self) -> Option<&Path> {
        self.glv.as_deref()
    }

    /// Get FIT path if set.
    pub fn fit(&self) -> Option<&Path> {
        self.fit.as_deref()
    }

    /// Attempts to extract Garmin VIRB UUID
    /// as a string from an MP4-file.
    pub fn uuid_mp4(mp4_path: &Path) -> Result<String, FitError> {
        let mut mp4 = Mp4::new(&mp4_path)?;

        // Find and seek to "uuid" atom position (only one per VIRB MP4-file)
        let _ = mp4.find("uuid")?;

        // Read atom data to string.
        // After size+fourcc everything is string data,
        // no null termination.
        let uuid = mp4.atom()?
            .read_to_string()?;

        return Ok(uuid)
    }

    /// Returns duration of linked clip/s, either MP4 or low-res GLV,
    /// depending on which is set, prioritising high-res MP4-file.
    pub fn duration(&self) -> Option<time::Duration> { // perhaps use proper error instead
        let video = match self.mp4() {
            Some(v) => Some(v),
            None => self.glv()
        };

        if let Some(vid) = video {
            return Mp4::new(vid).ok()?.duration().ok()
        }

        None
    }

    /// Returns `true` only if a FIT-file and
    /// at least one corresponding video file
    /// is set (`.GLV` or `.MP4`).
    pub(crate) fn matched(&self) -> bool {
        (self.mp4().is_some() || self.glv().is_some()) && self.fit().is_some()
    }
}