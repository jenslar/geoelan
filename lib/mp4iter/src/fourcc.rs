//! MP4 atom FourCC.
//! See https://developer.apple.com/library/archive/documentation/QuickTime/QTFF/QTFFChap2/qtff2.html#//apple_ref/doc/uid/TP40000939-CH204-56313.
//! Some non-standard Four CC listed, stemming from e.g. GoPro MP4-files.

/// MP4 atom Four CC.
/// See https://developer.apple.com/library/archive/documentation/QuickTime/QTFF/QTFFChap2/qtff2.html#//apple_ref/doc/uid/TP40000939-CH204-56313.
#[derive(Debug, Clone, PartialEq)]
pub enum FourCC {
    /// Data Information Atoms
    Dinf,
    Dref,
    Edts,
    Elst,
    Ftyp,
    Gmhd,
    Hdlr,
    Iods,
    Mdat,
    Mdhd,
    Mdia,
    Minf,
    /// Movie Atom
    Moov,
    /// Movie Header Atom
    Mvhd,
    Smhd,
    Stbl,
    Stco,
    Stsc,
    Stsd,
    Stss,
    Stsz,
    Stts,
    Tkhd,
    Tmcd,
    Trak,
    Tref,
    Udta,
    Vmhd,

    // // Device specific Four CC (not in Apple's documentation)
    // // Model dependent.
    // /// GPMF: GoPro GPMF data atom (undocumented)
    // Gpmf,
    // /// FIRM: GoPro firmware version
    // Firm,
    // /// LENS: GoPro lens serial number
    // Lens,
    // /// CAME: GoPro camera (serial?)
    // Came,
    // /// SETT: GoPro ?
    // Sett,
    // /// MUID: GoPro Media unique identifier, does not specify clip sequence
    // Muid,
    // /// HMMT: GoPro ?
    // Hmmt,

    Custom(String)
}

impl FourCC {
    pub fn from_bytes(fourcc: &[u8]) -> Self {
        match fourcc {
            b"dinf" => Self::Dinf,
            b"dref" => Self::Dref,
            b"edts" => Self::Edts,
            b"elst" => Self::Elst,
            b"ftyp" => Self::Ftyp,
            b"gmhd" => Self::Gmhd,
            b"hdlr" => Self::Hdlr,
            b"iods" => Self::Iods,
            b"mdat" => Self::Mdat,
            b"mdhd" => Self::Mdhd,
            b"mdia" => Self::Mdia,
            b"minf" => Self::Minf,
            b"moov" => Self::Moov,
            b"mvhd" => Self::Mvhd,
            b"smhd" => Self::Smhd,
            b"stbl" => Self::Stbl,
            b"stco" => Self::Stco,
            b"stsc" => Self::Stsc,
            b"stsd" => Self::Stsd,
            b"stss" => Self::Stss,
            b"stsz" => Self::Stsz,
            b"stts" => Self::Stts,
            b"tkhd" => Self::Tkhd,
            b"tmcd" => Self::Tmcd,
            b"trak" => Self::Trak,
            b"tref" => Self::Tref,
            b"udta" => Self::Udta,
            b"vmhd" => Self::Vmhd,
            // b"GPMF" => Self::Gpmf, // capitals in file
            _ => Self::Custom(String::from_utf8_lossy(fourcc).to_string()),
        }
    }

    pub fn from_str(fourcc: &str) -> Self {
        match fourcc {
            "dinf" => Self::Dinf,
            "dref" => Self::Dref,
            "edts" => Self::Edts,
            "elst" => Self::Elst,
            "ftyp" => Self::Ftyp,
            "gmhd" => Self::Gmhd,
            "hdlr" => Self::Hdlr,
            "iods" => Self::Iods,
            "mdat" => Self::Mdat,
            "mdhd" => Self::Mdhd,
            "mdia" => Self::Mdia,
            "minf" => Self::Minf,
            "moov" => Self::Moov,
            "mvhd" => Self::Mvhd,
            "smhd" => Self::Smhd,
            "stbl" => Self::Stbl,
            "stco" => Self::Stco,
            "stsc" => Self::Stsc,
            "stsd" => Self::Stsd,
            "stss" => Self::Stss,
            "stsz" => Self::Stsz,
            "stts" => Self::Stts,
            "tkhd" => Self::Tkhd,
            "tmcd" => Self::Tmcd,
            "trak" => Self::Trak,
            "tref" => Self::Tref,
            "udta" => Self::Udta,
            "vmhd" => Self::Vmhd,
            // "GPMF" => Self::Gpmf, // capitals in file
            _ => Self::Custom(fourcc.to_owned()),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Dinf => "dinf",
            Self::Dref => "dref",
            Self::Edts => "edts",
            Self::Elst => "elst",
            Self::Ftyp => "ftyp",
            Self::Gmhd => "gmhd",
            Self::Hdlr => "hdlr",
            Self::Iods => "iods",
            Self::Mdat => "mdat",
            Self::Mdhd => "mdhd",
            Self::Mdia => "mdia",
            Self::Minf => "minf",
            Self::Moov => "moov",
            Self::Mvhd => "mvhd",
            Self::Smhd => "smhd",
            Self::Stbl => "stbl",
            Self::Stco => "stco",
            Self::Stsc => "stsc",
            Self::Stsd => "stsd",
            Self::Stss => "stss",
            Self::Stsz => "stsz",
            Self::Stts => "stts",
            Self::Tkhd => "tkhd",
            Self::Tmcd => "tmcd",
            Self::Trak => "trak",
            Self::Tref => "tref",
            Self::Udta => "udta",
            Self::Vmhd => "vmhd",
            // Self::Gpmf => "GPMF", // capitals in file
            Self::Custom(s) => s.as_str()
        }
    }
}
// dinf
// dref
// edts
// elst
// ftyp
// gmhd
// hdlr
// iods
// mdat
// mdhd
// mdia
// minf
// moov
// mvhd
// smhd
// stbl
// stco
// stsc
// stsd
// stss
// stsz
// stts
// tkhd
// tmcd
// trak
// tref
// udta
// vmhd