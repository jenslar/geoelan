use std::ops::Range;

use nalgebra::{Matrix3, Matrix3x1};
use rayon::{iter::{IntoParallelRefIterator, ParallelIterator}, prelude::IntoParallelRefMutIterator};

use crate::{FitError, fit::DataMessage, Fit};

use super::{SensorType, SensorCalibration};

/// Parsed, re-formatted 1D, 3D sensor data message (no 2D sensors in FIT).
/// 
/// > **Note:** 1D sensors will only assign values to the `x`, `calibrated_x` fields
/// (y, z, are set to empty `Vec`s for 1D sensors).
/// 
/// 1D sensors:
/// - calibration: `one_d_sensor_calibration`, id = 210
/// - `barometer_data`, id = 209
/// 
/// 3D sensors:
/// - calibration: `three_d_sensor_calibration`, id = 167
/// - `gyrometer_data`, id = 164
/// - `accelerometer_data`, id = 165
/// - `magnetometer_data`, id = 208
#[derive(Debug, Clone)]
pub struct SensorData {
    pub sensor_type: SensorType,
    /// timestamp value, id:253, seconds
    pub timestamp: u32,               
    /// timestamp_ms value, id:0, milliseconds
    pub timestamp_ms: u16,            
    /// sample_time_offset value, id:1
    pub sample_time_offset: Vec<u16>, 
    /// x value, id:2 gyro_x, acc_x, mag_x
    /// Note: stored as u16 in FIT-file,
    /// but cast to u32 for compatibility
    /// with one_d_sensor_data, e.g. barometer/209
    pub x: Vec<u32>,                  
    /// y value, id:3 gyro_y, acc_y, mag_y
    /// Note: stored as u16 in FIT-file,
    /// but cast to u32 for compatibility
    /// with one_d_sensor_data, e.g. barometer/209
    pub y: Vec<u32>,                  
    /// z value, id:4 gyro_z, acc_z, mag_z
    /// Note: stored as u16 in FIT-file,
    /// but cast to u32 for compatibility
    /// with one_d_sensor_data, e.g. barometer/209
    pub z: Vec<u32>,                  
    // values below calculated post-extraction via corresponding three_d_sensor_calibration messages
    pub calibrated_x: Vec<f64>, // id:5 calibrated_gyro_x, calibrated_acc_x, calibrated_mag_x
    pub calibrated_y: Vec<f64>, // id:6 calibrated_gyro_y, calibrated_acc_y, calibrated_mag_y
    pub calibrated_z: Vec<f64>, // id:7 calibrated_gyro_z, calibrated_acc_z, calibrated_mag_z
    pub(crate) index: usize
}

impl SensorData {
    pub fn new(
        data_message: &DataMessage,
        sensor_type: &SensorType,
    ) -> Result<Self, FitError> {
        let global_id = sensor_type.global();

        if data_message.global != global_id {
            return Err(FitError::UnexpectedMessageType{expected: global_id, got: data_message.global})
        }

        let mut timestamp: Option<u32> = None;
        let mut timestamp_ms: Option<u16> = None;
        let mut sample_time_offset: Option<Vec<u16>> = None; //
        // x, y, z contain up to 30 values, so can not set vec capacity
        // stored as u16 in FIT-file, cast to u32 for compatibility
        // with one_d_sensor_data e.g. barometer.
        let mut x: Option<Vec<u32>> = None;
        let mut y: Option<Vec<u32>> = None;
        let mut z: Option<Vec<u32>> = None;

        for datafield in data_message.fields.iter() {
            match datafield.field_def_no() {
                253 => timestamp = datafield.data.as_ref().into(),
                0 => timestamp_ms = datafield.data.as_ref().into(),
                1 => sample_time_offset = datafield.data.as_ref().into(),
                2 => x = datafield.data.as_ref().into(),
                3 => y = datafield.data.as_ref().into(),
                4 => z = datafield.data.as_ref().into(),
                _ => (),
            }
        }

        // Set y, z to empty Vec for 1D sensor data
        // in order not to raise errors.
        if sensor_type.dim() == 1 {
            y = Some(Vec::new());
            z = Some(Vec::new());
        }

        Ok(Self {
            sensor_type: sensor_type.to_owned() ,
            timestamp: timestamp
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 253})?,
            timestamp_ms: timestamp_ms
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 0})?,
            sample_time_offset: sample_time_offset
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 1})?,
            x: x.ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 2})?,
            y: y.ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 3})?,
            z: z.ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 4})?,
            // calibration done post-extraction
            calibrated_x: Vec::new(),
            calibrated_y: Vec::new(),
            calibrated_z: Vec::new(),
            index: data_message.index
        })
    }

    /// Returns uncalibrated sensor data.
    /// Formats all 1D, 3D sensor data into a more accessible form.
    /// - SensorType::Gyroscope = 164,
    /// - SensorType::Accelerometer = 165,
    /// - SensorType::Magnetometer = 208,
    pub fn from_fit_uncalibrated(
        fit: &Fit,
        range: Option<&Range<usize>>, // slice indeces for session
        sensor_type: &SensorType,
    ) -> Result<Vec<Self>, FitError> {
        let global = sensor_type.global();

        let range = range.cloned().unwrap_or(0 .. fit.len());

        fit.records[range].par_iter()
            .filter(|rec| rec.global == global)
            .map(|rec| SensorData::new(rec, sensor_type))
            .collect()
    }

    /// Returns calibrated sensor data.
    /// 
    /// > **Note:** calibration is not yet well tested and may return incorrect results.
    /// 
    /// Parses and calibrates all 1D and 3D sensor data (no 2D sensors in current FIT specification)
    /// for specified sensor type and returns these in a more accessible form.
    /// 
    /// `three_d_sensor_type` sets correct 3D sensor type/FIT global id
    /// - `SensorType::Gyroscope`: global id 164, sensor type 1
    /// - `SensorType::Accelerometer`: global id 165, sensor type 0
    /// - `SensorType::Magnetometer`: global id 208, sensor type 2
    /// 
    /// `one_d_sensor_type` sets correct 1D sensor type/FIT global id
    /// 
    /// Error handling is for determining whether a required field could be assigned
    /// or nor, i.e. whether it was present in the input data.
    pub fn from_fit(
        fit: &Fit,
        range: Option<&Range<usize>>, // slice indeces for session
        sensor_type: &SensorType,
    ) -> Result<Vec<Self>, FitError> {

        // Compile raw sensor data, to mutate
        let mut sensor_data = Self::from_fit_uncalibrated(fit, range, sensor_type)?;
        println!("RAW OK");
        // 1. Compile calibration data (FIT global ID 167 for 3D, 210 for 1D sensor data),
        // 2. Then filter on sensor type (field data enum 0, 1, 2, 3)

        let sensor_calibration: Vec<_> = SensorCalibration::from_fit(fit, sensor_type)?;
        // let sensor_calibration: Vec<_> = SensorCalibration::from_fit(fit, sensor_type)?
        //     .into_iter()
        //     .filter(|cal| cal.sensor_type == sensor_type.kind())
        //     .collect();
        
        // Calibrate existing sensor data in place in parallel
        sensor_data.par_iter_mut().for_each(|msg| {
            // Determine correct sensor calibration value
            // (the one immediately preceding current sensor data message).
            let calibration = sensor_calibration.iter()
                .rev()
                .find(|c| c.index < msg.index);

            if let Some(cal) = calibration {
                msg.calibrate(cal)
            }
        });

        Ok(sensor_data)
    }

    /// Get FIT global id for sensor type
    pub fn global(&self) -> u16 {
        self.sensor_type.global()
    }

    /// Get sensor type enum, see Types sheet in Profile.xlsx
    pub fn kind(&self) -> u8 {
        self.sensor_type.as_u8()
    }

    /// Return dimensions (1 = 1D, 2 = 2D, 3 = 3D)
    pub fn dim(&self) -> u8 {
        self.sensor_type.dim()
    }

    /// Calibrate sensor values. Populates `calibrated_x`, `calibrated_y`,
    /// and `calibrated_z` fields for 3D sensors and `calibrated_x` for 1D sensors.
    pub fn calibrate(&mut self, sensor_calibration: &SensorCalibration) {
        // Pick X-axis to determine number of values since
        // it is filled for all sensor types.
        // X, Y, Z must be of equal length for 3D sensors.
        let len = self.x.len();

        match self.sensor_type.dim() {

            1 => {
                let offset_cal: Vec<_> = sensor_calibration.offset_cal.iter().map(|cal| *cal as f64).collect();

                let cal_factor = sensor_calibration.calibration_factor as f64 / sensor_calibration.calibration_divisor as f64;
        
                for i in 0..len {
                    let sample = self.x[i] as f64;
                    let calibrated_sample = cal_factor
                        * (sample - sensor_calibration.level_shift as f64)
                        - offset_cal[0];

                    // Push calibrated data
                    self.calibrated_x.push(calibrated_sample);
                }
            }

            3 => {
                // ORIENTATION MATRIX
                // create normalised (?) float vec for orientation matrix (see FIT SDK)
                // create 3x3 matrix from float vec
                let orientation_matrix = Matrix3::from_row_slice(
                    &sensor_calibration.orientation_matrix
                        .iter()
                        .map(|i| *i as f64 / 65535.0) // scale from Profile.xslx
                        .collect::<Vec<f64>>(),
                );

                let offset_cal = Matrix3x1::from_row_slice(
                    &sensor_calibration.offset_cal
                        .iter()
                        .map(|i| *i as f64)
                        .collect::<Vec<f64>>(),
                );
        
                let calibration_factor = sensor_calibration.calibration_factor as f64
                    / sensor_calibration.calibration_divisor as f64;
        
                for i in 0..len {
                    let sample =
                        Matrix3x1::from_column_slice(&[self.x[i] as f64, self.y[i] as f64, self.z[i] as f64]);
                    let calibrated_sample = calibration_factor
                        * orientation_matrix
                        * (sample
                            - Matrix3x1::from_column_slice(&[
                                sensor_calibration.level_shift as f64,
                                sensor_calibration.level_shift as f64,
                                sensor_calibration.level_shift as f64,
                            ])
                            - offset_cal);
                    // TODO 201104 check that calibrated_sample is indeed a 3x1 x,y,z matrix
                    // assert_eq!(calibrated_sample.len(), 3); // enough?
        
                    // Push calibrated data
                    self.calibrated_x.push(calibrated_sample[0]);
                    self.calibrated_y.push(calibrated_sample[1]);
                    self.calibrated_z.push(calibrated_sample[2]);
                }
            },
            
            // Only 1D and 3D sensors in FIT
            _ => ()
            // d => return Err(FitError::InvalidSensorDimension(d))
        }
    }
}