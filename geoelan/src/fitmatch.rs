use fit::structs::FitFile;

use crate::virb::{compile_virbfiles, select_session};
use crate::files::writefile;
use std::time::Instant;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

// main match sub-command
pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let timer = Instant::now();

    let indir = PathBuf::from(args.value_of("input-directory").unwrap()).canonicalize()?;
    let write_csv = args.is_present("write-csv");
    let uuid: Option<String> = if let Some(v) = args.value_of("video") {
        fit::get_video_uuid(&Path::new(v))?
    } else if let Some(u) = args.value_of("uuid") {
        Some(u.to_string())
    } else if let Some(f) = args.value_of("fit") {
        let fitfile = FitFile::new(&PathBuf::from(f));
        Some(select_session(&fitfile)?)
    } else {
        None
    };
    let duplicates = args.is_present("duplicates");
    let quiet = args.is_present("quiet");

    // NEW IN DEV PART START
    let virbfiles = compile_virbfiles(&indir, !quiet, duplicates)?;

    println!("---------------------");
    println!(
        "MATCHES {}",
        if duplicates {
            " (DUPLICATES)"
        } else {
            "(UNIQUE ONLY)"
        }
    );
    println!("---------------------");

    let mut csv = vec!["MP4,GLV,FIT,UUID,FITDATETIME".to_owned()];

    let mut mp4 = "NONE".to_owned();
    let mut glv = "NONE".to_owned();
    let mut fit = "NONE".to_owned();
    let mut vid_uuid = "NONE".to_owned();
    let mut fit_date = "NONE".to_owned();

    let mut match_count = 0;

    // setting session as HashMap if uuid.is_some()
    // to avoid code duplication
    let virbsession = match &uuid {
        Some(u) => {
            let mut hm: HashMap<String, Vec<String>> = HashMap::new();
            match virbfiles.session.get(&*u) {
                Some(s) => {
                    hm.insert(u.into(), s.clone());
                    hm
                }
                None => {
                    println!("No session starting with UUID {}", u);
                    std::process::exit(0)
                }
            }
        }
        None => virbfiles.session,
    };

    for (_, session) in virbsession.iter() {
        // iter .session to list session files in correct order
        for uuid_file in session.iter() {
            if let Some(files) = virbfiles.uuid.get(uuid_file) {
                if files.len() < 2 {
                    continue;
                }
                match_count += 1;
                println!("[{:02}] {}", match_count, uuid_file);
                for file in files.iter() {
                    println!("  {} {}", file.type_to_str(), file.path.display());
                    if write_csv {
                        match file.filetype {
                            crate::structs::VirbFileType::FIT => {
                                fit_date = match FitFile::new(&file.path).t0(0, true) {
                                    Ok(t) => t.format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
                                    Err(_) => "COULD NOT DERIVE TIMESTAMP".to_owned(),
                                };
                                fit = file.path.display().to_string();
                            }
                            crate::structs::VirbFileType::GLV => {
                                vid_uuid = uuid_file.to_owned();
                                glv = file.path.display().to_string();
                            }
                            crate::structs::VirbFileType::MP4 => {
                                vid_uuid = uuid_file.to_owned();
                                mp4 = file.path.display().to_string();
                            }
                        }
                    }
                }
                csv.push(format!("{},{},{},{},{}", mp4, glv, fit, vid_uuid, fit_date));
            }
        }
    }

    if csv.len() > 1 && write_csv {
        let csv_path = PathBuf::from("matches.csv");
        if let Err(e) = writefile(&csv.join("\n").as_bytes(), &csv_path) {
            println!("(!) Could note write {}: {}", csv_path.display(), e);
        };
    }

    println!("-------");
    println!("SUMMARY");
    println!("-------");
    println!("Matches: {}", match_count);
    for (filetype, count) in virbfiles.filetypes {
        println!("    {}: {}", filetype, count);
    }

    println!(
        "Done ({:.3}s)",
        (timer.elapsed().as_millis() as f64) / 1000.0
    );
    Ok(())
}
