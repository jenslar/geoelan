//! EAF header.

use std::path::{PathBuf, Path};

use serde::{Serialize, Deserialize};

use super::{
    Property,
    MediaDescriptor
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// EAF header.
/// 
/// Note: ETF not yet implemented. For ELAN template files (ETF) the header contains no media descriptiors or properties.
pub struct Header {
    #[serde(rename = "MEDIA_FILE", default)]
    /// Name or path of a media file, optional. Deprecated and ignored by ELAN.
    pub media_file: String,
    /// Milliseconds or NTSC-frames or PAL-frames, optional,
    /// default is milliseconds. ELAN only supports (and assumes) milliseconds.
    pub time_units: String,
    /// Linked media files. Seemingly optional,
    /// since ELAN opens EAF-file with no linked media.
    #[serde(default)]
    pub media_descriptor: Vec<MediaDescriptor>,
    #[serde(rename = "PROPERTY", default)]
    pub properties: Vec<Property>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            media_file: "".to_owned(),
            time_units: "milliseconds".to_owned(),
            media_descriptor: Vec::new(),
            properties: Vec::new(),
        }
    }
}

impl Header {
    pub fn new(paths: &[PathBuf]) -> Self {
        let mut hdr = Self::default();

        for path in paths.iter() {
            hdr.media_descriptor.push(MediaDescriptor::new(path, None));
        }

        hdr
    }

    /// Adds a new media descriptor to the header.
    pub fn add_media(&mut self, path: &Path, extracted_from: Option<&str>) {
        let mdsc = MediaDescriptor::new(path, extracted_from);
        self.media_descriptor.push(mdsc)
    }

    /// Removes specified media file if set.
    /// TODO Does not work as intended: should remove media descriptior entirely if filename matches,
    /// TODO or keep media descriptor with filename only for both media urls.
    // pub fn remove_media(&mut self, path: &Path, keep_filename: bool) {
    pub fn remove_media(&mut self, path: &Path) {
        self.media_descriptor.retain(|md| !md.contains(path))
    }

    /// Removes all media paths, with the option to keep the file name only.
    /// These sometimes contain user names and information which may be unwanted
    /// when e.g. sharing data.
    /// You may have to link media in ELAN again.
    pub fn scrub_media(&mut self, keep_filename: bool) {
        match keep_filename {
            false => self.media_descriptor = Vec::new(),
            true => {
                for md in self.media_descriptor.iter_mut() {
                    if let Some(filename) = md.file_name().map(|s| s.to_string_lossy().to_string()) {
                        md.media_url = format!("file://{}", filename);
                        md.relative_media_url = Some(format!("./{}", filename));
                    }
                }
            }
        }
    }

    /// Adds a new property to the header.
    pub fn add_property(&mut self, property: &Property) {
        self.properties.push(property.to_owned())
    }

    /// Returns all media paths as string tuples,
    /// `(media_url, relative_media_url)`
    pub fn media_paths(&self) -> Vec<(String, Option<String>)> {
        self.media_descriptor.iter()
            .map(|m| (m.media_url.to_owned(), m.relative_media_url.to_owned()))
            .collect()
    }

    /// Returns all absolute media paths as strings.
    pub fn media_abs_paths(&self) -> Vec<String> {
        self.media_descriptor.iter()
            .map(|m| m.media_url.to_owned())
            .collect()
    }

    /// Returns all relative media paths (optional value) as strings.
    pub fn media_rel_paths(&self) -> Vec<String> {
        self.media_descriptor.iter()
            .filter_map(|m| m.relative_media_url.to_owned())
            .collect()
    }
}
