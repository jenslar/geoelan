use crate::files::writefile;
use crate::virb::{advise_check, select_session, session_timespan};
use std::time::Instant;
use std::{
    path::{Path, PathBuf},
    process::exit,
};

// main eaf2geo sub-command
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let timer = Instant::now();

    let fit_path = PathBuf::from(args.value_of("fit").unwrap()).canonicalize()?; // required arg
    let fit_file = fit::structs::FitFile::new(&fit_path);
    let eaf_path = PathBuf::from(args.value_of("eaf").unwrap()).canonicalize()?; // required arg

    // For returning partial FIT-data reads in some cases
    let force = args.is_present("force");

    // GET UUID, PRIORITY: VIDEO -> UUID ARG -> EAF HEADER -> SELECT FROM FIT
    let uuid: String;
    let uuid_source: &str;
    if let Some(video) = args.value_of("video") {
        uuid_source = "MP4";
        uuid = fit::get_video_uuid(&Path::new(&video))?.expect("(!) No UUID in video");
    } else if let Some(u) = args.value_of("uuid") {
        uuid_source = "ARG";
        uuid = u.to_owned()
    } else {
        let mut eaf_uuid: Option<String> = None;
        for p in eaf::parse::header(&eaf_path).properties.iter() {
            if p.name == "fit_uuid" {
                let uuids = p
                    .value
                    .split(';')
                    .into_iter()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>();
                eaf_uuid = Some(uuids[0].to_owned()); // require first uuid in session
            }
            // While we're on it, check for FIT-file in the header as well.
            // if p.name == "fit_file" {
            //     fit_file = fit::structs::FitFile::new(&PathBuf::from(p.content));
            // }
        }
        if let Some(u) = eaf_uuid {
            uuid_source = "EAF";
            uuid = u
        } else {
            uuid_source = "FIT";
            uuid = select_session(&fit_file)?;
        }
    };
    println!("[{}] UUID: {}", uuid_source, uuid);

    let mut downsample_factor: usize = args
        .value_of("downsample-factor")
        .unwrap() // defaults to 1 via clap
        .parse()
        .expect("Unable to parse '--downsample'");
    let geoshape = args.value_of("geoshape").unwrap(); // defaults to points-all via clap
    let cdata = args.is_present("cdata");
    let offset_hours: i64 = args
        .value_of("time-offset")
        .unwrap() // defaults to 0 via clap
        .parse()
        .expect("Unable to parse '--time-offset'");

    let tiers = eaf::parse::tiers(&eaf_path); // vec of TIER_ID:s in eaf
    let tier = eaf::parse::select_tier(&tiers, "feature"); // select tier with content to map to coords
    let annotations = eaf::parse::annotations(&eaf_path, &Some(tier.clone())); // clone() to use tier later
    println!("Extracted {} annotations", annotations.len());

    // EXTRACT TIME, CAMERA EVENTS, GPS DATA FROM FIT
    let t0 = match fit_file.t0(offset_hours, force) {
        Ok(data) => data,
        Err(err) => {
            match err {
                fit::errors::FitError::Fatal(e) => println!("Unable to determine start time: {}", e),
                fit::errors::FitError::Partial(e,v) => println!("Extracted timestamp_correlation with error ({} data messages extracted): {}", v.len(), e),
            }
            println!("Try '{}'", advise_check(&fit_file.path, 162, &None, true));
            println!("Alternatively try using '--force'");
            exit(1)
        }
    };
    let cam = {
        match fit_file.cam(&None, force) {
            Ok(data) => {
                if data.is_empty() {
                    println!("No logged recording session in FIT-file");
                    println!("Try '{}'", advise_check(&fit_file.path, 161, &None, true));
                    exit(1)
                } else {
                    data
                }    
            },
            Err(err) => {
                match err {
                    fit::errors::FitError::Fatal(e) => println!("Unable to determine recording session: {}", e),
                    fit::errors::FitError::Partial(e,v) => println!("Extracted partial camera_event data with error ({} data messages extracted): {}", v.len(), e),
                }
                println!("Try '{}'", advise_check(&fit_file.path, 161, &None, true));
                println!("Alternatively try using '--force'");
                exit(1)
            }
        }
    };
    let gps = {
        match fit_file.gps(&Some(uuid.to_owned()), force) {
            Ok(data) => {
                if data.is_empty() {
                    println!("No logged points for UUID in FIT-file");
                    println!("Try '{}'", advise_check(&fit_file.path, 160, &None, true));
                    exit(1)
                } else {
                    data
                }
            },
            Err(err) => {
                match err {
                    fit::errors::FitError::Fatal(e) => println!("Unable to extract GPS data: {}", e),
                    fit::errors::FitError::Partial(e,v) => println!("Extracted partial GPS data with error ({} data messages extracted): {}", v.len(), e),
                }
                println!("Try '{}'", advise_check(&fit_file.path, 160, &Some(uuid), true));
                println!("Alternatively try using '--force'");
                exit(1)
            }
        }
    };
    let session_timespan = match session_timespan(&cam, &Some(uuid), false) {
        Some(t) => t,
        None => {
            // use relative timestamps if err?
            println!("Unable to determine timespan for specified recording session");
            exit(0)
        }
    };

    // NOTE 200818 extreme sample factor values may cause "line" to discard annotations
    //             do average in loop instead, then set max value to vec.len() if too large?
    if downsample_factor >= gps.len() {
        downsample_factor = gps.len() // prevent no points in the output
    }
    if geoshape == "point-single" {
        downsample_factor = 1 // assure all points used for averaging to single
    }
    let downsampled_points = crate::geo::downsample(downsample_factor, &gps);
    let downsampled_points_len = downsampled_points.len(); // prevent borrow moved value err
    let t_ms_video_start = session_timespan.start.num_milliseconds(); // gps

    let mut points: Vec<crate::structs::Point> = Vec::new(); // annotated points, output
    let mut points_buf: Vec<crate::structs::Point> = Vec::new(); // temp container for "single" and "lines"
    let mut lines: Vec<Vec<crate::structs::Point>> = Vec::new(); // annotated lines, output

    let mut prev_annotation_value: Option<String> = None; // compare annotation value in current loop to the previous one

    for (count, mut point) in downsampled_points.into_iter().enumerate() {
        let mut t_gps_ms = point.time.num_milliseconds() - t_ms_video_start as i64;
        if t_gps_ms < 0 {
            t_gps_ms = 0 // ok with check !< 0 then cast u64 below?
        };

        for annotation in annotations.iter() {
            // only map annotations with a timespan that has a corresponding set of points
            if annotation.time_slot_value1 <= t_gps_ms as u64
                && t_gps_ms as u64 <= annotation.time_slot_value2
            {
                point.text = Some(annotation.annotation_value.to_owned());
            }
        }

        match geoshape {
            // all points, default if no geoshape argument specified by user
            "point-all" => points.push(point),
            // all annotated points
            "point-multi" => {
                if point.text.is_some() {
                    points.push(point)
                }
            }
            // each annotation averaged to single point
            "point-single" => {
                if point.text.is_none() && prev_annotation_value.is_some() {
                    // push existing annotated point/s
                    let average = crate::geo::point_cluster_average(
                        &points_buf[..],
                        &prev_annotation_value.take(),
                    );
                    points.push(average);
                    points_buf = Vec::new(); // .clear() incurs borrow issue
                                             // prev_annotation_value = None;
                }

                if point.text.is_some() {
                    prev_annotation_value = point.text.clone();
                    points_buf.push(point);
                }

                if count + 1 == downsampled_points_len {
                    // last single average
                    if !points_buf.is_empty() {
                        let average = crate::geo::point_cluster_average(
                            &points_buf[..],
                            &prev_annotation_value.take(),
                        );
                        points.push(average);
                    }
                    points_buf = Vec::new(); // .clear() incurs borrow issue
                    break;
                }
            }
            // POLYLINE
            // Separating annotated content as connected, single line
            // Alternates between annotated and non-annotated sub-lines
            // line-all: similar to point-all, only as a continuous line
            "line-all" => {
                // new polyline starts if "no annotation" -> "annotation" between loops and vice versa
                // or of annotation value has changed since last loop
                // idx != 0 to prevent pushing empty line if first point is annotated
                if point.text != prev_annotation_value && count != 0 {
                    // add first point of next line to as last point for current to get unbroken path
                    // in output kml, but with annotation value of last loop
                    points_buf.push(crate::structs::Point {
                        text: prev_annotation_value.take(), // update with annotation value
                        ..point
                    });
                    prev_annotation_value = point.text.clone();
                    lines.push(points_buf);
                    points_buf = Vec::new(); // .clear() incurs borrow issue
                }

                points_buf.push(point); // important: push on every loop for line

                if count + 1 == downsampled_points_len {
                    // push last line
                    if !points_buf.is_empty() {
                        lines.push(points_buf)
                    }
                    points_buf = Vec::new(); // removal or .clear() incurs borrow issue
                    break;
                }
            }
            // line-multi: multiple polylines corresponding only to annotated content (c.f. multi)
            "line-multi" => {
                // broken up poly line, only inserting those with annotaion text (c.f. "multi" for points)
                // idx != 0 to prevent pushing empty line if first point is annotated
                if point.text.is_none() && count != 0 {
                    if !points_buf.is_empty() {
                        lines.push(points_buf);
                        points_buf = Vec::new();
                    }
                }

                if point.text.is_some() {
                    points_buf.push(point)
                }; // push only annotated points

                if count + 1 == downsampled_points_len {
                    // push last line
                    if !points_buf.is_empty() {
                        lines.push(points_buf);
                    }
                    points_buf = Vec::new(); // removal or .clear() incurs borrow issue
                    break;
                }
            }
            _ => (),
        }
    }

    let points_len = points.len(); // borrow... 0 if lines, but not used
    let lines_len = lines.len(); // borrow... 0 if points, multi, single, but not used

    // KML
    let kml_doc = match geoshape {
        "point-all" | "point-multi" | "point-single" => {
            println!("Generated {} points", points_len);
            crate::kml::write::build(
                &crate::structs::GeoType::POINT(points),
                &t0,
                &session_timespan.uuid,
                "Garmin VIRB",
                cdata,
            )
        }
        "line-all" | "line-multi" => {
            println!("Generated {} lines", lines_len);
            crate::kml::write::build(
                &crate::structs::GeoType::LINE(lines),
                &t0,
                &session_timespan.uuid,
                "Garmin VIRB",
                cdata,
            )
        }
        _ => {
            // redundant arm, since clap takes care of this, unless typos in code
            println!("Unable to determine 'geoshape' value: {}", geoshape);
            exit(0)
        }
    };

    let kml_parent = eaf_path
        .parent()
        .unwrap_or_else(|| panic!("Unable to determine parent for {}", eaf_path.display()));
    let mut kml_filestem = eaf_path
        .file_stem()
        .unwrap_or_else(|| panic!("Unable to determine file stem for {}", eaf_path.display()))
        .to_owned();
    kml_filestem.push(&format!("_{}", geoshape));
    let mut kml_path = kml_parent.join(kml_filestem);
    kml_path.set_extension("kml");
    writefile(&kml_doc.as_bytes(), &kml_path)?;

    println!("\n-------");
    println!("SUMMARY");
    println!("-------");
    println!("EAF:");
    println!("  Feature tier: {}", tier);
    println!("  Annotations:  {}", annotations.len());

    println!("Session time span (time offset = {}hrs):", offset_hours);
    println!(
        "  Start:        {}",
        (t0 + session_timespan.start).format("%Y-%m-%dT%H:%M:%S%.3f")
    );
    println!(
        "  End:          {}",
        (t0 + session_timespan.end).format("%Y-%m-%dT%H:%M:%S%.3f")
    );
    println!(
        "  Duration:     {}s {}ms",
        (session_timespan.end - session_timespan.start).num_seconds(),
        (session_timespan.end - session_timespan.start).num_milliseconds()
            - (session_timespan.end - session_timespan.start).num_seconds() * 1000
    );

    println!(
        "Points in session (downsample factor = {}):",
        downsample_factor
    );
    println!("  All:          {}", gps.len());
    println!("  Downsampled:  {}", downsampled_points_len);

    println!(
        "{} in output (geoshape = {}):",
        if geoshape == "line" {
            "Lines"
        } else {
            "Points"
        },
        geoshape
    );
    match geoshape {
        "point-all" | "point-multi" | "point-single" => println!("  Points:       {}", points_len),
        "line-all" | "line-multi" => println!("  Lines:        {}", lines_len),
        _ => (),
    }

    println!(
        "Done ({:.3}s)",
        (timer.elapsed().as_millis() as f64) / 1000.0
    );

    Ok(())
}
