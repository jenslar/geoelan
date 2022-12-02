//! Inspect Garmin FIT files. Supports non-VIRB files.

use std::collections::HashMap;
use std::path::PathBuf;

use fit_rs::{Fit, FitSessions, SensorType, VirbFile};

use crate::files::{affix_file_name, writefile};
use crate::files::virb::select_session;
use crate::geo::{Point, downsample};
use crate::geo::geo_fit::set_datetime_fit;
use crate::geo::json_gen::{geojson_point, geojson_from_features};
use crate::geo::kml_gen::{kml_point, kml_from_placemarks, kml_to_string};

pub fn inspect_fit(args: &clap::ArgMatches) -> std::io::Result<()> {

    let fit_path: Option<&PathBuf> = args.get_one("fit");

    let video: Option<&PathBuf> = args.get_one("video");

    let print_meta = *args.get_one::<bool>("meta").unwrap(); // clap: false if not present

    // Print UUID in MP4 then exit if no FIT specified
    if fit_path.is_none() {
        if let Some(path) = video {
            match VirbFile::new(path, None) {
                Ok(virb) => {
                    println!("UUID:     {}", virb.uuid);
                    if let Some(duration) = virb.duration() {
                        println!("Duration: {}", duration.to_string())
                    }

                    if print_meta {
                        match virb.meta() {
                            Ok(meta) => {
                                for field in meta.udta.iter() {
                                    println!("{:?}", field);
                                }
                            },
                            Err(err) => {
                                println!("(!) Failed to read custom metadata in '{}': {err}", path.display())
                            }
                        }
                    }
                    std::process::exit(0)
                },
                Err(err) => {
                    println!("(!) Failed to read '{}' (is it an original VIRB file?): {err}", path.display());
                    std::process::exit(1)
                }
            }
        }
    }

    let path = fit_path.unwrap();
    let mut fit = match Fit::new(&path) {
        Ok(data) => data,
        Err(err) => {
            println!("(!) Failed to parse '{}': {err}", path.display());
            std::process::exit(1)
        },
    };
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
    let full_gps = *args.get_one::<bool>("full-gps").unwrap();
    let save_json = *args.get_one::<bool>("json").unwrap();
    // NOTE data-type is u16 for fit, string for gpmf...
    let global_id: Option<u16> = match args.get_one::<String>("data-type") {
        Some(id) => {
            match id.parse() {
                Ok(g) => {
                    verbose = true;
                    Some(g)
                }
                Err(err) => {
                    println!("(!) 'global-id' must be a valid number: {err}");
                    std::process::exit(1)
                }
            }
        }
        None => None,
    };
    let fit_session = if Some(&true) == args.get_one::<bool>("session") {
        match select_session(&fit) {
            Ok(s) => Some(s),
            Err(err) => {
                println!("(!) Not a VIRB FIT-file or no sessions present: {err}");
                std::process::exit(1)
            }
        }
    } else {
        None
    };
    
    let range = fit_session.as_ref().map(|s| s.range());

    // Filter records
    let records = fit.filter(global_id, range.as_ref());
    
    // Get GPS log as points
    let points = match print_gps || save_kml || save_json { // add kml flag here later
        true => match fit.points(range.as_ref()) {
            Ok(gm) => {
                let mut pts: Vec<Point> = gm.iter()
                    .map(Point::from)
                    .collect();
                match set_datetime_fit(&mut pts, &fit, 0) {
                    Ok(_) => println!("Set date time for points."),
                    Err(_) => println!("Unable to set date time for points, not a VIRB file."),
                };
                Some(pts)
            },
            Err(err) => {
                println!("(!) Failed to extract GPS data: {err}");
                std::process::exit(1)
            }
        },
        false => None
    };

    if let Some(pts) = &points {
        if pts.is_empty() {
            println!("No GPS log found.")
        } else {

            if print_gps {
                for (i, point) in pts.iter().enumerate() {
                    println!("[{:6}]\n{point}", i+1);
                }
        
                if let Some(p) = pts.first() {
                    println!("-------------------");
                    println!("First logged point:\n{p}");
                }
        
                std::process::exit(0)
            }
    
            if save_kml || save_json {
                // Downsample FIT points to 1Hz / 1pt/sec (GoPro is that already)
                let downsampled_points = match full_gps {
                    true => pts.to_owned(),
                    false => downsample(10, pts, None)
                };
                
                if save_kml {
                    let kml_points: Vec<kml::types::Placemark> = downsampled_points.iter().enumerate()
                        .map(|(i, p)| {
                            let name = match indexed_kml {
                                true => Some((i+1).to_string()),
                                false => None
                            };
                            kml_point(p, name.as_deref(), None, false, None)
                        })
                        .collect();
                    let kml = kml_from_placemarks(&kml_points, &[]);
        
                    let kml_doc = kml_to_string(&kml);
                    let kml_path = affix_file_name(&path, None, Some("points")).with_extension("kml");
        
                    match writefile(&kml_doc.as_bytes(), &kml_path) {
                        Ok(true) => println!("Wrote {}", kml_path.display()),
                        Ok(false) => println!("User aborted writing ELAN-file"),
                        Err(err) => {
                            println!("(!) Failed to write '{}': {err}", kml_path.display());
                            std::process::exit(1)
                        },
                    }
                }
        
                if save_json {
                    let json_points: Vec<geojson::Feature> = downsampled_points.iter()
                        .map(|p| geojson_point(p, None))
                        .collect();
                    let geojson = geojson_from_features(&json_points);
        
                    // Serialize GeoJSON. Not indented (= smaller size for web use).
                    let geojson_doc = geojson.to_string();
                    let geojson_path = affix_file_name(&path, None, Some("points")).with_extension("geojson");
        
                    match writefile(&geojson_doc.as_bytes(), &geojson_path) {
                        Ok(true) => println!("Wrote {}", geojson_path.display()),
                        Ok(false) => println!("User aborted writing ELAN-file"),
                        Err(err) => {
                            println!("(!) Failed to write '{}': {err}", geojson_path.display());
                            std::process::exit(1)
                        },
                    }
                }

                println!("Done");
                std::process::exit(0)
            }
        }

    }

    if let Some(sensor_type) = print_sensor {
        let sensor_type = match sensor_type.as_str() {
            "gyro" => SensorType::Gyroscope,
            "accl" => SensorType::Accelerometer,
            "mag" => SensorType::Magnetometer,
            "baro" => SensorType::Barometer,
            _ => {
                println!("(!) Unknown sensor type.");
                std::process::exit(1)
            }
        };

        let calibrated_sensor_data = match fit.sensor(&sensor_type, range.as_ref()) {
            Ok(data) => data,
            Err(err) => {
                println!("(!) Failed to compile sensor data: {err}");
                std::process::exit(1)
            }
        };

        for data in calibrated_sensor_data.iter() {
            println!("{data:?}");
        }

        println!("Done");
        std::process::exit(0)
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


    println!("\nSUMMARY");
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

    if let Some(session) = &fit_session {
        if let Ok((start, end)) = session.datetime(None, true) {
            // if let (first_pt, last_pt) = points.as_deref().map(|pts| (pts.first(), pts.last())) {

            // };
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