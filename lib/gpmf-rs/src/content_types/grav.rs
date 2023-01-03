//! UNIMPLEMENTED
//! 
//! Process gravity data.

// Gravity vector
// pub fn gravity(&self) {
//     let streams = self.filter(&StreamType::GravityVector);

//     for strm in streams.iter() {
//         // let grav = strm.filter("GRAV");
//         // let scal: Option<i16> = strm.filter_single("SCAL")
//         let grav = strm.filter(FourCC::GRAV);
//         let scal: Option<i16> = strm.filter_single(FourCC::SCAL)
//             .as_ref()
//             .and_then(|s| s.into());

//     }
// }

// [DEVC: 24 | DVID: 1 | DVNM: HERO9 Black | SIZE: 2172 | DUR: Some(400)] | TS: Some(23000) | 403/408] STRM SIZE: 136, NAME: Some("Gravity Vector")
// STMP J 8 1
//   Uint64([23056779])
// TSMP L 4 1
//   Uint32([1170])
// SCAL s 2 1
//   Sint16([32767])
// GRAV s 6 20
//   Sint16([1649, 19716, 26119])
// ...
//   Sint16([525, 19982, 25963])
