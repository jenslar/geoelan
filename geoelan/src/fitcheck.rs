use crate::files::writefile;
use crate::virb::{select_session, session_timespan};
use chrono::offset::TimeZone;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Instant;

// main check sub-command
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let fitpath = PathBuf::from(args.value_of("fit").unwrap()).canonicalize()?;
    let fitfile = fit::structs::FitFile::new(&fitpath);

    let mut verbose = args.is_present("verbose");
    let (debug, debug_unchecked_strings) = match args.is_present("debug-unchecked") {
        true => (true, true),
        false => (args.is_present("debug"), false),
    };

    let global_id: Option<u16> = match args.value_of("global-id") {
        Some(id) => {
            verbose = true;
            Some(id.parse().expect("Error parsing global ID"))
        }
        None => None,
    };

    // Set UUID depending on source.
    // 'None' means all FIT-data will be extracted.
    // 'Some()' means only FIT-data for recording session with set starting UUID will be extracted.
    let uuid: Option<String> = if let Some(v) = args.value_of("video") {
        match fit::get_video_uuid(&Path::new(v))? {
            Some(u) => Some(u),
            None => {
                println!("Specified MP4 contains no UUID.");
                exit(0)
            }
        }
    } else if let Some(u) = args.value_of("uuid") {
        Some(u.to_string())
    } else if args.is_present("select") {
        Some(select_session(&fitfile)?)
    } else {
        None
    };

    let timer = Instant::now();

    let mut error_message: Option<fit::errors::ParseError> = None; // for partial parse with error

    let fitdata_result = if debug {
        verbose = false;
        fitfile.debug(debug_unchecked_strings)
    } else {
        fitfile.parse(&global_id, &uuid)
    };
    let fitdata = match fitdata_result {
        Ok(d) => d,
        Err(e) => match e {
            fit::errors::FitError::Fatal(e) => {
                eprintln!("Aborted. Fatal error: {}", e);
                exit(1)
            }
            fit::errors::FitError::Partial(e, d) => {
                error_message = Some(e);
                d
            }
        },
    };

    // Get video duration/time span via UUID restrictions
    let video_time_span = if uuid.is_some() {
        let cam = match fitfile.cam(&None, true) {
            Ok(data) => data,
            Err(err) => {
                println!("Could not determine session: {}", err);
                exit(1)
            }
        };
        session_timespan(&cam, &uuid, false) // only limit to uuid/session here
    } else {
        None
    };

    let mut stats: HashMap<(u16, String), usize> = HashMap::new(); // k: (global_id, description), v: count
    let mut count = 0; // k: (global_id, description), v: count

    for msg in fitdata.records.iter() {
        *stats
            .entry((msg.global, msg.description.clone()))
            .or_insert(0) += 1;
        if verbose {
            if global_id.is_some() && global_id != Some(msg.global) {
                continue;
            }
            count += 1;
            println!(
                "[{}] Global ID: {} | Message type: {} | Header: {3:?}/{3:#010b}",
                count, msg.global, msg.description, msg.header
            );
            for field in msg.fields.iter() {
                println!(
                    "    id: {:3} {:22}: {:?} {}",
                    field.field_definition_number,
                    field.description,
                    field.data,
                    fit::messages::field_types::get_enum(msg.global, &field)
                );
            }
            for field in msg.dev_fields.iter() {
                println!(
                    "DEV id: {:3} {:22}: {:?} {} (units: {})",
                    field.field_definition_number,
                    field.description,
                    field.data,
                    fit::messages::field_types::get_enum(msg.global, &field),
                    field.units.clone().unwrap_or_else(|| String::from("N/A"))
                );
            }
        }
    }

    println!("\nSUMMARY");
    println!("{}", "-".repeat(51));
    println!("Header\n");
    println!("      size: {}", fitdata.header.headersize);
    println!("  protocol: {}", fitdata.header.protocol);
    println!("   profile: {}", fitdata.header.profile);
    println!("  datasize: {}", fitdata.header.datasize);
    println!("    dotfit: {:?}", fitdata.header.dotfit);
    println!("       crc: {:?}", fitdata.header.crc); // TODO 201129 crc not verified

    // print table with fit message counts in file
    println!("{}", "-".repeat(51));
    println!("Data\n");
    println!(" Global ID | {:28} | Count", "Message type");
    println!("{}", ".".repeat(51));
    let required = vec![160, 161, 162];
    let mut req_count = 0;
    for (k, v) in stats.iter() {
        print!("{:10} | {:28} | {:6}", k.0, k.1, v);
        if required.contains(&k.0) {
            req_count += 1;
            println!(" *");
        } else {
            println!("");
        };
    }
    println!("{}", ".".repeat(51));
    println!("{:36}Total:{:8} ", " ", fitdata.len());

    // print time spans
    println!("{}", "-".repeat(51));
    if let Some(span) = video_time_span {
        // probably only VIRB
        println!("Session time span\n");
        match fitfile.t0(0, true) {
            Ok(t) => {
                println!(
                    "  Start:    {}",
                    (t + span.start).format("%Y-%m-%dT%H:%M:%S%.3f")
                );
                println!(
                    "  End:      {}",
                    (t + span.end).format("%Y-%m-%dT%H:%M:%S%.3f")
                );
            }
            Err(err) => { // only fatal errors since return_partial = true
                println!("  Could not determine absolute timeline in FIT-file: {}\n  Printing relative values.", err);
                println!(
                    "  Start:    {}s {}ms",
                    span.start.num_seconds(),
                    span.start.num_milliseconds() - span.start.num_seconds() * 1000
                );
                println!(
                    "  End:      {}s {}ms",
                    span.end.num_seconds(),
                    span.end.num_milliseconds() - span.end.num_seconds() * 1000
                );
            }
        }

        println!(
            "  Duration: {}s {}ms",
            (span.end - span.start).num_seconds(),
            (span.end - span.start).num_milliseconds()
                - (span.end - span.start).num_seconds() * 1000
        );
        println!("{}", "-".repeat(51));
    }

    // print uuids in file/recording session
    println!(
        "UUIDs in {}\n",
        if uuid.is_some() {
            "selected session"
        } else {
            "file"
        }
    );

    if let Ok(uuids) = fitfile.uuid(&uuid, true) {
        for (i, u) in uuids.iter().enumerate() {
            println!(" {:2}. {}", i + 1, u);
        }
        if uuids.is_empty() {
            println!("  None")
        }
    } else {
        println!("  None")
    };

    println!("{}", "-".repeat(51));
    println!("Result\n");
    print!("  Message types 160, 161, 162 (*) present: ");
    if uuid.is_none() && global_id.is_none() {
        if req_count == 3 {
            println!("YES [may lack required fields if non-VIRB FIT-file]")
        } else {
            println!("NO")
        }
    } else {
        println!("N/A")
    }
    match error_message {
        Some(e) => println!("  Partial parse with error: \"{}\"", e),
        None => println!("  Parsed in full, no errors"),
    }
    println!("{}", "-".repeat(51));

    // generate kml, points only
    if args.is_present("kml") {
        let downsample_factor: usize = args
            .value_of("downsample-factor")
            .unwrap()
            .parse()
            .expect("Could not parse '--downsample'");

        let gps = match fitfile.gps(&uuid, false) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error extracting GPS data: {}", err);
                println!("Retrying without UUID restrictions... ");
                match fitfile.gps(&None, false) {
                    Ok(g) => println!("Done.\n  Found {} GPS messages in total", g.len()),
                    Err(e) => match e {
                        fit::errors::FitError::Fatal(e) => eprintln!("Could not extract any GPS data from FIT-file: {}", e),
                        fit::errors::FitError::Partial(e,d) => eprintln!("Could only extract partial GPS data from FIT-file ({} data messages extracted): {}", d.len(), e),
                    }
                }
                exit(1)
            }
        };

        let downsampled_points = crate::geo::downsample(downsample_factor, &gps);

        let mut kml_path = PathBuf::from(&fitfile.path);
        kml_path.set_extension("kml");

        let uuids = fitfile.uuid(&uuid, true)?;

        let kml_doc = crate::kml::write::build(
            &crate::structs::GeoType::POINT(downsampled_points),
            &fitfile.t0(0, true).unwrap_or_else(|_| {
                chrono::offset::Utc
                    .ymd(1989, 12, 31)
                    .and_hms_milli(0, 0, 0, 0)
            }),
            &uuids,
            "Garmin VIRB",
            false,
        );

        writefile(&kml_doc.as_bytes(), &kml_path)?;
        println!("{}", "-".repeat(51));
    }
    println!(
        "Done ({:.3}s)",
        (timer.elapsed().as_millis() as f64) / 1000.0
    );
    Ok(())
}
