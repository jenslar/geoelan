use std::{path::PathBuf, io::ErrorKind};

use gpmf_rs::GoProSession;

pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let video = args.get_one::<PathBuf>("video").unwrap().canonicalize()?; // clap: required arg
    let input_dir = match args.get_one::<PathBuf>("input-directory") {
        Some(indir) => indir,
        None => video.parent().ok_or_else(|| {
            let msg = "(!) Failed to determine parent dir for GoPro video";
            std::io::Error::new(ErrorKind::Other, msg)
        })?
    };
    let verify_gpmf = *args.get_one::<bool>("verify").unwrap();
    let single = *args.get_one::<bool>("single").unwrap(); // defaults to false

    let mut gopro_session = if single {
        // Force single-clip session, ignoring other clips in the same session
        GoProSession::single(&video)?
    } else {
        let gopro_sessions = GoProSession::sessions_from_path(input_dir, Some(&video), verify_gpmf, true);
        match gopro_sessions.first() {
            Some(s) => s.to_owned(),
            None => {
                let msg = format!("(!) No recording sessions for {} in {}",
                    video.display(),
                    input_dir.display()
                );
                return Err(std::io::Error::new(ErrorKind::Other, msg))
            }
        }
    };

    super::gopro2eaf_session::run(args, &mut gopro_session)
}