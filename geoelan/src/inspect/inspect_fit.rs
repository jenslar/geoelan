//! Inspect Garmin FIT files. Supports non-VIRB files.

use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, ErrorKind};
use std::path::PathBuf;

use fit_rs::{Fit, FitSessions, SensorType};

use crate::files::{affix_file_name, writefile};
use crate::files::virb::select_session;
use crate::geo::{EafPoint, downsample, EafPointCluster};
use crate::geo::geo_fit::set_datetime_fit;

pub fn inspect_fit(args: &clap::ArgMatches) -> std::io::Result<()> {

    let fit_path: Option<&PathBuf> = args.get_one("fit");
    let debug = *args.get_one::<bool>("debug").unwrap();

    if debug {
        if let Some(path) = fit_path {
            // Want error while parsing in this case
            let _fit = Fit::debug(path);
        }
    }

    let path = fit_path.unwrap();

    let mut fit = Fit::new(&path)?;
    if let Err(err) = fit.index() {
        println!("(!) Failed to map sessions for {}: {err}", path.display());
        std::process::exit(1)
    };

    // populate message type name, offset, scale fields for records.
    fit.augment();

    // Collect remaining args.
    let mut verbose = *args.get_one::<bool>("verbose").unwrap();
    let print_sensor = args.get_one::<String>("sensor");
    let print_gps = *args.get_one::<bool>("gps").unwrap();
    let (save_kml, indexed_kml) = (
        *args.get_one::<bool>("kml").unwrap()
        || *args.get_one::<bool>("indexed-kml").unwrap(), *args.get_one::<bool>("indexed-kml").unwrap()
    );
    let full_gps = *args.get_one::<bool>("fullgps").unwrap();
    let save_json = *args.get_one::<bool>("json").unwrap();
    let save_csv = *args.get_one::<bool>("csv").unwrap(); // only for sensor data gyro, grav, accl, gps
    // NOTE data-type is u16 for fit, string for gpmf...
    let global_id: Option<u16> = match args.get_one::<String>("data-type") {
        Some(id) => {
            match id.parse() {
                Ok(g) => {
                    verbose = true;
                    Some(g)
                }
                Err(err) => {
                    let msg = format!("(!) 'global-id' must be a valid number: {err}");
                    return Err(std::io::Error::new(ErrorKind::Other, msg))
                }
            }
        }
        None => None,
    };
    let mut fit_session = if Some(&true) == args.get_one::<bool>("session") {
        Some(select_session(&fit)?)
    } else {
        None
    };
    
    let range = fit_session.as_ref().map(|s| s.range());

    // Filter records
    let records = fit.filter(global_id, range.as_ref());
    
    // Get GPS log as points
    let points = match print_gps || save_kml || save_json {
        true => match fit.points(range.as_ref()) {
            Ok(gm) => {
                let mut pts: Vec<EafPoint> = gm.iter()
                    .map(EafPoint::from)
                    .collect();
                match set_datetime_fit(&mut pts, &fit, 0) {
                    Ok(_) => println!("Set date time for points."),
                    Err(_) => println!("Unable to set date time for points, not a VIRB file."),
                };
                Some(pts)
            },
            Err(err) => return Err(err.into())
        },
        false => None
    };

    if let Some(pts) = &points {
        if pts.is_empty() {
            println!("(!) No GPS log found.")
        } else {
            let mut csv: Vec<String> = vec!["INDEX\tDATETIME\tTIMESTAMP\tLATITUDE\tLONGITUDE\tALTITUDE\tSPEED2D\tSPEED3D".to_owned()];

            if print_gps {
                for (i, point) in pts.iter().enumerate() {
                    println!("[{:6}]\n{point}", i+1);
                    csv.push(format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                        // counter,
                        i+1,
                        // !!! datetime = None? works for gpmf...
                        point.datetime_string().as_deref().unwrap_or("Unspecified"),
                        point.timestamp.map(|t| t.as_seconds_f64().to_string()).as_deref().unwrap_or("Unspecified"),
                        point.latitude,
                        point.longitude,
                        point.altitude,
                        point.speed2d,
                        point.speed3d,
                    ))
                }
        
                if let Some(p) = pts.first() {
                    println!("-------------------");
                    println!("First logged point:\n{p}");
                }

                if save_csv {
                    // Re-use and filename from e.g. GH010006.MP4 to GH010006_GPS,csv
                    // !!! TODO change affix_file_name to return Option<PathBuf> to avoid overwriting
                    let csv_path = affix_file_name(&path, None, Some("_GPS"), Some("csv"));
                    let mut csv_file = File::create(&csv_path)?;
                    csv_file.write_all(csv.join("\n").as_bytes())?;
                    println!("Wrote {}", csv_path.display());
                }
        
                return Ok(())
            }
    
            if save_kml || save_json {
                // Downsample FIT points to 1Hz / 1pt/sec (GoPro is already extracted as roughly 1Hz)
                let downsampled_points = match full_gps {
                    true => pts.to_owned(),
                    false => downsample(10, pts, None)
                };
                
                // Generate KML object and write to disk
                if save_kml {
                    let kml_doc = EafPointCluster::new(&downsampled_points, None)
                        .to_kml_string(indexed_kml);
                    let kml_path = affix_file_name(&path, None, Some("_points"), Some("kml"));
                    match writefile(&kml_doc.as_bytes(), &kml_path) {
                        Ok(true) => println!("Wrote {}", kml_path.display()),
                        Ok(false) => println!("User aborted writing KML-file"),
                        Err(err) => return Err(err),
                    }
                }
        
                // Generate GeoJSON object and write to disk
                if save_json {
                    let geojson_doc = EafPointCluster::new(&downsampled_points, None)
                        .to_json_string(indexed_kml);
                    let geojson_path = affix_file_name(&path, None, Some("points"), Some("geojson"));
                    match writefile(&geojson_doc.as_bytes(), &geojson_path) {
                        Ok(true) => println!("Wrote {}", geojson_path.display()),
                        Ok(false) => println!("User aborted writing GeoJSON-file"),
                        Err(err) => return Err(err),
                    }
                }

                println!("Done");
                return Ok(())
            }
        }

    }

    if let Some(sensor_type) = print_sensor {
        let sensor_type = match sensor_type.as_str() {
            "mag" | "magnetometer" => SensorType::Magnetometer,
            "gyr" | "gyroscope" => SensorType::Gyroscope,
            "acc" | "accelerometer" => SensorType::Accelerometer,
            "bar" | "barometer" => SensorType::Barometer,
            s => {
                let msg = format!("(!) Unknown VIRB sensor: '{s}'. Valid choices are: magnetometer, gyroscope, accelerometer, barometer");
                return Err(std::io::Error::new(ErrorKind::Other, msg))
            }
        };

        let calibrated_sensor_data = match fit.sensor(&sensor_type, range.as_ref()) {
            Ok(data) => data,
            Err(err) => {
                let msg = format!("(!) Failed to compile sensor data: {err}");
                return Err(std::io::Error::new(ErrorKind::Other, msg))
            }
        };

        for data in calibrated_sensor_data.iter() {
            println!("{data:?}");
        }

        println!("Done");
        return Ok(())
    }

    // Key: (Global ID, Message Type), Value: count
    let mut stats: HashMap<(u16, String), usize> = HashMap::new();
    let mut count: usize = 0;

    for record in records.iter() {
        *stats.entry((record.global, record.name())).or_insert(0) += 1;
        count += 1;
        if verbose {
            if global_id.is_some() && global_id != Some(record.global) {
                continue;
            }
            println!("[{count}] {record}"); // TODO 200809 reimplement Display check old alpha
        }
    }

    let mut stats_sorted: Vec<_> = stats.iter()
        .map(|((global, name), count)| (global, name, count))
        .collect();
    stats_sorted.sort_by_key(|(global, ..)| global.to_owned());


    println!("\nSummary");
    if Some(&true) == args.get_one::<bool>("meta") {
        println!("{}", "-".repeat(51));
        println!("Header\n");
        println!("      size: {}", fit.header.headersize);
        println!("  protocol: {}", fit.header.protocol);
        println!("   profile: {}", fit.header.profile);
        println!("  datasize: {}", fit.header.datasize);
        println!("    dotfit: {:?}", fit.header.dotfit);
        println!(
            "       crc: {:?}",
            fit.header.crc
        );
    }
    println!("{}", "-".repeat(51));
    println!("Data\n");
    println!(" Global ID | {:28} | Count", "Message type");
    println!("{}", ".".repeat(51));
    for (global, name, count) in stats_sorted.iter() {
        println!("{:10} | {:28} | {:6}", global, name, count);
    }
    println!("{}", ".".repeat(51));
    println!("{:36}Total:{:8} ", " ", count);

    if let Some(session) = &mut fit_session {
        if let Err(err) = session.derive() {
            println!("(!) Failed to derive session: {err}");
        };
        if let Ok((start, end)) = session.timespan_abs(None, true) {
            println!("Session time span:");
            println!("  Start:    {}", start.to_string());
            println!("  End:      {}", end.to_string());
            let duration = end - start;
            let (sec, ms) = (
                duration.whole_seconds(),
                duration.whole_milliseconds() - duration.whole_seconds() as i128 * 1000
            );
            println!("  Duration: {sec}s {ms}ms");
        }
    }
        
    if let Some(session) = &fit_session {
        println!("UUIDs in session:");
        for (i, u) in session.uuid.iter().enumerate() {
            println!(" {:2}. {}", i + 1, u);
        }
        if session.uuid.is_empty() {
            println!("  None")
        }
    } else {
        let sessions = FitSessions::from_fit(&fit)?;
        println!("Sessions in file:");
        for (i1, session) in sessions.iter().enumerate() {
            println!(" Session {:2}", i1 + 1);
            for (i2, u) in session.uuid.iter().enumerate() {
                println!(" {:2}. {}", i2 + 1, u);
            }
            if session.uuid.is_empty() {
                println!("  None")
            }
        }
        if sessions.is_empty() {
            println!("  None")
        }
    };

    println!("Done");

    Ok(())
}