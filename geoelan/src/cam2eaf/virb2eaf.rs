use std::io::ErrorKind;
use std::path::PathBuf;

use fit_rs::{Fit, VirbSession};

use crate::files::virb::select_session;

pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    // Options
    let fit_path = args.get_one::<PathBuf>("fit"); // required unless video, uuid
    let video_path: Option<&PathBuf> = args.get_one("video"); // required unless fit, uuid
    let uuid = args.get_one::<String>("uuid"); // required unless video, fit
    let input_dir: &PathBuf = args.get_one("input-directory").unwrap();

    println!("Determining recording session data...");
    let virb_session_result = match (fit_path, video_path, uuid) {
        (Some(p), None, None) => {
            let fit = Fit::new(&p)?;
            let fit_session = select_session(&fit)?;

            let uuid = match fit_session.uuid.get(0) {
                Some(u) => u,
                None => {
                    let msg = "(!) Failed to determine UUID.";
                    return Err(std::io::Error::new(ErrorKind::Other, msg))
                }
            };
            VirbSession::from_uuid(uuid, input_dir, true)
        }
        (None, Some(p), None) => VirbSession::from_mp4(p, input_dir, true),
        (None, None, Some(s)) => VirbSession::from_uuid(s, input_dir, true),
        _ => {
            let msg = "(!) Failed to determine recording session.";
            return Err(std::io::Error::new(ErrorKind::Other, msg))
        }
    };

    match virb_session_result {
        Some(s) => super::virb2eaf_session::run(args, &mut s.to_owned()),
        None => {
            let msg = "(!) Failed to determine recording session. At least one of 'video', 'fit, 'uuid' must be specified.";
            Err(std::io::Error::new(ErrorKind::Other, msg)
        )}
    }
}
