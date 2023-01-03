//! GoPro recording type, relating to whether e.g. the camera was set up to loop or record normally.
//! 
//! See <https://community.gopro.com/s/article/GoPro-Camera-File-Naming-Convention?language=en_US>

#[derive(Debug, Clone, PartialEq)]
pub enum RecordingType {
    Chaptered, // XXXX in GH01XXXX -> 1234 in GH011234, serves as session id
    Looping, // XX in GHXX1234 -> AA in GHAA1234, serves as session/loop id
    Unknown,
}

impl RecordingType {
    pub fn to_string(&self) -> String {
        match &self {
            Self::Chaptered => "Chaptered".to_string(),
            Self::Looping => "Looping".to_string(),
            Self::Unknown => "Unknown".to_string(),
        }
    }
}