//! Locate video-files (GoPro, Garmin VIRB) and FIT (Garmin VIRB), and generate an ELAN-file.

use std::{io::ErrorKind, path::PathBuf};

use crate::model::CameraModel;

pub mod batch2eaf;
pub mod cam2eaf;
pub mod gopro2eaf;
pub mod gopro2eaf_session; // single session -> eaf
pub mod virb2eaf;
pub mod virb2eaf_session; // single session -> eaf

// Checks whether GoPro, VIRB and/or batch, then runs the appropriate task
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    if args.contains_id("batch") {
        batch2eaf::run(args)
    } else if args.contains_id("fit") || args.contains_id("uuid") {
        virb2eaf::run(args)
    } else if args.contains_id("video") {
        let video_path = args.get_one::<PathBuf>("video").unwrap();
        let model = CameraModel::from(video_path.as_path());
        match model {
            CameraModel::Virb(_) => virb2eaf::run(args),
            CameraModel::GoPro(_) => gopro2eaf::run(args),
            CameraModel::Unknown => {
                let msg = "(!) Unknown or unsupported device.";
                Err(std::io::Error::new(ErrorKind::Other, msg))
            }
        }
    } else {
        let msg = "(!) Failed to process input parameters.";
        Err(std::io::Error::new(ErrorKind::Other, msg))
    }
}
