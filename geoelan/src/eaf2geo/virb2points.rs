//! Extracts and converts Garmin VIRB GPS log to generic `Point` structs.

use std::path::PathBuf;

use fit_rs::Fit;
use time::Duration;

use crate::{files::virb::select_session, geo::EafPoint};

pub fn run(args: &clap::ArgMatches) -> std::io::Result<Vec<EafPoint>> {
    let fit_path: &PathBuf = args.get_one("fit").unwrap(); // ensured by clap
    let fit = Fit::new(&fit_path)?;
    let fit_session = select_session(&fit)?;

    let range = fit_session.range();

    // Derive start if FIT-file absolute date time
    // Ignore custom time offset here, since done in mod during point conversion.
    let t0 = fit.t0(0, false)?;

    // TODO use fit_rs::virb::VirbSession::from_uuid() instead
    // let session_timestamps = match derive_session(&mut fit, Some(&uuid), time_offset, false) {
    // let session_timestamps = match derive_session(&mut fit, Some(&uuid), 0, false) {
    //     Ok(t) => t,
    //     Err(err) => {
    //         println!("(!) Unable to determine timespan for specified recording session: {err}");
    //         std::process::exit(1)
    //     }
    // };
    // TODO NOT YET TESTED
    // let start_time = match fit.session_duration(&uuid) {
    let start_time = match fit_session.timespan_rel() {
        Some((start, _)) => start,
        None => {
            println!("(!) Unable to determine start time for session.");
            println!("    Setting start time to 0.");
            Duration::seconds(0)
        }
    };

    // Extract points corresponding to session time span via setting range, derived above.
    let points: Vec<EafPoint> = fit.gps(Some(&range)).map(|gps| {
        gps.iter()
            .map(|p_in| {
                let mut p_out = EafPoint::from_fit(&p_in, Some(t0));
                p_out.timestamp = p_out.timestamp.map(|t| {
                    // Subtract relative start time for session
                    // as logged in the FIT-file to generate timeline
                    // where 0 seconds reflects start of session.
                    let tp = t - start_time;
                    if tp < Duration::ZERO {
                        Duration::ZERO
                    } else {
                        tp
                    }
                });
                p_out
            })
            .collect::<Vec<_>>()
    })?;

    Ok(points)
}
