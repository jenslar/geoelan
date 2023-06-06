//! Locate and match GoPro MP4-clips. Currently limited to file name matching since the embedded identifier does contain clip sequence.

use std::{path::PathBuf, time::Instant};

use gpmf_rs::GoProSession;

// MAIN GOPRO LOCATE
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let timer = Instant::now();

    // required arg
    let indir: PathBuf = args.get_one::<PathBuf>("input-directory")
        .unwrap().canonicalize()?;
    let video = args.get_one::<PathBuf>("video");
    let verify_gpmf = *args.get_one::<bool>("verify").unwrap();
    
    let sessions = GoProSession::sessions_from_path(&indir, video.map(|p| p.as_path()), verify_gpmf, true);

    println!("---");
    for (i1, session) in sessions.iter().enumerate() {
        println!("[ Session {} ]", i1+1);
        for (i2, file) in session.iter().enumerate() {
            println!("  {:2}. MP4: {}",
                i2+1,
                file.mp4.as_ref()
                    .and_then(|f| f.to_str())
                    .unwrap_or("High-resolution MP4 not set"));
            println!("      LRV: {}",
            file.lrv.as_ref()
                .and_then(|f| f.to_str())
                .unwrap_or("Low-resolution MP4 not set"));
    }
    }

    println!("Done ({:?}). {}",
        timer.elapsed(),
        if verify_gpmf {" Clips that fail GPMF verification ignored."} else {" Run with '--verify' to skip clips with GPMF errors."}
    );
    Ok(())
}