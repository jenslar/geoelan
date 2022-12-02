//! 1D, 2D, 3D sensor data type and calibration.
//! Note that no 2D sensors exist in current FIT specification.
//! Sensors covered are:
//! - Accelerometer (3D)
//! - Gyroscope (3D)
//! - Magnetometer (3D)
//! - Barometer (1D)

pub mod sensor_type;
pub mod sensor_calibration;
pub mod sensor_data;

pub use sensor_type::SensorType;
pub use sensor_calibration::SensorCalibration;
pub use sensor_data::SensorData;