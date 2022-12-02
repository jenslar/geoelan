use std::path::PathBuf;

use gpmf_rs::GoProSession;

use crate::elan::generate_eaf;
use crate::geo;
use crate::media::Media;

pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let video = args.get_one::<PathBuf>("video").unwrap().canonicalize()?; // clap required
    let ffmpeg = args.get_one::<PathBuf>("ffmpeg").unwrap().to_owned(); // clap default: "ffmpeg"/"ffmpeg.exe"
    let concatenate = *args.get_one::<bool>("concatenate").unwrap();
    // clap: default 0
    let time_offset = args.get_one::<isize>("time-offset").unwrap().to_owned();
    let output_dir = {
        let p = args.get_one::<PathBuf>("output-directory").unwrap();
        if !p.exists() {
            std::fs::create_dir_all(&p)? // canonicalise() returns err if p does not exist
        };
        p.canonicalize()?
    };
    let geotier = *args.get_one::<bool>("geotier").unwrap();
    let dryrun = *args.get_one::<bool>("dryrun").unwrap();

    // if args.contains_id("downsample-factor") {
    //     println!("'downsample-factor' is set, but will be ignored for GoPro")
    // }

    // Compile GoPro files, parse GPMF-data
    let mut gopro_session = match GoProSession::from_path(&video, !concatenate, true, false) {
        Ok(data) => data,
        Err(err) => {
            println!("(!) Failed to parse {}: {err}", video.display());
            std::process::exit(1)
        }
    };

    println!("Located the following high-res files in session:");
    for (i, gopro_file) in gopro_session.iter().enumerate() {
        println!("{}. {}", i+1, gopro_file.mp4_path.display());
    }
    println!("Located the following low-res files in session:");
    for (i, gopro_file) in gopro_session.iter().enumerate() {
        println!("{}. {}", i+1, gopro_file.lrv_path.display());
    }

    print!("Merging GPMF-data for {} files...", gopro_session.len());
    let gpmf = gopro_session.gpmf();
    println!(" Done");
    
    // Get the GPS-data and convert to geo::point::Point:s.
    let mut points: Option<Vec<geo::point::Point>> = None;

    if geotier {
        print!("Extracting GPS data with time offset {} hours... ", time_offset);
        let gps = gpmf.gps();
        points = Some(
            gps.iter()
            .map(|p| {
                let mut gp = geo::point::Point::from(p);
                // Add time offset 
                if let Some(dt) = gp.datetime {
                    gp.datetime = Some(dt + time::Duration::hours(time_offset as i64))
                }
                gp
            })
            .collect::<Vec<_>>()
        );
        println!("OK");
    }

    if dryrun {
        println!("(!) 'dryrun' set, no files changed.");
        std::process::exit(0)
    }

    // let (mut video_eaf, mut audio_eaf) = (None, None);
    let (video_eaf, audio_eaf): (Option<PathBuf>, Option<PathBuf>);
    if concatenate {
        let outdir = match video.parent() {
            Some(p) => p.join(output_dir),
            None => {
                println!("(!) Failed to determine parent dir for '{}'", video.display());
                std::process::exit(1)
            }
        };
        // TODO add lrv/lo-res video concat (optional)
        println!("Concatenating media to {}...", outdir.display());
        (video_eaf, audio_eaf) = match Media::concatenate(
            &gopro_session.mp4_paths(),
            &outdir,
            true,
            None,
            None,
            &format!("{}", ffmpeg.display()),
        ) {
            Ok((v,a)) => {
                println!("Wrote the following media files:");
                if let Some(vid) = &v {
                    println!("  {}", vid.display());
                }
                if let Some(aud) = &a {
                    println!("  {}", aud.display());
                }
                (v, a)
            },
            Err(err) => {
                println!("(!) Failed to concatenate media files: {err}");
                std::process::exit(1)
            }
        };
    } else {
        // Extract wav to same path as video, with wav extension.
        audio_eaf = match Media::wav(
            &video,
            &ffmpeg) {
            Ok(w) => Some(w),
            Err(e) => {
                println!("(!) Failed to extract wav from '{}': {e}", video.display());
                std::process::exit(1)
            }
        };
        video_eaf = Some(video);
    }

    // Generate EAF
    if let (Some(vid), Some(aud)) = (video_eaf, audio_eaf) {
        let eaf = match generate_eaf(
            &vid,
            &aud,
            points.as_deref(),
            // GoPro GPS points has a relative timestamp
            // from start derived from DEVC timestamp,
            // but it's fairly untested as to
            // whether this is reliable.
            None,
        ) {
            Ok(e) => e,
            Err(e) => {
                println!("(!) Failed to generate EAF: {e}");
                std::process::exit(1)
            }
        };
    
        let eaf_path = vid.with_extension("eaf");
        match eaf.write(&eaf_path) {
            Ok(true) => println!("Wrote {}", eaf_path.display()),
            Ok(false) => println!("User aborted writing ELAN-file"),
            Err(err) => {
                println!("(!) Failed to write '{}': {err}", eaf_path.display());
                std::process::exit(1)
            },
        }
    } else {
        println!("(!) Failed to set video and audio paths. No ELAN-file generated.")
    }

    Ok(())
}