//! Media processing, such as as concatenation and extracting audio from video.

use std::{process::Command, path::{Path, PathBuf}, io::{stdout, Write}};

use eaf_rs::EafError;

use crate::files::{writefile, affix_file_name};

pub struct Media;

impl Media {
    /// Extract WAV-file from video file.
    pub fn wav(video_path: &Path, ffmpeg_path: &Path) -> Result<PathBuf, EafError> {
        let wav = video_path.with_extension("wav");
        if wav.exists() {
            println!("      Audio target already exists.")
        } else {
            print!("      Extracting wav to {}... ", wav.display());
            stdout().flush()?;
            Command::new(&ffmpeg_path)
                .args(&[
                    "-i",
                    &video_path.display().to_string(),
                    "-vn", &wav.display().to_string()
                ])
                .output()?;
            println!("Done");
        }
    
        Ok(wav)
    }
    
    /// Concatenate video clips.
    /// Returns paths to resulting video and audio as
    /// a tuple `(video, audio)`.
    pub fn concatenate(
        session: &[PathBuf],
        output_dir: &Path,
        extract_wav: bool,
        prefix: Option<&str>,
        suffix: Option<&str>,
        ffmpeg_path: &str,
    ) -> std::io::Result<(Option<PathBuf>, Option<PathBuf>)> {
        // NOTE 200324: Assumes output_dir exists
        if session.is_empty() {
            return Err(std::io::ErrorKind::NotFound.into());
        } else {
            // SET UP PATHS
            let first_in_session = session[0].to_owned();
            let filestem = first_in_session.file_stem().unwrap().to_os_string();
    
            let video_out = affix_file_name(
                &output_dir.canonicalize()?
                    .join(&filestem),
                prefix,
                suffix,
                Some("mp4")
            );
    
            let audio_out = affix_file_name(
                &output_dir.canonicalize()?
                    .join(&filestem),
                prefix,
                suffix,
                Some("wav")
            );
    
            let concatenation_list_path = affix_file_name(
                &output_dir.canonicalize()?
                    .join(&filestem),
                prefix,
                suffix,
                Some("txt")
            );
    
            // concatenation_list_path.set_extension("txt");
    
            // POPULATE + WRITE CONCATENATION LIST
            let mut concatenation_list = String::new();
            for path in session.iter() {
                // Easier to get absolute path instead of verifying that relative ones are valid
                let abs_path = path.canonicalize()?;
                concatenation_list.push_str(&format!("file \'{}\'\n", abs_path.display()));
            }
    
            writefile(&concatenation_list.as_bytes(), &concatenation_list_path)?;
    
            // RUN FFMPEG
            // runs even for single-clip sessions to embed uuid, fit + fit checksum as metadata
            // copies original stream, no re-encoding, however since original is always
            // copied into new container (remux), embedded data (VIRB UUID, GoPro GPMF) is lost.
            Self::run(
                &concatenation_list_path,
                &video_out,
                extract_wav,
                ffmpeg_path,
            )?;
    
            return Ok((
                Some(video_out),
                if extract_wav { Some(audio_out) } else { None },
            ));
        }
    }
    
    fn run(
        concatenation_file_path: &Path,
        output_path: &Path,
        extract_wav: bool,
        ffmpeg_cmd: &str,
    ) -> std::io::Result<()> {
        let concatenation_file_path_str = concatenation_file_path.display().to_string();
        let output_path_str = output_path.display().to_string();
    
        if output_path.exists() {
            // don't want to return error here since wav extraction may still be needed...
            // perhaps restructure.
            // return Err(std::io::ErrorKind::AlreadyExists)
            println!("      Video target already exists.")
        } else {
            print!("      Concatenating to {}... ", output_path.display());
            stdout().flush()?;
    
            let ffmpeg_args = vec![
                "-f", "concat",                     // concatenate
                "-safe", "0",                       // ignore safety warning leading to exit
                "-i", &concatenation_file_path_str, // use file list as input
                "-c:v", "copy",                     // copy video data as is, no conversion
                "-c:a", "copy",                     // copy audio data as is, no conversion
                &output_path_str
            ];
    
            Command::new(&ffmpeg_cmd).args(&ffmpeg_args).output()?;
            println!("Done");
        }
    
        if extract_wav {
            let wav = output_path.with_extension("wav");
            if wav.exists() {
                println!("      Audio target already exists.")
            } else {
                print!("      Extracting wav to {}... ", wav.display());
                stdout().flush()?;
                Command::new(&ffmpeg_cmd)
                    .args(&[
                        "-i", &output_path_str,    // use video concat output as input
                        "-vn",                     // ensure no video (unecessary)
                        &wav.display().to_string()
                    ])
                    .output()?;
                println!("Done");
            }
        }
    
        Ok(())
    }

    /// Returns duration for the longest track in an MP4-file.
    pub fn duration(path: &Path) -> std::io::Result<time::Duration> {
        let mut mp4 = mp4iter::Mp4::new(path)?;
        let duration = mp4.duration(false)?;

        Ok(duration)
    }
}

