use crate::files::writefile;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

// CONCATENATE VIDEO
pub fn concatenate(
    session: &[PathBuf],
    output_dir: &PathBuf,
    glv: bool,
    extract_wav: bool,
    ffmpeg_path: &str,
    meta: Option<&crate::structs::FitMetaData>,
    mpeg2: bool,
) -> std::io::Result<(Option<PathBuf>, Option<PathBuf>)> {
    // NOTE 200324: Assumes output_dir exists
    if session.is_empty() {
        println!("No video files provided");
        return Err(std::io::ErrorKind::NotFound.into());
    } else {
        // SET UP PATHS
        let first_in_session = session[0].to_owned();
        let mut filestem = first_in_session.file_stem().unwrap().to_os_string();

        if glv {
            filestem.push("_GLV");
        }
        let mut video_out = output_dir.join(&filestem);

        if mpeg2 {
            video_out.set_extension("mpg");
        } else {
            video_out.set_extension("mp4");
        }

        let mut audio_out = output_dir.join(&filestem);
        audio_out.set_extension("wav");

        let mut concatenation_list_path = output_dir.join(&filestem);
        concatenation_list_path.set_extension("txt");

        // POPULATE + WRITE CONCATENATION LIST
        let mut concatenation_list = String::new();
        for path in session.iter() {
            concatenation_list.push_str(&format!("file \'{}\'\n", path.display()));
        }

        writefile(&concatenation_list.as_bytes(), &concatenation_list_path)?;

        // RUN FFMPEG
        // runs even for single-clip sessions to embed uuid, fit + fit checksum as metadata
        // copies original stream, no re-encoding
        run_ffmpeg(
            &concatenation_list_path,
            &video_out,
            extract_wav,
            ffmpeg_path,
            meta,
            mpeg2,
        )?;

        return Ok((
            Some(video_out),
            if extract_wav { Some(audio_out) } else { None },
        ));
    }
}

fn run_ffmpeg(
    concatenation_file_path: &Path,
    output_path: &Path,
    extract_wav: bool,
    ffmpeg_cmd: &str,
    meta: Option<&crate::structs::FitMetaData>,
    mpeg2: bool,
) -> std::io::Result<()> {
    let concatenation_file_path_str = concatenation_file_path.display().to_string();
    let output_path_str = output_path.display().to_string();

    if output_path.exists() {
        println!("      Video target already exists.")
    } else {
        print!("      Concatenating to {}... ", output_path.display());
        stdout().flush()?;

        let ffmpeg_args_head = vec![
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            &concatenation_file_path_str,
        ];
        let ffmpeg_args_tail = if mpeg2 {
            ["-c:v", "mpeg2video", "-q:v", "3", &output_path_str]
        } else {
            ["-c:v", "copy", "-c:a", "copy", &output_path_str]
        };

        // Add metadata to MP4
        let mut ffmpeg_args_mid: Vec<String> = Vec::new();
        if let Some(m) = meta {
            ffmpeg_args_mid.append(&mut vec![
                "-movflags".to_owned(),
                "use_metadata_tags".to_owned(),
            ]);
            ffmpeg_args_mid.push(String::from("-metadata"));
            ffmpeg_args_mid.push(format!("fit_uuid={}", m.uuid.join(";")));
            ffmpeg_args_mid.push(String::from("-metadata"));
            ffmpeg_args_mid.push(format!("fit_sha256={}", m.sha256));
            ffmpeg_args_mid.push(String::from("-metadata"));
            ffmpeg_args_mid.push(format!("fit_file={}", m.file));
            ffmpeg_args_mid.push(String::from("-metadata"));
            ffmpeg_args_mid.push(format!("fit_size={}", m.size));
            ffmpeg_args_mid.push(String::from("-metadata"));
            let start = (m.t0 + m.start).format("%Y-%m-%dT%H:%M:%S%.3f").to_string();
            ffmpeg_args_mid.push(format!("fit_start={}", start));
            ffmpeg_args_mid.push(String::from("-metadata"));
            let end = (m.t0 + m.end).format("%Y-%m-%dT%H:%M:%S%.3f").to_string();
            ffmpeg_args_mid.push(format!("fit_end={}", end));
        }
        let ffmpeg_args = [
            ffmpeg_args_head,
            ffmpeg_args_mid.iter().map(AsRef::as_ref).collect(),
            ffmpeg_args_tail.to_vec(),
        ]
        .concat();

        Command::new(&ffmpeg_cmd).args(&ffmpeg_args).output()?;
        println!("Done");
    }

    if extract_wav {
        let mut wav = output_path.to_owned();
        wav.set_extension("wav");
        if wav.exists() {
            println!("      Audio target already exists.")
        } else {
            print!("      Extracting wav to {}... ", wav.display());
            stdout().flush()?;
            Command::new(&ffmpeg_cmd)
                .args(&["-i", &output_path_str, "-vn", &wav.display().to_string()])
                .output()?;
            println!("Done");
        }
    }

    Ok(())
}
