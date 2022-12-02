//! Locate and match camera clips (GoPro, Garmin VIRB) and FIT-files (Garmin VIRB).

use std::path::PathBuf;

use crate::model::CameraModel;

pub mod locate_virb;
pub mod locate_gopro;

// MAIN LOCATE SUB-COMMAND
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {

    if args.get_one::<PathBuf>("fit").is_some() || args.get_one::<String>("uuid").is_some() {
        
        // If FIT or UUID specified run VIRB locate...
        if let Err(err) = locate_virb::run(&args) {
            println!("(!) Error locating Garmin Virb files: {err}");
            std::process::exit(1)
        }
        
    } else {
        // ...otherwise determine from camera model

        let model_from_string: Option<&String> = args.get_one("kind");
        let model_from_path: Option<PathBuf> = args.get_one("video").cloned();

        if let Some(m) = model_from_path.as_ref() {
            if !m.exists() {
                println!("(!) File does not exist: {}", m.display());
                std::process::exit(1)
            }
        }
        
        let model = match (model_from_string, model_from_path) {
            (Some(string), _) => CameraModel::from(string.as_str()),
            (_, Some(path)) => CameraModel::from(path.as_path()),
            _ => {
                println!("(!) Failed to determine camera model.");
                std::process::exit(1)
            }
        };

        match model {
            CameraModel::GoPro => if let Err(err) = locate_gopro::run(&args) {
                println!("(!) Error locating GoPro files: {err}");
                std::process::exit(1)
            },
            CameraModel::Virb => if let Err(err) = locate_virb::run(&args) {
                println!("(!) Error locating Garmin Virb files: {err}");
                std::process::exit(1)
            },
            CameraModel::Unknown => {
                println!("(!) Failed to determine camera model.");
                std::process::exit(1)
            }
        };

    }


    Ok(())
}