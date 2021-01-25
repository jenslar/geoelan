use std::path::PathBuf;
use crate::files::writefile;

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

    // write selected file to disk
    writefile(&content, &outpath)?;

    Ok(())
}
