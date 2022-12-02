use time::{PrimitiveDateTime, format_description};

use crate::GpmfError;

pub mod content_types;
pub mod accl;
pub mod gps;
pub mod grav;
pub mod gyro;

pub use content_types::ContentType;
pub use accl::{Acceleration, Accelerometer};
pub use gps::{GoProPoint, Gps};
pub use gyro::Orientation;

/// String representation for datetime objects.
pub(crate) fn primitivedatetime_to_string(datetime: &PrimitiveDateTime) -> Result<String, GpmfError> {
    // PrimitiveDateTime::to_string(&self.datetime) // sufficient?
    let format = format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]")
        .map_err(|e| GpmfError::TimeError(e.into()))?;
    datetime.format(&format)
        .map_err(|e| GpmfError::TimeError(e.into()))
}