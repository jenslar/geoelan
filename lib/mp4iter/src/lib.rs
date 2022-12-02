//! Iterate over MP4 atoms and find specific atoms via FourCC.
//! Does not and will not support any kind of video de/encoding.
//! The implementation was mostly done with help from
//! <https://developer.apple.com/library/archive/documentation/QuickTime/QTFF/QTFFPreface/qtffPreface.html>
//! (despite the warning on the front page above...).
//! 
//! //! ```
//! use mp4iter::Mp4;
//! use std::path::Path;
//! 
//! fn main() -> std::io::Result<()> {
//!     let path = Path::new("PATH/TO/VIDEO.MP4");
//!     let mut mp4 = Mp4::new(&path)?;
//! 
//!     println!("{:?}", mp4.duration());
//! 
//!     Ok(())
//! }
//! ```

pub mod mp4;
pub mod fourcc;
pub mod offset;
pub mod atom;
pub mod errors;

pub use mp4::Mp4;
pub use fourcc::FourCC;
pub use offset::Offset;
pub use atom::Atom;
pub use atom::Stts;
pub use atom::Stsz;
pub use atom::Stco;
pub use atom::Hdlr;
pub use atom::{Udta, UdtaField};
pub use errors::Mp4Error;