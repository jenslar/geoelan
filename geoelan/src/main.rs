use std::{path::PathBuf, process::ExitCode};

use clap::{builder::PossibleValuesParser, Arg, ArgAction, Command};
use time::OffsetDateTime;

use kml;

mod cam2eaf;
mod eaf2geo;
mod elan;
mod files;
mod geo;
mod inspect;
mod locate;
mod manual;
mod media;
mod model;
mod plot;
mod text;

const VERSION: &'static str = "2.7.0";
const AUTHOR: &'static str = "Jens Larsson";
const REPO: &'static str = "https://github.com/jenslar/geoelan";

fn main() -> ExitCode {
    let build = OffsetDateTime::now_utc().date().to_string();
    let help = format!(
        "GeoELAN {VERSION} (build: {build})

    GeoELAN is a tool for annotating action camera GPS logs using the free annotation software ELAN.

Source: {REPO}

Use 'geoelan --help' for a longer description."
    );
    // - Documentation: https://github.com/jenslar/geoelan/mdbook
    let long_help = format!(
"GeoELAN {VERSION} (build: {build})

GeoELAN is a tool for annotating action camera GPS logs using the free annotation software ELAN.
Supports recent GoPro (excluding Hero 12 Black since it has no GPS) or a Garmin VIRB Ultra 30.
Additional functionality includes inspecting and generating plots, and to also automatically locate,
group and join video clips by recording session. Refer to the manual for further information.

IMPORTANT:
  Keep your original files (renaming is fine). Concatenating/converting video
  clips will discard all embedded telemetry, such as GPS-logs and identifiers.

REQUIREMENTS:
- FFmpeg:              https://ffmpeg.org ('cam2eaf')
- ELAN:                https://archive.mpi.nl/tla/elan

HELP:
- Specific subcommand: geoelan help <subcommand>
- Example:             geoelan help eaf2geo

MANUAL:
- Print to screen:     geoelan manual
- Save as PDF:         geoelan manual --pdf

SOURCE:
- {REPO}

---");

    let args = Command::new("geoelan")

        .version(VERSION)
        .author(AUTHOR)
        .about(help)
        .long_about(long_help)
        .term_width(80)
        .arg_required_else_help(true)

        .subcommand(Command::new("cam2eaf")
            .about("Generate an ELAN-file from GoPro/VIRB footage.")
            .long_about("Generate an ELAN-file from GoPro/VIRB footage, with or without coordinates inserted as a tier. Requires FFmpeg for joining clips.")
            .visible_alias("c2e")

            .next_help_heading("General")
            .arg(Arg::new("video")
                .help("Unaltered GoPro/VIRB MP4 file used to determine remaining clips in session.")
                .long("video")
                .short('v')
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present_any(["batch", "uuid", "fit"]))
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

            .next_help_heading("GoPro")
            .arg(Arg::new("verify")
                .help("Verifies GPMF data and ignores corrupt clips.")
                .long("verify")
                .conflicts_with_all(&[
                    "fit", "uuid" // VIRB only
                ])
                .action(ArgAction::SetTrue))
            .arg(Arg::new("gpsfix")
                .help("Min GPS fix threshold. 0 = No lock, 2 = 2D lock, 3 = 3D lock.")
                .long("gpsfix")
                .default_value("3") // 3D lock for eaf
                .conflicts_with_all(&[
                    "fit", "uuid" // VIRB only
                ])
                .value_parser(clap::value_parser!(u32)))
            .arg(Arg::new("gpsdop")
                .help("Min GPS dilution of position threshold. 5.0 = good precision.")
                .long("gpsdop")
                .conflicts_with_all(&[
                    "fit", "uuid" // VIRB only
                ])
                .value_parser(clap::value_parser!(f64)))

            .next_help_heading("VIRB")
            .arg(Arg::new("fit")
                .help("VIRB FIT-file to use for locating MP4-clips.")
                .short('f')
                .long("fit")
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with_all(&[
                    "video", "uuid"
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
                    "video",            // either camera
                    "fit",              // VIRB only
                    "verify", "gpsfix", // GoPro only
                    "batch",
                ])
                .required_unless_present_any(&["video", "fit", "batch"]))
        )

        // Generate KML and GeoJson from EAF
        .subcommand(Command::new("eaf2geo")
            .about("Generate KML and GeoJson from specified ELAN-file.")
            .long_about(r#"Generate KML and GeoJson from specified ELAN-file.

ELAN annotation values become KML/GeoJSON descriptions if a logged point's timstamp intersects with the annotation timespan.

Use the '--geoshape' option to specify feature type (point, polyline, or circle).

Geoshape options:
  'point-all':   Points. Includes all points, meaning some will not have a description value.
  'point-multi': Points. Only includes points that intersect with an annotation value.
  'line-all':    Continuous poly-line. Includes all points, meaning some segments will not have a description value.
  'line-multi':  Segmented poly-line. Only includes points that intersect with an annotation value.
  'circle-2d':   Generates a flat circle around an average point derived from those logged within each annotation's timespan.
  'circle-3d':   Generates an extruded circle around an average point derived from those logged within each annotation's timespan. Extrusion height is equal to the altitude value, relative to ground.

  Defaults for circles (customizable):
    radius:        2 meters
    vertices:      40 (valid values are 3 - 255)"#)
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
                .help("Output options for KML and GeoJSON files.")
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
                .help("Verifies GPMF data and ignores corrupt clips.")
                .long("verify")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("gpsfix")
                .help("Min GPS fix threshold. 0 = No lock, 2 = 2D lock, 3 = 3D lock.")
                .long("gpsfix")
                .default_value("2") // 3D lock for eaf
                .conflicts_with_all(["fit", "geotier"])
                .value_parser(clap::value_parser!(u32)))
            .arg(Arg::new("gpsdop")
                .help("Min GPS dilution of position threshold. 5.0 = good precision.")
                .long("gpsdop")
                .conflicts_with_all(["fit", "geotier"])
                .value_parser(clap::value_parser!(f64)))
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
                .help("Any unedited GoPro or VIRB clip.")
                .short('v')
                .long("video")
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with("uuid"))
            .arg(Arg::new("quiet")
                .help("Do not print file-by-file search progress")
                .long("quiet")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("halt-on-error")
                .help("Halts on errors relating to locating clips.")
                .long("ignore-errors")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("verbose")
                .help("Print additional info for each clip")
                .long("verbose")
                .action(ArgAction::SetTrue))

            .next_help_heading("GoPro")
            .arg(Arg::new("verify")
                .help("Verifies GPMF data and ignores corrupt clips.")
                .long("verify")
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
        )

        // Inspect GoPro/Garmin telemetry
        .subcommand(Command::new("inspect")
            .about("Inspect GoPro GPMF and Garmin FIT  data and MP4 files.")
            .visible_alias("i")

            .next_help_heading("General")
            .arg(Arg::new("video")
                .help("Any MP4-file.")
                .long("video")
                .short('v')
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with("gpmf"))
            .arg(Arg::new("atoms")
                .help("Print MP4 atom information if '--video' is used.")
                .action(ArgAction::SetTrue)
                .long("atoms")
                .requires("video")
                .conflicts_with_all(["gpmf", "fit", "meta"]))
            .arg(Arg::new("meta")
                .help("Print MP4 custom metadata if '--video' is used.")
                .action(ArgAction::SetTrue)
                .long("meta")
                .requires("video")
                .conflicts_with_all(["gpmf", "fit", "atoms"]))
            .arg(Arg::new("offsets")
                .help("Print sample byte offsets for specified track in MP4-file.")
                .long("offsets")
                .short('o')
                .value_parser(clap::value_parser!(String))
                .requires("video")) // list all conflicts...?
                .arg(Arg::new("sensor")
                .help("Print sensor data. Sensors differ between brands and models.")
                .long("sensor")
                .value_parser(PossibleValuesParser::new([
                    "acc", "accelerometer",
                    "gyr", "gyroscope",
                    "mag", "magnetometer", // VIRB only
                    "grv", "gravity",      // GoPro only
                    "bar", "barometer"     // VIRB only
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
                .help("Use full GPS log for KML/GeoJson (10-18Hz depending on model).")
                .long("fullgps")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("verbose")
                .help("Prints telemetry.")
                .long("verbose")
                .conflicts_with_all(&["gps", "debug"])
                .action(ArgAction::SetTrue))
            .arg(Arg::new("data-type")
                .help("Print specified data in raw form. FIT: e.g. 160 for GPS. GPMF: e.g. 'GPS (Lat., Long., Alt., 2D speed, 3D speed)' for GPS (note the citation marks).")
                .long("type")
                .short('t')
                .conflicts_with_all(
                    &["gps", "sensor", "debug", "verbose"])
                )
            .arg(Arg::new("gps")
                .help("Print processed GPS log.")
                .long("gps")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(
                    &["verbose", "sensor", "debug", "data-type"])
                )
            .arg(Arg::new("debug")
                .help("Print debug info while parsing.")
                .long("debug")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(&["gps", "verbose", "data-type"])) // gps always prints points
            .arg(Arg::new("csv")
                .help("Save sensor data or GPS data as CSV.")
                .long("csv")
                // how to require EITHER --gps or --sensor <SENSOR>?
                .action(ArgAction::SetTrue))

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
            .arg(Arg::new("gpsfix")
                .help("Min GPS fix threshold. 0 = No lock, 2 = 2D lock, 3 = 3D lock.")
                .long("gpsfix")
                .requires("gpmf")
                .value_parser(clap::value_parser!(u32)))
            .arg(Arg::new("gpsdop")
                .help("Min GPS dilution of position threshold. 5.0 = good precision.")
                .long("gpsdop")
                .requires("gpmf")
                .value_parser(clap::value_parser!(f64)))
            .arg(Arg::new("verify")
                .help("Verifies GPMF data and ignores corrupt clips.")
                .long("verify")
                .requires("gpmf")
                .action(ArgAction::SetTrue))

            .next_help_heading("VIRB")
            .arg(Arg::new("fit")
                .help("Garmin FIT-file.")
                .long("fit")
                .short('f')
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present_any(["video", "gpmf"])
                .conflicts_with("gpmf"))
        )

        .subcommand(Command::new("plot")
            .about("Plot telemetry data. Note that not all combinations are valid.")
            .visible_alias("p")

            .next_help_heading("GoPro")
            .arg(Arg::new("gpmf")
                .help("Unedited GoPro MP4-file, or extracted GPMF-track. Exctracted GPMF-tracks do not contain relative timestamps, since these are derived via the MP4 file.")
                .long("gpmf")
                .short('g')
                .required_unless_present("fit")
                .value_parser(clap::value_parser!(PathBuf)))
            .arg(Arg::new("input-directory")
                .help("Input directory for locating GoPro clips.")
                .long("indir")
                .short('i')
                .requires("gpmf")
                .value_parser(clap::value_parser!(PathBuf)))
            .arg(Arg::new("gps5")
                .help("Force the use of GPS5 for cameras that log both (currently only Hero11).")
                .long("gps5")
                .requires("gpmf")
                .action(clap::ArgAction::SetTrue))

            .next_help_heading("VIRB")
            .arg(Arg::new("fit")
                .help("Garmin FIT-file. Non-VIRB FIT-files work depending on options used.")
                .long("fit")
                .short('f')
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present("gpmf"))

            .next_help_heading("General")
            .arg(Arg::new("session")
                .help("Compile telemetry for a recording session.")
                .long("session")
                .short('s')
                .action(ArgAction::SetTrue))
            .arg(Arg::new("y-axis")
                .help("Data to plot on Y-axis.")
                .long("y-axis")
                .short('y')
                .required(true)
                .value_parser([
                    // Sensors
                    "acc", "accelerometer", // GoPro, VIRB
                    "gyr", "gyroscope",     // GoPro, VIRB
                    "grv", "gravity",     // GoPro (Gravity Vector)
                    "bar", "barometer",     // VIRB
                    "mag", "magnetometer",   // VIRB, some GoPro models (Fusion only?)

                    // GPS
                    "lat", "latitude",
                    "lon", "longitude",
                    "alt", "altitude",
                    "s2d", "speed2d",
                    "s3d", "speed3d",
                    "dop", "dilution",  // GoPro dilution of precision, GoPro 11 and later
                    "fix", "gpsfix",   // GoPro satellite lock level/GPS fix, 2D or 3D lock etc
                ]))
            .arg(Arg::new("x-axis")
                .help("Data to plot on X-axis. Defaults to count/data index if not specified.")
                .long("x-axis")
                .short('x')
                .value_parser([
                    // General
                    "c", "count",      // Sample count (default)
                    "t", "time",       // Time of recording session in seconds

                    // GPS
                    "dst", "distance", // GPS travel distance/displacement
                ])
                .default_value("count"))
            .arg(Arg::new("fill")
                .help("Fill area under plot.")
                .long("fill")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("average")
                .help("Generate a linear average for each sensor data cluster before plotting.")
                .long("average")
                .short('a')
                .action(clap::ArgAction::SetTrue))
        )

        // Print or save manual
        .subcommand(Command::new("manual")
            .about("Print the manual or save as a file to disk.")
            .visible_alias("m")
            .arg(Arg::new("pdf")
                .help("Save the full manual as a PDF to current directory.")
                .long("pdf")
                .action(clap::ArgAction::SetTrue))
        )
        .get_matches();

    // VIEW, SAVE MANUAL
    if let Some(arg_matches) = args.subcommand_matches("manual") {
        if let Err(err) = manual::run(&arg_matches) {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    }

    // ACTION CAMERA FOOTAGE TO EAF, GORP+VIRB
    if let Some(arg_matches) = args.subcommand_matches("cam2eaf") {
        if let Err(err) = cam2eaf::run(&arg_matches) {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    }

    // EAF TO KML/GEOJSON
    if let Some(arg_matches) = args.subcommand_matches("eaf2geo") {
        if let Err(err) = eaf2geo::run(&arg_matches) {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    }

    // INSPECT TELEMETRY, VIRB + GOPRO
    if let Some(arg_matches) = args.subcommand_matches("inspect") {
        if let Err(err) = inspect::run(&arg_matches) {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    }

    // PLOT TELEMETRY, VIRB + GOPRO
    if let Some(arg_matches) = args.subcommand_matches("plot") {
        if let Err(err) = plot::run(&arg_matches) {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    }

    // LOCATE AND MATCH FILES, VIRB + GOPRO
    if let Some(arg_matches) = args.subcommand_matches("locate") {
        if let Err(err) = locate::run(&arg_matches) {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
