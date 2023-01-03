//! UNIMPLEMENTED
//! 
//! Full record (global ID 20).

/// Full `record`/20 message as per Profile.xlsx in official FIT SDK.
pub struct RecordFull {
    /// 253, timestamp, date_time
    pub timestamp: u32,
    /// 0, position_lat, sint32
    pub position_lat: i32,
    /// 1, position_long, sint32
    pub position_long: i32,
    /// 2, altitude, uint16
    pub altitude: u16,
    /// 3, heart_rate, uint8
    pub heart_rate: u8,
    /// 4, cadence, uint8
    pub cadence: u8,
    /// 5, distance, uint32
    pub distance: u32,
    /// 6, speed, uint16
    pub speed: u16,
    /// 7, power, uint16
    pub power: u16,
    /// 8, compressed_speed_distance, byte, [u8; 3]
    pub compressed_speed_distance: Vec<u8>,
    /// 9, grade, sint16
    pub grade: i16,
    /// 10, resistance, uint8
    pub resistance: u8,
    /// 11, time_from_course, sint32
    pub time_from_course: i32,
    /// 12, cycle_length, uint8
    pub cycle_length: u8,
    /// 13, temperature, sint8
    pub temperature: i8,
    /// 17, speed_1s, uint8, [u8; N]
    pub speed_1s: Vec<u8>,
    /// 18, cycles, uint8
    pub cycles: u8,
    /// 19, total_cycles, uint32
    pub total_cycles: u32,
    /// 28, compressed_accumulated_power, uint16
    pub compressed_accumulated_power: u16,
    /// 29, accumulated_power, uint32
    pub accumulated_power: u32,
    // /// 30, left_right_balance, left_right_balance
    // pub left_right_balance left_right_balance: ,
    /// 31, gps_accuracy, uint8
    pub gps_accuracy: u8,
    /// 32, vertical_speed, sint16
    pub vertical_speed: i16,
    /// 33, calories, uint16
    pub calories: u16,
    /// 39, vertical_oscillation, uint16
    pub vertical_oscillation: u16,
    /// 40, stance_time_percent, uint16
    pub stance_time_percent: u16,
    /// 41, stance_time, uint16
    pub stance_time: u16,
    // /// 42, activity_type, activity_type
    // pub activity_type activity_type: ,
    /// 43, left_torque_effectiveness, uint8
    pub left_torque_effectiveness: u8,
    /// 44, right_torque_effectiveness, uint8
    pub right_torque_effectiveness: u8,
    /// 45, left_pedal_smoothness, uint8
    pub left_pedal_smoothness: u8,
    /// 46, right_pedal_smoothness, uint8
    pub right_pedal_smoothness: u8,
    /// 47, combined_pedal_smoothness, uint8
    pub combined_pedal_smoothness: u8,
    /// 48, time128, uint8
    pub time128: u8,
    // /// 49, stroke_type, stroke_type
    // pub stroke_type stroke_type: ,
    /// 50, zone, uint8
    pub zone: u8,
    /// 51, ball_speed, uint16
    pub ball_speed: u16,
    /// 52, cadence256, uint16
    pub cadence256: u16,
    /// 53, fractional_cadence, uint8
    pub fractional_cadence: u8,
    /// 54, total_hemoglobin_conc, uint16
    pub total_hemoglobin_conc: u16,
    /// 55, total_hemoglobin_conc_min, uint16
    pub total_hemoglobin_conc_min: u16,
    /// 56, total_hemoglobin_conc_max, uint16
    pub total_hemoglobin_conc_max: u16,
    /// 57, saturated_hemoglobin_percent, uint16
    pub saturated_hemoglobin_percent: u16,
    /// 58, saturated_hemoglobin_percent_min, uint16
    pub saturated_hemoglobin_percent_min: u16,
    /// 59, saturated_hemoglobin_percent_max, uint16
    pub saturated_hemoglobin_percent_max: u16,
    // /// 62, device_index, device_index
    // pub device_index device_index: ,
    /// 67, left_pco, sint8
    pub left_pco: i8,
    /// 68, right_pco, sint8
    pub right_pco: i8,
    /// 69, left_power_phase, uint8, [N]
    pub left_power_phase: Vec<u8>,
    /// 70, left_power_phase_peak, uint8, [N]
    pub left_power_phase_peak: Vec<u8>,
    /// 71, right_power_phase, uint8, [N]
    pub right_power_phase: Vec<u8>,
    /// 72, right_power_phase_peak, uint8, [N]
    pub right_power_phase_peak: Vec<u8>,
    /// 73, enhanced_speed, uint32
    pub enhanced_speed: u32,
    /// 78, enhanced_altitude, uint32
    pub enhanced_altitude: u32,
    /// 81, battery_soc, uint8
    pub battery_soc: u8,
    /// 82, motor_power, uint16
    pub motor_power: u16,
    /// 83, vertical_ratio, uint16
    pub vertical_ratio: u16,
    /// 84, stance_time_balance, uint16
    pub stance_time_balance: u16,
    /// 85, step_length, uint16
    pub step_length: u16,
    /// 91, absolute_pressure, uint32
    pub absolute_pressure: u32,
    /// 92, depth, uint32
    pub depth: u32,
    /// 93, next_stop_depth, uint32
    pub next_stop_depth: u32,
    /// 94, next_stop_time, uint32
    pub next_stop_time: u32,
    /// 95, time_to_surface, uint32
    pub time_to_surface: u32,
    /// 96, ndl_time, uint32
    pub ndl_time: u32,
    /// 97, cns_load, uint8
    pub cns_load: u8,
    /// 98, n2_load, uint16
    pub n2_load: u16,
    /// 114, grit, float32
    pub grit: f32,
    /// 115, flow, float32
    pub flow: f32,
    /// 117, ebike_travel_range, uint16
    pub ebike_travel_range: u16,
    /// 118, ebike_battery_level, uint8
    pub ebike_battery_level: u8,
    /// 119, ebike_assist_mode, uint8
    pub ebike_assist_mode: u8,
    /// 120, ebike_assist_level_percent, uint8
    pub ebike_assist_level_percent: u8,
    pub(crate) index: usize
}