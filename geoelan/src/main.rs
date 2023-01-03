use std::path::PathBuf;

use clap::{Arg, Command, builder::PossibleValuesParser, ArgAction};
use time::OffsetDateTime;

use kml;

mod eaf2geo;
mod cam2eaf;
mod inspect;
mod locate;
mod manual;
mod geo;
mod elan;
mod files;
mod media;
mod text;
mod model;

fn main() -> std::io::Result<()> {
    let about = format!(
        "GeoELAN, build: {}
https://gitlab.com/rwaai/geoelan

GeoELAN is a tool for annotating action camera GPS logs using the free annotation software ELAN. Supported cameras are a recent GoPro or a Garmin VIRB Ultra 30. Additional functionality includes locating files, and inspecting GoPro's GPMF-format and Garmin FIT-files. Refer to the manual for further information.

IMPORTANT:
  Keep your original files. Concatenating/converting video clips will
  discard embedded data such as the Garmin VIRB UUID, and GoPro GPMF data.

REQUIREMENTS:
- FFmpeg:              https://ffmpeg.org ('virb2eaf', 'gopro2eaf')
- ELAN:                https://archive.mpi.nl/tla/elan

HELP:
- Specific subcommand: geoelan help <subcommand>
- Example:             geoelan help gopro2geo

MANUAL:
- Print to screen:     geoelan manual
- Save as PDF:         geoelan manual --pdf

FORMATS:
- GoPro GPMF:          https://github.com/gopro/gpmf-parser
- Garmin FIT:          https://developer.garmin.com/fit/overview/

PUBLICATION:
- https://doi.org/10.1080/13645579.2020.1763705

---",
        OffsetDateTime::now_utc().date().to_string()
    );

    let args = Command::new("geoelan")

        .version("2.0.0")
        .author("Jens Larsson")
        .about(about)
        .term_width(80)

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

        // Generate EAF from VIRB footage
        .subcommand(Command::new("virb2eaf")
            .about("Generate an ELAN-file from Garmin VIRB footage, with or without coordinates inserted as a tier. Requires FFmpeg for concatenating clips.")
            .visible_alias("v2e")

            .arg(Arg::new("fit")
                .help("VIRB FIT-file to use for locating MP4-clips.")
                .short('f')
                .long("fit")
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with_all(&["video", "uuid"])
                .required_unless_present_any(&["video", "uuid"]))
            .arg(Arg::new("uuid")
                .help("UUID for a VIRB clip in a session")
                .short('u')
                .long("uuid")
                .conflicts_with_all(&["video", "fit"])
                .required_unless_present_any(&["video", "fit"]))
            .arg(Arg::new("video")
                .help("Unaltered VIRB MP4 file. Must be the first clip in the recording session.")
                .long("video")
                .short('v')
                .value_parser(clap::value_parser!(PathBuf))
                .required(true))
            .arg(Arg::new("low-res-only")
                .help("Only concatenate low resolution clips (.GLV).")
                .short('l')
                .long("low-res-only")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("ffmpeg")
                .help("Custom path to FFmpeg")
                .long("ffmpeg")
                .value_parser(clap::value_parser!(PathBuf))
                .default_value(if cfg!(windows) {"ffmpeg.exe"} else {"ffmpeg"}))
            .arg(Arg::new("time-offset")
                .help("Time offset, +/- hours. Modifies logged timestamps.")
                .long("time-offset")
                .short('t')
                .value_parser(clap::value_parser!(isize))
                .allow_hyphen_values(true) // negative values and value > 24 ok
                .default_value("0"))
            .arg(Arg::new("downsample-factor")
                .help("Downsample factor for coordinates. Must be a positive value. Defaults to 10 (results in 1 point/sec for the VIRB Ultra 30). Set to 1 to use the full log, but be aware that this will generate large ELAN-files. Important: Will be set to largest applicable value if too high.")
                .long("downsample")
                .short('d')
                // TODO .range() does not work for usize for clap 3.2?
                // .value_parser(clap::value_parser!(usize).range(1..))
                .value_parser(clap::value_parser!(usize))
                .default_value("1")) // default = every point
            .arg(Arg::new("input-directory")
                .help("Input path for locating GoPro MP4 clips.")
                .long("indir")
                .short('i')
                .value_parser(clap::value_parser!(PathBuf))
                .required(true))
            .arg(Arg::new("output-directory")
                .help("Output path for resulting files.")
                .long("outdir")
                .short('o')
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("OUTPUT"))
            .arg(Arg::new("geotier")
                .help("Insert tier with synchronised coordinates in ELAN-file.")
                .long("geotier")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("dryrun")
                .help("Only show results, does not concatenate video or generate ELAN-file.")
                .long("dryrun")
                .action(ArgAction::SetTrue))
        )
        
        // Generate EAF from GoPro footage
        .subcommand(Command::new("gopro2eaf")
            .about("Generate an ELAN-file from GoPro footage, with or without coordinates inserted as a tier. Requires ffmpeg for concatenating clips.")
            .visible_alias("g2e")
            
            .arg(Arg::new("video")
                .help("Unaltered GoPro MP4 file.")
                .long("video")
                .short('v')
                .value_parser(clap::value_parser!(PathBuf))
                .required(true))
            .arg(Arg::new("ffmpeg")
                .help("Custom path to FFmpeg")
                .long("ffmpeg")
                .value_parser(clap::value_parser!(PathBuf))
                .default_value(if cfg!(windows) {"ffmpeg.exe"} else {"ffmpeg"})
            )
            .arg(Arg::new("concatenate")
                .help("Concate clips for recording session starting with 'video' parameter.")
                .long("concat")
                .short('c'))
            .arg(Arg::new("time-offset")
                .help("Time offset, +/- hours. Modifies logged timestamps.")
                .long("time-offset")
                .short('t')
                .value_parser(clap::value_parser!(isize))
                .allow_hyphen_values(true) // negative values and value > 24 ok
                .default_value("0"))
            .arg(Arg::new("downsample-factor")
                .help("Downsample factor for coordinates. Must be a positive value. Defaults to 1 (results in 1 point/sec for the GoPro 5 Black and later). Important: Will be set to largest applicable value if too high.")
                .long("downsample")
                .short('d')
                .value_parser(clap::value_parser!(usize))
                .default_value("1")) // default = every point
            .arg(Arg::new("input-directory")
                .help("Input path for locating GoPro MP4 clips.")
                .long("indir")
                .short('i')
                .value_parser(clap::value_parser!(PathBuf))
                .required(true))
            .arg(Arg::new("output-directory")
                .help("Output path for resulting files.")
                .long("outdir")
                .short('o')
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("OUTPUT"))
            .arg(Arg::new("geotier")
                .help("Insert tier with synchronised coordinates in ELAN-file.")
                .long("geotier"))
            .arg(Arg::new("dryrun")
                .help("Only show results, does not concatenate video or generate ELAN-file.")
                .long("dryrun"))
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
            .about("Generate KML and GeoJson from specified ELAN-file. The 'geoshape' option allows for generating points, polylines and circles in the output KML and GeoJSON files. ELAN annotation values will be added as description for points whose log time intersect with an annotation timespan regardless of geoshape value.")
            .visible_alias("e2g")

            // Shared options
            .arg(Arg::new("eaf")
                .help("ELAN-file")
                .long("eaf")
                .short('e')
                .value_parser(clap::value_parser!(PathBuf))
                .required(true)
            )
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
                ]))
            )
            .arg(Arg::new("radius")
                .help("Circle radius as a float value, e.g. 3.2 (m). Only affects geoshape 'circle'.")
                .long("radius")
                .value_parser(clap::value_parser!(f64))
                .default_value("2.0")
            )
            .arg(Arg::new("vertices")
                .help("Circle vertices ('roundness' of the circle polygon). An integer between 3-255. Only affects geoshape 'circle'")
                .long("vertices")
                .value_parser(clap::value_parser!(u8).range(3..)) // no polygon with < 3 vertices...
                .default_value("40")
            )
            .arg(Arg::new("height")
                .help("Geoshape relative height above ground (KML extrude option). Float value.")
                .long("height")
                .value_parser(clap::value_parser!(f64))
                // no default value, since circle-3d is now circle with optional height
            )
            .arg(Arg::new("cdata")
                .help("KML-option, added visuals in Google Earth")
                .long("cdata")
                .action(ArgAction::SetTrue))

            // Virb/FIT specific options
            .arg(Arg::new("fit")
                .help("[VIRB] Garmin VIRB FIT-file")
                .short('f')
                .long("fit")
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present("gpmf")
            )
                
            // GoPro/GPMF options
            .arg(Arg::new("gpmf")
                .help("[GoPro] GoPro MP4-file")
                .short('g')
                .long("gpmf")
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present("fit")
            )
        )

        // Locate and match files belonging to the same recording session.
        .subcommand(Command::new("locate")
            .about("Locates and matches either Garmin VIRB-files (MP4, FIT) or GoPro-files (MP4) that belong to the same recording session. Note that due to format differences a recursive search that matches between parallel directories is only possible for Garmin VIRB. For GoPro, files that belong to the same session must be in the same dir.")
            .visible_alias("l")

            // Options applicable to both Virb and GoPro
            .arg(Arg::new("input-directory")
                .help("[GoPro/VIRB] Start path for locating files")
                .short('i')
                .long("indir")
                .value_parser(clap::value_parser!(PathBuf))
                .required(true))
            .arg(Arg::new("kind")
                .help("[GoPro/VIRB] If no other options are given, specify camera type to locate and match. Other arguments will be ignored if 'kind' is specified.")
                .short('k')
                .long("kind")
                // TODO change below to CameraType enum?
                .value_parser(PossibleValuesParser::new(["gopro", "virb"]))
                // .value_parser(EnumValueParser::<CameraModel>::new())
                .conflicts_with("video")
                .required_unless_present_any(&["uuid", "video", "fit"]))
            .arg(Arg::new("video")
                .help("[GoPro/VIRB] Any original GoPro or VIRB clip.")
                .short('v')
                .long("video")
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with("uuid"))
            // .arg(Arg::new("write-csv")
            //     .help("[GoPro/VIRB] Write result to CSV plain-text file")
            //     .long("csv"))
            .arg(Arg::new("quiet")
                .help("[GoPro/VIRB] Do not print file-by-file search progress")
                .long("quiet")
                .action(ArgAction::SetTrue))

            // Virb/FIT options
            .arg(Arg::new("uuid")
                .help("[VIRB] UUID for first VIRB clip in a session")
                .short('u')
                .long("uuid")
                .conflicts_with("video"))
            .arg(Arg::new("fit")
                .help("[VIRB] VIRB FIT-file for selecting session")
                .short('f')
                .long("fit")
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with_all(&["uuid", "video"]))
            .arg(Arg::new("duplicates")
                .help("[VIRB] Include duplicate files in match results")
                .long("duplicates")
                .action(ArgAction::SetTrue))

            // GoPro/GPMF options
        )

        // Inspect GoPro/Garmin telemetry
        .subcommand(Command::new("inspect")
            .about("Inspect a Garmin FIT-file ('--fit <FIT-FILE>'), or a GoPro MP4/GPMF stream ('--gpmf <MP4-FILE>'). Some arguments can be used with both FIT and GPMF, denoted by '[GoPro/VIRB]'")
            .visible_alias("i")

            // FIT options
            .arg(Arg::new("fit")
                .help("[VIRB] Garmin FIT-file.")
                .long("fit")
                .short('f')
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present_any(&["video", "gpmf"])
                .conflicts_with("gpmf"))
            .arg(Arg::new("video")
                .help("[VIRB] Garmin VIRB MP4-file. This is used to filter on data for a specific session. If no FIT-file is specified, the MP4-file's UUID will be printed.")
                .long("video")
                .short('v')
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with("gpmf"))
            .arg(Arg::new("full-gps")
                .help("[VIRB] Use the full 10Hz GPS log for KML/GeoJson output of VIRB data.")
                .long("full-gps")
                .conflicts_with("gpmf")
                .action(ArgAction::SetTrue))
            // TODO 220615 add GoPro support for sensor data
            .arg(Arg::new("sensor")
                .help("[VIRB] Print calibrated sensor data. 3D: magnetometer/208, gyroscope/164, accelerometer/165. 1D: barometer/209")
                .long("sensor")
                .value_parser(PossibleValuesParser::new(["mag", "gyro", "accl", "baro"]))
                .requires("fit"))
                
            // GPMF options
            .arg(Arg::new("gpmf")
                .help("[GoPro] Unedited GoPro MP4-file, or extracted GPMF-track. Exctracted GPMF-tracks do not contain relative timestamps, since these are derived via the MP4 file.")
                .long("gpmf")
                .short('g')
                .value_parser(clap::value_parser!(PathBuf))
                .required_unless_present_any(&["video", "fit"])
                .conflicts_with_all(&["fit", "video", "global"]))
            
            // General inspect options.
            .arg(Arg::new("session")
                .help("[GoPro/VIRB] Select recording session via UUID for GarminVIRB/FIT. GoPro-files in the same folder as '--gpmf' will be located automatically.")
                .long("session")
                .short('s')
                .action(ArgAction::SetTrue))
            .arg(Arg::new("kml")
                .help("[GoPro/VIRB] Generate a KML file from GPS-logs. Points only. Garmin VIRB GPS-data will be automatically downsampled to roughly 1 point/second.")
                .long("kml")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("indexed-kml")
                .help("[GoPro/VIRB] Generate a KML file from GPS-logs, where each point gets a counter/index as name. Points only. Garmin VIRB GPS-data will be automatically downsampled to roughly 1 point/second.")
                .long("ikml")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("json")
                .help("[GoPro/VIRB] Generate a GeoJSON file from GPS-logs. Points only. Garmin VIRB GPS-data will be automatically downsampled to roughly 1 point/second.")
                .long("json")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("verbose")
                .help("[GoPro/VIRB] Prints all telemetry data.")
                .long("verbose")
                .conflicts_with_all(&["gps", "debug"])
                .action(ArgAction::SetTrue))
            .arg(Arg::new("data-type")
                .help("[GoPro/VIRB] Print telemetry data for specified data type. For FIT, this means specifying the numerical ID. For GPMF, a string. For e.g. GPS-data this means the global FIT ID 160, and for GPMF the string 'GPS (Lat., Long., Alt., 2D speed, 3D speed)'. Both of these can be looked up by running 'inspect' with only the data file specified, which will print a summary for either format.")
                .long("type")
                .short('t')
                // TODO value parser possible? string for gopro, u16 for virb
                .conflicts_with_all(&["gps", "debug", "verbose"]))
            .arg(Arg::new("gps")
                .help("[GoPro/VIRB] Print GPS log.")
                .long("gps")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(&["verbose", "debug", "data-type"])) // gps always prints points
            .arg(Arg::new("meta")
                .help("[GoPro/VIRB] Print additional information, such as the FIT header and custom metadata (udta) for GoPro MP4-files.")
                .action(ArgAction::SetTrue)
                .long("meta"))
            .arg(Arg::new("atoms")
                .help("[GoPro/VIRB] Print MP4 atom information if specified file is a video.")
                .action(ArgAction::SetTrue)
                .long("atoms"))
                // .conflicts_with_all(&["verbose", "debug", "data-type"])) // gps always prints points
            .arg(Arg::new("debug")
                .help("[GoPro/VIRB] Print debug info for all data while parsing.")
                .long("debug")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(&["gps", "verbose", "data-type"])) // gps always prints points
        )
        .get_matches();
    
    // VIEW, SAVE MANUAL
    if let Some(arg_matches) = args.subcommand_matches("manual") {
        manual::run(&arg_matches)?;
    }

    // ACTION CAMERA FOOTAGE TO EAF, VIRB
    if let Some(arg_matches) = args.subcommand_matches("virb2eaf") {
        cam2eaf::virb2eaf::run(&arg_matches)?;
    }
    
    // ACTION CAMERA FOOTAGE TO EAF, GOPRO
    if let Some(arg_matches) = args.subcommand_matches("gopro2eaf") {
        cam2eaf::gopro2eaf::run(&arg_matches)?;
    }

    // EAF TO KML/GEOJSON
    if let Some(arg_matches) = args.subcommand_matches("eaf2geo") {
        eaf2geo::run(&arg_matches)?;
    }
    
    // INSPECT TELEMETRY, VIRB + GOPRO
    if let Some(arg_matches) = args.subcommand_matches("inspect") {
        inspect::run(&arg_matches)?;
    }

    // LOCATE AND MATCH FILES, VIRB + GOPRO
    if let Some(arg_matches) = args.subcommand_matches("locate") {
        locate::run(&arg_matches)?;
    }

    Ok(())
}
