//! Extracts and converts Garmin VIRB GPS log to generic `Point` structs.

use std::path::PathBuf;

use time::Duration;
use fit_rs::Fit;

use crate::files::virb::select_session;

pub fn run(args: &clap::ArgMatches) -> std::io::Result<Vec<crate::geo::point::Point>> {

    // let fit_path = args.value_of("fit").map(Path::new).unwrap(); // ensured by clap
    let fit_path: &PathBuf = args.get_one("fit").unwrap(); // ensured by clap
    let fit = match Fit::new(&fit_path) {
        Ok(f) => f,
        Err(err) => {
            println!("(!) Failed to parse {}: {err}", fit_path.display());
            std::process::exit(1)
        }
    };
    // if let Err(err) = fit.index() {
    //     println!("(!) Failed to map sessions for {}: {err}", fit_path.display());
    //     std::process::exit(1)
    // };

    let fit_session = match select_session(&fit) {
        Ok(session) => session,
        Err(err) => {
            println!("(!) Error reading input: {err}");
            std::process::exit(1)
        }
    };

    let range = fit_session.range();

    // Derive start if FIT-file absolute date time
    // Ignore custom time offset here, since done in mod during point conversion.
    let t0 = match fit.t0(0, false) {
        Ok(t) => t,
        Err(err) => {
            println!("(!) Failed to determine absolute date time: '{err}'");
            std::process::exit(1)
        }
    };

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
    let start_time = match fit_session.duration() {
        Some((start, _)) => start,
        None => {
            println!("(!) Unable to determine start time for session.");
            println!("    Setting start time to 0.");
            Duration::seconds(0)
        }
    };

    // Extract points corresponding to session time span via setting range, derived above.
    let points: Vec<crate::geo::point::Point> = fit.gps(Some(&range))
        .map(|gps| 
            gps.iter().map(|p_in| {
                let mut p_out = crate::geo::point::Point::from_fit(&p_in, Some(t0));
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
        )?;

    Ok(points)
}