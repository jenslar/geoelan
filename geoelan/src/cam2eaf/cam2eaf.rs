//! Locate video-files (GoPro, Garmin VIRB) and FIT (Garmin VIRB), and generate an ELAN-file.

use std::path::{Path, PathBuf};

use crate::{
    elan::generate_eaf,
    files::writefile,
    geo::{EafPoint, EafPointCluster},
    media::Media,
};

// Concatenate clips, generate EAF, KML and GeoJSON.
pub fn run(
    session_hi: &[PathBuf],
    session_lo: &[PathBuf],
    points: Option<&[EafPoint]>,
    session_start_ms: Option<i64>, // VIRB ONLY
    fit_path: Option<&Path>,       // VIRB ONLY
    args: &clap::ArgMatches,
) -> std::io::Result<()> {
    let ffmpeg = args.get_one::<PathBuf>("ffmpeg").unwrap().to_owned();
    let output_dir = {
        let p = args.get_one::<PathBuf>("output-directory").unwrap();
        if !p.exists() {
            // canonicalise() returns err if p does not exist
            std::fs::create_dir_all(&p)?
        };
        p.canonicalize()?
    };
    let low_res_only = *args.get_one::<bool>("low-res-only").unwrap();
    let link_high_res = *args.get_one::<bool>("link-high-res").unwrap();
    let geotier = *args.get_one::<bool>("geotier").unwrap();
    let dryrun = *args.get_one::<bool>("dryrun").unwrap();

    // Add 'LO' to denote that low-res video is used,
    // and 'HI' for high-res video.
    let media_suffix_hi = match session_hi.is_empty() {
        true => None,
        false => Some("_HI"),
    };
    let media_suffix_lo = match session_lo.is_empty() {
        true => None,
        false => Some("_LO"),
    };

    // Set up paths for files in recording session etc.
    // Use file stem of first clip in session as dir.
    // In case high-res does not exist use low-res basename and vice versa.
    let basename_hi = session_hi.first().and_then(|p| p.file_stem());
    let basename_lo = session_lo.first().and_then(|p| p.file_stem());
    let maybe_basename = match low_res_only {
        true => basename_lo.or_else(|| basename_hi),
        false => basename_hi.or_else(|| basename_lo),
    };

    let Some(basename) = maybe_basename else {
        let msg = "(!) Failed to determine basename for session.";
        return Err(std::io::Error::new(std::io::ErrorKind::Other, msg));
    };

    let outdir_session = output_dir.join(&Path::new(&basename));
    if !outdir_session.exists() {
        std::fs::create_dir_all(&outdir_session)?;
    }

    println!("High-resolution clips in session:");
    for (i, clip) in session_hi.iter().enumerate() {
        println!("      {:2}. {}", i + 1, clip.display());
    }

    let (video_eaf_hi, audio_eaf_hi) = if dryrun {
        println!("      Skipping: '--dryrun' set.");
        (None, None)
    } else if session_hi.is_empty() {
        println!("      Skipping: Unable to locate high-resolution clips.");
        (None, None)
    } else if low_res_only {
        println!("      Skipping: '--low-res-only' set.");
        (None, None)
    } else {
        Media::concatenate(
            &session_hi,
            &outdir_session,
            true,
            None,
            media_suffix_hi,
            // TODO use Path for concatenate()
            &format!("{}", ffmpeg.display()),
        )?
    };

    // Extract wav from low-res if hi-res mp4 not found/not used
    let extract_wav_lo = match audio_eaf_hi {
        None => true,
        Some(_) => false,
    };

    println!("Low-resolution clips in session:");
    for (i, clip) in session_lo.iter().enumerate() {
        println!("      {:2}. {}", i + 1, clip.display());
    }

    let (video_eaf_lo, audio_eaf_lo) = if dryrun {
        println!("      Skipping: '--dryrun' set");
        (None, None)
    } else if session_lo.is_empty() {
        println!("      Skipping: Unable to locate low-resolution clips");
        (None, None)
    } else {
        Media::concatenate(
            &session_lo,
            &outdir_session,
            extract_wav_lo,
            None,
            media_suffix_lo,
            // TODO use Path for concatenate()
            &format!("{}", ffmpeg.display()),
        )?
    };

    // SET EAF MEDIA PATHS
    let video_eaf = match (video_eaf_lo, link_high_res) {
        (Some(v), false) => v,
        // Either low-res does not exist,
        // or 'link_high_res' is true
        _ => match video_eaf_hi {
            Some(v) => v,
            None => {
                let msg = "(!) Unable to set EAF video path.";
                return Err(std::io::Error::new(std::io::ErrorKind::Other, msg));
            }
        },
    };
    let audio_eaf = match (audio_eaf_lo, link_high_res) {
        (Some(v), false) => v,
        // Either low-res does not exist,
        // or 'link_high_res' is true
        _ => match audio_eaf_hi {
            Some(v) => v,
            None => {
                let msg = "(!) Unable to set EAF audio path.";
                return Err(std::io::Error::new(std::io::ErrorKind::Other, msg));
            }
        },
    };

    println!(
        "ELAN media paths:\n  {}\n  {}",
        video_eaf.display(),
        audio_eaf.display(),
    );

    if dryrun {
        println!("(!) '--dryrun' set, no files changed.");
        return Ok(());
    }

    let eaf_path = Path::new(&video_eaf).with_extension("eaf");

    // Generate and write KML + GeoJSON
    if let Some(p) = points.as_deref() {
        let cluster = EafPointCluster::new(p, None);
        let kml_path = eaf_path.with_extension("kml");
        match cluster.write_kml(true, &kml_path) {
            Ok(true) => println!("Wrote {}", kml_path.display()),
            Ok(false) => println!("Aborted writing KML-file"),
            Err(err) => println!("(!) Failed to write '{}': {err}", kml_path.display()),
        }
        let json_path = eaf_path.with_extension("json");
        match cluster.write_json(true, &json_path) {
            Ok(true) => println!("Wrote {}", json_path.display()),
            Ok(false) => println!("Aborted writing GeoJSON-file"),
            Err(err) => println!("(!) Failed to write '{}': {err}", json_path.display()),
        }
    }

    // Generate EAF
    let eaf = match generate_eaf(
        &video_eaf,
        &audio_eaf,
        if geotier { points.as_deref() } else { None },
        // GoPro start ms: GPS points have a relative timestamp
        // from start derived from DEVC timestamp. Set to None for GoPro.
        // VIRB start ms: not the same as start of FIT, so has to be provided
        session_start_ms,
    ) {
        Ok(e) => e,
        Err(err) => {
            let msg = format!("(!) Failed to generate EAF: {err}"); // !!! error on overlapping annotation timespans for gopro fullgps option
            return Err(std::io::Error::new(std::io::ErrorKind::Other, msg));
        }
    };

    let eaf_string = match eaf.to_string(Some(4)) {
        Ok(s) => s,
        Err(err) => {
            let msg = format!("(!) Failed to generate EAF: {err}");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, msg));
        }
    };
    // Not using the Eaf::write() method, as it does not return a Result<bool, EafError>
    match writefile(eaf_string.as_bytes(), &eaf_path) {
        Ok(true) => println!("Wrote {}", eaf_path.display()),
        Ok(false) => println!("User aborted writing ELAN-file"),
        Err(err) => {
            let msg = format!("(!) Failed to write '{}': {err}", eaf_path.display());
            return Err(std::io::Error::new(std::io::ErrorKind::Other, msg));
        }
    }

    // Copy FIT-file (VIRB)
    if let Some(path) = fit_path {
        let path_out =
            outdir_session.join(path.file_name().expect("Failed to extract FIT file name."));
        match std::fs::copy(path, &path_out) {
            Ok(_) => println!("Copied {} to {}", path.display(), outdir_session.display()),
            Err(err) => {
                let msg = format!(
                    "(!) Failed to copy {} to {}: {err}",
                    path.display(),
                    path_out.display()
                );
                return Err(std::io::Error::new(std::io::ErrorKind::Other, msg));
            }
        }
    }

    Ok(())
}
