//! Crate for reading Garmin FIT-files with additional Garmin VIRB specific functionality.
//! Custom developer data is supported, but not compressed time stamp headers.
//! Most data is only available in a fairly raw form, as it was stored in the FIT-file,
//! and needs to be processed further.
//! 
//! The crate is a bit focused on Garmin VIRB FIT-data since it was developed for the
//! CLI tool [GeoELAN](https://gitlab.com/rwaai/geoelan). Some "VIRB bloat" is to be expected.
//! The VIRB does not store detailed GPS data within the `record` message (global ID 20),
//! but rather the `gps_metadata` message (global ID 160).
//! 
//! Calibration of sensor data is supported (magnetometer, accelerometer, gyroscope, and barometer),
//! but note that this is not well tested. Use `Fit::sensor()` to try this out.
//! 
//! There is also an attempt to show field names when printing records as they are presented
//! in the [FIT SDK](https://developer.garmin.com/fit/), but errors may exist.
//! 
//! ```rs
//! use crate::fit_rs::Fit;
//! use std::path::Path;
//! 
//! fn main -> std::io::Result<()> {
//!     // Parse a FIT-file.
//!     let fit_path = Path::new("FITFILE.fit");
//!     let fit = Fit::new(&fit_path)?;
//! 
//!     for record in fit.records.iter() {
//!         println!("{record}");
//!     }
//! 
//!     // Extract UUID from Garmin VIRB action camera.
//!     let mp4_path = PathBuf::from("VIRB_MP4FILE.MP4");
//!     let uuid = Fit::uuid_mp4(&mp4_path)?;
//! 
//!     Ok(())
//! }
//! ```

mod errors;
mod fit;
mod virb;
mod types;
mod files;
mod geo;
mod profile;

// Core FIT struct
pub use fit::Fit;

// Recording session structs
pub use virb::FitSession;
pub use virb::FitSessions;
pub use virb::VirbFile;
pub use virb::VirbSession;

// FIT message type structs, these are more accessible via
// `Fit` methods.
pub use types::CameraEvent;
pub use types::FieldDescriptionMessage;
pub use types::{GpsMetadata, FitPoint};
pub use types::Record;
pub use types::{
    SensorCalibration,
    SensorData,
    SensorType
};
pub use types::TimestampCorrelation;

// Errors
pub use errors::FitError;