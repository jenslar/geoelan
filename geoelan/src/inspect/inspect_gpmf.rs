//! Inspect GoPro GPMF data. Supports "raw" GPMP-files, e.g. by having extracted the `GoPro MET` track from a GoPro MP4-file.

use std::path::PathBuf;

use gpmf_rs::{Gpmf, ContentType, GoProSession, FourCC};
use mp4iter::Mp4;

use crate::{
    geo::{
        point::Point,
        kml_gen::{kml_point, kml_from_placemarks, kml_to_string},
        json_gen::{geojson_point, geojson_from_features}
    },
    files::{affix_file_name, writefile, has_extension}
};

pub fn inspect_gpmf(args: &clap::ArgMatches) -> std::io::Result<()> {

    let path: &PathBuf = args.get_one("gpmf").unwrap();  // clap: required arg
    // let gpmf_path: &PathBuf = args.get_one("gpmf").unwrap();  // clap: required arg
    // let video_path: &PathBuf = args.get_one("video").unwrap();  // clap: required arg
    // let jpeg_path = args.get_one::<PathBuf>("jpeg");  // clap: required arg

    // match Gpmf::from_jpg(path) {
    //     Ok(_) => std::process::exit(0),
    //     Err(err) => println!("Not a (GoPro) JPEG: {err}")
    // }

    let verbose = *args.get_one::<bool>("verbose").unwrap();   // clap: conflicts with debug, gps
    let debug = *args.get_one::<bool>("debug").unwrap();       // clap: conflicts with verbose, gps
    let print_gps = *args.get_one::<bool>("gps").unwrap();     // clap: conflicts with debug, verbose
    let print_meta = *args.get_one::<bool>("meta").unwrap();     // clap: conflicts with debug, verbose
    let print_atoms = *args.get_one::<bool>("atoms").unwrap();     // clap: conflicts with debug, verbose
    let (save_kml, indexed_kml) = (
        *args.get_one::<bool>("kml").unwrap() ||
        *args.get_one::<bool>("indexed-kml").unwrap(), *args.get_one::<bool>("indexed-kml").unwrap()
    );
    let save_json = *args.get_one::<bool>("json").unwrap();
    let session = *args.get_one::<bool>("session").unwrap();     // clap: conflicts with debug, verbose
    // let content_type = args.value_of("data-type");     // clap: conflicts with debug, verbose
    let content_type: Option<&String> = args.get_one("data-type");     // clap: conflicts with debug, verbose

    let timer_gpmf = std::time::Instant::now();

    if has_extension(&path, "jpg") {
    // if let Some(path) = jpeg_path {
        let gpmf = match Gpmf::from_jpg(&path) {
            Ok(g) => g,
            Err(err) => {
                eprintln!("(!) Failed to parse GPMF data in {}: {err}", path.display());
                std::process::exit(1)
            }
        };

        if verbose {
            gpmf.print();
        }

        println!("SUMMARY");
        println!("  Found {} DEVC streams (no descriptions in GoPro JPEG)", gpmf.len());
        print!("  Device:           ");
        if let Some(stream) = gpmf.find(&FourCC::MINF) {
            println!("{:?}", stream.values())
        } else {
            println!("{}", gpmf.device_name().join(", "));
        }

        println!("Done");
        std::process::exit(0)
    }

    println!("Locating GoPro-files and parsing GPMF-data...");

    // TODO 220813 REGRESSION CHECK: DONE. GoProSession::from_path 2-3x slower with new code if parsing immediately. Code change to only parse when files in the same session have been matched. Only Stream::new/compile remains as performance issue now (20-30ms slower with new code on M1)

    if print_atoms {
        let mp4 = match Mp4::new(&path) {
            Ok(f) => f,
            Err(err) => {
                eprintln!("(!) Failed to parse MP4-file {}: {err}", path.display());
                std::process::exit(1)
            }
        };
        
        // print atom fourcc, size, offsets
        // container_size contains 'atom size - 8' since 8 byte header is already read
        // each value will decrease until it's 0 which flags that it shold be removed
        // last value is last added and will be removed first as it indicates
        // the container atom is child to another container atom
        let mut sizes: Vec<u64> = Vec::new();
        for atom in mp4.into_iter() {
            let mut pop = false;
            let indent = sizes.len();
            let is_container = atom.is_container();
            for size in sizes.iter_mut() {
                if is_container {
                    *size -= 8;
                } else {
                    *size -= atom.size;
                }
                if size == &mut 0 {
                    pop = true;
                }
            }
            println!("{}{} @{} size: {}",
                "    ".repeat(indent as usize),
                atom.name.to_str(),
                atom.offset,
                atom.size,
            );
            if is_container {
                sizes.push(atom.size - 8);
            }
            if pop {
                loop {
                    match sizes.last() {
                        Some(&0) => {_ = sizes.pop()},
                        _ => break
                    }
                }
            }
        }

        std::process::exit(0)
    }
    
    // Compile GoPro files, parse GPMF-data
    let mut gopro_session_result = GoProSession::from_path(&path, !session, true, debug);
    if session {
        // If 'session' flag is passed the file/s must parse as MP4s
        match &gopro_session_result {
            Ok(gopro_session) => {
                println!("Located the following session files:");
                for (i, gopro_file) in gopro_session.iter().enumerate() {
                    println!("{}. {} ({:4} DEVC-streams)", i+1, gopro_file.mp4_path.display(), gopro_file.gpmf.len());
                }
            },
            Err(err) => {
                println!("(!) Failed to parse {}: {err}", path.display());
                std::process::exit(1)
            }
        }
    }

    let gpmf = match &mut gopro_session_result {
        Ok(gopro_session) => {
            println!("Merging GPMF-data for {} files...", gopro_session.len());
            gopro_session.gpmf()
        },
        Err(err) => {
            println!("(!) Failed to parse as GoPro MP4: {err}");
            println!("--> Retrying parse as raw GPMF-track...");
            match Gpmf::new(&path) {
                Ok(g) => g,
                Err(err) => {
                    println!("(!) Failed to parse GPMF {}: {err}", path.display());
                    std::process::exit(1)
                }
            }
        }
    };

    println!("Done ({} ms{})", timer_gpmf.elapsed().as_millis(), if debug {", debug parse"} else {""});

    let size = gpmf.len();

    let gps = gpmf.gps();

    if print_gps {
        let points: Vec<_> = gpmf.gps().iter().map(|p| Point::from(p)).collect();
        for (i,point) in points.iter().enumerate() {
            println!("[{:4}]\n{}", i+1, point);
        }

        if let Some(point) = points.first() {
            println!("-------------------");
            println!("First logged point:\n{point}");
        }
    } else if verbose {
        gpmf.print();
    }

    if let Some(styp) = content_type {
        let ctype = ContentType::from_str(styp);
        for (i, stream) in gpmf.filter_iter(&ctype).enumerate() {
            stream.print(Some(i+1), None)
        }
    }

    if let Ok(gopro_session) = &gopro_session_result {
        if print_meta {
            println!("Meta (MP4 udta atom):");
            for meta in gopro_session.meta().iter() {
                for udta_field in meta.udta.iter() {
                    println!("  {} SIZE: {}", udta_field.name.to_str(), udta_field.size);
                    println!("     RAW: {:?}", udta_field.data.get_ref());
                }
                for stream in meta.gpmf.iter() {
                    stream.print(None, None)
                }
            }
        }
    }

    if save_kml || save_json {
        let points: Vec<crate::geo::point::Point> = gpmf.gps().iter()
            .map(crate::geo::point::Point::from)
            .collect();
        
        if save_kml {
            let kml_points: Vec<kml::types::Placemark> = points.iter().enumerate()
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
            let json_points: Vec<geojson::Feature> = points.iter()
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
    }

    println!("SUMMARY");
    println!("  Unique data stream types ({size} DEVC streams in total):");
    for name in &gpmf.names() {
        println!("    {name}");
    }
    match (gps.t0_as_string(), gps.t_last_as_string()) {
        (Some(t1), Some(t2)) => println!("  Start time:       {t1}\n  End time:         {t2}"),
        _ => (),
    }
    println!("  Device:           {}",
        gpmf.device_name().join(", "));
        // gpmf.device_name().unwrap_or("Unable to determine device".to_owned()));
    
    // TODO 220809 fix udta/muid
    // println!("  Clip or session IDs (MUID/'media unique identifier'):");
    // if let Ok(gopro_session) = gopro_session_result {
    //     for (i, gopro_file) in gopro_session.iter().enumerate() {
    //         if let (Some(muid_gpmf), Some(muid_udta)) = (gopro_file.muid_gpmf(), gopro_file.muid_udta()) {
    //             println!("  {:2}. {}\n      GPMF: {}\n      udta: {}", i+1, gopro_file.mp4_path.file_name().unwrap().to_str().unwrap(), muid_gpmf, muid_udta)
    //         }
    //         if let Some(muid_bytes) = gopro_file.muid_bytes() {
    //             println!("     Bytes: {}", muid_bytes)
    //         }
    //     }
    // }
    
    println!("Done");

    Ok(())
}
