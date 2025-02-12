//! Extracts and converts GoPro GPS log to generic `Point` structs.

use std::path::PathBuf;

use gpmf_rs::{DeviceName, GoProSession};

use crate::geo::EafPoint;

pub fn run(args: &clap::ArgMatches) -> std::io::Result<Vec<EafPoint>> {
    let gpmf_path = args.get_one::<PathBuf>("gpmf").unwrap();
    let indir = args.get_one::<PathBuf>("input-directory");
    let verify_gpmf = *args.get_one::<bool>("verify").unwrap();
    let gpsfix = args.get_one::<u32>("gpsfix"); // default to 3
    let gpsdop = args.get_one::<f64>("gpsdop");

    let gopro_session = GoProSession::from_path(
        gpmf_path,
        indir.map(|p| p.as_path()),
        verify_gpmf,
        true,
        true,
    )?;

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

    // Merge GPMF-streams in session, then export and convert GPS-log.
    // Prune points that do not have at least 2D lock.
    let gps = if matches!(gopro_session.device(), Some(&DeviceName::Hero11Black)) {
        gopro_session.gpmf()?.gps9().prune(gpsfix.copied(), gpsdop.copied())
    } else {
        gopro_session.gpmf()?.gps5().prune(gpsfix.copied(), gpsdop.copied())
    };
    let points: Vec<EafPoint> = gps.iter().map(EafPoint::from).collect();

    Ok(points)
}
