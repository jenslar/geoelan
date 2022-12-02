use std::io::Cursor;

use crate::tag::JpegTag;

/// JPEG segment
#[derive(Debug)]
pub struct Segment {
    pub tag: JpegTag,
    /// Data, excluding tag (2 bytes)
    /// and size (2 bytes)
    pub data: Cursor<Vec<u8>>
}

impl Segment {
    /// End of file.
    pub fn eof() -> Self {
        Self{
            tag: JpegTag::EOI,
            data: Cursor::default()
        }
    }
}
