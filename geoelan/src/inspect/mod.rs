//! Inspect camera telemetry, such as GPS logs.

use std::{path::PathBuf, io::ErrorKind};

use fit_rs::VirbFile;
use gpmf_rs::GoProFile;

use crate::model::CameraModel;

mod inspect_gpmf;
mod inspect_fit;

pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {

    // Inspect GoPro GPMF or Garmin FIT telemetry
    if args.get_one::<PathBuf>("gpmf").is_some() {
        return inspect_gpmf::inspect_gpmf(args)
    } else if args.get_one::<PathBuf>("fit").is_some() {
        return inspect_fit::inspect_fit(args)
    }

    // Inspect GoPro (no telemetry) or VIRB MP4
    if let Some(path) = args.get_one::<PathBuf>("video") {
        let model = CameraModel::from(path.as_path());
        let print_atoms = *args.get_one::<bool>("atoms").unwrap();
        let print_meta = *args.get_one::<bool>("meta").unwrap();

        if print_atoms {
            let mp4 = match mp4iter::Mp4::new(path) {
                Ok(v) => v,
                Err(err) => {
                    let msg = format!("(!) Failed to read MP4: {err}");
                    return Err(std::io::Error::new(ErrorKind::Other, msg))
                }
            };
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
                        *size -= header.size;
                    }
                    if size == &mut 0 {
                        pop = true;
                    }
                }
                // if header.name == FourCC::Hdlr {
                //     let hdlr = mp4.atom(&mut header)?;
                // }
                println!("{}{} @{} size: {}",
                    "    ".repeat(indent as usize),
                    header.name.to_str(),
                    header.offset,
                    header.size,
                );
                if is_container {
                    sizes.push(header.size - 8);
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
            println!("---");
        }

        match model {
            CameraModel::GoPro(devname) => {
                // let gopro = match GoProFile::new(path.as_path(), false, false) {
                // let gopro = match GoProFile::new(path.as_path(), false, false) {
                let gopro = match GoProFile::new(path.as_path()) {
                    Ok(g) => g,
                    Err(err) => {
                        let msg = format!("(!) Failed to read as GoPro MP4: {err}");
                        return Err(std::io::Error::new(ErrorKind::Other, msg))
                    }
                };

                if print_meta {
                    let meta = match gopro.meta() {
                        Ok(m) => m,
                        Err(err) => {
                            let msg = format!("(!) Failed to extract metadata from GoPro MP4: {err}");
                            return Err(std::io::Error::new(ErrorKind::Other, msg))
                        }
                    };
                    println!("Metadata (MP4 'udta' atom):");
                    for field in meta.udta.iter() {
                        println!("  {} SIZE: {}", field.name.to_str(), field.size);
                        println!("     RAW: {:?}", field.data.get_ref());
                    }
                    for stream in meta.gpmf.iter() {
                        stream.print(None, None)
                    }
                    println!("---");
                }

                // let dvnm = DeviceName::from_mp4(&path)?;
                println!("Identified as {} MP4 file\n  MUID: {:?}\n  GUMI: {:?}",
                    devname.to_str(),
                    gopro.muid,
                    gopro.gumi,
                );
                let time_dur = gopro.time()?;
                println!("Creation time: {}", time_dur.0.to_string());
                println!("Duration:      {:.3}s", time_dur.1.as_seconds_f64());
                println!("To inspect GPMF run 'geoelan inspect --gpmf {}'", path.display());
                
                return Ok(())
            },
            CameraModel::Virb(uuid) => {
                let virb = match VirbFile::new(path.as_path(), None) {
                    Ok(v) => v,
                    Err(err) => {
                        let msg = format!("(!) Failed to read as VIRB MP4: {err}");
                        return Err(std::io::Error::new(ErrorKind::Other, msg))
                    }
                };
                if print_meta {
                    let meta = match virb.meta() {
                        Ok(m) => m,
                        Err(err) => {
                            let msg = format!("(!) Failed to extract metadata from VIRB MP4: {err}");
                            return Err(std::io::Error::new(ErrorKind::Other, msg))
                        }
                    };
                    println!("Metadata (MP4 'udta' atom):");
                    for field in meta.fields.iter() {
                        println!("  {} SIZE: {}", field.name.to_str(), field.size);
                        println!("     RAW: {:?}", field.data.get_ref());
                    }
                    println!("---");
                }
                println!("Identified as VIRB MP4 file with UUID:\n{}", uuid);
                std::process::exit(0)
            },
            CameraModel::Unknown => {
                if print_meta {
                    let mut mp4 = match mp4iter::Mp4::new(path.as_path()) {
                        Ok(v) => v,
                        Err(err) => {
                            let msg = format!("(!) Failed to read MP4: {err}");
                            return Err(std::io::Error::new(ErrorKind::Other, msg))
                        }
                    };
                    let meta = match mp4.udta() {
                        Ok(v) => v,
                        Err(err) => {
                            let msg = format!("(!) Failed to extract metadata from MP4: {err}");
                            return Err(std::io::Error::new(ErrorKind::Other, msg))
                        }
                    };
    
                    println!("Metadata (MP4 'udta' atom):");
                    for field in meta.fields.iter() {
                        println!("  {} SIZE: {}", field.name.to_str(), field.size);
                        println!("     RAW: {:?}", field.data.get_ref());
                    }
                    println!("---");
                }

                print!("Unknown camera model ({}).", path.file_name().unwrap().to_string_lossy());
                if let Ok(gp) = GoProFile::new(&path) {
                    println!(" Possibly GoPro with no GPMF data and MUID {:?}", gp.muid)
                } else {
                    println!(" No GoPro GPMF data or VIRB UUID found.")
                }
                
                return Ok(())
            },
        }
    }

    Ok(())
}