use std::{path::{PathBuf, Path}, ops::Range, collections::HashMap};

use time::{PrimitiveDateTime, Duration};
use walkdir::WalkDir;

use crate::{FitError, files::match_extension, Fit, GpsMetadata};
use super::{
    {FitSession, FitSessions},
    VirbFile,
    VirbMeta
};

#[derive(Debug, Clone, Default)]
pub struct VirbSession{
    // pub session: Vec<VirbFile>,
    // pub fit: PathBuf,
    pub virb: Vec<VirbFile>,
    pub fit: FitSession,
    /// Date time for first message in fit.
    /// Note that this does not represent
    /// start of session.
    pub t0: Option<PrimitiveDateTime>,
    /// Date time for start of session.
    pub start: Option<Duration>,
    /// Date time for end of session.
    pub end: Option<Duration>,
    processed: bool // defaults to false
}

impl VirbSession {
    /// Determine recording sessions in `dir` (recursive).
    /// Does not parse FIT-files if located, only paths are set.
    pub fn sessions_from_path(dir: &Path, verbose: bool) -> Vec<VirbSession> {
        let mut uuid2virbfile: HashMap<String, VirbFile> = HashMap::new();
        // let mut fit2sessions: HashMap<PathBuf, FitSessions> = HashMap::new();
        let mut fit_sessions_vec: Vec<FitSessions> = Vec::new();

        let mut count = 0;

        // 1. Go through files, must complete before sessions can be derived
        for result in WalkDir::new(dir) {
            let path = match result {
                Ok(f) => f.path().to_owned(),
                Err(_) => continue
            };

            // Ignoring errors for now
            if match_extension(&path, "mp4") {
                if let Ok(vf) = VirbFile::new(&path, None) {
                    if verbose {
                        count += 1;
                        println!("{:4} [MP4] {}", count, path.display());
                    }
                    uuid2virbfile.entry(vf.uuid.to_owned())
                        .or_insert(vf).mp4 = Some(path.to_owned()); // sets path twice...
                }
            } else if match_extension(&path, "glv") {
                if let Ok(vf) = VirbFile::new(&path, None) {
                    if verbose {
                        count += 1;
                        println!("{:4} [GLV] {}", count, path.display());
                    }
                    uuid2virbfile.entry(vf.uuid.to_owned())
                        .or_insert(vf).glv = Some(path.to_owned()); // sets path twice...
                }
            } else if match_extension(&path, "fit") {
                if let Ok(fit_sessions) = FitSessions::new(&path) {
                    if verbose {
                        count += 1;
                        println!("{:4} [FIT] {} ({})", count, path.display(), fit_sessions.len());
                    }
                    for fit_session in fit_sessions.iter() {
                        fit_session.iter()
                            .for_each(|u| {
                                if let Ok(vf) = VirbFile::new(&fit_session.path, Some(u)) {
                                    uuid2virbfile.entry(u.to_owned())
                                        .or_insert(vf).fit = Some(fit_session.path.to_owned())
                                }
                            });
                    }
                    fit_sessions_vec.push(fit_sessions);
                }
            }
        }

        // 2. Match and group files into VirbSessions via FitSession UUIDs.
        let mut virb_sessions: Vec<VirbSession> = Vec::new();
        
        for fit_sessions in fit_sessions_vec.iter() {
            
            for fit_session in fit_sessions.iter() {

                let mut virb_session = VirbSession::default();

                let virb: Vec<VirbFile> = fit_session.iter()
                    .filter_map(|uuid| uuid2virbfile.get(uuid))
                    .cloned()
                    .collect();

                virb_session.virb = virb;
                virb_session.fit = fit_session.to_owned();

                // Ensures a FIT-file with at least one corresponding video file was found (.GLV or .MP4)
                if virb_session.matched() {
                    virb_sessions.push(virb_session);
                }
                
            }
        }

        virb_sessions
    }

    /// Returns `true` if all files in session
    /// have a FIT-file and at least one corresponding video file.
    fn matched(&self) -> bool {
        if self.virb.is_empty() {
            return false
        }

        let matched = self.virb.iter()
            .map(|vf| vf.matched())
            .collect::<Vec<_>>();
        
        if matched.is_empty() {
            false
        } else if matched.iter().any(|m| m == &false) {
            false
        } else {
            true
        }
    }

    /// Derives all files for the recording session `mp4_path`
    /// is part of. `dir` is used as starting
    /// point to search recursively for matches.
    pub fn from_mp4(mp4_path: &Path, dir: &Path, verbose: bool) -> Option<Self> {
        let virbfile = VirbFile::new(mp4_path, None).ok()?;

        Self::from_uuid(&virbfile.uuid, dir, verbose)
    }

    /// Derives all files for the recording session that contains
    /// `uuid`. `dir` is used as starting
    /// point to search recursively for matches.
    pub fn from_uuid(uuid: &str, dir: &Path, verbose: bool) -> Option<Self> {
        let virbsessions = VirbSession::sessions_from_path(dir, verbose);
        for virbsession in virbsessions.iter() {
            if virbsession.contains(&uuid) {
                return Some(virbsession.to_owned())
            }
        }

        None
    }

    pub fn get_from_index(sessions: &[Self], index: usize, offset_hours: Option<i64>) -> Result<Self, FitError> {
        if let Some(session) = sessions.get(index) {
            let mut session = session.to_owned();
            if !session.processed {
                session.process(offset_hours.unwrap_or(0))?;
            }
            return Ok(session)
        }
        Err(FitError::NoSuchSession)
    }

    pub fn get_from_uuid(sessions: &[Self], uuid: &str, offset_hours: Option<i64>) -> Result<Self, FitError> {
        // .contains() does not accept &str, only &String, see: https://github.com/rust-lang/rust/issues/42671
        // .find(|session| session.uuid().contains(uuid))
        if let Some(session) = sessions.iter().find(|session| session.uuid().iter().any(|u| u == uuid)) {
            let mut session = session.to_owned();
            if !session.processed {
                session.process(offset_hours.unwrap_or(0))?;
            }
            return Ok(session)
        }
        Err(FitError::NoSuchSession)
    }

    pub fn meta(&self) -> Vec<VirbMeta> {
        self.virb.iter()
            .filter_map(|v| v.meta().ok())
            .collect()
    }

    /// Returns number of video clips in session.
    pub fn len(&self) -> usize {
        self.virb.len()
    }

    /// Returns `true` is session contains no clips.
    pub fn is_empty(&self) -> bool {
        self.virb.is_empty()
    }

    pub fn contains(&self, uuid: &str) -> bool {
        self.virb.iter()
            .any(|virbfile| virbfile.uuid == uuid)
    }

    /// Returns FIT-data if set.
    /// Run `VirbSession::process()` if `None` is returned.
    pub fn fit(&self) -> Option<&Fit> {
        self.fit.fit.as_ref()
    }

    pub fn mp4(&self) -> Vec<PathBuf> {
        self.virb.iter()
            .filter_map(|v| v.mp4().map(|p| p.to_path_buf()))
            .collect()
    }

    pub fn glv(&self) -> Vec<PathBuf> {
        self.virb.iter()
            .filter_map(|v| v.glv().map(|p| p.to_path_buf()))
            .collect()
    }

    /// Returns `Vec<GpsMetadata>` filtered on session timespan.
    pub fn gps(&self) -> Result<Vec<GpsMetadata>, FitError> {
        if let Some(fit) = self.fit.fit.as_ref() {
            return fit.gps(Some(&self.range()))
        }
        Err(FitError::NoData)
    }

    /// Returns indeces corresponding to session FIT data slice.
    /// Generated from camera event start end event types, where
    /// 0 = session start, and 2 = session end.
    /// See FIT SDK Profile.xlsx.
    pub fn range(&self) -> Range<usize> {
        self.fit.range()
    }

    /// Parse FIT-file and assign to `self.data`.
    fn parse(&mut self) -> Result<(), FitError> {
        // self.data = Some(Fit::new(&self.fit.path)?);
        self.fit.parse()?;
        Ok(())
    }

    fn set_time(&mut self, offset_hours: i64) -> Result<(), FitError> {
        if let Some(fit) = &self.fit.fit {
            self.t0 = Some(fit.t0(offset_hours, false)?);

            let cam = fit.camera(Some(&self.range()))?;

            // Fit::sessions() generates start/end index range based on camera event type
            // with 0 = session start, 2 = session end.
            // VirbSession::range() reflects this, so first()/last() should be ok to use.
            self.start = cam.first()
                .map(|evt| {
                    Duration::seconds(evt.timestamp as i64)
                    + Duration::milliseconds(evt.timestamp_ms as i64)
                });
            self.end = cam.last()
                .map(|evt| {
                    Duration::seconds(evt.timestamp as i64)
                    + Duration::milliseconds(evt.timestamp_ms as i64)
                });
        }

        Ok(())
    }

    /// Parses and processes matched FIT-file, then populates data, and time fields.
    pub fn process(&mut self, offset_hours: i64) -> Result<(), FitError> {
        self.fit.derive()?;
        self.set_time(offset_hours)?;

        Ok(())
    }

    /// Returns duration derived from summed video duration.
    pub fn video_duration(&self) -> Option<time::Duration> {
        // if let (Some(v1), Some(v2)) = (self.virb.first(), self.virb.last()) {
        //     if self.virb.len() == 1 {
        //         return v1.duration()
        //     } else {
        //         return Some(v2.duration()? - v1.duration()?)
        //     }
        // }
        if self.virb.len() == 1 {
            self.virb.first().and_then(|s| s.duration())
        } else {
            Some(self.virb.iter()
                .filter_map(|s| s.duration()) // not great for error reporting...
                .fold(time::Duration::ZERO, |acc, dur| acc + dur))
            // self.virb.iter()
            //     .map(|s| s.duration()?)
            //     .sum()
        }

        // None
    }

    /// Returns duration derived from FIT camera events.
    /// May differ slightly from video duration.
    pub fn fit_duration(&self) -> Option<time::Duration> {
        if let (Some(start), Some(end)) = (self.start, self.end) {
            return Some(end - start)
        }

        None
    }

    /// Returns all unique UUIDs in recording session
    /// in chronological order.
    pub fn uuid(&self) -> Vec<String> {
        self.virb.iter()
            .map(|v| v.uuid.to_owned())
            .collect()
    }

    /// Extracts UUID from linked FIT-file if set.
    pub fn uuid_from_fit(&self) -> Result<Vec<String>, FitError> {
        if let Some(data) = &self.fit.fit {
            let cam = data.camera(Some(&self.range()))?;
            let mut uuids: Vec<_> = cam.iter()
                .map(|evt| evt.camera_file_uuid.to_owned())
                .collect();
            uuids.dedup();

            Ok(uuids)
        } else {
            Ok(Vec::new())
        }
    }
}