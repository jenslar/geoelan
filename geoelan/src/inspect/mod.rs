//! Inspect camera telemetry, such as GPS logs.

use std::path::PathBuf;

mod inspect_gpmf;
mod inspect_fit;

pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {

    if args.get_one::<PathBuf>("gpmf").is_some() {
        inspect_gpmf::inspect_gpmf(args)?
    } else {
        inspect_fit::inspect_fit(args)?
    }

    Ok(())
}