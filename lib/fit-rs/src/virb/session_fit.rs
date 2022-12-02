use std::{ops::Range, path::{PathBuf, Path}};

use time::{Duration, PrimitiveDateTime};

use crate::{Fit, FitError, CameraEvent};

/// VIRB recording session UUIDs and index range
/// in `Fit.records`.
#[derive(Debug, Default, Clone)]
pub struct FitSession {
    /// Path to FIT-file.
    pub path: PathBuf,
    /// FIT data
    pub fit: Option<Fit>,
    /// UUIDs in VIRB MP4-files
    /// for this session.
    pub uuid: Vec<String>,
    /// Slice start index in corresponding `Fit.records`
    /// for this session.
    pub start: usize,
    /// Slice end index in corresponding `Fit.records`
    /// for this session.
    pub end: usize,
}

impl FitSession {
    /// Returns session slice range for `Fit.records`.
    pub fn range(&self) -> Range<usize> {
        self.start .. self.end
    }

    /// Returns number of UUIDs,
    /// which corresponds to the number of video clips
    /// for this session.
    pub fn len(&self) -> usize {
        self.uuid.len()
    }

    /// Iterate UUIDs in session.
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.uuid.iter()
    }

    pub fn parse(&mut self) -> Result<(), FitError> {
        self.fit = Some(Fit::new(&self.path)?);
        Ok(())
    }

    /// Derives FIT indeces for session, if session
    /// already has UUIDs set.
    pub fn derive(&mut self) -> Result<(), FitError> {
        self.parse()?;

        if let Some(fit) = &self.fit {
            // Get all camera events.
            let cam = fit.camera(None)?;
            
            for evt in cam.iter() {
                // Match event types.
                // Logged chronologically so
                // the very first encountered event should
                // always be 0 and never e.g. 2 (session end).

                if self.uuid.contains(&evt.camera_file_uuid) {
                    match evt.camera_event_type {
                        // 0 = recording session start
                        0 => self.start = evt.index,
                        // 2 = recording session end
                        2 => self.end = evt.index,
                        // Ignore alternative camera events for session start/end
                        _ => (),
                    }
                }
            }
        }

        Ok(())
    }

    /// Returns start, end timestamps for session as tuple `(start, end)`,
    /// where `start`, `end` represent time offsets from FIT-file start.
    /// 
    /// Note that deriving duration via `end - start` may differ slightly
    /// from the duration for the corresponding MP4-file/s.
    pub fn duration(&self) -> Option<(Duration, Duration)> {
        if let Some(fit) = &self.fit {

            // Find first camera_event/161 in session index range
            let start_msg = fit.records[self.range()].iter()
                .find(|rec| rec.global == 161)
                .map(|rec| CameraEvent::new(rec));

            // Find last camera_event/161 in session index range
            let end_msg = fit.records[self.range()].iter().rev()
                .find(|rec| rec.global == 161)
                .map(|rec| CameraEvent::new(rec));

            // Convert camera_events to duration
            if let (Some(Ok(start)), Some(Ok(end))) = (start_msg, end_msg) {
                return Some((start.to_duration(), end.to_duration()))
            }

        }
        
        None
    }

    /// Derives start, end date time for recording as tuple `(start, end)`.
    /// 
    /// `default_on_error` sets start time for FIT-file to FIT base start time
    /// 1989-12-31T00:00:00.000 if no timestamp correlation value (FIT global 162)
    /// can be found.
    pub fn datetime(&self, offset_hrs: Option<i64>, default_on_error: bool) -> Result<(PrimitiveDateTime, PrimitiveDateTime), FitError> {
        if let Some(fit) = &self.fit {
            let t0 = fit.t0(offset_hrs.unwrap_or(0), default_on_error)?;

            let (start_dur, end_dur) = self.duration().ok_or_else(|| FitError::NoSuchSession)?;
            return Ok((t0 + start_dur, t0 + end_dur))
        }

        Err(FitError::NoData)
    }
}

// TODO perhaps add fit: Option<Fit> to FitSessions rather than single session?
#[derive(Debug, Default)]
pub struct FitSessions{
    path: PathBuf,
    fit: Option<Fit>,
    filtered: Option<u16>,
    sessions: Vec<FitSession>
}

impl FitSessions {
    /// Derive all sessions in specified FIT-file.
    pub fn new(path: &Path) -> Result<Self, FitError> {
        let mut sessions = FitSessions::default();
        sessions.path = path.to_owned();
        sessions.derive(false)?;

        Ok(sessions)
    }

    pub fn from_fit(fit: &Fit) -> Result<Self, FitError> {
        let mut sessions = FitSessions::default();
        sessions.path = fit.path.to_owned();
        sessions.fit = Some(fit.to_owned());
        sessions.derive(false)?;

        Ok(sessions)
    }

    /// Returns `FitSession` that contains specified UUID.
    pub fn find(&self, uuid: &str) -> Option<&FitSession> {
        self.sessions.iter().find(|s| s.uuid.contains(&uuid.to_owned()))
    }

    /// Parse of linked FIT-file.
    /// Optionally limit to specific message type via FIT global ID,
    /// see Profile.xlsx in FIT SDK.
    pub fn parse(&mut self, global: Option<u16>) -> Result<(), FitError> {
        self.fit = match global {
            Some(_) => {
                self.filtered = global;
                Some(Fit::parse(&self.path, global)?)
            },
            None => {
                self.filtered = None;
                Some(Fit::new(&self.path)?)
            }
        };
        Ok(())
    }

    /// VIRB only.
    /// Derive start/end indeces in `FIT.records`
    /// for all recording sessions. Use `FitSession::range()`
    /// to get range for specific recording session
    /// for other `FIT` methods.
    pub fn derive(&mut self, full_parse: bool) -> Result<(), FitError> {
        // Only extract camera events at this point
        // for performance reasons if FIT not parsed already.
        if self.fit.is_none() {
            match full_parse {
                true => self.parse(None)?,
                false => self.parse(Some(161))?
            }
        }

        let mut sessions: Vec<FitSession> = Vec::new();
        let mut session = FitSession::default();

        if let Some(fit) = &self.fit {

            // Get all camera events.
            let cam = fit.camera(None)?;
            
            for evt in cam.iter() {
                // Match event types.
                // Logged chronologically so
                // the very first encountered event should
                // always be 0 and never e.g. 2 (session end).
                match evt.camera_event_type {
                    // 0 = recording session start
                    0 => {
                        session.path = self.path.to_owned();
                        session.start = evt.index;
                        session.uuid.push(evt.camera_file_uuid.to_owned());
                    },
                    // 2 = recording session end
                    2 => {
                        session.end = evt.index;
                        session.uuid.dedup(); // works without sort since logged chronologically
                        sessions.push(session);
                        session = FitSession::default();
                    },
                    // Ignore alternative camera events for session start/end
                    4 | 6 => (),
                    // Push UUID in between event types 0 and 2
                    // Duplicate UUIDs will always sit next to each other.
                    _ => session.uuid.push(evt.camera_file_uuid.to_owned())
                }
            }
        }

        self.sessions = sessions;

        Ok(())
    }

    pub fn range(&self, uuid: &str) -> Option<Range<usize>> {
        self.find(&uuid.to_owned())
            .map(|session| session.range())
    }

    pub fn fit(&self) -> Option<&Fit> {
        self.fit.as_ref()
    }

    pub fn sessions(&self) -> &[FitSession] {
        &self.sessions
    }

    pub fn len(&self) -> usize {
        self.sessions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    pub fn into_iter(self) -> impl Iterator<Item = FitSession> {
        self.sessions.into_iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = &FitSession> {
        self.sessions.iter()
    }
    
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut FitSession> {
        self.sessions.iter_mut()
    }
}