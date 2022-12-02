use std::io::Cursor;

use super::ExifTag;

pub struct Exif {
    tag: ExifTag,
    /// Data section.
    /// Excluding tag (2 bytes) and size (2 bytes) fields.
    data: Cursor<Vec<u8>>
}