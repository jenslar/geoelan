//! Custom user data atom (`udta`). Fields are in raw form as `Cursor<Vec<u8>>`.

use std::io::Cursor;

use crate::fourcc::FourCC;

/// Custom user data atom. Optional.
/// Field content differ between recording devices or encoders.
#[derive(Debug)]
pub struct Udta {
    pub fields: Vec<UdtaField>
}

/// Custom user data (`udta`) field.
/// Data content types differ between recording devices or encoders.
/// E.g. GoPro cameras mix "normal" MP4 atom structure with
/// device information embedded as GPMF.
#[derive(Debug, Clone)]
pub struct UdtaField{
    /// Four CC
    pub name: FourCC,
    /// Totalt size in bytes
    pub size: u32,
    /// Data, excluding 8 byte header
    pub data: Cursor<Vec<u8>>
    // pub data: Vec<u8>
}
