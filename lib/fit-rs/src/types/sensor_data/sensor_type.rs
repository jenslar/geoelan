//! FIT sensor type.
//! 
//! Covers:
//! - Accelerometer
//!     - 3D sensor
//!     - FIT global ID 165
//!     - Sensor type enum 0.
//!     - Calibration message `three_d_calibration`/167
//! - Gyroscope
//!     - 3D sensor
//!     - FIT global ID 164
//!     - Sensor type enum 1.
//!     - Calibration message `three_d_calibration`/167
//! - Magnetometer
//!     - 3D sensor
//!     - FIT global ID 208
//!     - Sensor type enum 2.
//!     - Calibration message `three_d_calibration`/167
//! - Barometer
//!     - 1D sensor
//!     - FIT global ID 209
//!     - Sensor type enum 3.
//!     - Calibration message `one_d_calibration`/210

/// The available 1D, and 3D sensor types as specified in FIT SDK.
#[derive(Debug, Copy, Clone)]
pub enum SensorType {
    /// 3D sensor. FIT global ID 165, sensor type enum 0.
    /// Calibration `three_d_calibration`/167
    Accelerometer,
    /// 3D sensor. FIT global ID 164, sensor type enum 1.
    /// Calibration `three_d_calibration`/167
    Gyroscope,
    /// 3D sensor. FIT global ID 208, sensor type enum 2.
    /// Calibration `three_d_calibration`/167
    Magnetometer,
    /// 1D sensor. FIT global ID 209, sensor type enum 3.
    /// Calibration `one_d_calibration`/210
    Barometer,
}

impl SensorType {
    /// Derives `SensorType` from `sensor_type` field
    /// in sensor data.
    pub fn from_u8(sensor_type: u8) -> Option<Self> {
        match sensor_type {
            0 => Some(SensorType::Accelerometer),
            1 => Some(SensorType::Gyroscope),
            2 => Some(SensorType::Magnetometer),
            3 => Some(SensorType::Barometer),
            _ => None
        }
    }

    /// Derives `SensorType` from FIT global ID.
    pub fn from_global(global: u16) -> Option<Self> {
        match global {
            165 => Some(SensorType::Accelerometer),
            164 => Some(SensorType::Gyroscope),
            208 => Some(SensorType::Magnetometer),
            209 => Some(SensorType::Barometer),
            _ => None
        }
    }

    /// Returns FIT global ID for the sensor.
    pub fn global(&self) -> u16 {
        match self {
            SensorType::Accelerometer => 165,
            SensorType::Gyroscope => 164,
            SensorType::Magnetometer => 208,
            SensorType::Barometer => 209,
        }
    }

    /// Returns FIT global ID for the sensor's
    /// corresponding calibration message.
    /// Sensors with the same dimensions (1D, 3D)
    /// log to the same FIT global ID. Hence
    /// a second check is required to determin
    /// the exact sensor type - global ID for
    /// the caliration message is not enough.
    pub fn global_cal(&self) -> u16 {
        match self {
            SensorType::Accelerometer
            | SensorType::Gyroscope
            | SensorType::Magnetometer => 167,
            SensorType::Barometer => 210,
        }
    }

    /// Returns sensor type enum as `u8`,
    /// as specified in the _Types_ sheet in
    /// FIT SDK Profile.xlsx.
    pub fn as_u8(&self) -> u8 {
        match self {
            SensorType::Accelerometer => 0,
            SensorType::Gyroscope => 1,
            SensorType::Magnetometer => 2,
            SensorType::Barometer => 3,
        }
    }

    /// Return dimensions (1 = 1D, 3 = 3D)
    /// Note that the current FIT SDK only specifies
    /// 1D and 3D data.
    pub fn dim(&self) -> u8 {
        match self {
            SensorType::Accelerometer
            | SensorType::Gyroscope
            | SensorType::Magnetometer => 3,
            SensorType::Barometer => 1,
        }
    }
}