//! Parse GoPro GPMF data. Returned in unprocessed form.
//! Processing of sensor data into more common forms will added gradually.
//! ```
//! use gpmf_rs::Gpmf;
//! use std::path::Path;
//! fn main() -> std::io::Result<()> {
//!     let path = Path::new("PATH/TO/GOPRO.MP4");
//!     let gpmf = Gpmf::new(&path)?;
//!     Ok(())
//! }
//! ```

pub mod gpmf;
mod files;
mod errors;
mod content_types;
mod gopro;
mod geo;

pub use gpmf::{
    Gpmf,
    FourCC,
    Stream,
    StreamType,
    Timestamp
};
pub use content_types::{ContentType,Gps, GoProPoint};
pub use errors::GpmfError;
pub use gopro::GoProFile;
pub use gopro::GoProSession;
