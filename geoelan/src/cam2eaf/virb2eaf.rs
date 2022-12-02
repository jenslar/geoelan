use std::io::Write;
use std::path::{Path, PathBuf};

use fit_rs::{Fit, VirbSession};

use crate::elan::generate_eaf;
use crate::files::virb::select_session;
use crate::geo::point_cluster::PointCluster;
use crate::media::Media;

pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {

    // Options
    // The fit, video, uuid cluster is for determining clips in session
    // and the corresponding telemetry.
    // Required unless video, uuid
    let fit_path: Option<&PathBuf> = args.get_one("fit");
    let fit = fit_path
        .map(|f| match Fit::new(&f) {
            Ok(fit) => fit,
            Err(err) => {
                println!("(!) failed to parse '{}': {}", f.display(), err);
                std::process::exit(1)
            }
        });
    let video_path: Option<&PathBuf> = args.get_one("video"); // required unless fit, uuid

    let uuid = args.get_one::<String>("uuid");

    // clap: required arg
    let input_dir: &PathBuf = args.get_one("input-directory")
        .unwrap();
    // clap default: "OUTPUT"
    let output_dir: &PathBuf = args.get_one("output-directory")
        .unwrap();

    // clap default: "ffmpeg"/"ffmpeg.exe"
    let ffmpeg: &PathBuf = args.get_one("ffmpeg").unwrap();
    
    // clap default: 0
    let time_offset: isize = *args.get_one("time-offset").unwrap();

    let mut downsample_factor = args.get_one::<usize>("downsample-factor")
        .unwrap().to_owned();
    if downsample_factor == 0 {
        // clap: can not use value_parser!(usize).range(1..)?
        println!("(!) 'downsample' can not be 0.");
        std::process::exit(1);
    }
    
    // Flags
    let geotier = *args.get_one::<bool>("geotier").unwrap();
    let low_res_only = *args.get_one::<bool>("low-res-only").unwrap();

    println!("Determining recording session data...");
    let virbsession_result = match (fit, video_path, uuid) {
        (Some(f), None, None) => {
            let fit_session = select_session(&f)?; // TODO 220809 add exit
            let uuid = match fit_session.uuid.get(0) {
                Some(u) => u,
                None => {
                    println!("(!) Failed to determine UUID.");
                    std::process::exit(1)
                }
            };
            VirbSession::from_uuid(uuid, input_dir, true)
        }
        (None, Some(p), None) => VirbSession::from_mp4(p, input_dir, true),
        (None, None, Some(s)) => VirbSession::from_uuid(s, input_dir, true),
        _ => {
            println!("(!) Failed to determine recording session.");
            std::process::exit(1)
        }
    };
    let mut virbsession = match virbsession_result {
        Some(s) => s,
        None => {
            println!("(!) Failed to determine recording session. At least one of 'video', 'fit, 'uuid' must be specified.");
            std::process::exit(1)
        }
    };

    
    // Parse linked FIT and set start/end time stamps.
    if let Err(err) = virbsession.process(time_offset as i64) {
        println!("(!) Failed to process session data: {err}");
        std::process::exit(1)
    }

    // OUTPUT
    // files in session
    let mp4_session = virbsession.mp4();
    let glv_session = virbsession.glv();
    // Add 'LO' to denote low-res video if GLV files are used
    let media_suffix_hi = match mp4_session.is_empty() {
        true => None,
        false => Some("_HI"),
    };
    let media_suffix_lo = match glv_session.is_empty() {
        true => None,
        false => Some("_LO"),
    };

    let mut pointcluster: Option<PointCluster> = None;

    if geotier {
        // EXTRACT GPS, DERIVE TIME DATA FROM FIT
        if let Ok(gps) = virbsession.gps() {
            if gps.is_empty() {
                println!("(!) No logged points for UUID in FIT-file");
                std::process::exit(1)
            } else {
                let (t0, end) = match (virbsession.t0, virbsession.end) {
                    (Some(t), Some(e)) => (t, e),
                    _ => {
                        println!("(!) Failed to determine time values for session.");
                        std::process::exit(1)
                    }
                };

                // let mut cluster = PointCluster::from_virb(&gps, None, &end.naive_utc());
                let mut cluster = PointCluster::from_virb(&gps, None, &t0, &end);
                
                // prevent no points in the output
                if downsample_factor >= cluster.len() {
                    downsample_factor = cluster.len()
                }

                cluster.downsample(downsample_factor, None);
                cluster.offset_hrs(time_offset as i64);
                cluster.set_virbtime(&t0, &end);

                // println!("points: {}", cluster.len());
                // println!("gps last time {:#?}", cluster.last());


                pointcluster = Some(cluster);
            }
        } else {
            println!("(!) Failed to extract GPS data");
            std::process::exit(1)
        }
    }

    // Set up paths for files in recording session etc
    // VIRB001 if first file is VIRB001.MP4, used for output dir etc
    let basename = if let Some(path) = mp4_session.get(0) {
        path.file_stem().unwrap()
    } else if let Some(path) = glv_session.get(0) {
        path.file_stem().unwrap()
    } else {
        println!("(!) Unable to determine input clips.");
        std::process::exit(1)
    };
    let outdir_session = output_dir.join(&Path::new(&basename));
    if !outdir_session.exists() {
        std::fs::create_dir_all(&outdir_session)?;
    }

    // CONCATENATE SESSION CLIPS + EXTRACT WAV TO OUTDIR, SET EAF MEDIA PATHS
    println!("[MP4] Found the following files:");
    for (i, f) in mp4_session.iter().enumerate() {
        println!("      {:2}. {}", i + 1, f.display());
    }

    let (video_eaf_mp4, audio_eaf_mp4) = if mp4_session.is_empty() {
        println!("      Unable to locate high-resolution clips. Skipping.");
        (None, None)
    } else if low_res_only {
        println!("      'low-res-only' set: Skipping concatenation for high-resolution MP4.");
        (None, None)
    } else {
        Media::concatenate(
            &mp4_session,
            &outdir_session,
            true,
            None,
            media_suffix_hi,
            // TODO use Path for concatenate()
            &format!("{}", ffmpeg.display()),
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
        println!("      Unable to locate low-resolution clips. Skipping.");
        (None, None)
    } else {
        Media::concatenate(
            &glv_session,
            &outdir_session,
            extract_wav_glv,
            None,
            media_suffix_lo,
            // TODO use Path for concatenate()
            &format!("{}", ffmpeg.display()),
        )?
    };

    // SET EAF MEDIA PATHS
    let video_eaf = match video_eaf_glv {
        Some(v) => v,
        None => match video_eaf_mp4 {
            Some(v) => v,
            None => {
                println!("(!) Unable to set EAF video path.");
                std::process::exit(1)
            }
        }
    };
    let audio_eaf = match audio_eaf_glv {
        Some(a) => a,
        None => match audio_eaf_mp4 {
            Some(v) => v,
            None => {
                println!("(!) Unable to set EAF audio path.");
                std::process::exit(1)
            }
        }
    };

    // Generate EAF, with proved max time.
    let eaf = match generate_eaf(
        &video_eaf,
        &audio_eaf,
        pointcluster.map(|pc| pc.points).as_deref(),
        // Start time required if points are passed above
        // in order to determine the offset between video start
        // and time of first logged point
        virbsession.start.map(|n| n.whole_milliseconds() as i64),
    ) {
        Ok(e) => e,
        Err(e) => {
            println!("(!) Failed to generate EAF: {e}");
            std::process::exit(1)
        }
    };

    let eaf_path = Path::new(&video_eaf).with_extension("eaf");

    match eaf.write(&eaf_path) {
        Ok(true) => println!("Wrote {}", eaf_path.display()),
        Ok(false) => println!("User aborted writing ELAN-file"),
        Err(err) => {
            println!("(!) Failed to write '{}': {err}", eaf_path.display());
            std::process::exit(1)
        },
    }
    // COPY INPUT FIT-FILE
    // TODO use copy command instead...
    let fit_outpath = outdir_session.join(&Path::new(virbsession.fit.path.file_name().unwrap()));

    if !fit_outpath.exists() {
        print!(
            "[FIT] Copying {}\n           to {}... ",
            virbsession.fit.path.display(),
            fit_outpath.display()
        );
        std::io::stdout().flush()?;
        std::fs::copy(&virbsession.fit.path, &fit_outpath)?;
        println!("Done");
    }

    Ok(())
}