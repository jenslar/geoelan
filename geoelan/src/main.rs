//! GeoElan is a tool that geo-references ELAN annotations for Garmin VIRB footage
use chrono::Local;
use clap::{App, AppSettings, Arg, SubCommand};

// local - subcommands
mod cam2eaf;
mod eaf2geo;
mod fitcheck;
mod fitmatch;

// local - support
mod ffmpeg;
mod geo;
mod kml;
mod manual;
mod files;
mod virb;
mod structs;

fn main() -> std::io::Result<()> {
    let about = format!(
        "{}

GeoElan is a tool for geo-referencing ELAN-annotations (https://archive.mpi.nl/tla/elan)
of Garmin VIRB footage. Additional functionality includes inspecting the contents
of FIT-files or matching these with the corresponding VIRB video clips.
Some functionality requires FFmpeg. Refer to the geoelan manual for further information.

MANUAL:
    - Print to screen:     geoelan manual
    - Save as PDF:         geoelan manual --pdf

HELP:
    - Specific subcommand: geoelan help <subcommand>
    - e.g.                 geoelan help eaf2geo

PUBLICATION:
    - https://doi.org/10.1080/13645579.2020.1763705

---",
        Local::now().format("%Y-%m-%d").to_string()
    );
    let args = App::new("geoelan")
        .version("1.0.0")
        .author("Jens Larsson")
        .about(&about[..])
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("manual")
            .about("Print the manual to screen or save as a file to disk")
            .display_order(1)
            .arg(Arg::with_name("pdf")
                .help("Save the full manual as a PDF to current directory")
                .long("pdf"))
            .arg(Arg::with_name("pdf-a4")
                .help("Save the A4-guide as a PDF to current directory")
                .long("pdf-a4"))
        )
        .subcommand(SubCommand::with_name("cam2eaf")
            .about("Concatenates Garmin VIRB video clips and generates an ELAN-file with these pre-linked. Also locates the FIT-file in '--indir' if not specified. A KML-file for the specified session is generated. Use '--sample-factor' for smaller KML-files. Requires FFmpeg.")
            // .setting(AppSettings::SubcommandRequiredElseHelp)
            .display_order(2)
            .arg(Arg::with_name("fit")
                .help("VIRB FIT-file")
                .short("f")
                .long("fit")
                .takes_value(true)
                .conflicts_with_all(&["video", "uuid"])
                .required_unless_one(&["video", "uuid"]))
            .arg(Arg::with_name("uuid")
                .help("UUID for first VIRB clip in a session")
                .short("u")
                .long("uuid")
                .takes_value(true)
                .conflicts_with_all(&["video", "fit"])
                .required_unless_one(&["video", "fit"]))
            .arg(Arg::with_name("video")
                .help("First VIRB clip in a session")
                .short("v")
                .long("video")
                .takes_value(true)
                .conflicts_with_all(&["uuid", "fit"])
                .required_unless_one(&["uuid", "fit"]))
            .arg(Arg::with_name("low-res-only")
                .help("Only concatenate low resolution clips (.GLV)")
                .short("l")
                .long("low-res-only"))
            .arg(Arg::with_name("mpeg2")
                .help("Use mpeg2 compression for low-resolution video output (.GLV)")
                .long("mpeg2"))
            .arg(Arg::with_name("copy")
                .help("Copy, do not concatenate, high resolution clips")
                .long("copy")
                .requires("low-res-only"))
            .arg(Arg::with_name("ffmpeg")
                .help("Custom path to FFmpeg")
                .long("ffmpeg")
                .default_value(if cfg!(windows) {"ffmpeg.exe"} else {"ffmpeg"})
                .takes_value(true))
            .arg(Arg::with_name("time-offset")
                .help("Time offset, +/- hours")
                .short("t")
                .long("time-offset")
                .takes_value(true)
                .allow_hyphen_values(true) // negative values and value > 24 ok
                .default_value("0"))
            .arg(Arg::with_name("downsample-factor")
                .help("Downsample factor for coordinates. Must be a positive value.")
                .short("d")
                .long("downsample")
                .default_value("1")) // default = every point
            .arg(Arg::with_name("no-metadata")
                .help("Do not embed FIT metadata in output MP4")
                .short("n")
                .long("no-meta"))
            .arg(Arg::with_name("force")
                .help("Try forcing a partial FIT data extraction if the process fails.")
                .long("force"))
            .arg(Arg::with_name("input-directory")
                .help("Input path for locating files")
                .short("i")
                .long("indir")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("output-directory")
                .help("Output path for resulting files")
                .short("o")
                .long("outdir")
                .default_value("OUTPUT"))
            .arg(Arg::with_name("geotier")
                .help("Insert tier with synchronised coordinates in ELAN-file")
                .long("geotier"))
            .arg(Arg::with_name("quiet")
                .help("Do not print file-by-file search progress")
                .long("quiet"))
        )
        .subcommand(SubCommand::with_name("eaf2geo")
            .about("Geo-references ELAN annotations by matching and synchronising these against the GPS data in the corresponding FIT-file. Use '--geoshape' to specify whether the generated KML-file should contain polylines or points.")
            // .setting(AppSettings::SubcommandRequiredElseHelp)
            .display_order(3)
            .arg(Arg::with_name("fit")
                .help("VIRB FIT-file")
                .short("f")
                .long("fit")
                .required(true) // required for the moment, add locating fit later
                .takes_value(true))
            .arg(Arg::with_name("eaf")
                .help("ELAN-file")
                .short("e")
                .long("eaf")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("uuid")
                .help("UUID for first VIRB clip in a session")
                .short("u")
                .long("uuid")
                .conflicts_with("video")
                // .conflicts_with("select")
                .takes_value(true))
            .arg(Arg::with_name("video")
                .help("First VIRB clip in a session")
                .short("v")
                .long("video")
                .conflicts_with("uuid")
                // .conflicts_with("select")
                .takes_value(true))
            // .arg(Arg::with_name("select")
            //     .long("select")
            //     .conflicts_with("uuid")
            //     .conflicts_with("video")
            //     .help("Select UUID corresponding to first clip in session from list"))
            .arg(Arg::with_name("time-offset")
                .help("Time offset, +/- hours")
                .short("t")
                .long("time-offset")
                .takes_value(true)
                .allow_hyphen_values(true) // negative values and value > 24 ok
                .default_value("0"))
            .arg(Arg::with_name("downsample-factor")
                .help("Downsample factor for coordinates. Must be a positive value.")
                .short("d")
                .long("downsample")
                .default_value("1"))
            .arg(Arg::with_name("geoshape")
                .help("Output options for KML-file")
                .long("geoshape")
                .default_value("point-all")
                .possible_values(&[
                    "point-all", "point-multi", "point-single",
                    "line-all", "line-multi",
                    ]))
            .arg(Arg::with_name("cdata")
                .help("KML-option, added visuals in Google Earth")
                .long("cdata"))
            .arg(Arg::with_name("force")
                .help("Try forcing a partial FIT data extraction if the process fails.")
                .long("force"))
        )
        .subcommand(SubCommand::with_name("match")
            .about("Locate and match VIRB FIT-files and video clips.")
            // .setting(AppSettings::SubcommandRequiredElseHelp)
            // TODO 200816 add possibility to specify any clip in session, not only first
            .display_order(4)
            .arg(Arg::with_name("input-directory")
                .help("Input path for locating files")
                .short("i")
                .long("indir")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("video")
                .help("First VIRB clip in a session")
                .short("v")
                .long("video")
                .takes_value(true)
                .conflicts_with("uuid"))
            .arg(Arg::with_name("uuid")
                .help("UUID for first VIRB clip in a session")
                .short("u")
                .long("uuid")
                .takes_value(true)
                .conflicts_with("video"))
            .arg(Arg::with_name("fit")
                .help("VIRB FIT-file for selecting session")
                .short("f")
                .long("fit")
                .takes_value(true)
                .conflicts_with("uuid")
                .conflicts_with("video"))
            .arg(Arg::with_name("duplicates")
                .help("Include duplicate files in match results")
                .long("duplicates"))
            .arg(Arg::with_name("write-csv")
                .help("Write result to CSV plain-text file")
                .long("csv"))
            .arg(Arg::with_name("quiet")
                .help("Do not print file-by-file search progress")
                .long("quiet"))
        )
        .subcommand(SubCommand::with_name("check")
            .about("Inspect the contents of a FIT-file.")
            // .setting(AppSettings::SubcommandRequiredElseHelp)
            .display_order(5)
            .arg(Arg::with_name("fit")
                .help("FIT-file")
                .short("f")
                .long("fit")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("global-id")
                .help("FIT data type to print (see Profile.xlsx in FIT SDK)")
                .short("g")
                .long("global-id")
                .takes_value(true))
            .arg(Arg::with_name("video")
                .help("Garmin VIRB clip, first clip in session")
                .short("v")
                .long("video")
                .takes_value(true)
                .conflicts_with("uuid"))
            .arg(Arg::with_name("uuid")
                .help("UUID for first clip in a session")
                .short("u")
                .long("uuid")
                .takes_value(true)
                .conflicts_with("video"))
            .arg(Arg::with_name("select")
                .help("Select session from those present in the FIT-file")
                .short("s")
                .long("select")
                .conflicts_with("uuid")
                .conflicts_with("video"))
            .arg(Arg::with_name("verbose")
                .help("Print FIT-data to screen, `--global-id` sets this automatically")
                .long("verbose"))
            .arg(Arg::with_name("kml")
                .help("Generate a KML-file")
                .long("kml"))
            .arg(Arg::with_name("debug")
                .help("Print FIT definitions and data while parsing")
                // .hidden(true)
                .long("debug"))
            .arg(Arg::with_name("debug-unchecked")
                .help("Print FIT definitions and data while parsing, but strings are also unchecked UTF-8")
                // .hidden(true)
                .long("debug-unchecked"))
            .arg(Arg::with_name("downsample-factor")
                .help("Downsample factor for coordinates. Must be a positive value.")
                .short("d")
                .long("downsample")
                .default_value("1")) // default = every point
        )
        .get_matches();

    //////////////////////////
    // PRINT OR SAVE MANUAL //
    //////////////////////////
    if let Some(arg_matches) = args.subcommand_matches("manual") {
        manual::run(&arg_matches)?;
    }

    ////////////////////////////////////////
    // MATCH: MATCH FIT AND MP4/GLV FILES //
    ////////////////////////////////////////
    if let Some(arg_matches) = args.subcommand_matches("match") {
        fitmatch::run(&arg_matches)?;
    }

    //////////////////////////////////////////////////
    // CHECK: PRINT VALUES FOR SPECIFIC FIT MESSAGE //
    //////////////////////////////////////////////////
    if let Some(arg_matches) = args.subcommand_matches("check") {
        fitcheck::run(&arg_matches)?;
    }

    //////////////////////////////////////////////////////////////////////////////////////
    // CAM2EAF: Generate ELAN-file (with/without geo tier) and concatenate VIRB session //
    //////////////////////////////////////////////////////////////////////////////////////
    if let Some(arg_matches) = args.subcommand_matches("cam2eaf") {
        cam2eaf::run(&arg_matches)?;
    }

    ///////////////////////////////////////////////////////////////
    // EAF2GEO: Map EAF annotations to coordinates, generate KML //
    ///////////////////////////////////////////////////////////////
    if let Some(arg_matches) = args.subcommand_matches("eaf2geo") {
        eaf2geo::run(&arg_matches)?;
    }

    Ok(())
}
