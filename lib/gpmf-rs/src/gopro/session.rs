//! GoPro recording session.

use std::path::{Path, PathBuf};

use crate::{Gpmf, GpmfError};

use super::{GoProFile, GoProMeta};

#[derive(Debug, PartialEq)]
pub struct GoProSession(Vec<GoProFile>);

impl GoProSession {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, gopro_file: &GoProFile) {
        self.0.push(gopro_file.to_owned());
    }

    pub fn iter(&self) -> impl Iterator<Item = &GoProFile> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut GoProFile> {
        self.0.iter_mut()
    }

    pub fn first(&self) -> Option<&GoProFile> {
        self.0.first()
    }

    pub fn first_mut(&mut self) -> Option<&mut GoProFile> {
        self.0.first_mut()
    }

    pub fn last(&self) -> Option<&GoProFile> {
        self.0.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut GoProFile> {
        self.0.last_mut()
    }

    pub fn is_parsed(&self) -> bool {
        // self.first()
        //     .map(|gp| gp.parsed)
        //     .unwrap_or(false)
        self.iter().any(|gp| gp.parsed)
    }

    /// Merges GPMF-data for all
    /// files in session to a single
    /// `Gpmf` struct.
    pub fn gpmf(&mut self) -> Gpmf {
        let mut gpmf = Gpmf::default();
        for gopro in self.iter_mut() {
            gpmf.merge_mut(&mut gopro.gpmf);
        }
        gpmf
    }

    pub fn meta(&self) -> Vec<GoProMeta> {
        self.0.iter()
            .filter_map(|gp| gp.meta().ok())
            .collect()
    }

    pub fn mp4_paths(&self) -> Vec<PathBuf> {
        self.iter()
            .map(|g| g.mp4_path.to_owned())
            .collect()
    }

    pub fn lrv_paths(&self) -> Vec<PathBuf> {
        self.iter()
            .map(|g| g.lrv_path.to_owned())
            .collect()
    }

    // pub fn meta(&self) -> Vec<Udta> {
    //     self.iter()
    //         .filter_map(|g| g.meta.to_owned())
    //         .collect()
    // }

    // pub fn print_meta(&self) {
    //     for (i, gopro_file) in self.iter().enumerate() {
    //         println!("{:2}. {}", i+1, gopro_file.mp4_path.display());
    //         println!("    {}", gopro_file.lrv_path.display());
    //         gopro_file.print_meta()
    //     }
    // }

    /// Compile GoPro session from GoPro MP4-file.
    /// `single` = `true` only considers the specified file.
    /// `parse` = `true` parses the GPMF-data for each file.
    /// File name to determine which files belong to a session.
    pub fn from_path(mp4_path: &Path, single: bool, parse: bool, debug: bool) -> Result<Self, GpmfError> {
        
        let first_gopro = GoProFile::new(mp4_path, parse, debug)?;
        let file_id = first_gopro.file_id.to_owned();
        let mut session = GoProSession(vec!(first_gopro));

        let mut startdir = mp4_path.parent()
            .unwrap_or(Path::new("./"));
        // Set startdir to current dir as relative path,
        // if startdir is only a single file name.
        if startdir == Path::new("") {
            startdir = Path::new(".");
        }

        if single {
            return Ok(session)
        }

        // TODO 220812 REGRESSION CHECK: DONE. walkdir loop 4-5x slower with new code
        // Do not recurse, only look in parent dir of 'mp4_path'
        for file in walkdir::WalkDir::new(startdir).max_depth(1) {
            let path = match file {
                Ok(f) => f.path().to_owned(),
                Err(_) => { // usually err due to missing user permissions (sys dirs etc)
                    // println!("[SKIP]     Skipping path: {}", e);
                    continue;
                }
            };

            // fails on some paths, e.g. ./FILE1.TXT != FILE1.TXT
            // if path == mp4_path {
            // below should work since only considering current dir, not recursive dirwalk
            if path.file_name() == mp4_path.file_name() {
                continue;
            }

            // Do not parse at this stage, since most files may be discarded anyway
            match GoProFile::new(&path, false, debug) {
                Ok(gp) => if let (Some(id1), Some(id2)) = (file_id.as_ref(), gp.file_id.as_ref()) {
                    if id1 == id2 {
                        session.push(&gp)
                    }
                },
                Err(_) => continue
            }
        }

        session.sort();
        if parse {
            session.parse()?;
        }

        Ok(session)
    }

    /// Sort contained `VirbFile`s on sequence (chronologically)
    /// for concatenating GPMF streams or clips to single video.
    pub fn sort(&mut self) {
        self.0.sort_by_key(|gp| gp.sequence)
    }

    /// Parses GPMF-data for contained `GoProFile`s
    /// if not done.
    pub fn parse(&mut self) -> Result<(), GpmfError> {
        for gopro in self.iter_mut() {
            if !gopro.parsed {
                // gopro.meta = Some(Udta::new(&mut File::open(&gopro.mp4_path)?, false)?);
                gopro.gpmf = Gpmf::new(&gopro.mp4_path)?;
                gopro.parsed = true;
            }
        }
        Ok(())
    }
}