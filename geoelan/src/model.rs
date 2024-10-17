//! Determine camera model.

use std::{ffi::OsString, path::Path};

use fit_rs::VirbFile;
use gpmf_rs::DeviceName;

#[derive(Debug, Clone)]
pub enum CameraModel {
    /// Garmin VIRB with UUID
    Virb(String),
    /// Garmin VIRB with GoPro device name
    GoPro(DeviceName),
    /// Unknown device
    Unknown,
}

impl From<&str> for CameraModel {
    fn from(kind: &str) -> Self {
        match kind {
            "v" | "virb" => CameraModel::Virb(String::default()),
            "g" | "gopro" => CameraModel::GoPro(DeviceName::default()),
            _ => CameraModel::Unknown,
        }
    }
}

impl From<&Path> for CameraModel {
    fn from(path: &Path) -> Self {
        if let Ok(uuid) = VirbFile::uuid_mp4(path) {
            return CameraModel::Virb(uuid);
        }

        if let Ok(devname) = DeviceName::from_path(path) {
            return CameraModel::GoPro(devname);
        }

        return CameraModel::Unknown;
    }
}

impl From<&OsString> for CameraModel {
    fn from(kind: &OsString) -> Self {
        let kind_str = kind.to_string_lossy().to_string();
        match kind_str.trim() {
            "virb" => CameraModel::Virb(String::default()),
            "gopro" => CameraModel::GoPro(DeviceName::default()),
            _ => CameraModel::Unknown,
        }
    }
}
