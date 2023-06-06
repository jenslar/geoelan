//! Locate and match camera clips (GoPro, Garmin VIRB) and FIT-files (Garmin VIRB).

use std::{path::PathBuf, io::ErrorKind};

use crate::model::CameraModel;

pub mod locate_virb;
pub mod locate_gopro;

// MAIN LOCATE SUB-COMMAND
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {

    if args.get_one::<PathBuf>("fit").is_some() || args.get_one::<String>("uuid").is_some() {
        
        // If FIT or UUID specified run VIRB locate...
        if let Err(err) = locate_virb::run(&args) {
            let msg = format!("(!) Error locating Garmin VIRB files: {err}");
            return Err(std::io::Error::new(ErrorKind::Other, msg))
        }
        
    } else {
        // ...otherwise determine from camera model
        let model_from_string: Option<&String> = args.get_one("kind");
        let model_from_path: Option<PathBuf> = args.get_one("video").cloned();

        if let Some(m) = model_from_path.as_ref() {
            if !m.exists() {
                let msg = format!("(!) File does not exist: {}", m.display());
                return Err(std::io::Error::new(ErrorKind::Other, msg))
            }
        }
        
        let model = match (model_from_string, model_from_path) {
            (Some(string), _) => CameraModel::from(string.as_str()),
            (_, Some(path)) => CameraModel::from(path.as_path()),
            _ => {
                let msg = "(!) Failed to determine camera model.";
                return Err(std::io::Error::new(ErrorKind::Other, msg))
            }
        };

        match model {
            CameraModel::GoPro(_) => if let Err(err) = locate_gopro::run(&args) {
                let msg = format!("(!) Error locating GoPro files: {err}");
                return Err(std::io::Error::new(ErrorKind::Other, msg))
            },
            CameraModel::Virb(_) => if let Err(err) = locate_virb::run(&args) {
                let msg = format!("(!) Error locating Garmin VIRB files: {err}");
                return Err(std::io::Error::new(ErrorKind::Other, msg))
            },
            CameraModel::Unknown => {
                let msg = "(!) Failed to determine camera model.";
                return Err(std::io::Error::new(ErrorKind::Other, msg))
            }
        };
    }

    Ok(())
}