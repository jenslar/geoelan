//! Support for automatically extracting sections from linked media
//! when e.g. extracting a time span of the EAF-file as a new file.
//! 
//! Requires `ffmpeg` (<https://ffmpeg.org>). Either in `PATH` or specified
//! custom path.

pub mod process;
pub mod stats;