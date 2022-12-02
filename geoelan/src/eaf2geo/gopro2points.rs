//! Extracts and converts GoPro GPS log to generic `Point` structs.

use std::path::PathBuf;

use gpmf_rs::GoProSession;

pub fn run(args: &clap::ArgMatches) -> std::io::Result<Vec<crate::geo::point::Point>> {

    let gpmf_path = args.get_one::<PathBuf>("gpmf").unwrap();
    let mut session = GoProSession::from_path(gpmf_path, false, true, false)?;

    // Merge GPMF-streams in session, then export and convert GPS-log
    let points: Vec<crate::geo::point::Point> = session.gpmf().gps().iter()
        .map(crate::geo::point::Point::from)
        .collect();

    Ok(points)
}