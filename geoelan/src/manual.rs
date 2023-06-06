//! GeoELAN manual embedding.

use std::{path::Path, io::ErrorKind};

// MAIN MANUAL SUB-COMMAND
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    // let outpath: PathBuf;
    let outpath: &Path;
    let content: &[u8];

    if Some(&true) == args.get_one::<bool>("pdf") {
        content = include_bytes!("../../doc/pdf/geoelan.pdf");
        outpath = Path::new("geoelan.pdf");
    } else if Some(&true) == args.get_one::<bool>("pdf-a4") {
        content = include_bytes!("../../doc/pdf/geoelan-a4.pdf");
        outpath = Path::new("geoelan-a4.pdf");
    } else {
        println!("{}", include_str!("../../doc/txt/geoelan.txt"));
        return Ok(())
    }

    // write selected file to disk, asks for confirmation if file exists
    match crate::files::writefile(&content, &outpath) {
        Ok(true) => println!("Wrote {}", outpath.display()),
        Ok(false) => println!("User aborted writing documentation."),
        Err(err) => {
            let msg = format!("(!) Failed to write '{}': {err}", outpath.display());
            return Err(std::io::Error::new(ErrorKind::Other, msg))
        },
    }

    Ok(())
}