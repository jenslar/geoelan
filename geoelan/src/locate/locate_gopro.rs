//! Locate and match GoPro MP4-clips. Currently limited to file name matching since the embedded identifier does contain clip sequence.

use std::{path::PathBuf, time::Instant};

use walkdir::WalkDir;

use gpmf_rs::GoProSession;

use crate::files::{has_extension, is_hidden};
use crate::model::CameraModel;

// MAIN GOPRO LOCATE
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let timer = Instant::now();

    // required arg
    let indir: PathBuf = args.get_one::<PathBuf>("input-directory")
        .unwrap().canonicalize()?;
    
    let mut sessions: Vec<GoProSession> = Vec::new();

    let mut count = 0;

    for result in WalkDir::new(indir) {
        
        let path = match result {
            Ok(f) => f.path().to_owned(),
            Err(_) => { // ignore errors... usually only lacking permissions for system paths
                continue;
            }
        };

        // Only consider files with mp4-extension (faster than returning parsing error),
        // and completely ignore files in hidden dirs (such as Dropbox placeholders)
        if !has_extension(&path, "mp4") || is_hidden(&path, true) {
            continue
        }

        if let CameraModel::GoPro = CameraModel::from(path.as_path()) {
            count += 1;

            println!("[{count:04}] GOPRO {}", path.display());

            if let Ok(session) = GoProSession::from_path(&path, false, false, false) {
                if !sessions.contains(&session) {
                    sessions.push(session);
                }
            }
        }

    }

    println!("---");
    for (i1, session) in sessions.iter().enumerate() {
        println!("[ Session {} ]", i1+1);
        for (i2, file) in session.iter().enumerate() {
            println!("  {}. {}", i2+1, file.mp4_path.display());
        }
    }

    println!("Done ({:?})", timer.elapsed());
    Ok(())
}