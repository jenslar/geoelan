use crate::files::writefile;
use crate::virb::{select_session, session_timespan};
use chrono::offset::TimeZone;
use fit_rs::{errors::FitError, get_video_uuid, structs::FitFile};
use itertools::Itertools;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Instant;

//////////////////////////
// MAIN CHECK SUB-COMMAND
//////////////////////////
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let fit_path = PathBuf::from(args.value_of("fit").unwrap()).canonicalize()?;

    let mut verbose = args.is_present("verbose");
    let (debug, debug_unchecked_strings) = match args.is_present("debug-unchecked") {
        true => (true, true),
        false => (args.is_present("debug"), false),
    };

    let mut argument_error_messages: Vec<String> = Vec::new();
    let mut parse_error_messages: Vec<String> = Vec::new();

    let global_id: Option<u16> = match args.value_of("global-id") {
        Some(id) => {
            match id.parse() {
                Ok(g) => {
                    verbose = true;
                    Some(g)
                }
                Err(e) => {
                    argument_error_messages.push(format!(
                        "'--global-id': Invalid value '{}'. Must be a number: {}",
                        id, e
                    ));
                    None // print full overview on error
                }
            }
        }
        None => None,
    };

    let timer_parse = Instant::now();

    let fit_file_result = if debug {
        verbose = false;
        FitFile::debug(&fit_path, debug_unchecked_strings)
    } else {
        FitFile::parse(&fit_path, false)
    };
    let fit_file = match fit_file_result {
        Ok(d) => d,
        Err(FitError::Fatal(e)) => {
            println!("Aborted. Fatal parse error: {}", e);
            exit(1)
        }
        Err(FitError::Partial(e, d)) => {
            parse_error_messages.push(format!("{}", e));
            d
        }
    };

    // exclude selection time for `--select`
    let timer_parse_span = timer_parse.elapsed();

    // Set UUID depending on source.
    // 'None' means all FIT-data will be extracted.
    // 'Some()' means only FIT-data for recording session
    // starting with specified UUID will be extracted.
    let uuid: Option<String> = if let Some(v) = args.value_of("video") {
        match get_video_uuid(&Path::new(v))? {
            Some(u) => Some(u),
            None => {
                argument_error_messages.push(format!("'--video': Specified MP4 contains no UUID."));
                None // print full overview on error
            }
        }
    } else if let Some(u) = args.value_of("uuid") {
        Some(u.to_string())
    } else if args.is_present("select") {
        match select_session(&fit_file) {
            Ok(s) => Some(s),
            Err(e) => {
                argument_error_messages.push(format!(
                    "Not a VIRB FIT-file or required data not present: {}",
                    e
                ));
                None // print full overview on error
            }
        }
    } else {
        None
    };

    // Get video duration/time span via UUID restrictions
    let video_time_span = if uuid.is_some() {
        match fit_file.cam(None) {
            Ok(data) => {
                session_timespan(&data, uuid.as_ref(), false) // only limit to uuid/session here
            }
            Err(err) => {
                parse_error_messages.push(format!("Could not determine session/duration: {}", err));
                None
            }
        }
    } else {
        None
    };

    let mut stats: HashMap<(u16, String), usize> = HashMap::new(); // k: (global_id, description), v: count
    let mut count = 0; // k: (global_id, description), v: count

    let timer_filter = std::time::Instant::now();

    let records = match (global_id, &uuid) {
        (Some(g), None) => fit_file.filter(g),
        (_, Some(u)) => fit_file.filter_session(u, global_id),
        (None, None) => fit_file.records.to_owned(),
    };

    let timer_filter_span = timer_filter.elapsed();

    for msg in records.iter() {
        *stats
            .entry((msg.global, msg.description.clone()))
            .or_insert(0) += 1;
        if verbose {
            if global_id.is_some() && global_id != Some(msg.global) {
                continue;
            }
            count += 1;
            println!("[{}] {}", count, msg); // msg has impl Display
        }
    }

    println!("\nSUMMARY");
    println!("{}", "-".repeat(51));
    println!("Header\n");
    println!("      size: {}", fit_file.header.headersize);
    println!("  protocol: {}", fit_file.header.protocol);
    println!("   profile: {}", fit_file.header.profile);
    println!("  datasize: {}", fit_file.header.datasize);
    println!("    dotfit: {:?}", fit_file.header.dotfit);
    println!(
        "       crc: {:?} (for bytes 0-11 of header)",
        fit_file.header.crc
    );

    // print table with fit message counts in file
    println!("{}", ".".repeat(51));
    println!("   FIT crc: {:?} (for file)", fit_file.crc);
    println!("{}", "-".repeat(51));
    println!("Data\n");
    println!(" Global ID | {:28} | Count", "Message type");
    println!("{}", ".".repeat(51));
    let required: Vec<u16> = vec![160, 161, 162];
    let mut req_found: Vec<u16> = Vec::new();
    for (k, v) in stats.iter().sorted() {
        print!("{:10} | {:28} | {:6}", k.0, k.1, v);
        if required.contains(&k.0) {
            req_found.push(k.0);
            println!(" *");
        } else {
            println!();
        };
    }
    println!("{}", ".".repeat(51));
    println!("{:36}Total:{:8} ", " ", records.len());

    // print time spans
    println!("{}", "-".repeat(51));
    if let Some(span) = video_time_span {
        // probably only VIRB
        println!("Session time span\n");
        match fit_file.t0(0) {
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
            Err(err) => {
                // only fatal errors since return_partial = true
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

    if let Some(start_uuid) = &uuid {
        for session in fit_file.sessions()? {
            if &session[0] == start_uuid {
                for (i, s) in session.iter().enumerate() {
                    println!("  {}. {}", i + 1, s)
                }
            }
        }
    } else if let Ok(uuids) = fit_file.uuid() {
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
    println!("  Required message types for ELAN workflow (VIRB FIT-files only):");
    if uuid.is_none() && global_id.is_none() {
        for id in required.iter() {
            print!("    {}: ", id);
            if req_found.contains(&id) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    } else {
        println!("    N/A, run again without '--uuid'/'--global-id'/'--select'.")
    }
    println!("  Errors");
    if parse_error_messages.is_empty() {
        println!("    Parse error: None")
    } else {
        for e in parse_error_messages.iter() {
            println!("    Parse error: {}", e)
        }
    }
    if argument_error_messages.is_empty() {
        println!("    Input error: None")
    } else {
        for e in argument_error_messages.iter() {
            println!("    Input error: {}", e);
        }
    }
    println!("{}", "-".repeat(51));

    // generate kml, points only
    if args.is_present("kml") {
        let downsample_factor: usize = args
            .value_of("downsample-factor")
            .unwrap()
            .parse()
            .expect("Could not parse '--downsample'");

        let gps = match fit_file.gps(uuid.as_ref()) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error extracting GPS data: {}", err);
                println!("Retrying without UUID restrictions... ");
                match fit_file.gps(None) {
                    Ok(g) => println!("Done.\n  Found {} GPS messages.", g.len()),
                    Err(e) => println!("Unable to extract GPS data: {}", e),
                }
                exit(1)
            }
        };

        let downsampled_points = crate::geo::downsample(downsample_factor, &gps);

        let mut kml_path = PathBuf::from(&fit_file.path);
        kml_path.set_extension("kml");

        let uuids = fit_file.uuid()?; // FIXME currently ALL uuids

        let kml_doc = crate::kml::write::build(
            &crate::structs::GeoType::POINT(downsampled_points),
            &fit_file.t0(0).unwrap_or_else(|_| {
                chrono::offset::Utc
                    .ymd(1989, 12, 31)
                    .and_hms_milli(0, 0, 0, 0)
            }),
            &uuids,
            "Garmin VIRB",
            false,
            None,
        );

        writefile(&kml_doc.as_bytes(), &kml_path)?;
        println!("{}", "-".repeat(51));
    }

    let timer_sessions = std::time::Instant::now();
    let _ = fit_file.sessions();

    println!(
        "Done\n  parse    {:?}\n  filter   {:?}\n  sessions {:?}",
        timer_parse_span,
        timer_filter_span,
        timer_sessions.elapsed()
    );
    Ok(())
}
