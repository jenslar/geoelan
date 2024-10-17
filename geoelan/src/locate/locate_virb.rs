//! Locate and match Garmin VIRB MP4-clips. Uses embedded UUID to derive clip sequence, regardless of file name.

use std::time::Instant;
use std::{io::ErrorKind, path::PathBuf};

use fit_rs::{Fit, VirbSession, FIT_DEFAULT_DATETIME};

use crate::files::virb::select_session;

// MAIN VIRB LOCATE
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let timer = Instant::now();

    let indir = args.get_one::<PathBuf>("input-directory").unwrap(); // required arg
    let video_path_opt = args.get_one::<PathBuf>("video");
    let fit_path_opt = args.get_one::<PathBuf>("fit");
    let uuid_opt = args.get_one::<String>("uuid");
    let verbose = *args.get_one::<bool>("verbose").unwrap();

    let session = match (video_path_opt, fit_path_opt, uuid_opt) {
        (Some(path), ..) => VirbSession::from_mp4(path, indir, true),
        (_, Some(path), _) => {
            let fit = Fit::parse(path, Some(161), false)?; // only need camera_event/161
            let fit_session = select_session(&fit)?;

            // Any UUID in the session is fine,
            // but a recording session
            // must consist of at least one video clip,
            // and thus have at least corresponding UUID.
            let uuid = fit_session.uuid.get(0);

            match uuid {
                Some(u) => VirbSession::from_uuid(u, indir, true),
                None => None,
            }
        }
        (.., Some(string)) => VirbSession::from_uuid(string, indir, true),
        _ => None,
    };

    // Check if session was specified and found...
    let session_specified =
        video_path_opt.is_some() || fit_path_opt.is_some() || uuid_opt.is_some();
    let session_found = session.is_some();

    // ...and exit if not
    if session_specified && !session_found {
        let msg = "(!) No files could be located for specified recording session.";
        return Err(std::io::Error::new(ErrorKind::Other, msg));
    }

    let mut sessions = match session {
        Some(s) => vec![s],
        None => VirbSession::sessions_from_path(&indir, true),
    };

    sessions.sort_by_key(|v| v.start().unwrap_or_else(|| FIT_DEFAULT_DATETIME));

    println!("---");
    for (i1, session) in sessions.iter().enumerate() {
        // println!("[ Session {} ]\n      FIT: {}", i1+1, session.fit.path.display());
        println!(
            "┏━[ Session {} {} - {} ({}sec)]",
            i1 + 1,
            session
                .start()
                .map(|t| t.to_string())
                .unwrap_or("Failed to determine start".to_owned()),
            session
                .end()
                .map(|t| t.to_string())
                .unwrap_or("Failed to determine end".to_owned()),
            session
                .video_duration()
                .map(|t| t.as_seconds_f32().to_string())
                .unwrap_or("Failed to determine duration".to_owned()),
        );

        println!("┃ FIT       {}", session.fit_path().display());
        println!("┠─────");

        for (i2, virbfile) in session.virb.iter().enumerate() {
            if verbose {
                println!("┃{:3}. UUID: {}", i2 + 1, virbfile.uuid);
                println!(
                    "┃     DATE: {}",
                    virbfile
                        .created()
                        .map(|t| t.to_string())
                        .unwrap_or("Could not determine creation time".to_owned())
                );
                println!(
                    "┃      MP4: {}",
                    virbfile
                        .mp4()
                        .and_then(|f| f.to_str())
                        .unwrap_or("High-resolution MP4 not found")
                );
            } else {
                println!(
                    "┃{:3}.  MP4: {}",
                    i2 + 1,
                    virbfile
                        .mp4()
                        .and_then(|f| f.to_str())
                        .unwrap_or("High-resolution MP4 not found")
                );
            }
            println!(
                "┃      GLV: {}",
                virbfile
                    .glv()
                    .and_then(|f| f.to_str())
                    .unwrap_or("Low-resolution MP4 not found")
            );
        }
        println!("┗━━━━");
    }

    println!("Done ({:?})", timer.elapsed());
    println!("Sessions are sorted by time for start of recording, but may be misreprepresentative, depending on camera setup.");

    Ok(())
}
