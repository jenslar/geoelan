//! Inspect camera telemetry, such as GPS logs.

use std::{fs::File, io::{ErrorKind, Write}, path::PathBuf, result};

use fit_rs::VirbFile;
use gpmf_rs::GoProFile;
use mp4iter::{track::Track, Mp4, Mp4Error, TrackIdentifier};

use crate::{files::{affix_file_name, confirm, has_extension_any, writefile, Units}, model::CameraModel};

mod inspect_fit;
mod inspect_gpmf;

pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    // Inspect GoPro GPMF or Garmin FIT telemetry
    if args.get_one::<PathBuf>("gpmf").is_some() {
        return inspect_gpmf::inspect_gpmf(args);
    } else if args.get_one::<PathBuf>("fit").is_some() {
        return inspect_fit::inspect_fit(args);
    }

    // Inspect MP4 atoms, tracks etc
    if let Some(path) = args.get_one::<PathBuf>("video") {
        let model = CameraModel::from(path.as_path());

        let print_atoms = *args.get_one::<bool>("atoms").unwrap();
        let print_meta = *args.get_one::<bool>("meta").unwrap();

        // Print offsests, raw sample data, or dump raw sample data to disk.
        // Contains either track name (string) or track id (u32).
        let track_offsets = args.get_one::<String>("offsets");
        let track_samples = args.get_one::<String>("samples");
        let track_dump = args.get_one::<String>("dump");

        let mut mp4 = match mp4iter::Mp4::new(path) {
            Ok(v) => v,
            Err(err) => {
                let msg = format!("(!) Failed to read video file: {err}");
                return Err(std::io::Error::new(ErrorKind::Other, msg));
            }
        };

        if let Some(track_id) = track_offsets {

            // if has_extension_any(&path, &["glv", "lrv", "mp4", "mov"]) {

                // let mut mp4 = mp4iter::Mp4::new(&path)?;
                // let mut mp4 = match mp4iter::Mp4::new(&path) {
                //     Ok(vid) => vid,
                //     Err(err) => {
                //         let msg = format!("(!) Failed to read video file: {err}");
                //         return Err(std::io::Error::new(ErrorKind::InvalidData, msg))
                //     },
                // };

                let track_identifier = TrackIdentifier::from(track_id.as_str());

                let track = match Track::new(&mut mp4, track_identifier, false) {
                    Ok(t) => t,
                    Err(err) => {
                        let msg = format!("Error reading track: {err}");
                        return Err(std::io::Error::new(ErrorKind::Other, msg))
                    },
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
            // } else {
            //     let msg = format!("(!) Incompatible file format: must be an MP4 file.");
            //     return Err(std::io::Error::new(ErrorKind::Other, msg));
            // }
        }

        if let Some(track_id) = track_samples {

            // if has_extension_any(&path, &["glv", "lrv", "mp4", "mov"]) {

                // let mut mp4 = mp4iter::Mp4::new(&path)?;
                // let mut mp4 = match mp4iter::Mp4::new(&path) {
                //     Ok(vid) => vid,
                //     Err(err) => {
                //         let msg = format!("(!) Failed to read video file: {err}");
                //         return Err(std::io::Error::new(ErrorKind::InvalidData, msg))
                //     },
                // };

                let track_identifier = TrackIdentifier::from(track_id.as_str());

                let mut track = match Track::new(&mut mp4, track_identifier, false) {
                    Ok(t) => t,
                    Err(err) => {
                        let msg = format!("Error reading track: {err}");
                        return Err(std::io::Error::new(ErrorKind::Other, msg))
                    },
                };

                let name = track.name().to_owned();
                let id = track.id().to_owned();

                println!("Track {name}/{id} selected. Total sample size: {} bytes.", track.size());
                let msg = format!("This will print {} of data. Continue?", Units::from(track.size()));
                if !confirm(&msg)? {
                    println!("Aborting.");
                    return Ok(())
                }

                for (i, result) in track.samples().enumerate() {
                    match result {
                        Ok(sample) => println!("{:6}. time: {} | duration: {}\n  {:?}",
                            i + 1,
                            sample.relative(),
                            sample.duration(),
                            sample.raw()
                        ),
                        Err(err) => println!("{:6}. Error reading sample: {err}", i + 1),
                    }
                }

                return Ok(());
            // } else {
            //     let msg = format!("(!) Incorrect file format for '--offsets', must be an MP4 file.");
            //     return Err(std::io::Error::new(ErrorKind::Other, msg));
            // }
        }

        if let Some(track_id) = track_dump {

            // if has_extension_any(&path, &["glv", "lrv", "mp4", "mov"]) {

                // let mut mp4 = mp4iter::Mp4::new(&path)?;
                // let mut mp4 = match mp4iter::Mp4::new(&path) {
                //     Ok(vid) => vid,
                //     Err(err) => {
                //         let msg = format!("(!) Failed to read video file: {err}");
                //         return Err(std::io::Error::new(ErrorKind::InvalidData, msg))
                //     },
                // };

                let track_identifier = TrackIdentifier::from(track_id.as_str());

                let mut track = match Track::new(&mut mp4, track_identifier, false) {
                    Ok(t) => t,
                    Err(err) => {
                        let msg = format!("Error reading track: {err}");
                        return Err(std::io::Error::new(ErrorKind::Other, msg))
                    },
                };

                let name = track.name().to_owned();
                let id = track.id().to_owned();

                println!("Track {name}/{id} selected. Total sample size: {} bytes.", track.size());
                let msg = format!("This will save {} of data to disk. Continue?", Units::from(track.size()));
                if !confirm(&msg)? {
                    println!("Aborting.");
                    return Ok(())
                }

                let binpath = affix_file_name(
                    &path,
                    None,
                    Some(&format!("_TRACK_{}_{}", name, id)),
                    Some("bin"),
                    Some("_")
                );

                let samples = track.samples().enumerate()
                    .inspect(|(i, result)|
                        if let Err(err) = result.as_ref() {
                            println!("Failed to read sample {} in track {}/{} to file: {err}",
                                i+1,
                                name,
                                id
                            );
                        }
                    )
                    .map(|(_, result)| result)
                    .collect::<Result<Vec<_>, Mp4Error>>()?
                    .iter()
                    .flat_map(|sample| sample.raw().to_vec())
                    .collect::<Vec<_>>();

                match writefile(&samples, &binpath) {
                    Ok(true) => println!("Wrote {}", binpath.display()),
                    Ok(false) => println!("Aborted writing file."),
                    Err(err) => println!("Failed to write file: {err}"),
                };

                return Ok(());
            // } else {
            //     let msg = format!("(!) Incorrect file format for '--offsets', must be an MP4 file.");
            //     return Err(std::io::Error::new(ErrorKind::Other, msg));
            // }
        }

        // println!("Tracks:");
        // let tracks = mp4.track_list(false)?;
        // println!(" IDX │ NAME             │ ID │ DURATION    │ SAMPLES │ TYPE │ MEDIA INFO");
        // println!("─────┼──────────────────┼────┼─────────────┼─────────┼──────┼────────────");
        // for (i, track) in tracks.iter().enumerate() {
        //     let ttype = track.sub_type();
        //     print!(" {:2}. │ {:16} │ {:2} │ {:10.3}s │ {:7} │ {ttype}",
        //         i+1,
        //         track.name(),
        //         track.id(),
        //         track.duration().as_seconds_f64(),
        //         track.offsets().len()
        //     );
        //     match ttype {
        //         "vide" => {
        //             println!(" │ Video ({} x {} @ {} fps)",
        //                 track.width(),
        //                 track.height(),
        //                 track.frame_rate(),
        //             )
        //         },
        //         "soun" => println!(" │ Audio ({} Hz)",
        //             track.sample_rate().map(|s| s.to_string()).unwrap_or("Unknown".to_owned())
        //         ),
        //         "tmcd" => println!(" │ Timecode"),
        //         _ => println!(" │ N/A")
        //     }
        // }

        // println!("---");

        if print_atoms {

            // mp4.reset()?;

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

            return Ok(())
        }

        println!("Tracks:");
        let tracks = mp4.track_list(false)?;
        println!(" IDX │ NAME             │ ID │ DURATION    │ SAMPLES │ TYPE │ MEDIA INFO");
        println!("─────┼──────────────────┼────┼─────────────┼─────────┼──────┼────────────");
        for (i, track) in tracks.iter().enumerate() {
            let ttype = track.sub_type();
            print!(" {:2}. │ {:16} │ {:2} │ {:10.3}s │ {:7} │ {ttype}",
                i+1,
                track.name(),
                track.id(),
                track.duration().as_seconds_f64(),
                track.offsets().len(),
            );
            match ttype {
                "vide" => {
                    println!(" │ Video ({} x {} @ {} fps)",
                        track.width(),
                        track.height(),
                        track.frame_rate(),
                    )
                },
                "soun" => println!(" │ Audio ({} Hz)",
                    track.sample_rate().map(|s| s.to_string()).unwrap_or("Unknown".to_owned())
                ),
                "tmcd" => println!(" │ Timecode"),
                _ => println!(" │ N/A")
            }
        }

        match model {
            CameraModel::GoPro(devname) => {
                let gopro = match GoProFile::new(path.as_path()) { // err with hero13 file with moov before mdat, "no mdhd"
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
                    "Device: {}\n  MUID: {:?}\n  GUMI: {:?}",
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
                    println!("No GoPro or VIRB data found. If this is a GoPro/VIRB video, ensure you use the original files.");
                    let mut mp4 = Mp4::new(&path)?;
                    let (start, duration) = mp4.time(false)?;
                    let end = start + duration;
                    println!(
                        "{} - {} ({:10.3}s)",
                        start.to_string(),
                        end.to_string(),
                        duration.as_seconds_f64()
                    );
                }

                return Ok(());
            }
        }
    }

    Ok(())
}
