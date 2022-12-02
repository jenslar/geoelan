use std::path::{Path, PathBuf};

use mp4iter::{
    self,
    Mp4,
    UdtaField,
};

use crate::{Stream, GpmfError};

#[derive(Debug, Default)]
pub struct GoProMeta {
    pub path: PathBuf,
    pub udta: Vec<UdtaField>,
    pub gpmf: Vec<Stream>
}

impl GoProMeta {
    /// Extract custom GoPro metadata from MP4 `udta` atom.
    /// Mix of "normal" MP4 atom structures and GPMF-stream.
    pub fn new(path: &Path) -> Result<Self, GpmfError> {
        let mut mp4 = Mp4::new(path)?;
        let mut udta = mp4.udta()?;

        let mut meta = Self::default();
        meta.path = path.to_owned();

        let fourcc_gpmf = mp4iter::FourCC::Custom(String::from("GPMF"));

        for field in udta.fields.iter_mut() {
            if fourcc_gpmf == field.name {
                meta.gpmf.extend(Stream::new(&mut field.data, None)?)
            } else {
                meta.udta.push(field.to_owned())
            }
        }

        Ok(meta)
    }
}