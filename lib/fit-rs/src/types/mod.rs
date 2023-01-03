//! A few conversions from raw data into easier to use forms and (sometimes) units, such as GPS and sensor data.
//! Please note that automatic conversion is only covered for the message types covered here.

pub mod field_description;
pub mod gps_metadata;
pub mod record_virb;
pub mod record_full;
pub mod timestamp_correlation;
pub mod camera_event;
pub mod sensor_data;

pub use camera_event::CameraEvent;
pub use field_description::FieldDescriptionMessage;
pub use gps_metadata::{GpsMetadata, FitPoint};
pub use record_virb::Record;
pub use sensor_data::{
    SensorCalibration,
    SensorData,
    SensorType
};
pub use timestamp_correlation::TimestampCorrelation;