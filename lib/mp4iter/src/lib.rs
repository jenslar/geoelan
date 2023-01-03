//! Iterate over MP4 atoms and find specific atoms via FourCC.
//! Does not and will not support any kind of video de/encoding.
//! 
//! This crate was developed for the CLI tool [GeoELAN](https://gitlab.com/rwaai/geoelan)
//! to extract data from action camera MP4 files so its functionality and methods is often specific,
//! and possibly odd. Be aware of possibly misguided assumptions in terms of how to interpret
//! data in some atoms.
//! 
//! Note that reads are currently not buffered so perhaps avoid reading the `mdat` atom...
//! 
//! The implementation was mostly done with help from
//! <https://developer.apple.com/library/archive/documentation/QuickTime/QTFF/QTFFPreface/qtffPreface.html>
//! (despite the warning on the front page above).
//! 
//! ```rs
//! use mp4iter::Mp4;
//! //! use std::path::Path;
//! 
//! fn main() -> std::io::Result<()> {
//!     let mp4 = Mp4::new(Path::new("VIDEO.MP4"))?;
//!     
//!     for atom_header in mp4.into_iter() {
//!         println!("{atom_header:?}")
//!     }
//!
//!     // Derives duration for MP4 without the need for FFmpeg or similar.
//!     println!("{:?}", mp4.duration());
//! 
//!     Ok(())
//! }
//! ```

pub mod mp4;
pub mod fourcc;
pub mod offset;
pub mod atom;
pub mod consts;
pub mod errors;

pub use mp4::Mp4;
pub use fourcc::FourCC;
pub use offset::Offset;
pub use atom::{Atom, AtomHeader};
pub use atom::Stts;
pub use atom::Stsz;
pub use atom::Stco;
pub use atom::Hdlr;
pub use atom::{Udta, UdtaField};
pub use consts::CONTAINER;
pub use errors::Mp4Error;