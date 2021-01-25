use fit::structs::FitFile;

use crate::ffmpeg::concatenate;
use crate::files::{
    checksum, writefile
};
use crate::virb::{
    advise_check, compile_virbfiles, select_session, session_timespan
};
use crate::structs::{FitMetaData, VirbFileType};
use std::fs::{copy, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Instant;

pub fn generate_eaf(
    points: &[crate::structs::Point],
    meta: &FitMetaData,
    video_eaf: &PathBuf,
    audio_eaf: &PathBuf,
    geotier: bool,
) -> String {
    let mut eaf_points: Vec<String> = Vec::new();
    let mut eaf_timeslots: Vec<String> = Vec::new();

    for (point_count, p) in points.iter().enumerate() {
        let mut time_ms = (p.time - meta.start).num_milliseconds(); // only relative time needed
        if time_ms < 0 {
            time_ms = 0 // start eaf geo tier on t=0
        };

        let timestamp = (meta.t0 + p.time)
            .format("%Y-%m-%dT%H:%M:%S%.3f")
            .to_string();

        // EAF CONTENT
        if geotier {
            let annotation = format!(
                "LAT:{:.6};LON:{:.6};ALT:{:.1};HDG:{:.1};VEL:{:.3};SPE:{:.3};T:{}",
                p.latitude, p.longitude, p.altitude, p.heading, p.velocity, p.speed, timestamp
            );
            eaf_points.push(eaf::write::annotation(
                point_count + 1,
                point_count + 1,
                point_count + 2,
                annotation,
            ));
            eaf_timeslots.push(eaf::write::timeslot(point_count + 1, time_ms as usize));
        }
    }

    // FINAL EAF TIMESLOT
    if !eaf_timeslots.is_empty() {
        let video_duration_ms = (meta.end - meta.start).num_milliseconds();
        eaf_timeslots.push(eaf::write::timeslot(
            eaf_timeslots.len() + 1,
            video_duration_ms as usize,
        ));
    }

    // BUILD, RETURN EAF DOC
    eaf::write::build(
        &video_eaf,
        &audio_eaf,
        &meta.file,
        &meta.uuid,
        &(meta.t0 + meta.start)
            .format("%Y-%m-%dT%H:%M:%S%.3f")
            .to_string(),
        &(meta.t0 + meta.end)
            .format("%Y-%m-%dT%H:%M:%S%.3f")
            .to_string(),
        &eaf_points,
        &eaf_timeslots,
        geotier,
    )
}

///////////////////
// MAIN CAM2EAF.RS
///////////////////
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let timer = Instant::now();

    // INPUT
    let mut fit_path = args.value_of("fit").map(|p| PathBuf::from(p)); // set later if not specified
    let indir = PathBuf::from(args.value_of("input-directory").unwrap()).canonicalize()?;
    let outdir = {
        let p = PathBuf::from(&args.value_of("output-directory").unwrap());
        if !p.exists() {
            create_dir_all(&p)? // canonicalise() returns err if p does not exist
        };
        p.canonicalize()?
    };
    let video = args.value_of("video");
    // ownership issues if uuid is not Option<String>... otherwise not necessary
    let uuid: Option<String> = if let Some(v) = video {
        fit::get_video_uuid(&Path::new(v))?
    } else if let Some(u) = args.value_of("uuid") {
        Some(u.to_owned())
    } else if let Some(f) = fit_path.as_ref() {
        Some(select_session(&fit::structs::FitFile {
            path: f.canonicalize()?,
        })?)
    } else {
        None
    };

    if uuid.is_none() {
        println!("Unable to assign UUID.");
        exit(0)
    }

    let mut downsample_factor: usize = args
        .value_of("downsample-factor")
        .unwrap()
        .parse()
        .unwrap_or_else(|e| panic!("Unable to parse '--downsample': {}", e));
    let quiet = args.is_present("quiet");
    let geotier = args.is_present("geotier");
    let ffmpeg = args.value_of("ffmpeg").unwrap(); // defaults to "ffmpeg"/"ffmpeg.exe"
    let mpeg2 = args.is_present("mpeg2");
    let copy_mp4 = args.is_present("copy");
    let low_res_only = args.is_present("low-res-only");
    let offset_hours: i64 = args
        .value_of("time-offset")
        .unwrap()
        .parse()
        .unwrap_or_else(|e| panic!("Unable to parse '--time-offset': {}", e));
    
    // For returning partial FIT-data reads in some cases
    let force = args.is_present("force");

    // OUTPUT
    let virbfiles = compile_virbfiles(&indir, !quiet, false)?; // does not filter on uuid

    // files in session
    let mut mp4_session: Vec<PathBuf> = Vec::new();
    let mut glv_session: Vec<PathBuf> = Vec::new();

    if let Some(session) = virbfiles.session.get(&uuid.clone().unwrap()) {
        for u in session.iter() {
            if let Some(files) = virbfiles.uuid.get(&*u) {
                for file in files.iter() {
                    match file.filetype {
                        VirbFileType::MP4 => mp4_session.push(file.path.to_owned()),
                        VirbFileType::GLV => glv_session.push(file.path.to_owned()),
                        VirbFileType::FIT => {
                            if fit_path.is_none() {
                                fit_path = Some(file.path.to_owned())
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("No matches found");
        exit(0)
    }

    if fit_path.is_none() {
        println!(
            "Unable to locate a corresponding FIT-file in {}.\nTry using '-f <FITFILE>'",
            indir.display()
        );
        exit(1)
    }

    let fit_file = FitFile::new(&fit_path.unwrap().canonicalize()?);

    // EXTRACT TIME, CAMERA, AND GPS DATA FROM FIT
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
        match fit_file.gps(&uuid, force) {
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
                println!("Try '{}'", advise_check(&fit_file.path, 160, &uuid, true));
                println!("Alternatively try using '--force'");
                exit(1)
            }
        }
    };

    let session_timespan = match session_timespan(&cam, &uuid, false) {
        Some(t) => t,
        None => {
            // use relative timestamps if err?
            println!("Unable to determine timespan for specified recording session");
            exit(1)
        }
    };

    if downsample_factor >= gps.len() {
        downsample_factor = gps.len() // prevents no points in the output
    }
    let points = crate::geo::downsample(downsample_factor, &gps);
    let points_len = points.len();

    // VIRB001 if first file is VIRB001.MP4, used for output dir etc
    let basename = if let Some(path) = mp4_session.get(0) {
        path.file_stem().unwrap()
    } else {
        if let Some(path) = glv_session.get(0) {
            path.file_stem().unwrap()
        } else {
            println!("Unable to determine input clips.");
            exit(0)
        }
    };
    let outdir_session = outdir.join(&Path::new(&basename));
    if !outdir_session.exists() {
        create_dir_all(&outdir_session)?;
    }

    println!("Done\n-----------");

    // Create metadata to embed in output MP4: UUIDs, FIT filename, FIT checksum etc
    let metadata = {
        let (fit_checksum, fit_size) = checksum(&fit_file.path)?;
        let fit_filename = fit_file
            .path
            .file_name()
            .expect("Unable to extract filename from FIT path")
            .to_str()
            .expect("FIT path is not a valid UTF-8 string")
            .to_owned();
        FitMetaData {
            uuid: session_timespan.uuid,
            sha256: fit_checksum,
            file: fit_filename,
            size: fit_size,
            t0,
            start: session_timespan.start,
            end: session_timespan.end,
        }
    };

    // CONCATENATE SESSION CLIPS + EXTRACT WAV TO OUTDIR, SET EAF MEDIA PATHS
    println!("[MP4] Found the following files:");
    for (i, f) in mp4_session.iter().enumerate() {
        println!("      {:2}. {}", i + 1, f.display());
    }

    let (video_eaf_mp4, audio_eaf_mp4) = if mp4_session.is_empty() {
        println!("      Unable to locate high-resolution clips");
        (None, None)
    } else if low_res_only {
        // copy original clips
        println!("      'low-res-only' set: Skipping concatenation for high-resolution MP4.");
        if copy_mp4 {
            println!("      Copying hi-resolution clips...");
            let out_dir_org = outdir_session.join("original");
            if !out_dir_org.exists() {
                create_dir_all(&out_dir_org)?;
            };
            for mp4_in in mp4_session.iter() {
                let mp4_out = out_dir_org.join(&mp4_in.file_name().unwrap());
                if mp4_out.exists() {
                    println!("\n      {} already exists. Skipping.", mp4_out.display())
                } else {
                    print!(
                        "[MP4] Copying {}\n           to {}...",
                        mp4_in.display(),
                        outdir_session.display()
                    );
                    std::io::stdout().flush()?;
                    copy(mp4_in, &mp4_out)?;
                    println!(" Ok");
                }
            }
            println!("      Done");
        } else {
            println!("      Use '--copy' to copy the original files as-is.");
        }
        (None, None)
    } else {
        concatenate(
            &mp4_session,
            &outdir_session,
            false,
            true,
            ffmpeg,
            if args.is_present("no-metadata") {
                None
            } else {
                Some(&metadata)
            },
            if glv_session.is_empty() { mpeg2 } else { false },
        )?
    };

    // if hi-res mp4 no found/not used -> extract wav from glv instead
    let extract_wav_glv = match audio_eaf_mp4 {
        None => true,
        Some(_) => false,
    };

    println!("[GLV] Found the following files:");
    for (i, f) in glv_session.iter().enumerate() {
        println!("      {:2}. {}", i + 1, f.display());
    }
    let (video_eaf_glv, audio_eaf_glv) = if glv_session.is_empty() {
        println!("      Unable to locate low-resolution clips");
        (None, None)
    } else {
        concatenate(
            &glv_session,
            &outdir_session,
            true,
            extract_wav_glv,
            ffmpeg,
            if args.is_present("no-metadata") {
                None
            } else {
                Some(&metadata)
            },
            mpeg2,
        )?
    };

    // SET EAF MEDIA PATHS
    let video_eaf = match video_eaf_glv {
        Some(v) => v,
        None => video_eaf_mp4.expect("Unable to set EAF video path"),
    };
    let audio_eaf = match audio_eaf_glv {
        Some(a) => a,
        None => audio_eaf_mp4.expect("Unable to set EAF audio path"),
    };

    // GENERATE EAF
    print!("[EAF] ");
    std::io::stdout().flush()?;
    let eaf = generate_eaf(&points, &metadata, &video_eaf, &audio_eaf, geotier);
    let mut eaf_path = outdir_session.join(PathBuf::from(&basename));
    eaf_path.set_extension("eaf");
    writefile(&eaf.as_bytes(), &eaf_path)?;

    // GENERATE KML
    print!("[KML] ");
    std::io::stdout().flush()?;
    let kml = crate::kml::write::build(
        &crate::structs::GeoType::POINT(points),
        &metadata.t0,
        &metadata.uuid,
        "Garmin VIRB",
        false,
    );
    let mut kml_path = outdir_session.join(PathBuf::from(&basename));
    kml_path.set_extension("kml");
    writefile(&kml.as_bytes(), &kml_path)?;

    // COPY INPUT FIT-FILE
    let fit_outpath = outdir_session.join(&Path::new(fit_file.path.file_name().unwrap()));

    if !fit_outpath.exists() {
        print!(
            "[FIT] Copying {}\n           to {}... ",
            fit_file.path.display(),
            fit_outpath.display()
        );
        std::io::stdout().flush()?;
        copy(&fit_file.path, &fit_outpath)?;
        println!(" Done");
    }

    // SUMMARY
    println!("\n-------");
    println!("SUMMARY");
    println!("-------");
    println!("UUID:");
    for uuid in metadata.uuid.iter() {
        println!("  {}", uuid);
    }
    println!("FIT:");
    println!("  Path:        {}", fit_file.path.display());
    println!("  Size:        {} bytes", metadata.size);
    println!("  SHA-256:     {}", metadata.sha256);
    println!("Session time span (time offset = {}hrs):", offset_hours);
    println!(
        "  Start:       {}",
        (metadata.t0 + metadata.start)
            .format("%Y-%m-%dT%H:%M:%S%.3f")
            .to_string()
    );
    println!(
        "  End:         {}",
        (metadata.t0 + metadata.end)
            .format("%Y-%m-%dT%H:%M:%S%.3f")
            .to_string()
    );
    println!(
        "  Duration:    {}s {}ms",
        (metadata.end - metadata.start).num_seconds(),
        (metadata.end - metadata.start).num_milliseconds()
            - (metadata.end - metadata.start).num_seconds() * 1000
    );
    println!(
        "GPS, points in session (sample factor = {}):",
        downsample_factor
    );
    println!("  All:         {}", gps.len());
    println!("  Downsampled: {}", points_len);
    println!(
        "Done ({:.3}s)",
        (timer.elapsed().as_millis() as f64) / 1000.0
    );
    Ok(())
}
