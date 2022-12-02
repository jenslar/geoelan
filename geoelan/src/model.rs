//! Determine camera model.

use std::{path::Path, ffi::OsString};

use fit_rs::VirbFile;
use gpmf_rs::GoProFile;

#[derive(Debug, Clone)]
pub enum CameraModel {
    Virb,
    GoPro,
    Unknown
}

// impl clap::ValueEnum for CameraModel {
//     fn value_variants<'a>() -> &'a [Self] {
//         &[Self::Virb, Self::GoPro, Self::Unknown]
//     }

//     fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
//         match self {
//             Self::Virb => Some(clap::PossibleValue::new("virb")),
//             Self::GoPro => Some(clap::PossibleValue::new("gopro")),
//             Self::Unknown => Some(clap::PossibleValue::new("unknown")),
//         }
//     }
// }

impl From<&str> for CameraModel {
    fn from(kind: &str) -> Self {
        match kind {
            "virb" => CameraModel::Virb,
            "gopro" => CameraModel::GoPro,
            _ => CameraModel::Unknown,
        }
    }
}

impl From<&Path> for CameraModel {
    fn from(path: &Path) -> Self {
        if let Ok(_) = VirbFile::uuid_mp4(path) {
            return CameraModel::Virb
        }

        if let Ok(true) = GoProFile::is_gopro(path) {
            return CameraModel::GoPro
        }

        return CameraModel::Unknown
    }
}

impl From<&OsString> for CameraModel {
    fn from(kind: &OsString) -> Self {
        let kind_str = kind.to_string_lossy().to_string();
        match kind_str.trim() {
            "virb" => CameraModel::Virb,
            "gopro" => CameraModel::GoPro,
            _ => CameraModel::Unknown,
        }
    }
}