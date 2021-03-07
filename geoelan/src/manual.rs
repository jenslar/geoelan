use crate::files::writefile;
use std::path::PathBuf;

///////////////////////////
// MAIN MANUAL SUB-COMMAND
///////////////////////////
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let outpath: PathBuf;
    let content: &[u8];

    if args.is_present("pdf") {
        content = include_bytes!("../../doc/geoelan.pdf");
        outpath = PathBuf::from("geoelan.pdf");
    } else if args.is_present("pdf-a4") {
        content = include_bytes!("../../doc/geoelan-a4.pdf");
        outpath = PathBuf::from("geoelan-a4.pdf");
    } else {
        println!("{}", include_str!("../../doc/geoelan.txt"));
        std::process::exit(0)
    }

    // write selected file to disk, asks for confirmation if file exists
    writefile(&content, &outpath)?;

    Ok(())
}
