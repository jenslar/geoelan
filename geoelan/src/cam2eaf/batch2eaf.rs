//! Batch concatenating clips and generating ELAN-files.
//! Invoked via '--batch' argument.

use std::{io::ErrorKind, path::PathBuf};

use fit_rs::VirbSession;
use gpmf_rs::GoProSession;

use super::gopro2eaf_session;
use super::virb2eaf_session;

/// Batch concatenating clips and generating ELAN-files.
/// Invoked via '--batch' argument.
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let indir = args
        .get_one::<PathBuf>("input-directory")
        .unwrap_or(&PathBuf::default())
        .to_owned();

    // 1. determine model (gopro/virb)
    match args.get_one::<String>("batch").map(|s| s.as_str()) {
        // Batch GoPro sessions
        Some("g" | "gopro") => {
            let sessions = GoProSession::sessions_from_path(&indir, None, false, true);
            for (i, session) in sessions.iter().enumerate() {
                println!("--[Session {:02}.]--------", i + 1);
                match gopro2eaf_session::run(args, session) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("(!) Failed to process GoPro session: {err}");
                        continue;
                    }
                }
                println!("-----------------------\n");
            }

            Ok(())
        }
        // Batch VIRB sessions
        Some("v" | "virb") => {
            let mut sessions = VirbSession::sessions_from_path(&indir, true);
            for (i, session) in sessions.iter_mut().enumerate() {
                println!("--[Session {:02}.]--------", i + 1);
                match virb2eaf_session::run(args, session) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("(!) Failed to process VIRB session: {err}");
                        continue;
                    }
                }
            }

            Ok(())
        }
        // clap should catch this
        Some(m) => {
            return Err(std::io::Error::new(
                ErrorKind::Other,
                format!("(!) Unknown device '{m}'"),
            ))
        }
        // No batch, single session
        None => Ok(()),
    }
}
