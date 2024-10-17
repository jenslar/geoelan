//! Locate and match GoPro MP4-clips. Currently limited to file name matching since the embedded identifier does contain clip sequence.

use std::{
    path::{Path, PathBuf},
    time::Instant,
};

use gpmf_rs::{GoProSession, GOPRO_DATETIME_DEFAULT};

fn path2string(path: &Path, count: Option<usize>) -> String {
    if let Some(c) = count {
        format!("{:02}. {}", c + 1, path.display())
    } else {
        format!("{}", path.display())
    }
}

// MAIN GOPRO LOCATE
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let timer = Instant::now();

    // required arg
    let indir: PathBuf = args
        .get_one::<PathBuf>("input-directory")
        .unwrap()
        .canonicalize()?;
    let video = args.get_one::<PathBuf>("video");
    let verify_gpmf = *args.get_one::<bool>("verify").unwrap();
    let verbose = *args.get_one::<bool>("verbose").unwrap();
    let halt_on_error = *args.get_one::<bool>("halt-on-error").unwrap();

    let mut sessions = GoProSession::sessions_from_path(
        &indir,
        video.map(|p| p.as_path()),
        verify_gpmf,
        true,
        !halt_on_error,
    )?;
    // let sessions = GoProSession::sessions_from_path_par(
    //     &indir,
    //     video.map(|p| p.as_path()),
    //     verify_gpmf,
    //     true,
    //     Some(path2string),
    // );
    sessions.sort_by_key(|s| s.start().unwrap_or(GOPRO_DATETIME_DEFAULT)); // Add this to sessions_from_path instead

    println!("---");
    for (i1, session) in sessions.iter().enumerate() {
        println!(
            "┏━[ Session {} | {} {} - {} ({}sec)]",
            i1 + 1,
            session
                .start()
                .map(|t| t.date().to_string())
                .unwrap_or("Failed to determine start date".to_owned()),
            session
                .start()
                .map(|t| t.time().to_string())
                .unwrap_or("Failed to determine start time".to_owned()),
            session
                .end()
                .map(|t| t.time().to_string())
                .unwrap_or("Failed to determine end time".to_owned()),
            // session.duration()?.as_seconds_f32(),
            session.duration().as_seconds_f32(),
        );
        for (i2, file) in session.iter().enumerate() {
            if verbose {
                println!(
                    "┃{:2}. MUID: {:?}\n┃    GUMI: {:?}\n┃    DATE: {}\n┃     1FR: {}",
                    i2 + 1,
                    file.muid,
                    file.gumi,
                    file.start().to_string(),
                    file.first_frame().to_string()
                );
                println!(
                    "┃     MP4: {}",
                    file.mp4
                        .as_ref()
                        .and_then(|f| f.to_str())
                        .unwrap_or("High-resolution MP4 not found")
                );
            } else {
                println!(
                    "┃{:2}.  MP4: {}",
                    i2 + 1,
                    file.mp4
                        .as_ref()
                        .and_then(|f| f.to_str())
                        .unwrap_or("High-resolution MP4 not found")
                );
            }
            println!(
                "┃     LRV: {}",
                file.lrv
                    .as_ref()
                    .and_then(|f| f.to_str())
                    .unwrap_or("Low-resolution MP4 not found")
            );
        }
        println!("┗━━━━");
    }

    println!(
        "Done ({:?}). {}",
        timer.elapsed(),
        if verify_gpmf {
            " Clips that fail GPMF verification ignored."
        } else {
            " Run with '--verify' to skip clips with GPMF errors."
        }
    );
    println!("Sessions are sorted by time for start of recording, but may be misreprepresentative, depending on camera setup.");

    Ok(())
}
