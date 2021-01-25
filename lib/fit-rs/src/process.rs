use crate::{
    errors::{FitError, ParseError},
    structs::{
        CameraEvent, DataMessage, FieldDescriptionMessage, FitData, GpsMetadata,
        ThreeDSensorCalibration, ThreeDSensorData, ThreeDSensorType, TimestampCorrelation,
    },
};
use nalgebra::{Matrix3, Matrix3x1}; // for three_d_sensor_calibration

pub fn field_description_message(
    data_message: &DataMessage,
) -> Result<FieldDescriptionMessage, ParseError> {
    let global_id = 206;

    // REQUIRED?
    let mut developer_data_index: Result<Vec<u8>, _> =
        Err(ParseError::ErrorAssigningFieldValue(global_id, 0)); // id: 0
    let mut field_definition_number: Result<Vec<u8>, _> =
        Err(ParseError::ErrorAssigningFieldValue(global_id, 1)); // id: 1
    let mut fit_base_type_id: Result<Vec<u8>, _> =
        Err(ParseError::ErrorAssigningFieldValue(global_id, 2)); // id: 2
    let mut field_name: Result<String, _> = Err(ParseError::ErrorAssigningFieldValue(global_id, 3)); // id: 3, 64 bytes (up to? 0-padded?)

    // OPTIONAL? NOT IN WAHOO RIVAL (WATCH) FIT
    let mut units: Result<String, _> = Err(ParseError::ErrorAssigningFieldValue(global_id, 8)); // id: 8, 16 bytes (up to? 0-padded?)

    // OPTIONAL?
    let mut array: Result<Vec<u8>, _> = Err(ParseError::ErrorAssigningFieldValue(global_id, 4)); // id: 4
    let mut components: Result<String, _> = Err(ParseError::ErrorAssigningFieldValue(global_id, 5)); // id: 5
    let mut scale: Result<Vec<u8>, _> = Err(ParseError::ErrorAssigningFieldValue(global_id, 6)); // id: 6
    let mut offset: Result<Vec<i8>, _> = Err(ParseError::ErrorAssigningFieldValue(global_id, 7)); // id: 7
    let mut bits: Result<String, _> = Err(ParseError::ErrorAssigningFieldValue(global_id, 9)); // id: 9
    let mut accumulate: Result<String, _> =
        Err(ParseError::ErrorAssigningFieldValue(global_id, 10)); // id: 10
    let mut fit_base_unit_id: Result<Vec<u16>, _> =
        Err(ParseError::ErrorAssigningFieldValue(global_id, 13)); // id: 13 in Profile: only 0,1,2
    let mut native_mesg_num: Result<Vec<u16>, _> =
        Err(ParseError::ErrorAssigningFieldValue(global_id, 14)); // id: 14 global id
    let mut native_field_num: Result<Vec<u8>, _> =
        Err(ParseError::ErrorAssigningFieldValue(global_id, 15)); // id: 15

    for field in data_message.fields.iter() {
        match field.field_definition_number {
            0 => developer_data_index = field.data.get_u8(global_id, field.field_definition_number),
            1 => {
                field_definition_number =
                    field.data.get_u8(global_id, field.field_definition_number)
            }
            2 => fit_base_type_id = field.data.get_u8(global_id, field.field_definition_number),
            3 => {
                field_name = field
                    .data
                    .clone()
                    .get_string(global_id, field.field_definition_number)
            }
            8 => {
                units = field
                    .data
                    .clone()
                    .get_string(global_id, field.field_definition_number)
            }
            // OPTIONAL?
            4 => array = field.data.get_u8(global_id, field.field_definition_number),
            5 => {
                components = field
                    .data
                    .clone()
                    .get_string(global_id, field.field_definition_number)
            }
            6 => scale = field.data.get_u8(global_id, field.field_definition_number),
            7 => offset = field.data.get_i8(global_id, field.field_definition_number),
            9 => {
                bits = field
                    .data
                    .clone()
                    .get_string(global_id, field.field_definition_number)
            }
            10 => {
                accumulate = field
                    .data
                    .clone()
                    .get_string(global_id, field.field_definition_number)
            }
            13 => fit_base_unit_id = field.data.get_u16(global_id, field.field_definition_number),
            14 => native_mesg_num = field.data.get_u16(global_id, field.field_definition_number),
            15 => native_field_num = field.data.get_u8(global_id, field.field_definition_number),
            _ => (),
        }
    }

    Ok(FieldDescriptionMessage {
        developer_data_index: developer_data_index?[0],
        field_definition_number: field_definition_number?[0],
        fit_base_type_id: fit_base_type_id?[0],
        field_name: field_name?,
        // OPTIONAL? NOT IN WAHOO RIVAL FIT
        units: match units {
            Ok(v) => Some(v),
            Err(_) => None,
        },
        // OPTIONAL? Hence Option<>, but ugly re-pack...
        array: match array {
            Ok(v) => Some(v[0]),
            Err(_) => None,
        },
        components: match components {
            Ok(v) => Some(v),
            Err(_) => None,
        },
        scale: match scale {
            Ok(v) => Some(v[0]),
            Err(_) => None,
        },
        offset: match offset {
            Ok(v) => Some(v[0]),
            Err(_) => None,
        },
        bits: match bits {
            Ok(v) => Some(v),
            Err(_) => None,
        },
        accumulate: match accumulate {
            Ok(v) => Some(v),
            Err(_) => None,
        },
        fit_base_unit_id: match fit_base_unit_id {
            Ok(v) => Some(v[0]),
            Err(_) => None,
        },
        native_mesg_num: match native_mesg_num {
            Ok(v) => Some(v[0]),
            Err(_) => None,
        },
        native_field_num: match native_field_num {
            Ok(v) => Some(v[0]),
            Err(_) => None,
        },
    })
}

pub fn parse_fielddescription(
    // fitdata: &std::collections::HashMap<u16, Vec<DataMessage>>,
    fitdata: &FitData,
) -> Result<Vec<FieldDescriptionMessage>, FitError> {
    let global_id = 206_u16;

    // let data = match fitdata.get(&global_id) {
    //     Some(d) => d,
    //     None => return Err(FitError::Fatal(ParseError::NoDataForMessageType(global_id))),
    // };
    let data = fitdata.filter(global_id);
    if data.is_empty() {
        return Err(FitError::Fatal(ParseError::NoDataForMessageType(global_id)));
    }

    let mut fdesc: Vec<FieldDescriptionMessage> = Vec::new();

    for message in data.iter() {
        match field_description_message(&message) {
            Ok(m) => {
                fdesc.push(m);
            }
            Err(e) => return Err(FitError::Fatal(e)),
        }
    }

    Ok(fdesc)
}

pub fn parse_timestampcorrelation(
    // fitdata: &std::collections::HashMap<u16, Vec<DataMessage>>,
    // fitdata: &Vec<DataMessage>,
    fitdata: &FitData,
) -> Result<TimestampCorrelation, FitError> {
    let global_id = 162_u16;

    // let data = match fitdata.get(&global_id) {
    // let data = match fitdata.get(global_id) {
    //     Some(d) => d,
    //     None => return Err(FitError::Fatal(ParseError::NoDataForMessageType(global_id))),
    // };
    // let data = fitdata.get(global_id);
    let data = fitdata.filter(global_id);
    if data.is_empty() {
        return Err(FitError::Fatal(ParseError::NoDataForMessageType(global_id)));
    }

    let mut timestamp: Result<Vec<u32>, _> =
        Err(ParseError::ErrorAssigningFieldValue(global_id, 253));
    let mut system_timestamp: Result<Vec<u32>, _> =
        Err(ParseError::ErrorAssigningFieldValue(global_id, 1));
    let mut timestamp_ms: Result<Vec<u16>, _> =
        Err(ParseError::ErrorAssigningFieldValue(global_id, 4));
    let mut system_timestamp_ms: Result<Vec<u16>, _> =
        Err(ParseError::ErrorAssigningFieldValue(global_id, 5));

    // for message in data.iter() {
    for message in data.iter() {
        for datafield in message.fields.iter() {
            match datafield.field_definition_number {
                253 => {
                    // UTC seconds at time of logging timestamp_correlation
                    timestamp = datafield
                        .data
                        .get_u32(global_id, datafield.field_definition_number)
                }
                1 => {
                    // seconds since start of fit file
                    system_timestamp = datafield
                        .data
                        .get_u32(global_id, datafield.field_definition_number)
                }
                4 => {
                    // UTC fractional/milliseconds at time of logging timestamp_correlation
                    timestamp_ms = datafield
                        .data
                        .get_u16(global_id, datafield.field_definition_number)
                }
                5 => {
                    // milliseconds since start of fit file
                    system_timestamp_ms = datafield
                        .data
                        .get_u16(global_id, datafield.field_definition_number)
                }
                _ => (),
            }
        }
    }

    Ok(TimestampCorrelation {
        timestamp: timestamp?[0],
        timestamp_ms: timestamp_ms?[0],
        system_timestamp: system_timestamp?[0],
        system_timestamp_ms: system_timestamp_ms?[0],
    })
}

pub fn parse_cameraevent(
    // fitdata: &std::collections::HashMap<u16, Vec<DataMessage>>,
    // fitdata: &Vec<DataMessage>,
    fitdata: &FitData,
) -> Result<Vec<CameraEvent>, FitError> {
    let global_id = 161_u16;

    let mut cam = Vec::new();

    let data = fitdata.filter(global_id);
    if data.is_empty() {
        return Err(FitError::Fatal(ParseError::NoDataForMessageType(global_id)));
    }

    for message in data.iter() {
        let mut timestamp: Result<Vec<u32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 253));
        let mut timestamp_ms: Result<Vec<u16>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 0));
        let mut camera_file_uuid: Result<String, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 2));
        let mut camera_event_type: Result<Vec<u8>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 1));
        let mut camera_orientation: Result<Vec<u8>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 3));

        for datafield in message.fields.iter() {
            match datafield.field_definition_number {
                253 => {
                    timestamp = datafield
                        .data
                        .get_u32(global_id, datafield.field_definition_number);
                }
                0 => {
                    timestamp_ms = datafield
                        .data
                        .get_u16(global_id, datafield.field_definition_number);
                }
                1 => {
                    camera_event_type = datafield
                        .data
                        .get_enum(global_id, datafield.field_definition_number);
                }
                2 => {
                    camera_file_uuid = datafield
                        .data
                        .clone()
                        .get_string(global_id, datafield.field_definition_number);
                }
                3 => {
                    camera_orientation = datafield
                        .data
                        .get_enum(global_id, datafield.field_definition_number);
                }
                _ => (),
            }
        }

        cam.push(CameraEvent {
            timestamp: timestamp?[0],
            timestamp_ms: timestamp_ms?[0],
            camera_file_uuid: camera_file_uuid?,
            camera_event_type: camera_event_type?[0],
            camera_orientation: camera_orientation?[0],
        });
    }

    Ok(cam)
}

pub fn parse_gpsmetadata(
    // fitdata: &std::collections::HashMap<u16, Vec<DataMessage>>,
    // fitdata: &Vec<DataMessage>,
    fitdata: &FitData,
) -> Result<Vec<GpsMetadata>, FitError> {
    let global_id = 160_u16; // gps_metadata

    let data = fitdata.filter(global_id);
    if data.is_empty() {
        return Err(FitError::Fatal(ParseError::NoDataForMessageType(global_id)));
    }

    let mut gps = Vec::new();

    for message in data.iter() {
        let mut timestamp: Result<Vec<u32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 253));
        let mut timestamp_ms: Result<Vec<u16>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 0));
        let mut latitude: Result<Vec<i32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 1));
        let mut longitude: Result<Vec<i32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 2));
        let mut altitude: Result<Vec<u32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 3));
        let mut speed: Result<Vec<u32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 4));
        let mut heading: Result<Vec<u16>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 5));
        let mut utc_timestamp: Result<Vec<u32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 6));
        let mut velocity: Result<Vec<i16>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 7)); // [i16;3]

        for datafield in message.fields.iter() {
            match datafield.field_definition_number {
                253 => {
                    timestamp = datafield
                        .data
                        .get_u32(global_id, datafield.field_definition_number)
                }
                0 => {
                    timestamp_ms = datafield
                        .data
                        .get_u16(global_id, datafield.field_definition_number)
                }
                1 => {
                    latitude = datafield
                        .data
                        .get_i32(global_id, datafield.field_definition_number)
                }
                2 => {
                    longitude = datafield
                        .data
                        .get_i32(global_id, datafield.field_definition_number)
                }
                3 => {
                    altitude = datafield
                        .data
                        .get_u32(global_id, datafield.field_definition_number)
                }
                4 => {
                    speed = datafield
                        .data
                        .get_u32(global_id, datafield.field_definition_number)
                }
                5 => {
                    heading = datafield
                        .data
                        .get_u16(global_id, datafield.field_definition_number)
                }
                6 => {
                    utc_timestamp = datafield
                        .data
                        .get_u32(global_id, datafield.field_definition_number)
                }
                7 => {
                    velocity = datafield
                        .data
                        .get_i16(global_id, datafield.field_definition_number)
                }
                // 8 | 9 | 10 | 11 | 12 => (), // EXIST IN VIRB FITDATA, NOT IN PROFILE.XSLX
                _ => (), // ignore undocumented id:s, found 8, 9, 10, 11, 12 so far
            }
        }

        gps.push(GpsMetadata {
            timestamp: timestamp?[0],
            utc_timestamp: utc_timestamp?[0],
            timestamp_ms: timestamp_ms?[0],
            latitude: latitude?[0],
            longitude: longitude?[0],
            altitude: altitude?[0],
            speed: speed?[0],
            velocity: velocity?,
            heading: heading?[0],
        })
    }
    Ok(gps)
}

// Record/20, alternative gps log
// fn parse_record(fit: &mut File, uuid: &Option<String>, global_id: u16) -> Result<Vec<crate::structs::ThreeDSensorData>, FitError> {

// }

pub fn parse_threedsensordata(
    // fitdata: &std::collections::HashMap<u16, Vec<DataMessage>>,
    fitdata: &FitData,
    three_d_sensor_type: ThreeDSensorType,
) -> Result<Vec<ThreeDSensorData>, FitError> {
    // gyroscope_data global id = 164, 1
    // accelerometer_data global id = 165, 0
    // magnetometer_data global id = 208, 2

    let global_id = match three_d_sensor_type {
        ThreeDSensorType::Gyroscope => 164_u16,
        ThreeDSensorType::Accelerometer => 165_u16,
        ThreeDSensorType::Magnetometer => 208_u16,
    };

    // let data = match fitdata.get(&global_id) {
    //     Some(c) => c,
    //     None => return Err(FitError::Fatal(ParseError::NoDataForMessageType(global_id))),
    // };
    let data = fitdata.filter(global_id);
    if data.is_empty() {
        return Err(FitError::Fatal(ParseError::NoDataForMessageType(global_id)));
    }

    let mut sensor_data: Vec<ThreeDSensorData> = Vec::new();

    for message in data.iter() {
        let mut timestamp: Result<Vec<u32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 253));
        let mut timestamp_ms: Result<Vec<u16>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 0)); //
        let mut sample_time_offset: Result<Vec<u16>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 1)); //
        let mut x: Result<Vec<u16>, _> = Err(ParseError::ErrorAssigningFieldValue(global_id, 2)); // up to 30 values, so can not set vec capacity
        let mut y: Result<Vec<u16>, _> = Err(ParseError::ErrorAssigningFieldValue(global_id, 3)); // up to 30 values, so can not set vec capacity
        let mut z: Result<Vec<u16>, _> = Err(ParseError::ErrorAssigningFieldValue(global_id, 4)); // up to 30 values, so can not set vec capacity

        for datafield in message.fields.iter() {
            match datafield.field_definition_number {
                253 => {
                    timestamp = datafield
                        .data
                        .get_u32(global_id, datafield.field_definition_number)
                }
                0 => {
                    timestamp_ms = datafield
                        .data
                        .get_u16(global_id, datafield.field_definition_number)
                }
                1 => {
                    sample_time_offset = datafield
                        .data
                        .get_u16(global_id, datafield.field_definition_number)
                }
                2 => {
                    x = datafield
                        .data
                        .get_u16(global_id, datafield.field_definition_number)
                }
                3 => {
                    y = datafield
                        .data
                        .get_u16(global_id, datafield.field_definition_number)
                }
                4 => {
                    z = datafield
                        .data
                        .get_u16(global_id, datafield.field_definition_number)
                }
                _ => (),
            }
        }

        sensor_data.push(ThreeDSensorData {
            timestamp: timestamp?[0],
            timestamp_ms: timestamp_ms?[0],
            sample_time_offset: sample_time_offset?,
            x: x?,
            y: y?,
            z: z?,
            calibrated_x: Vec::new(), // calculated post extraction via three_d_sensor_calibration
            calibrated_y: Vec::new(), // calculated post extraction via three_d_sensor_calibration
            calibrated_z: Vec::new(), // calculated post extraction via three_d_sensor_calibration
        })
    }

    Ok(sensor_data)
}

pub fn parse_threedsensorcalibration(
    // fitdata: &std::collections::HashMap<u16, Vec<DataMessage>>,
    fitdata: &FitData,
    three_d_sensor_type: ThreeDSensorType,
) -> Result<Vec<ThreeDSensorCalibration>, FitError> {
    // gyroscope_data: id=164, sensor type=1
    // accelerometer_data: id=165, sensor type: 0
    // magnetometer_data: id=208, sensor type=2 (a.k.a. compass)

    let global_id = 167_u16;

    let sensor_type_id = match three_d_sensor_type {
        ThreeDSensorType::Gyroscope => 1_u8,
        ThreeDSensorType::Accelerometer => 0_u8,
        ThreeDSensorType::Magnetometer => 2_u8,
    };

    // let data = match fitdata.get(&global_id) {
    //     Some(c) => c,
    //     None => return Err(FitError::Fatal(ParseError::NoDataForMessageType(global_id))),
    // };
    let data = fitdata.filter(global_id);
    if data.is_empty() {
        return Err(FitError::Fatal(ParseError::NoDataForMessageType(global_id)));
    }

    let mut three_d_sensor_calibration: Vec<ThreeDSensorCalibration> = Vec::new();

    for message in data.iter() {
        let mut timestamp: Result<Vec<u32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 253));
        let mut sensor_type: Result<Vec<u8>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 0));
        let mut calibration_factor: Result<Vec<u32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 1));
        let mut calibration_divisor: Result<Vec<u32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 2));
        let mut level_shift: Result<Vec<u32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 3));
        let mut offset_cal: Result<Vec<i32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 4)); // [3]
        let mut orientation_matrix: Result<Vec<i32>, _> =
            Err(ParseError::ErrorAssigningFieldValue(global_id, 5)); // 3x3 matrix [9]

        for datafield in message.fields.iter() {
            match datafield.field_definition_number {
                253 => {
                    timestamp = datafield
                        .data
                        .get_u32(global_id, datafield.field_definition_number)
                }
                0 => {
                    sensor_type = datafield
                        .data
                        .get_enum(global_id, datafield.field_definition_number)
                }
                1 => {
                    calibration_factor = datafield
                        .data
                        .get_u32(global_id, datafield.field_definition_number)
                }
                2 => {
                    calibration_divisor = datafield
                        .data
                        .get_u32(global_id, datafield.field_definition_number)
                }
                3 => {
                    level_shift = datafield
                        .data
                        .get_u32(global_id, datafield.field_definition_number)
                }
                4 => {
                    offset_cal = datafield
                        .data
                        .get_i32(global_id, datafield.field_definition_number)
                }
                5 => {
                    orientation_matrix = datafield
                        .data
                        .get_i32(global_id, datafield.field_definition_number)
                }
                _ => (),
            }
        }

        match sensor_type {
            Ok(s) => {
                if s[0] == sensor_type_id {
                    three_d_sensor_calibration.push(ThreeDSensorCalibration {
                        timestamp: timestamp?[0],
                        sensor_type: s[0],
                        calibration_factor: calibration_factor?[0],
                        calibration_divisor: calibration_divisor?[0],
                        level_shift: level_shift?[0],
                        offset_cal: offset_cal?,
                        orientation_matrix: orientation_matrix?,
                    })
                }
            }
            Err(e) => return Err(FitError::Fatal(e)),
        }
    }

    Ok(three_d_sensor_calibration)
}

pub fn calibrate_threedsensordata(
    // fitdata: &std::collections::HashMap<u16, Vec<DataMessage>>,
    fitdata: &FitData,
    three_d_sensor_type: ThreeDSensorType,
) -> Result<Vec<ThreeDSensorData>, FitError> {
    let mut calibrated_sensor_data: Vec<ThreeDSensorData> = Vec::new();

    // Gyroscope: global 164, type: 1
    // Accelerometer: global 165, type: 0
    // Magnetometer: global 208, type: 2

    // Compile input data: raw sensor data + calibration values
    let sensor_data = match parse_threedsensordata(&fitdata, three_d_sensor_type) {
        Ok(data) => data,
        Err(err) => return Err(err),
    };
    let sensor_calibration = match parse_threedsensorcalibration(&fitdata, three_d_sensor_type) {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let mut calibration_index = 0; // index into "calibration", value +1 each time a data values timestamp reaches calibration timestamp with index "calibration_index"

    for mut msg in sensor_data.into_iter() {
        // Determine correct sensor calibration value (the first one preceding sensor data message)
        for (idx_cal, cal) in sensor_calibration.iter().enumerate() {
            if msg.timestamp * 1000 + msg.timestamp_ms as u32 > cal.timestamp * 1000 {
                calibration_index = idx_cal;
            }
            // could else {break} but fairly few cal messages in a fit
        }
        let cal = &sensor_calibration[calibration_index]; // no need for .get() + err handling?

        // ORIENTATION MATRIX
        // create normalised (?) float vec for orientation matrix (see FIT SDK)
        // create 3x3 matrix from float vec
        let orientation_matrix = Matrix3::from_row_slice(
            &cal.orientation_matrix
                .clone()
                // NOTE 201028 in fit sdk pdf these values are already divided by u16::MAX (?)
                // not so for virb data, but should perhaps test -sqrt(3) < 'i' < sqrt(3)
                // before dividing
                .into_iter()
                .map(|i| i as f64 / u16::MAX as f64)
                .collect::<Vec<f64>>(),
        );

        let offset_cal = Matrix3x1::from_row_slice(
            &cal.offset_cal
                .clone()
                .into_iter()
                .map(|i| i as f64)
                .collect::<Vec<f64>>(),
        );
        let cal_factor = cal.calibration_factor as f64 / cal.calibration_divisor as f64;
        let len_sens = msg.x.len();

        let mut calibrated_x: Vec<f64> = Vec::new();
        let mut calibrated_y: Vec<f64> = Vec::new();
        let mut calibrated_z: Vec<f64> = Vec::new();

        for i in 0..len_sens {
            let sample =
                Matrix3x1::from_column_slice(&[msg.x[i] as f64, msg.y[i] as f64, msg.z[i] as f64]);
            // TODO 201104 check that calibrated_sample is indeed a 3x1 x,y,z matrix
            let calibrated_sample = cal_factor
                * orientation_matrix
                * (sample
                    - Matrix3x1::from_column_slice(&[
                        cal.level_shift as f64,
                        cal.level_shift as f64,
                        cal.level_shift as f64,
                    ])
                    - offset_cal);
            calibrated_x.push(calibrated_sample[0]);
            calibrated_y.push(calibrated_sample[1]);
            calibrated_z.push(calibrated_sample[2]);
        }

        msg.calibrated_x = calibrated_x;
        msg.calibrated_y = calibrated_y;
        msg.calibrated_z = calibrated_z;
        calibrated_sensor_data.push(msg);
    }

    Ok(calibrated_sensor_data)
}
