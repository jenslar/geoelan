//! FIT sensor calibration message/s.
//! Covers `one_d_sensor_calibration`/210, `three_d_sensor_calibration`/167 messages.

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{fit::DataMessage, FitError, Fit};

use super::SensorType;

/// VIRB only. (?)
/// Parsed `one_d_sensor_calibration`/210,
/// `three_d_sensor_calibration`/167 messages.
/// FIT SDK Profile.xslx does not include any "2D sensor data" types.
/// Contains calibration values for global id 164, 165, 208
/// Accelerometer/165 = sensor type 0
/// Gyroscope/164 = sensor type 1
/// Magnetometer/208 = sensor type 2 (compass)
#[derive(Debug)]
pub struct SensorCalibration {
    pub timestamp: u32,               // id:253, seconds
    pub sensor_type: u8,              // id:0, enum
    pub calibration_factor: u32,      // id:1
    pub calibration_divisor: u32,     // id:2
    pub level_shift: u32,             // id:3
    pub offset_cal: Vec<i32>,         // using Vec::with_capacity(3) id:4 [3]
    pub orientation_matrix: Vec<i32>, // using Vec::with_capacity(9)  id:5 3x3 matrix [9]
    pub(crate) index: usize
}

impl SensorCalibration {
    /// Reformat a single 1D, 3D sensor calibration message
    /// into a more accessible form (no 2D sensor type).
    /// 1D sensors, `one_d_calibration`/210:
    /// - Barometer/165 = sensor type 3
    /// 3D sensors, `three_D_calibration`/167:
    /// - Accelerometer/165 = sensor type 0
    /// - Gyroscope/164 = sensor type 1
    /// - Magnetometer/208 = sensor type 2 (compass)
    pub fn new(
        data_message: &DataMessage,
    ) -> Result<Self, FitError> {

        // Default to 3D sensor calibration (for error reporting only),
        // derived from sensor_type below.
        // 1D cal = 210, 3D cal = 167, no 2D sensor type according to Profile.xslx.
        let global_id = data_message.global;

        let mut timestamp: Option<u32> = None;
        let mut sensor_type: Option<u8> = None;
        let mut calibration_factor: Option<u32> = None;
        let mut calibration_divisor: Option<u32> = None;
        let mut level_shift: Option<u32> = None;
        let mut offset_cal: Option<Vec<i32>> = None; // [3]
        let mut orientation_matrix: Option<Vec<i32>> = None; // 3x3 row matrix [9]

        for field in data_message.fields.iter() {
            match field.field_def_no() {
                253 => timestamp = field.data.as_ref().into(),
                0 => sensor_type = field.data.as_ref().into(),
                1 => calibration_factor = field.data.as_ref().into(),
                2 => calibration_divisor = field.data.as_ref().into(),
                3 => level_shift = field.data.as_ref().into(),
                4 => offset_cal = field.data.as_ref().into(),
                5 => orientation_matrix = field.data.as_ref().into(),
                _ => (),
            }
        }

        if let Some(sensor) = sensor_type.and_then(|s| SensorType::from_u8(s)) {
            if sensor.dim() == 1 {
                // set orientation_matrix to Some with empty vec since not used for 1D sensor data
                orientation_matrix = Some(Vec::new())
            }
        }

        // TODO 220811 fix logical error in global ID check below, everything else works
        // // Can not check before deriving sensor type (should ideally be first check in method)
        // if data_message.global != 167 || data_message.global != 210 {
        //     println!("SENSOR CAL ERROR");
        //     return Err(FitError::UnexpectedMessageType{expected: global_id, got: data_message.global})
        // }

        Ok(Self {
            timestamp: timestamp
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 253})?,
            sensor_type: sensor_type
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 0})?,
            calibration_factor: calibration_factor
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 1})?,
            calibration_divisor: calibration_divisor
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 2})?,
            level_shift: level_shift
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 3})?,
            offset_cal: offset_cal
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 4})?,
            orientation_matrix: orientation_matrix
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 5})?,
            index: data_message.index
        })
    }

    /// Returns sensor type as a `SensorType` enum.
    pub fn sensor_type(&self) -> Option<SensorType> {
        SensorType::from_u8(self.sensor_type)
    }

    /// Formats sensor calibration data into a
    /// more accessible form. Returns calibration
    /// messages for specified sensor type only,
    /// e.g. gyroscope.
    /// 
    /// > **Note:** No filtering on range,
    /// since the required calibration data may exist
    /// outside of session data (VIRB) ranges.
    pub fn from_fit(
        fit: &Fit,
        sensor_type: &SensorType
    ) -> Result<Vec<Self>, FitError> {

        // Get global FIT ID for either (all) 1D, or 3D sensors
        let global_cal = sensor_type.global_cal();
        // Get u8 representing specific sensor, e.g. 1 for gyroscope
        let kind = sensor_type.as_u8();

        // Old, returned calibration messages for e.g. all 3D sensors,
        // not just the one specified
        // fit.records.par_iter()
        //     .filter(|rec| rec.global == global_cal)
        //     .map(Self::new)
        //     .collect()
        // New, should only return calibration messages for specified sensor type,
        // e.g. gyroscope calibrations only
        fit.records.par_iter()
            .filter_map(|rec| {
                if rec.global != global_cal {
                    None
                } else {
                    let cal = Self::new(rec);
                    match cal {
                        Ok(c) => {
                            if c.sensor_type == kind {
                                Some(Ok(c))
                            } else {
                                None
                            }
                        },
                        Err(err) => Some(Err(err))
                    }
                }
            })
            .collect()
    }
}
