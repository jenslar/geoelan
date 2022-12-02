//! Work-in-progress. Not yet functional.

mod jpeg;
mod exif;
mod errors;
mod tag;
mod segment;

pub use jpeg::Jpeg;
pub use tag::JpegTag;
pub use exif::ExifTag;
pub use segment::Segment;
pub use errors::JpegError;
