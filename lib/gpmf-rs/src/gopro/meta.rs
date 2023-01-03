//! GoPro MP4 metadata logged in the user data atom `udta`.
//! 
//! GoPro embeds undocumented GPMF streams in the `udta` atom
//! that is also extracted.

use std::path::{Path, PathBuf};

use mp4iter::{FourCC, Mp4, UdtaField};

use crate::{Stream, GpmfError};

/// Parsed MP4 `udta` atom.
/// GoPro cameras embed an undocumented
/// GPMF stream in the `udta` atom.
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

        // MP4 FourCC, not GPMF FourCC
        let fourcc_gpmf = FourCC::from_str("GPMF");

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