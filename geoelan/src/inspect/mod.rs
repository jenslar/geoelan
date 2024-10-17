//! Inspect camera telemetry, such as GPS logs.

use std::{io::ErrorKind, path::PathBuf};

use fit_rs::VirbFile;
use gpmf_rs::GoProFile;
use mp4iter::{track::Track, Mp4};

use crate::{files::has_extension_any, model::CameraModel};

mod inspect_fit;
mod inspect_gpmf;

pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    // Inspect GoPro GPMF or Garmin FIT telemetry
    if args.get_one::<PathBuf>("gpmf").is_some() {
        return inspect_gpmf::inspect_gpmf(args);
    } else if args.get_one::<PathBuf>("fit").is_some() {
        return inspect_fit::inspect_fit(args);
    }

    // Inspect MP4 atom hierarchy
    if let Some(path) = args.get_one::<PathBuf>("video") {
        let model = CameraModel::from(path.as_path());

        let print_atoms = *args.get_one::<bool>("atoms").unwrap();
        let print_meta = *args.get_one::<bool>("meta").unwrap();
        let track_offsets = args.get_one::<String>("offsets");

        let mut mp4 = match mp4iter::Mp4::new(path) {
            Ok(v) => v,
            Err(err) => {
                let msg = format!("(!) Failed to read MP4: {err}");
                return Err(std::io::Error::new(ErrorKind::Other, msg));
            }
        };

        if let Some(track_id) = track_offsets {
            // if has_extension(&path, "lrv") || has_extension(&path, "mp4") {
            if has_extension_any(&path, &["glv", "lrv", "mp4", "mov"]) {
                let mut mp4 = mp4iter::Mp4::new(&path)?;
                // let offsets = mp4.offsets("GoPro MET", false)?;
                let track = match track_id.parse::<u32>() {
                    Ok(id) => Track::from_id(&mut mp4, id, false)?,
                    Err(_) => Track::from_name(&mut mp4, &track_id, false)?,
                };

                for (i, offset) in track.offsets().enumerate() {
                    println!(
                        "[{:4} {}/{}] @{:<10} size: {:<6} duration: {}",
                        i + 1,
                        track.name(),
                        track.id(),
                        offset.position,
                        offset.size,
                        offset.duration
                    )
                }

                return Ok(());
            } else {
                let msg = format!("(!) Incorrect file format for '--offsets', must be a GoPro MP4.\n    Try 'geoelan inspect --video {}", path.display());
                return Err(std::io::Error::new(ErrorKind::Other, msg));
            }
        }

        println!("Tracks:");
        let tracks = mp4.track_list(false)?;
        for (i, track) in tracks.iter().enumerate() {
            print!("  {:2}. {:16} Id: {:2} Duration: {:10.3}s Samples: {:6} Type: ",
                i+1,
                track.name(),
                track.id(),
                track.duration().as_seconds_f64(),
                track.offsets().len()
            );
            let ttype = track.track_type();
            match ttype {
                "vide" => println!("Video ({} x {})", track.width(), track.height()),
                "soun" => println!("Audio"),
                _ => println!("{}", ttype)
            }
        }

        println!("---");

        if print_atoms {

            mp4.reset()?;

            // Print atom fourcc, size, offsets
            // 'sizes' contains 'atom size - 8' since 8 byte header is already read.
            // Each value will decrease until it's 0 which flags that it shold be removed.
            // Last value is added last and will be removed first as it indicates
            // the container atom is child to another container atom.
            let mut sizes: Vec<u64> = Vec::new();
            for header in mp4.into_iter() {
                let mut pop = false;
                let indent = sizes.len();
                let is_container = header.is_container();
                for size in sizes.iter_mut() {
                    if is_container {
                        *size -= 8;
                    } else {
                        *size -= header.atom_size();
                    }
                    if size == &mut 0 {
                        pop = true;
                    }
                }

                println!(
                    "{}{} @{} size: {}",
                    "    ".repeat(indent as usize),
                    header.name().to_str(),
                    header.offset(),
                    header.atom_size(),
                );
                if is_container {
                    sizes.push(header.atom_size() - 8);
                }
                if pop {
                    loop {
                        match sizes.last() {
                            Some(&0) => {let _popped = sizes.pop();},
                            _ => break,
                        }
                    }
                }
            }
            println!("---");
        }

        match model {
            CameraModel::GoPro(devname) => {
                let gopro = match GoProFile::new(path.as_path()) {
                    Ok(g) => g,
                    Err(err) => {
                        let msg = format!("(!) Failed to read as GoPro MP4: {err}");
                        return Err(std::io::Error::new(ErrorKind::Other, msg));
                    }
                };

                if print_meta {
                    let meta = match gopro.meta() {
                        Ok(m) => m,
                        Err(err) => {
                            let msg =
                                format!("(!) Failed to extract metadata from GoPro MP4: {err}");
                            return Err(std::io::Error::new(ErrorKind::Other, msg));
                        }
                    };
                    println!("Metadata (MP4 'udta' atom):");
                    for (name, bytes) in meta.raw.iter() {
                        println!("  {} SIZE: {}", name, bytes.len());
                        println!("     RAW: {:?}", bytes);
                    }

                    println!("GPMF formatted user data:");
                    meta.gpmf.print();
                    println!("---");
                }

                println!(
                    "Identified as {} MP4 file\n  MUID: {:?}\n  GUMI: {:?}",
                    devname.to_str(),
                    gopro.muid,
                    gopro.gumi,
                );

                let (gp_start, gp_duration) = (gopro.start(), gopro.duration());
                println!("Creation time: {}", gp_start.to_string());
                println!("Duration:      {:.3}s", gp_duration.as_seconds_f64());
                println!(
                    "To inspect GPMF run 'geoelan inspect --gpmf {}'",
                    path.display()
                );

                return Ok(());
            }
            CameraModel::Virb(uuid) => {
                let virb = match VirbFile::new(path.as_path(), None) {
                    Ok(v) => v,
                    Err(err) => {
                        let msg = format!("(!) Failed to read as VIRB MP4: {err}");
                        return Err(std::io::Error::new(ErrorKind::Other, msg));
                    }
                };
                if print_meta {
                    let meta = match virb.meta() {
                        Ok(m) => m,
                        Err(err) => {
                            let msg =
                                format!("(!) Failed to extract metadata from VIRB MP4: {err}");
                            return Err(std::io::Error::new(ErrorKind::Other, msg));
                        }
                    };
                    println!("Metadata (MP4 'udta' atom):");
                    for (name, bytes) in meta.iter() {
                        println!("  {} SIZE: {}", name, bytes.len());
                        println!("     RAW: {:?}", bytes);
                    }
                    println!("---");
                }
                println!("Identified as VIRB MP4 file with UUID:\n{}", uuid);
                std::process::exit(0)
            }
            CameraModel::Unknown => {
                if print_meta {
                    let mut mp4 = match mp4iter::Mp4::new(path.as_path()) {
                        Ok(v) => v,
                        Err(err) => {
                            let msg = format!("(!) Failed to read MP4: {err}");
                            return Err(std::io::Error::new(ErrorKind::Other, msg));
                        }
                    };

                    let meta = match mp4.user_data_cursors() {
                        Ok(m) => m,
                        Err(err) => {
                            let msg = format!("(!) Failed to extract metadata from MP4: {err}");
                            return Err(std::io::Error::new(ErrorKind::Other, msg));
                        }
                    };

                    println!("Metadata (MP4 'udta' atom):");
                    for (name, bytes) in meta.iter() {
                        println!("  {} SIZE: {}", name, bytes.get_ref().len());
                        println!("     RAW: {:?}", bytes);
                    }
                    println!("---");
                }

                if let Ok(gp) = GoProFile::new(&path) {
                    println!("Possibly GoPro with no GPMF data and MUID {:?}", gp.muid)
                } else {
                    println!("No GoPro GPMF data or VIRB UUID found. Make sure to use the original files.");
                    let mut mp4 = Mp4::new(&path)?;
                    let (start, duration) = mp4.time(false)?;
                    let end = start + duration;
                    println!(
                        "{} - {} ({} s)",
                        start.to_string(),
                        end.to_string(),
                        duration.as_seconds_f32()
                    );
                }

                return Ok(());
            }
        }
    }

    Ok(())
}
