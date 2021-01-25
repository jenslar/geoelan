use fit::structs::FitFile;
use std::collections::HashMap;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::io::Write; // for .flush()
use crate::structs::{SessionTimeSpan, VirbFile, VirbFiles};

/// Advise command for checking whether specified data exist in FIT-file
pub fn advise_check(path: &PathBuf, global_id: u16, uuid: &Option<String>, verbose: bool) -> String {
    let uuid_opt = match uuid {
        Some(u) => format!(" -u {}", u),
        None => String::from("")
    };
    return format!(
        "geoelan check -f '{}' -g {}{}{}",
        path.display(),
        global_id,
        uuid_opt,
        if verbose { " --verbose" } else { "" }
    );
}


/// Select session from those present in FIT-file
/// by returning UUID for first clip in session
pub fn select_session(fitfile: &FitFile) -> std::io::Result<String> {
    let sessions = fitfile.sessions(true)?;
    if sessions.is_empty() {
        println!(
            "No UUIDs in specified file. Try running {}",
            advise_check(&fitfile.path.to_owned(), 162, &None, true)
        );
        std::process::exit(1)
    }

    println!(" Session | Clips | First UUID in session");
    println!(".......................{}", ".".repeat(100));

    for (i, session) in sessions.iter().enumerate() {
        println!(
            " {:2}.     | {:2}    | {}",
            i + 1,
            session.len(),
            session[0]
        );
    }

    println!(".......................{}", ".".repeat(100));

    loop {
        print!("Select session: ");
        std::io::stdout().flush()?;
        let mut select = String::new();
        std::io::stdin()
            .read_line(&mut select)
            .expect("(!) Failed to read input");
        let num = match select.trim().parse::<usize>() {
            Ok(n) => n - 1,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
        match sessions.get(num) {
            Some(u) => return Ok(u[0].to_owned()),
            None => {
                println!("No such item");
                continue;
            }
        }
    }
}

pub fn session_timespan(
    camera_events: &[fit::structs::CameraEvent],
    uuid: &Option<String>,
    single_clip: bool, // not yet implemented
) -> Option<SessionTimeSpan> {
    // TODO 200319: add custom camera_event_type for start/end to support single clips
    let video_start_event = if single_clip { 4 } else { 0 }; // is 4 true even for single-clip session?
    let video_end_event = if single_clip { 1 } else { 2 }; // is 1 true even for single-clip session?
                                                           // let video_end_event = if single {6} else {2}; // is 6 true even for single-clip session?

    let uuid_start = if let Some(u) = uuid { u } else { return None };
    let mut video_start: Option<chrono::Duration> = None;
    let mut video_end: Option<chrono::Duration> = None;
    let mut uuid_session = Vec::new();

    for event in camera_events.iter() {
        if video_start.is_none()
            && &event.camera_file_uuid == uuid_start
            && event.camera_event_type == video_start_event
        {
            uuid_session.push(event.camera_file_uuid.clone());
            let sec = chrono::Duration::seconds(event.timestamp as i64);
            let ms = chrono::Duration::milliseconds(event.timestamp_ms as i64);
            video_start = Some(sec + ms)
        }

        if video_start.is_some() && video_end.is_none() {
            uuid_session.push(event.camera_file_uuid.clone());
            if event.camera_event_type == video_end_event {
                let sec = chrono::Duration::seconds(event.timestamp as i64);
                let ms = chrono::Duration::milliseconds(event.timestamp_ms as i64);
                video_end = Some(sec + ms)
            }
        }
    }

    uuid_session.dedup(); // enough or check at push()?

    Some(SessionTimeSpan {
        start: video_start.expect("Could not assign start time"), // TODO 200530: handle error
        end: video_end.expect("Could not assign end time"),       // TODO 200530: handle error
        uuid: uuid_session,
    })
}

pub fn compile_virbfiles(
    // -> match_virbfiles
    dir_start: &Path,
    // uuid_start: &Option<String>, // first uuid in session
    verbose: bool,
    duplicate_types: bool,
) -> std::io::Result<VirbFiles> {
    // NOTE need Vec<String> for uuids to keep record of them in order
    let mut uuid: HashMap<String, Vec<VirbFile>> = HashMap::new(); // k: uuid, v: files with uuid
    let mut session: HashMap<String, Vec<String>> = HashMap::new(); // k: 1st session uuid, v: session uuid
    let mut filetypes: HashMap<String, usize> = HashMap::new(); //  mp4/glv/fit stats

    let mut virb_file_count = 0;

    for file in WalkDir::new(dir_start) {
        let path = match file {
            Ok(f) => f.path().to_owned(),
            Err(e) => {
                if verbose {
                    println!("[SKIP]     Skipping path: {}", e)
                };
                continue;
            }
        };

        if let Some(virbfile) = VirbFile::new(&path) {
            virb_file_count += 1;

            let filetype = virbfile.type_to_str();

            // log filetypes
            *filetypes.entry(filetype.into()).or_insert(0) += 1;

            if verbose {
                print!(
                    "[{:04}] {} {} ",
                    virb_file_count,
                    filetype,
                    virbfile.path.display()
                );
                std::io::stdout().flush()?;
            }

            // log first uuid in session + uuid for entire session
            if let Some(sessions) = virbfile.sessions.to_owned() {
                for s in sessions.into_iter() {
                    session.insert(s[0].to_owned(), s);
                }
            }

            // log uuid + file
            if let Some(uuids) = virbfile.uuid.to_owned() {
                for u in uuids.into_iter() {
                    let mut insert = false;
                    if duplicate_types {
                        insert = true;
                    } else {
                        // let mut insert = false;
                        match &uuid.get(&u) {
                            Some(files) => {
                                if !files.iter().any(|f| f.filetype == virbfile.filetype) {
                                    insert = true;
                                }
                            }
                            None => insert = true,
                        }
                    }
                    if insert {
                        uuid.entry(u).or_insert(vec![]).push(virbfile.to_owned());
                    }
                }

                if verbose {
                    println!("[ OK ]");
                }
            } else {
                if verbose {
                    println!("[NONE]");
                }
            }
        }
    }

    Ok(VirbFiles {
        uuid,
        session,
        filetypes,
    })
}
