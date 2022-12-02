// /// Jpeg 2-byte tags
// #[derive(Debug, PartialEq)]
// pub enum JpegTag {
//     /// Start of image
//     Start = 0xffd8,
//     /// End of image
//     End   = 0xffd9,
//     /// JFIF/ApplicationDefaultHeader
//     Jfif  = 0xffe0, // APP0 + // ApplicationDefaultHeader?
//     /// Exif data
//     Exif  = 0xffe1, // APP1 NOTE may be more than on exif section in eg a jpg
//     /// ???
//     Other = 0xffe2, // APP1 NOTE may be more than on exif section in eg a jpg
//     /// Quantization table
//     Quant = 0xffdb,
//     /// Start of frame
//     Frame = 0xffc0,
//     /// Define Huffman table
//     Huff  = 0xffc4,
//     /// Start of scan
//     Scan  = 0xffda,
// }

// impl JpegTag {
//     pub fn new(val: u16) -> Option<Self> {
//         match val as isize {
//             0xffd8 => Some(Self::Start),
//             0xffd9 => Some(Self::End),
//             0xffe0 => Some(Self::Jfif),
//             0xffe1 => Some(Self::Exif),
//             0xffdb => Some(Self::Quant),
//             0xffc0 => Some(Self::Frame),
//             0xffc4 => Some(Self::Huff),
//             0xffda => Some(Self::Scan),
//             0xffe2 => Some(Self::Other),
//             _ => None
//         }
//     }
// }

/// Jpeg 2-byte tags
#[derive(Debug, PartialEq)]
pub enum JpegTag {
    // HEX	MARKER	MARKER NAME	DESCRIPTION
    /// Start of Frame 0, Baseline DCT
    SOF0 = 0xFFC0,
    /// Start of Frame 1, Extended Sequential DCT
    SOF1 = 0xFFC1,
    /// Start of Frame 2, Progressive DCT
    SOF2 = 0xFFC2,
    /// Start of Frame 3, Lossless (sequential)
    SOF3 = 0xFFC3,
    /// Define Huffman Table, 
    DHT = 0xFFC4,
    /// Start of Frame 5, Differential sequential DCT
    SOF5 = 0xFFC5,
    /// Start of Frame 6, Differential progressive DCT
    SOF6 = 0xFFC6,
    /// Start of Frame 7, Differential lossless (sequential)
    SOF7 = 0xFFC7,
    /// JPEG Extensions
    JPG = 0xFFC8,
    /// Start of Frame 9, Extended sequential DCT, Arithmetic coding
    SOF9 = 0xFFC9,
    /// Start of Frame 10, Progressive DCT, Arithmetic coding
    SOF10 = 0xFFCA,
    /// Start of Frame 11, Lossless (sequential), Arithmetic coding
    SOF11 = 0xFFCB,
    /// Define Arithmetic Coding
    DAC = 0xFFCC,
    /// Start of Frame 13, Differential sequential DCT, Arithmetic coding
    SOF13 = 0xFFCD,
    /// Start of Frame 14, Differential progressive DCT, Arithmetic coding
    SOF14 = 0xFFCE,
    /// Start of Frame 15, Differential lossless (sequential), Arithmetic coding
    SOF15 = 0xFFCF,
    /// Restart Marker 0
    RST0 = 0xFFD0,
    /// Restart Marker 1
    RST1 = 0xFFD1,
    /// Restart Marker 2
    RST2 = 0xFFD2,
    /// Restart Marker 3
    RST3 = 0xFFD3,
    /// Restart Marker 4
    RST4 = 0xFFD4,
    /// Restart Marker 5
    RST5 = 0xFFD5,
    /// Restart Marker 6
    RST6 = 0xFFD6,
    /// Restart Marker 7
    RST7 = 0xFFD7,
    /// Start of Image
    SOI = 0xFFD8,
    /// End of Image
    EOI = 0xFFD9,
    /// Start of Scan
    SOS = 0xFFDA,
    /// Define Quantization Table
    DQT = 0xFFDB,
    /// Define Number of Lines, (Uncommon)
    DNL = 0xFFDC,
    /// Define Restart Interval
    DRI = 0xFFDD,
    /// Define Hierarchical Progression, (Uncommon)
    DHP = 0xFFDE,
    /// Expand Reference Component, (Uncommon)
    EXP = 0xFFDF,
    /// Application Segment 0, JFIF – JFIF JPEG image,
    /// AVI1 – Motion JPEG (MJPG)
    APP0 = 0xFFE0,
    /// Application Segment 1, EXIF Metadata, TIFF IFD format, JPEG Thumbnail (160×120), Adobe XMP
    APP1 = 0xFFE1,
    /// Application Segment 2, ICC color profile, FlashPix
    APP2 = 0xFFE2,
    /// Application Segment 3, JPS Tag for Stereoscopic JPEG images
    APP3 = 0xFFE3,
    /// Application Segment 4, Uncommon
    APP4 = 0xFFE4,
    /// Application Segment 5, Uncommon
    APP5 = 0xFFE5,
    /// Application Segment 6, NITF Lossles profile
    APP6 = 0xFFE6,
    /// Application Segment 7, Uncommon
    APP7 = 0xFFE7,
    /// Application Segment 8, Uncommon
    APP8 = 0xFFE8,
    /// Application Segment 9, Uncommon
    APP9 = 0xFFE9,
    /// Application Segment 10  PhoTags, (Uncommon), ActiveObject (multimedia messages / captions)
    APP10 = 0xFFEA,
    /// Application Segment 11, (Uncommon), HELIOS JPEG Resources (OPI Postscript)
    APP11 = 0xFFEB,
    /// Application Segment 12, Picture Info (older digicams), Photoshop Save for Web: Ducky
    APP12 = 0xFFEC,
    /// Application Segment 13, Photoshop Save As: IRB, 8BIM, IPTC
    APP13 = 0xFFED,
    /// Application Segment 14, (Uncommon)
    APP14 = 0xFFEE,
    /// Application Segment 15, (Uncommon)
    APP15 = 0xFFEF,
    /// JPEG Extension 0, JPEG Extension 0	(Uncommon)
    JPG0 = 0xFFF0,
    /// JPEG Extension 1, (Uncommon)
    JPG1 = 0xFFF1,
    /// JPEG Extension 2, (Uncommon)
    JPG2 = 0xFFF2,
    /// JPEG Extension 3, (Uncommon)
    JPG3 = 0xFFF3,
    /// JPEG Extension 4, (Uncommon)
    JPG4 = 0xFFF4,
    /// JPEG Extension 5, (Uncommon)
    JPG5 = 0xFFF5,
    /// JPEG Extension 6, (Uncommon)
    JPG6 = 0xFFF6,
    /// JPEG Extension 7, JPEG-LS	Lossless JPEG
    JPG7 = 0xFFF7,
    // /// JPEG Extension 7, JPEG-LS	Lossless JPEG
    // SOF48 = 0xFFF7, // same value as above
    /// JPEG Extension 8, JPEG-LS Extension, Lossless JPEG Extension Parameters
    JPG8 = 0xFFF8,
    // /// JPEG Extension 8    JPEG-LS Extension, Lossless JPEG Extension Parameters
    // LSE = 0xFFF8, // same value as above
    /// JPEG Extension 9	(Uncommon)
    JPG9 = 0xFFF9,
    /// JPEG Extension 10	(Uncommon)
    JPG10 = 0xFFFA,
    /// JPEG Extension 11	(Uncommon)
    JPG11 = 0xFFFB,
    /// JPEG Extension 12	(Uncommon)
    JPG12 = 0xFFFC,
    /// JPEG Extension 13	(Uncommon)
    JPG13 = 0xFFFD,
    /// Comment
    COM = 0xFFFE,
    /// Unknown
    Unknown
    // Unknown(u16)
}

impl From<u16> for JpegTag {
    fn from(val: u16) -> Self {
        match val as isize {
            0xFFC0 => Self::SOF0,
            0xFFC1 => Self::SOF1,
            0xFFC2 => Self::SOF2,
            0xFFC3 => Self::SOF3,
            0xFFC4 => Self::DHT,
            0xFFC5 => Self::SOF5,
            0xFFC6 => Self::SOF6,
            0xFFC7 => Self::SOF7,
            0xFFC8 => Self::JPG,
            0xFFC9 => Self::SOF9,
            0xFFCA => Self::SOF10,
            0xFFCB => Self::SOF11,
            0xFFCC => Self::DAC,
            0xFFCD => Self::SOF13,
            0xFFCE => Self::SOF14,
            0xFFCF => Self::SOF15,
            0xFFD0 => Self::RST0,
            0xFFD1 => Self::RST1,
            0xFFD2 => Self::RST2,
            0xFFD3 => Self::RST3,
            0xFFD4 => Self::RST4,
            0xFFD5 => Self::RST5,
            0xFFD6 => Self::RST6,
            0xFFD7 => Self::RST7,
            0xFFD8 => Self::SOI,
            0xFFD9 => Self::EOI,
            0xFFDA => Self::SOS,
            0xFFDB => Self::DQT,
            0xFFDC => Self::DNL,
            0xFFDD => Self::DRI,
            0xFFDE => Self::DHP,
            0xFFDF => Self::EXP,
            0xFFE0 => Self::APP0,
            0xFFE1 => Self::APP1,
            0xFFE2 => Self::APP2,
            0xFFE3 => Self::APP3,
            0xFFE4 => Self::APP4,
            0xFFE5 => Self::APP5,
            0xFFE6 => Self::APP6,
            0xFFE7 => Self::APP7,
            0xFFE8 => Self::APP8,
            0xFFE9 => Self::APP9,
            0xFFEA => Self::APP10,
            0xFFEB => Self::APP11,
            0xFFEC => Self::APP12,
            0xFFED => Self::APP13,
            0xFFEE => Self::APP14,
            0xFFEF => Self::APP15,
            0xFFF0 => Self::JPG0,
            0xFFF1 => Self::JPG1,
            0xFFF2 => Self::JPG2,
            0xFFF3 => Self::JPG3,
            0xFFF4 => Self::JPG4,
            0xFFF5 => Self::JPG5,
            0xFFF6 => Self::JPG6,
            0xFFF7 => Self::JPG7,
            0xFFF8 => Self::JPG8,
            0xFFF9 => Self::JPG9,
            0xFFFA => Self::JPG10,
            0xFFFB => Self::JPG11,
            0xFFFC => Self::JPG12,
            0xFFFD => Self::JPG13,
            0xFFFE => Self::COM,
            _ => Self::Unknown
            // _ => Self::Unknown(val) // would like to preserve value
        }
    }
}
