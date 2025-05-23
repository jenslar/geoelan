//! Inspect GoPro GPMF data. Supports "raw" GPMP-files, e.g. by having extracted the `GoPro MET` track from a GoPro MP4-file.

use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::{fs::File, path::Path};

use gpmf_rs::{DataType, FourCC, GoProFile, GoProSession, Gpmf, SensorType};

use crate::files::Units;
use crate::{
    files::{affix_file_name, has_extension},
    geo::{downsample, point::EafPoint, EafPointCluster},
};

pub fn inspect_gpmf(args: &clap::ArgMatches) -> std::io::Result<()> {
    let path = args.get_one::<PathBuf>("gpmf").unwrap().canonicalize()?; // clap: required arg
    let indir = match args.get_one::<PathBuf>("input-directory") {
        // Some(p) => p.to_owned().canonicalize()?, // derive absolute path, must exist
        Some(p) => p.to_owned(),
        None => match path.parent() {
            Some(d) => {
                if d == Path::new("") {
                    PathBuf::from(".")
                } else {
                    d.to_owned()
                }
            }
            None => {
                let msg = "(!) Failed to determine input directory";
                return Err(std::io::Error::new(ErrorKind::Other, msg));
            }
        },
    };

    let verbose = *args.get_one::<bool>("verbose").unwrap(); // clap: conflicts with debug, gps
    let debug = *args.get_one::<bool>("debug").unwrap(); // clap: conflicts with verbose, gps
    let print_gps = *args.get_one::<bool>("gps").unwrap(); // clap: conflicts with debug, verbose
    let full_gps = *args.get_one::<bool>("fullgps").unwrap();
    let sensor_type = args.get_one::<String>("sensor");
    let min_fix = args.get_one::<u32>("gpsfix").map(|n| *n);
    let max_dop = args.get_one::<f64>("gpsdop").map(|n| *n);
    let (save_kml, indexed_kml) = (
        *args.get_one::<bool>("kml").unwrap() || *args.get_one::<bool>("indexed-kml").unwrap(),
        *args.get_one::<bool>("indexed-kml").unwrap(),
    );
    let save_json = *args.get_one::<bool>("json").unwrap();
    let save_csv = *args.get_one::<bool>("csv").unwrap(); // only for sensor data gyro, grav, accl, gps
    let session = *args.get_one::<bool>("session").unwrap(); // clap: conflicts with debug, verbose
    let verify_gpmf = *args.get_one::<bool>("verify").unwrap();
    let data_type = args.get_one::<String>("data-type"); // clap: conflicts with debug, verbose

    let timer_gpmf = std::time::Instant::now();

    // Max size if reading as raw GPMF file, e.g. when extracting track via FFmpeg.
    let max_raw_size: u64 = 50_000_000;

    if has_extension(&path, "jpg") {
        let gpmf = Gpmf::from_jpg(&path, debug)?;

        if verbose {
            gpmf.print();
        }

        println!("SUMMARY");
        println!(
            "  Found {} DEVC streams (no descriptions in GoPro JPEG)",
            gpmf.len()
        );
        print!("  Device:           ");
        if let Some(stream) = gpmf.find(&FourCC::MINF) {
            println!("{:?}", stream.values())
        } else {
            println!("{}", gpmf.device_name().join(", "));
        }

        println!("Done.");
        return Ok(());
    }

    // Either merged GPMF data from multiple files
    // or from a single MP4 clip
    let gpmf: Gpmf;

    if session {
        println!("Locating GoPro-files and parsing GPMF-data...");

        // Compile GoPro files, parse GPMF-data
        let gopro_session = GoProSession::from_path(&path, Some(&indir), verify_gpmf, true, false)?;

        // If 'session' flag is passed the file/s must parse as MP4s
        println!("Located the following session files:");
        for (i, gopro_file) in gopro_session.iter().enumerate() {
            println!(
                "{:4}. MP4: {}",
                i + 1,
                gopro_file
                    .mp4
                    .as_ref()
                    .and_then(|f| f.to_str())
                    .unwrap_or("High-resolution MP4 not set")
            );
            println!(
                "      LRV: {}",
                gopro_file
                    .lrv
                    .as_ref()
                    .and_then(|f| f.to_str())
                    .unwrap_or("Low-resolution MP4 not set")
            );
        }

        println!("Merging GPMF-data for {} files...", gopro_session.len());
        gpmf = match gopro_session.gpmf() {
            Ok(g) => g,
            Err(err) => {
                // Print error then retry to parse as binary GPMF file
                println!("(!) Failed to merge GPMF: {err}");
                println!("--> Retrying specified file as raw GPMF-track (max file size {})...",
                    Units::from(max_raw_size)
                );
                Gpmf::from_raw(&path, Some(max_raw_size), debug)?
            }
        };

        println!(
            "Done ({} ms{})",
            timer_gpmf.elapsed().as_millis(),
            if debug { ", debug parse" } else { "" }
        );
    } else {
        gpmf = match GoProFile::new(&path) {
            Ok(gp) => match gp.gpmf() {
                    Ok(g) => g,
                    Err(err) => {
                        let msg = format!("(!) Failed to extract GPMF: {err}");
                        return Err(std::io::Error::new(ErrorKind::Other, msg));
                    }
                },
            Err(_err) => {
                println!("Not an MP4, reading as raw GPMF-track (at most {})...",
                    Units::from(max_raw_size)
                );
                match Gpmf::from_raw(&path, Some(max_raw_size), debug) {
                    Ok(g) => g,
                    Err(err) => {
                        let msg = format!("(!) Failed to extract GPMF: {err}");
                        return Err(std::io::Error::new(ErrorKind::Other, msg));
                    },
                }
            }
        };
    }

    let size = gpmf.len();
    println!("Extracted {} streams", size);
    let mut gps = gpmf.gps();
    let pruned_len = gps.prune_mut(min_fix, max_dop);

    if print_gps {
        let mut csv: Vec<String> = vec![
            "INDEX\tDATETIME\tTIMESTAMP\tLATITUDE\tLONGITUDE\tALTITUDE\tSPEED2D\tSPEED3D"
                .to_owned(),
        ];
        let point_cluster =
            EafPointCluster::new(&gps.iter().map(EafPoint::from).collect::<Vec<_>>(), None);

        for (i, point) in point_cluster.iter().enumerate() {
            println!("[{:4}]\n{}", i + 1, point);
            if save_csv {
                csv.push(format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    i + 1,
                    point.datetime_string().as_deref().unwrap_or("Unspecified"),
                    point
                        .timestamp
                        .map(|t| t.as_seconds_f64().to_string())
                        .as_deref()
                        .unwrap_or("Unspecified"),
                    point.latitude,
                    point.longitude,
                    point.altitude,
                    point.speed2d,
                    point.speed3d,
                ))
            }
        }

        if let Some(point) = point_cluster.first() {
            println!("-------------------");
            println!("First logged point:\n{point}");
        }

        if save_csv {
            // Re-use and filename from e.g. GH010006.MP4 to GH010006_GPS,csv
            // !!! TODO change affix_file_name to return Option<PathBuf> to avoid overwriting
            let csv_path = affix_file_name(&path, None, Some("_GPS"), Some("csv"), None);
            let mut csv_file = File::create(&csv_path)?;
            csv_file.write_all(csv.join("\n").as_bytes())?;
            println!("Wrote {}", csv_path.display());
        }

        println!("---");
        println!("Points: {}", gps.len());
        if min_fix.is_none() && max_dop.is_none() {
            println!("Showing all points, including those with no satellite lock.")
        } else {
            let lock = match min_fix {
                Some(0) | None => "No lock",
                Some(2) => "2D lock",
                Some(3) => "3D lock",
                Some(_) => "Invalid value, must be one of 0, 2, 3.",
            };
            let dop = match max_dop {
                Some(x) => {
                    if x.is_sign_negative() {
                        "Invalid value: {x}. DOP must be positive."
                    } else {
                        &format!("> {x}")
                    }
                },
                None => "No threshold set",
            };
            println!(
                "{} points pruned due to bad satellite lock (< {} = {}) and/or high dilution of precision ({})",
                pruned_len, min_fix.unwrap_or(0), lock, dop
            )
        }
        println!("---");
    } else if verbose {
        gpmf.print();
    }

    if let Some(sensor) = sensor_type {
        let mut csv: Vec<String> =
            vec!["INDEX\tTIME\tSENSOR\tPHYSICAL_QUANTITY\tUNIT\tX\tY\tZ".to_owned()];
        let stype = match sensor.as_str() {
            "acc" | "accelerometer" => SensorType::Accelerometer,
            "grv" | "gravity" => SensorType::GravityVector,
            "gyr" | "gyroscope" => SensorType::Gyroscope,
            s => {
                let msg = format!("(!) Unknown GoPro sensor: '{s}'. Valid choices are: gyroscope/gyr, accelerometer/acc, gravity/grv");
                return Err(std::io::Error::new(ErrorKind::Other, msg));
            }
        };

        // let sensor_data = SensorData::from_gpmf(&gpmf, &stype);
        let sensor_data = gpmf.sensor(&stype);

        let mut counter = 0;
        for (i1, data) in sensor_data.iter().enumerate() {
            println!(
                "[{:4}] {} [{}, {}] {}",
                i1 + 1,
                data.sensor,
                data.quantifier,
                data.units.as_deref().unwrap_or("Unspecified"),
                data.device.to_str()
            );
            for (i2, field) in data.fields.iter().enumerate() {
                println!("  {:4}. {}", i2 + 1, field.to_string());
                if save_csv {
                    counter += 1;
                    csv.push(format!(
                        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                        counter,
                        data.timestamp.map(|t| t.as_seconds_f64()).unwrap_or(0.),
                        data.sensor,
                        data.quantifier,
                        data.units.as_deref().unwrap_or("Unspecified"),
                        field.x,
                        field.y,
                        field.z,
                    ))
                }
            }
        }

        if save_csv {
            // Re-use and filename from e.g. GH010006.MP4 to GH010006_GPS,csv
            let csv_path = affix_file_name(&path, None, Some(&format!("_{}", sensor)), Some("csv"), None);
            let mut csv_file = File::create(&csv_path)?;
            csv_file.write_all(csv.join("\n").as_bytes())?;
            println!("Wrote {}", csv_path.display());
        }

        if sensor_data.is_empty() {
            println!("Sensor type {stype:?} not present")
        }
    }

    if let Some(dt) = data_type {
        let dtype = DataType::from_str(dt);
        for (i, stream) in gpmf.filter_iter(&dtype).enumerate() {
            stream.print(Some(i + 1), None)
        }
    }

    if save_kml || save_json {
        let points = gps.iter().map(EafPoint::from).collect::<Vec<_>>();

        let downsampled_points = match full_gps {
            true => points.to_owned(),
            false => downsample(10, &points, None),
        };

        let cluster = EafPointCluster::new(&downsampled_points, None);

        // Generate KML and save to disk
        if save_kml {
            let kml_path = affix_file_name(&path, None, Some("_points"), Some("kml"), None);
            match cluster.write_kml(indexed_kml, &kml_path) {
                Ok(true) => println!("Wrote {}", kml_path.display()),
                Ok(false) => println!("Aborted writing KML-file"),
                Err(err) => {
                    let msg = format!("(!) Failed to write '{}': {err}", kml_path.display());
                    return Err(std::io::Error::new(ErrorKind::Other, msg));
                }
            }
        }

        // Generate GeiJSON and save to disk
        if save_json {
            let geojson_path = affix_file_name(&path, None, Some("_points"), Some("json"), None);
            match cluster.write_json(indexed_kml, &geojson_path) {
                Ok(true) => println!("Wrote {}", geojson_path.display()),
                Ok(false) => println!("Aborted writing GeoJSON-file"),
                Err(err) => {
                    let msg = format!("(!) Failed to write '{}': {err}", geojson_path.display());
                    return Err(std::io::Error::new(ErrorKind::Other, msg));
                }
            }
        }
    }

    println!("SUMMARY");
    println!("  Unique data stream types ({size} DEVC streams in total):");
    for name in &gpmf.types() {
        println!("    {name}");
    }
    match (gps.t0_as_string(min_fix), gps.t_last_as_string()) {
        (Some(t1), Some(t2)) => println!("  Start time:       {t1}\n  End time:         {t2}"),
        _ => (),
    }
    let device = gpmf.device_name();
    println!(
        "  Device name:      {}{}",
        device.join(", "),
        if device.contains(&"Camera".to_owned()) {
            " (Hero5 Black)"
        } else {
            ""
        }
    );

    println!("Done");

    Ok(())
}
