use fit::{
    structs::FitFile,
    errors::FitError
};
use std::collections::HashMap;
use std::path::PathBuf;

// VIRB related struct, used by cam2eaf, eaf2geo
#[derive(Debug)]
pub struct SessionTimeSpan {
    pub start: chrono::Duration,
    pub end: chrono::Duration,
    pub uuid: Vec<String>,
}

// VIRB related struct, used by match, cam2eaf
#[derive(Debug, Clone, PartialEq)]
pub enum VirbFileType {
    FIT, // fit file, contains multiple uuids
    MP4, // hi-res video, contains single uuid
    GLV, // lo-res video, contains single uuid
}

// VIRB related struct, used by match, cam2eaf
#[derive(Debug, Clone)]
pub struct VirbFile {
    pub filetype: VirbFileType,
    pub path: PathBuf,
    pub uuid: Option<Vec<String>>,          // unique uuids
    pub sessions: Option<Vec<Vec<String>>>, // unique uuids
}

// VIRB related struct, used by match, cam2eaf
impl VirbFile {
    /// Creates new VirbFile struct, sets VirbFileType,
    /// and extracts uuid if present
    pub fn new(path: &PathBuf) -> Option<VirbFile> {
        let ext = path.extension()?.to_str()?;
        let mut uuid: Option<Vec<String>> = None;
        let mut sessions: Option<Vec<Vec<String>>> = None;
        let mut filetype: Option<VirbFileType> = None;
        match ext.to_lowercase().as_ref() {
            "mp4" => {
                uuid = match fit::get_video_uuid(&path) {
                    // no ok() since need to vec!(data)
                    Ok(data) => Some(vec![data?]), // means mp4 without uuid not listed at all, but faster
                    Err(_) => None,
                };
                filetype = Some(VirbFileType::MP4);
            }
            "glv" => {
                uuid = match fit::get_video_uuid(&path) {
                    // no ok() since need to vec!(data)
                    Ok(data) => Some(vec![data?]), // means mp4 without uuid not listed at all, but faster
                    Err(_) => None,
                };
                filetype = Some(VirbFileType::GLV);
            }
            "fit" => {
                let fitdata = match FitFile::new(&path.to_owned()).parse(&Some(161_u16), &None) {
                    Ok(d) => d,
                    Err(e) => {
                        match e {
                            FitError::Fatal(_) => return None,
                            FitError::Partial(_, d) => d // want partial reads as well
                        }
                    }
                };
                uuid = fitdata.uuid();
                sessions = fitdata.sessions();
                filetype = Some(VirbFileType::FIT);
            }
            _ => return None,
        }
        Some(VirbFile {
            path: path.to_owned(),
            filetype: filetype?,
            uuid,
            sessions,
        })
    }

    pub fn type_to_str(&self) -> &str {
        match self.filetype {
            VirbFileType::GLV => "GLV",
            VirbFileType::MP4 => "MP4",
            VirbFileType::FIT => "FIT",
        }
    }

    pub fn is_glv(&self) -> bool {
        matches!(&self.filetype, VirbFileType::GLV)
    }
    pub fn is_mp4(&self) -> bool {
        matches!(&self.filetype, VirbFileType::MP4)
    }
    pub fn is_fit(&self) -> bool {
        matches!(&self.filetype, VirbFileType::FIT)
    }
}

// VIRB related struct, used by match, cam2eaf
#[derive(Debug, Clone)]
pub struct VirbFiles {
    pub uuid: HashMap<String, Vec<VirbFile>>, // k: unique uuid, v: files containing k
    pub session: HashMap<String, Vec<String>>, // k: 1st uuid in session, v: uuid for entire session
    pub filetypes: HashMap<String, usize>,    // stats for glv/mp4/fit
}

// VIRB related struct, used by cam2eaf. Will eventually be checked by eaf2geo.
// Generic metadata describing video session + corresponding FIT-file
pub struct FitMetaData {
    pub uuid: Vec<String>,
    pub sha256: String,
    pub file: String,
    pub size: u64,
    pub t0: chrono::DateTime<chrono::Utc>,
    pub start: chrono::Duration,
    pub end: chrono::Duration,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub latitude: f64,  // degrees
    pub longitude: f64, // degrees
    pub altitude: f64,  // meters
    pub heading: f32,   // degrees
    pub velocity: f32,  // m/s (3d vector in fit, but currently just vector sum/scalar)
    pub speed: f32,     // m/s
    pub time: chrono::Duration,
    pub text: Option<String>, // description
}

// mainly for generating kml to be able to switch between polyline and points
pub enum GeoType {
    POINT(Vec<Point>),
    LINE(Vec<Vec<Point>>),
}
