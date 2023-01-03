//! UNIMPLEMENTED
//! 
//! Process gyroscope data.

// [DEVC: 24 | DVID: 1 | DVNM: HERO9 Black | SIZE: 2172 | DUR: Some(400)] | TS: Some(23000) | 393/408] STRM SIZE: 536, NAME: Some("Gyroscope")
//   STMP J 8 1
//     Uint64([23058550])
//   TSMP L 4 1
//     Uint32([4682])
//   ORIN c 3 1
//     Ascii("ZXY")
//   SIUN c 5 1
//     Ascii("rad/s")
//   SCAL s 2 1
//     Sint16([939])
//   TMPC f 4 1
//     Float32([33.564453])
//   GYRO s 6 84
//     Sint16([-34, -37, 186])
//     ...
//     Sint16([-251, 4, 136])

use crate::{Stream, FourCC};

#[derive(Debug, Default)]
struct Rotation {
    x: f64,
    y: f64,
    z: f64,
}

pub enum Orientation {
    XYZ,
    XZY,
    YZX,
    YXZ,
    ZXY,
    ZYX,
}

// impl From<&str> for Option<Orientation> {
//     fn from(ori: &str) -> Option<Self> {
//         match ori.to_lowercase().as_str() {
//             "xyz" => Some(Self::XYZ),
//             "xzy" => Some(Self::XZY),
//             "yzx" => Some(Self::YZX),
//             "yxz" => Some(Self::YXZ),
//             "zxy" => Some(Self::ZXY),
//             "zyx" => Some(Self::ZYX),
//             _ => None
//         }
//     }
// }

#[derive(Debug, Default)]
struct Gyroscope {
    orientation: String, // 3 chars specifying axis order, e.g. ZXY
    rotation: Vec<Rotation>,
    timestamp: Option<u32>,
    duration: Option<u32>,
}

impl Gyroscope {
    pub fn new(devc_stream: &Stream) -> Option<Self> {
        // Scale, should only be a single value for Gyro
        let scale = *devc_stream
            .find(&FourCC::SCAL)
            .and_then(|s| s.to_f64())?
            .first()?;

        // See https://github.com/gopro/gpmf-parser/issues/165#issuecomment-1207241564
        let orientation: String = devc_stream
            .find(&FourCC::ORIN)
            .and_then(|s| s.first_value())
            .and_then(|s| s.into())?;

        let gyro = devc_stream
            .find(&FourCC::GYRO)
            .and_then(|s| s.to_vec_f64())?;

        // let rotation: Vec<Rotation> = gyro.iter()
        //     .map(|g| {

        //     })
        
        Some(Gyroscope::default())
    }
}