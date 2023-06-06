//! Locate and match Garmin VIRB MP4-clips. Uses embedded UUID to derive clip sequence, regardless of file name.

use std::{path::PathBuf, io::ErrorKind};
use std::time::Instant;

use fit_rs::{Fit, VirbSession};

use crate::files::virb::select_session;

// MAIN VIRB LOCATE
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let timer = Instant::now();

    let indir = args.get_one::<PathBuf>("input-directory").unwrap(); // required arg
    let video_path_opt = args.get_one::<PathBuf>("video");
    let fit_path_opt = args.get_one::<PathBuf>("fit");
    let uuid_opt = args.get_one::<String>("uuid");

    let session = match (video_path_opt, fit_path_opt, uuid_opt) {
        (Some(path), ..) => VirbSession::from_mp4(path, indir, true),
        (_, Some(path), _) => {
            let fit = Fit::parse(path, Some(161))?; // only need camera_event/161
            let fit_session = select_session(&fit)?;
            
            // Any UUID in the session is fine,
            // but a recording session
            // must consist of at least one video clip,
            // and thus have at least corresponding UUID.
            let uuid = fit_session.uuid.get(0);

            match uuid {
                Some(u) => VirbSession::from_uuid(u, indir, true),
                None => None
            }
        },
        (.., Some(string)) => VirbSession::from_uuid(string, indir, true),
        _ => None
    };

    // Check if session was specified and found...
    let session_specified = video_path_opt.is_some()
        || fit_path_opt.is_some()
        || uuid_opt.is_some();
    let session_found = session.is_some();

    // ...and exit if not
    if session_specified && !session_found {
        let msg = "(!) No files could be located for specified recording session.";
        return Err(std::io::Error::new(ErrorKind::Other, msg))
    }

    let sessions = match session {
        Some(s) => vec![s],
        None => VirbSession::sessions_from_path(&indir, true)
    };

    println!("---");
    for (i1, session) in sessions.iter().enumerate() {
        println!("[ Session {} ]\n  FIT-file: {}", i1+1, session.fit.path.display());
        for (i2, virbfile) in session.virb.iter().enumerate() {
            println!(" {:2}. UUID: {}", i2+1, virbfile.uuid);
            println!("      MP4: {}", virbfile.mp4()
                .and_then(|f| f.to_str())
                .unwrap_or("High-resolution MP4 not set"));
            println!("      GLV: {}", virbfile.glv()
                .and_then(|f| f.to_str())
                .unwrap_or("Low-resolution MP4 not set"));
        }
    }

    println!("Done ({:?})", timer.elapsed());

    Ok(())
}