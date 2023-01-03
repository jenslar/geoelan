//! A few convenience MP4 atom structures and methods for deriving offsets and timing.

mod stco;
mod stsz;
mod stts;
mod udta;
mod hdlr;
mod atom;

pub use atom::{Atom, AtomHeader};
pub use stts::Stts;
pub use stsz::Stsz;
pub use stco::Stco;
pub use hdlr::Hdlr;
pub use udta::{Udta, UdtaField};