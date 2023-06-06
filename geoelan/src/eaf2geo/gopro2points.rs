//! Extracts and converts GoPro GPS log to generic `Point` structs.

use std::{path::PathBuf, io::ErrorKind};

use gpmf_rs::{DeviceName, GoProSession};

use crate::geo::EafPoint;

pub fn run(args: &clap::ArgMatches) -> std::io::Result<Vec<EafPoint>> {
    let gpmf_path = args.get_one::<PathBuf>("gpmf").unwrap();
    let indir = args.get_one::<PathBuf>("input-directory");
    let verify_gpmf = *args.get_one::<bool>("verify").unwrap();
    let gpsfix = *args.get_one::<u32>("gpsfix").unwrap(); // default to 3

    let gopro_session =
        match GoProSession::from_path(gpmf_path, indir.map(|p| p.as_path()), verify_gpmf, true) {
            Some(session) => session,
            None => {
                let msg = "(!) Failed to determine GoPro session";
                return Err(std::io::Error::new(ErrorKind::Other, msg))
            }
        };

    println!("Using data from the following session:");
    for (i, gp) in gopro_session.iter().enumerate() {
        println!(
            "{:4}. MP4: {}\n      LRV: {}",
            i + 1,
            gp.mp4
                .as_deref()
                .map(|p| p.display().to_string())
                .as_deref()
                .unwrap_or("None"),
            gp.lrv
                .as_deref()
                .map(|p| p.display().to_string())
                .as_deref()
                .unwrap_or("None"),
        )
    }

    // Merge GPMF-streams in session, then export and convert GPS-log
    let gps = if matches!(gopro_session.device(), Some(&DeviceName::Hero11Black)) {
        gopro_session.gpmf()?.gps9().prune(gpsfix, None) // prune points that do not have at least 2D lock
    } else {
        gopro_session.gpmf()?.gps5().prune(gpsfix, None) // prune points that do not have at least 2D lock
    };
    let points: Vec<EafPoint> = gps.iter().map(EafPoint::from).collect();

    Ok(points)
}
