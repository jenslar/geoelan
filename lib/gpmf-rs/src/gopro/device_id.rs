//! GoPro device ID (DVID).

use crate::FourCC;

/// For older devices (Hero5, Fusion?) it seems
/// DVID can be either a u32 or a FourCC? Or just parse as BaseType...?
#[derive(Debug, Clone)]
pub enum Dvid {
    Uint32(u32),
    FourCC(FourCC),
}

impl Into<Option<u32>> for &Dvid {
    fn into(self) -> Option<u32> {
        match self {
            Dvid::Uint32(n) => Some(*n),
            Dvid::FourCC(_) => None,
        }
    }
}

impl Into<Option<FourCC>> for &Dvid {
    fn into(self) -> Option<FourCC> {
        match self {
            Dvid::Uint32(_) => None,
            Dvid::FourCC(f) => Some(f.to_owned()),
        }
    }
}

impl Into<Option<String>> for &Dvid {
    fn into(self) -> Option<String> {
        match self {
            Dvid::Uint32(_) => None,
            Dvid::FourCC(f) => Some(f.to_str().to_owned()),
        }
    }
}
