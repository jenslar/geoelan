// use crate::{fourcc::FourCC, strm::Strm};
use crate::{FourCC, Stream};

// [DEVC: 24 | DVID: 1 | DVNM: HERO9 Black | SIZE: 2172 | DUR: Some(400)] | TS: Some(23000) | 392/408] STRM SIZE: 532, NAME: Some("Accelerometer")
//   STMP J 8 1
//     Uint64([23058550])
//   TSMP L 4 1
//     Uint32([4682])
//   ORIN c 3 1
//     Ascii("ZXY")
//   SIUN c 4 1
//     Ascii("m/s²")
//   SCAL s 2 1
//     Sint16([417])
//   TMPC f 4 1
//     Float32([33.564453])
//   ACCL s 6 84
//     Sint16([2484, 146, 3413])
//     ...
//     Sint16([2607, 208, 3384])

/// Acceleration vector, axis order (e.g. xyz or zxy etc) varies and depends
/// on orientation value (`ORIN`) in accelerometer stream.
pub struct Acceleration {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

// hero5+6 have no orientation messages at all? if none, what's the default? xyz?
// hero6a+'6+ble'+7+8+max has ORIN + ORIO + MTRX (seems similar to fit orientation matrix)?
// hero9 only has ORIN?
pub struct Accelerometer {
    pub orientation: String,             // ORIN
    pub temperature: f32,                // TMPC device temperature C, not ambient
    pub acceleration: Vec<Acceleration>, // ACCL populated in the order specified by orientation
    pub timestamp: Option<u32>,          // derived from DEVC parent struct, for MP4-files only
    pub duration: Option<u32>,           // derived from DEVC parent struct, for MP4-files only
}

impl Accelerometer {
    pub fn new(stream: &Stream) {
        let orin: Option<String> = stream
            .find(&FourCC::ORIN)
                .and_then(|s| s.first_value())
                .and_then(|b| b.into());

        // let scal: Option<i16> = stream
        //     .find(&FourCC::SCAL)
        //     .and_then(|s| s.first_value())
        //     .and_then(|b| b.into());
    }
}
