use std::{path::PathBuf, process::ExitCode};

use clap::{Arg, Command, builder::PossibleValuesParser, ArgAction};
use time::OffsetDateTime;

use kml;

mod eaf2geo;
mod cam2eaf;
mod inspect;
mod plot;
mod locate;
mod manual;
mod geo;
mod elan;
mod files;
mod media;
mod text;
mod model;

// fn main() -> std::io::Result<()> {
fn main() -> ExitCode {
    let about = format!(
        "GeoELAN, build: {}
- Source:        https://github.com/jenslar/geoelan
- Documentation: https://github.com/jenslar/geoelan/mdbook

GeoELAN is a tool for annotating action camera GPS logs using the free annotation software ELAN. Supported cameras are a recent GoPro or a Garmin VIRB Ultra 30. Additional functionality includes inspecting and plotting data in GoPro's GPMF-format and Garmin FIT-files, and also to automatically locate and group video clips by recording session. Refer to the manual for further information.

IMPORTANT:
  Keep your original files (renaming is fine). Concatenating/converting video clips will
  discard embedded data, such as GPS-logs and identifiers.

REQUIREMENTS:
- FFmpeg:              https://ffmpeg.org ('cam2eaf')
- ELAN:                https://archive.mpi.nl/tla/elan

HELP:
- Specific subcommand: geoelan help <subcommand>
- Example:             geoelan help eaf2geo

MANUAL:
- Print to screen:     geoelan manual
- Save as PDF:         geoelan manual --pdf

PUBLICATION:
- https://doi.org/10.1080/13645579.2020.1763705

---",
        OffsetDateTime::now_utc().date().to_string()
    );

    let args = Command::new("geoelan")

        .version("2.5")
        .author("Jens Larsson")
        .about(about)
        .term_width(80)
        .arg_required_else_help(true)

        // // Print or save manual
        // .subcommand(Command::new("manual")
        //     .about("Print the manual or save as a file to disk.")
        //     .visible_alias("m")
        //     .arg(Arg::new("pdf")
        //         .help("Save the full manual as a PDF to current directory.")
        //         .long("pdf")
        //         .action(clap::ArgAction::SetTrue))
        //         .arg(Arg::new("pdf-a4")
        //         .help("Save the A4-guide as a PDF to current directory.")
        //         .long("pdf-a4")
        //         .action(clap::ArgAction::SetTrue))
        // )

        .subcommand(Command::new("cam2eaf")
            .about("Generate an ELAN-file from GoPro/VIRB footage, with or without coordinates inserted as a tier. Requires FFmpeg for concatenating clips.")
            .visible_alias("c2e")

            .next_help_heading("VIRB")
            .arg(Arg::new("fit")
                .help("VIRB FIT-file to use for locating MP4-clips.")
                .short('f')
                .long("fit")
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with_all(&[
                    "video", "uuid"    // VIRB only args
                ])
                .required_unless_present_any(&[
                    "video",
                    "uuid",
                    "batch",
                ]))
            .arg(Arg::new("uuid")
                .help("UUID for a VIRB clip in a session")
                .short('u')
                .long("uuid")
                .conflicts_with_all(&[
                    "video",                // either camera
                    "fit",                  // VIRB only
                    "verify-gpmf", "gpsfix", // GoPro only
                    "batch",
                ])
                .required_unless_present_any(&["video", "fit", "batch"]))
            
            .next_help_heading("GoPro")
            .arg(Arg::new("verify-gpmf")
                .help("Verifies GPMF data and discards corrupt clips before grouping as session.")
                .long("verify-gpmf")
                .conflicts_with_all(&[
                    "fit", "uuid" // VIRB only args
                ])
                .action(ArgAction::SetTrue))
            .arg(Arg::new("gpsfix")
                .help("Filter GPS-log to only include points with a specific number of satellites locked. 0 = No lock (all points), 2 = 2D lock, 3 = 3D lock.")
                .long("gpsfix")
                .default_value("3") // 3D lock for eaf
                .conflicts_with_all(&[
                    "fit", "uuid" // VIRB only args
                ])
                .value_parser(clap::value_parser!(u32)))

            .next_help_heading("General")
            .arg(Arg::new("video")
                .help("Unaltered GoPro/VIRB MP4 file used to determine remaining clips in session.")
                .long("video")
                .short('v')
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present("batch"))
            .arg(Arg::new("ffmpeg")
                .help("Custom path to FFmpeg.")
                .long("ffmpeg")
                .value_parser(clap::value_parser!(PathBuf))
                .default_value(if cfg!(windows) {"ffmpeg.exe"} else {"ffmpeg"}))
            .arg(Arg::new("low-res-only")
                .help("Only concatenate low resolution clips (.LRV/.GLV).")
                .short('l')
                .long("low-res-only")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("link-high-res")
                .help("Link high-resolution video in ELAN-file.")
                .long("link-high-res")
                .conflicts_with("low-res-only")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("time-offset")
                .help("Time offset, +/- hours. Modifies logged timestamps.")
                .long("time-offset")
                .short('t')
                .value_parser(clap::value_parser!(isize))
                .allow_hyphen_values(true) // negative values and value > 24 ok
                .default_value("0"))
            .arg(Arg::new("input-directory")
                .help("Input path for locating GoPro/VIRB MP4 clips.")
                .long("indir")
                .short('i')
                .value_parser(clap::value_parser!(PathBuf)))
            .arg(Arg::new("output-directory")
                .help("Output path for resulting files.")
                .long("outdir")
                .short('o')
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("geoelan"))
            .arg(Arg::new("geotier")
                .help("Insert tier with synchronised coordinates in ELAN-file.")
                .long("geotier")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("fullgps")
                .help("Use the full GPS log for the ELAN geotier. Results in large ELAN-files.")
                .long("fullgps")
                .requires("geotier")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("single")
                .help("Use only the clip specified. Does not attempt to locate remaining clips in session.")
                .long("single")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("batch")
                .help("Process all encountered files for specified camera model.")
                .long("batch")
                .value_parser(PossibleValuesParser::new([
                    "v", "virb",
                    "g", "gopro",
                ]))
                .conflicts_with_all(&[
                    "single", "video",
                    "uuid",
                    "fit" // TODO all sessions in specified fit
                ]))
            .arg(Arg::new("dryrun")
                .help("Only show results, does not concatenate video or generate ELAN-file.")
                .long("dryrun")
                .action(ArgAction::SetTrue))
        )

        // 'point-all':   Points. Includes all points, meaning some will not have a description value.
        // 'point-multi': Points. Only includes points that intersect with an annotation value.
        // 'line-all':    Continuous poly-line. Includes all points, meaning some segments
        //                will not have a description value.
        // 'line-multi':  Segmented poly-line. Only includes points that intersect with an annotation value.
        // 'circle-2d':   Generates a flat circle around an average point derived from those logged within
        //                each annotation's timespan.
        // 'circle-3d':   Generates an extruded circle around an average point derived from those logged within
        //                each annotation's timespan. Extrusion height is equal to the altitude value,
        //                relative to ground.

        // Defaults for circles (customizable):
        // radius:        2 meters
        // vertices:      40 (valid values are 3 - 255)
        // Generate KML and GeoJson from EAF
        .subcommand(Command::new("eaf2geo")
            .about("Generate KML and GeoJson from specified ELAN-file. Use the 'geoshape' option to specify feature type (point, polyline, or circle) for output KML and GeoJSON files. ELAN annotation values become descriptions if a logged point's timstamp intersects with the annotation timespan.")
            .visible_alias("e2g")

            .next_help_heading("General")
            .arg(Arg::new("eaf")
                .help("ELAN-file")
                .long("eaf")
                .short('e')
                .value_parser(clap::value_parser!(PathBuf))
                .required(true))
            .arg(Arg::new("time-offset")
                .help("Time offset, +/- hours")
                .long("time-offset")
                .short('t')
                .value_parser(clap::value_parser!(isize))
                .allow_hyphen_values(true) // negative values and value > 24 ok
                .default_value("0"))
            .arg(Arg::new("downsample-factor")
                .help("Downsample factor for coordinates. Must be a positive value. Important: Will be set to largest applicable value if too high. E.g. poly-line must contain a minimum of 2 points, and the value can not exceed the number of points in cluster.")
                .long("downsample")
                .short('d')
                .value_parser(clap::value_parser!(usize))
                .default_value("1"))
            .arg(Arg::new("geoshape")
                .help("Output options for KML-file. ELAN annotation values will be added as description for points that intersect with an annotation timespan regardless of geoshape value.")
                .long("geoshape")
                .default_value("point-all")
                // TODO change below to GeoTypes enum
                .value_parser(PossibleValuesParser::new([
                    "point-all", "point-multi", "point-single",
                    "line-all", "line-multi",
                    "circle"
                ])))
            .arg(Arg::new("radius")
                .help("Circle radius as a float value, e.g. 3.2 (m). Only affects geoshape 'circle'.")
                .long("radius")
                .value_parser(clap::value_parser!(f64))
                .default_value("2.0"))
            .arg(Arg::new("vertices")
                .help("Circle vertices ('roundness' of the circle polygon). An integer between 3-255. Only affects geoshape 'circle'")
                .long("vertices")
                .value_parser(clap::value_parser!(u8).range(3..)) // no polygon with < 3 vertices...
                .default_value("40"))
            .arg(Arg::new("height")
                .help("Geoshape relative height above ground (KML extrude option). Float value.")
                .long("height")
                .value_parser(clap::value_parser!(f64)))
            .arg(Arg::new("geotier")
                .help("Use an ELAN-tier with coordinates for geo-referencing.")
                .long("geotier")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("cdata")
                .help("KML-option, added visuals in Google Earth")
                .long("cdata")
                .action(ArgAction::SetTrue))

            .next_help_heading("VIRB")
            .arg(Arg::new("fit")
                .help("[VIRB] Garmin VIRB FIT-file")
                .short('f')
                .long("fit")
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present_any(["gpmf", "geotier"]))
                
            .next_help_heading("GoPro")
            .arg(Arg::new("gpmf")
                .help("GoPro MP4-file")
                .short('g')
                .long("gpmf")
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present_any(["fit", "geotier"]))
            .arg(Arg::new("input-directory")
                .help("Start path for locating files")
                .short('i')
                .long("indir")
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present_any(["fit", "geotier"]))
            .arg(Arg::new("verify")
                .help("Verifies GPMF data and discards corrupt clips before grouping as session.")
                .long("verify")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("gpsfix")
                .help("Filter GPS-log to only include points with a specific number of satellites locked. 0 = No lock (all points), 2 = 2D lock, 3 = 3D lock.")
                .long("gpsfix")
                .default_value("3") // 3D lock for eaf
                .conflicts_with_all(["fit", "geotier"])
                .value_parser(clap::value_parser!(u32)))
        )

        // Locate and match files belonging to the same recording session.
        .subcommand(Command::new("locate")
            .about("Locate and group GoPro-files (MP4) or Garmin VIRB-files (MP4, FIT) belonging to the same recording session.")
            .visible_alias("l")

            .next_help_heading("General")
            .arg(Arg::new("input-directory")
                .help("Start path for locating files")
                .short('i')
                .long("indir")
                .value_parser(clap::value_parser!(PathBuf))
                .required(true))
            .arg(Arg::new("kind")
                .help("If no other options are given, specify camera type to locate and match. Other arguments will be ignored if 'kind' is specified.")
                .short('k')
                .long("kind")
                // TODO change below to CameraType enum?
                .value_parser(PossibleValuesParser::new([
                    "g", "gopro", // g short for gopro
                    "v", "virb"   // v short for virb
                ]))
                .required_unless_present_any(&["uuid", "video", "fit"]))
            .arg(Arg::new("video")
                .help("Any original GoPro or VIRB clip.")
                .short('v')
                .long("video")
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with("uuid"))
            .arg(Arg::new("quiet")
                .help("Do not print file-by-file search progress")
                .long("quiet")
                .action(ArgAction::SetTrue))

            .next_help_heading("VIRB")
            .arg(Arg::new("uuid")
                .help("UUID for first VIRB clip in a session")
                .short('u')
                .long("uuid")
                .conflicts_with("video"))
            .arg(Arg::new("fit")
                .help("VIRB FIT-file for selecting session")
                .short('f')
                .long("fit")
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with_all(&["uuid", "video"]))
            .arg(Arg::new("duplicates")
                .help("Include duplicate files in match results")
                .long("duplicates")
                .action(ArgAction::SetTrue))

            .next_help_heading("GoPro")
            .arg(Arg::new("verify")
                .help("Verifies GPMF data and discards corrupt clips before grouping as session.")
                .long("verify")
                .action(ArgAction::SetTrue))
        )

        // Inspect GoPro/Garmin telemetry
        .subcommand(Command::new("inspect")
            .about("Inspect Garmin FIT-files  ('--fit <FIT-FILE>'), or GoPro GPMF data ('--gpmf <MP4-FILE>').")
            .visible_alias("i")

            .next_help_heading("General")
            .arg(Arg::new("video")
                .help("Any MP4-file.")
                .long("video")
                .short('v')
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with("gpmf"))
            .arg(Arg::new("atoms")
                .help("Print MP4 atom information if specified file is a video.")
                .action(ArgAction::SetTrue)
                .long("atoms")
                .conflicts_with_all(["gpmf", "fit"]))
            .arg(Arg::new("meta")
                .help("Print MP4 custom metadata (udta).")
                .action(ArgAction::SetTrue)
                .long("meta")
                .conflicts_with_all(["gpmf", "fit"]))
            
            .next_help_heading("VIRB")
            .arg(Arg::new("fit")
                .help("Garmin FIT-file.")
                .long("fit")
                .short('f')
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present_any(&["video", "gpmf"])
                .conflicts_with("gpmf"))
                
            .next_help_heading("GoPro")
            .arg(Arg::new("gpmf")
                .help("Unedited GoPro MP4-file, or extracted GPMF-track.")
                .long("gpmf")
                .short('g')
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present_any(&["video", "fit"])
                .conflicts_with_all(&["fit", "video", "global"]))
            .arg(Arg::new("input-directory")
                .help("Start path for locating GoPro MP4 clips.")
                .long("indir")
                .short('i')
                .value_parser(clap::value_parser!(PathBuf)))
            .arg(Arg::new("offsets")
                .help("Print DEVC byte offsets for GoPro MP4-file.")
                .long("offsets")
                .short('o')
                .action(clap::ArgAction::SetTrue)
                .requires("gpmf")
                .conflicts_with_all(&["fit", "video", "global"])) // list all conflicts...?
            .arg(Arg::new("gpsfix")
                .help("Filter GPS-log to only include points with a specific number of satellites locked. 0 = No lock (all points), 2 = 2D lock, 3 = 3D lock.")
                .long("gpsfix")
                .default_value("2") // 2D lock
                .value_parser(clap::value_parser!(u32))
                .requires("gpmf")
                .conflicts_with_all(&["fit", "video", "global"])) // list all conflicts...?
            .arg(Arg::new("verify")
                .help("Verifies GPMF data and discards corrupt clips before grouping as session.")
                .long("verify")
                .requires("gpmf")
                .action(ArgAction::SetTrue))
            
            .next_help_heading("General")
            .arg(Arg::new("sensor")
                .help("Print sensor data. Sensors differ between brands and models.")
                .long("sensor")
                .value_parser(PossibleValuesParser::new([
                    "acc", "accelerometer",
                    "gyr", "gyroscope",
                    "mag", "magnetometer", // VIRB only
                    "grv", "gravity", // GoPro only
                    "bar", "barometer" // VIRB only
                ])))
            .arg(Arg::new("session")
                .help("Filter telemetry on recording session. GoPro: automatic selection. VIRB: select from list in FIT-file.")
                .long("session")
                .short('s')
                .action(ArgAction::SetTrue))
            .arg(Arg::new("kml")
                .help("Generate a KML file from GPS-logs. Points only, downsampled to roughly 1 point/second.")
                .long("kml")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("indexed-kml")
                .help("Same as '--kml', but each point is tagged with a counter.")
                .long("ikml")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("json")
                .help("Generate a GeoJSON file from GPS-logs. Points only, downsampled to roughly 1 point/second.")
                .long("json")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("fullgps")
                .help("Use full 10Hz GPS log for KML/GeoJson.")
                .long("fullgps")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("verbose")
                .help("Prints telemetry.")
                .long("verbose")
                .conflicts_with_all(&["gps", "debug"])
                .action(ArgAction::SetTrue))
            .arg(Arg::new("data-type")
                .help("Print telemetry data for specified data type. FIT: specify the numerical ID, e.g. for 160 for GPS. GPMF: specify a string, e.g. 'GPS (Lat., Long., Alt., 2D speed, 3D speed)' (citation marks may be required) for GPS.")
                .long("type")
                .short('t')
                // TODO value parser possible? string for gopro, u16 for virb
                .conflicts_with_all(&["gps", "debug", "verbose"]))
            .arg(Arg::new("gps")
                .help("Print GPS log.")
                .long("gps")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(&["verbose", "debug", "data-type"])) // gps always prints points
            .arg(Arg::new("debug")
                .help("Print debug info for all data while parsing.")
                .long("debug")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(&["gps", "verbose", "data-type"])) // gps always prints points
            .arg(Arg::new("csv")
                .help("Save sensor data or GPS data as CSV.")
                .long("csv")
                .action(ArgAction::SetTrue)) // how to require EITHER --gps or --sensor <SENSOR>?
        )

        .subcommand(Command::new("plot")
            .about("Plot telemetry data. Note that not all combinations are valid.")
            .visible_alias("p")

            .next_help_heading("GoPro")
            .arg(Arg::new("gpmf")
                .help("Unedited GoPro MP4-file, or extracted GPMF-track. Exctracted GPMF-tracks do not contain relative timestamps, since these are derived via the MP4 file.")
                .long("gpmf")
                .short('g')
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present("fit"))

            .next_help_heading("Garmin VIRB")
            .arg(Arg::new("fit")
                .help("Garmin FIT-file. Non-VIRB FIT-files work depending on options used.")
                .long("fit")
                .short('f')
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present("gpmf"))

            .next_help_heading("General")
            .arg(Arg::new("y-axis")
                .help("Data to plot on Y-axis.")
                .long("y-axis")
                .short('y')
                .required(true)
                .value_parser([
                    "acc", "accelerometer", // GoPro, VIRB
                    "gyr", "gyroscope",     // GoPro, VIRB
                    "grv", "gravity",     // GoPro (Gravity Vector)
                    "bar", "barometer",     // VIRB
                    "mag", "magnetometer",   // VIRB, some GoPro models (Fusion only?)
                    "alt", "altitude", // TODO GPS altitude. GoPro, VIRB
                ]))
            .arg(Arg::new("x-axis")
                .help("Data to plot on X-axis. Defaults to count/data index if not specified.")
                .long("x-axis")
                .short('x')
                .value_parser([
                    "time",     // Time of recording session in seconds
                    "dist", "distance", // GPS travel distance/displacement
                ]))
        )

        // Print or save manual
        .subcommand(Command::new("manual")
        .about("Print the manual or save as a file to disk.")
        .visible_alias("m")
        .arg(Arg::new("pdf")
            .help("Save the full manual as a PDF to current directory.")
            .long("pdf")
            .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("pdf-a4")
            .help("Save the A4-guide as a PDF to current directory.")
            .long("pdf-a4")
            .action(clap::ArgAction::SetTrue))
        )
        .get_matches();
    
    // VIEW, SAVE MANUAL
    if let Some(arg_matches) = args.subcommand_matches("manual") {
        if let Err(err) = manual::run(&arg_matches) {
            eprintln!("{err}");
            return ExitCode::FAILURE
        }
    }

    // ACTION CAMERA FOOTAGE TO EAF, GORP+VIRB
    if let Some(arg_matches) = args.subcommand_matches("cam2eaf") {
        if let Err(err) = cam2eaf::run(&arg_matches) {
            eprintln!("{err}");
            return ExitCode::FAILURE
        }
    }

    // EAF TO KML/GEOJSON
    if let Some(arg_matches) = args.subcommand_matches("eaf2geo") {
        if let Err(err) = eaf2geo::run(&arg_matches) {
            eprintln!("{err}");
            return ExitCode::FAILURE
        }
    }
    
    // INSPECT TELEMETRY, VIRB + GOPRO
    if let Some(arg_matches) = args.subcommand_matches("inspect") {
        if let Err(err) = inspect::run(&arg_matches) {
            eprintln!("{err}");
            return ExitCode::FAILURE
        }
    }

    // PLOT TELEMETRY, VIRB + GOPRO
    if let Some(arg_matches) = args.subcommand_matches("plot") {
        if let Err(err) = plot::run(&arg_matches) {
            eprintln!("{err}");
            return ExitCode::FAILURE
        }
    }

    // LOCATE AND MATCH FILES, VIRB + GOPRO
    if let Some(arg_matches) = args.subcommand_matches("locate") {
        if let Err(err) = locate::run(&arg_matches) {
            eprintln!("{err}");
            return ExitCode::FAILURE
        }
    }

    ExitCode::SUCCESS
}
