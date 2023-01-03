//! [FIT SDK](https://developer.garmin.com/fit/overview/) message types taken from Profile.xslx.

pub mod message_type;
pub mod profile;

pub use message_type::{FitFieldType, FitMessageType};