use std::path::{PathBuf, Path};

use mp4iter::{Mp4, UdtaField};

use crate::FitError;

#[derive(Debug, Default)]
pub struct VirbMeta {
    pub path: PathBuf,
    pub udta: Vec<UdtaField>,
}

impl VirbMeta {
    /// Extracts custom metadata from MP4 `udta` atom.
    pub fn new(path: &Path) -> Result<Self, FitError> {
        let mut mp4 = Mp4::new(path)?;
        let mut udta = mp4.udta()?;

        let mut meta = Self::default();
        meta.path = path.to_owned();

        for field in udta.fields.iter_mut() {
            meta.udta.push(field.to_owned())
        }

        Ok(meta)
    }
}