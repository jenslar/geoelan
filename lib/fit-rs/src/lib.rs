//! Crate for reading Garmin FIT-files with additional Garmin VIRB specific functionality.
//! ```rs
//! use crate::fit_rs::Fit;
//! fn main -> std::io::Result<()> {
//!     // Parse a FIT-file.
//!     let fit_path = PathBuf::from("VIRB_FITFILE.fit");
//!     let fit = Fit::new(&fit_path)?;
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