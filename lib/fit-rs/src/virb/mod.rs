//! Garmin VIRB related functionality.

pub mod session_fit;
pub mod session_fit_new;
pub mod session_virb;
pub mod file;

pub use session_fit::{FitSession, FitSessions};
pub use session_virb::VirbSession;
pub use file::VirbFile;