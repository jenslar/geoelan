//! [FIT SDK](https://developer.garmin.com/fit/overview/) message types taken from Profile.xslx.

use std::collections::HashMap;

use super::message_type::{FitFieldType, FitMessageType};

pub(crate) fn get_messagetype(global_id: u16) -> FitMessageType {
    let mut name = format!("UNKNOWN_TYPE_{}", global_id);
    let mut fields: HashMap<u8, FitFieldType> = HashMap::new();
    match global_id {
        // Profile.xlsx row 3, file_id/0
        0 => {
            name = "file_id".into();
            fields.insert(0, FitFieldType::new((0, "type".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "manufacturer".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "product".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "serial_number".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "time_created".into(), None, None, None)),
            );
            fields.insert(5, FitFieldType::new((5, "number".into(), None, None, None)));
            fields.insert(
                8,
                FitFieldType::new((8, "product_name".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 14, file_creator/49
        49 => {
            name = "file_creator".into();
            fields.insert(
                0,
                FitFieldType::new((0, "software_version".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "hardware_version".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 18, timestamp_correlation/162
        162 => {
            name = "timestamp_correlation".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((
                    0,
                    "fractional_timestamp".into(),
                    Some(32768),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "system_timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                2,
                FitFieldType::new((
                    2,
                    "fractional_system_timestamp".into(),
                    Some(32768),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "local_timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "timestamp_ms".into(), None, None, Some("ms".into()))),
            );
            fields.insert(
                5,
                FitFieldType::new((
                    5,
                    "system_timestamp_ms".into(),
                    None,
                    None,
                    Some("ms".into()),
                )),
            );
        }
        // Profile.xlsx row 28, software/35
        35 => {
            name = "software".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "version".into(), Some(100), None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "part_number".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 33, slave_device/106
        106 => {
            name = "slave_device".into();
            fields.insert(
                0,
                FitFieldType::new((0, "manufacturer".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "product".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 39, capabilities/1
        1 => {
            name = "capabilities".into();
            fields.insert(
                0,
                FitFieldType::new((0, "languages".into(), None, None, None)),
            );
            fields.insert(1, FitFieldType::new((1, "sports".into(), None, None, None)));
            fields.insert(
                21,
                FitFieldType::new((21, "workouts_supported".into(), None, None, None)),
            );
            fields.insert(
                23,
                FitFieldType::new((23, "connectivity_supported".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 45, file_capabilities/37
        37 => {
            name = "file_capabilities".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(0, FitFieldType::new((0, "type".into(), None, None, None)));
            fields.insert(1, FitFieldType::new((1, "flags".into(), None, None, None)));
            fields.insert(
                2,
                FitFieldType::new((2, "directory".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "max_count".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "max_size".into(), None, None, Some("bytes".into()))),
            );
        }
        // Profile.xlsx row 53, mesg_capabilities/38
        38 => {
            name = "mesg_capabilities".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(0, FitFieldType::new((0, "file".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "mesg_num".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "count_type".into(), None, None, None)),
            );
            fields.insert(3, FitFieldType::new((3, "count".into(), None, None, None)));
        }
        // Profile.xlsx row 63, field_capabilities/39
        39 => {
            name = "field_capabilities".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(0, FitFieldType::new((0, "file".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "mesg_num".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "field_num".into(), None, None, None)),
            );
            fields.insert(3, FitFieldType::new((3, "count".into(), None, None, None)));
        }
        // Profile.xlsx row 71, device_settings/2
        2 => {
            name = "device_settings".into();
            fields.insert(
                0,
                FitFieldType::new((0, "active_time_zone".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "utc_offset".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "time_offset".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "time_mode".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((
                    5,
                    "time_zone_offset".into(),
                    Some(4),
                    None,
                    Some("hr".into()),
                )),
            );
            fields.insert(
                12,
                FitFieldType::new((12, "backlight_mode".into(), None, None, None)),
            );
            fields.insert(
                36,
                FitFieldType::new((36, "activity_tracker_enabled".into(), None, None, None)),
            );
            fields.insert(
                39,
                FitFieldType::new((39, "clock_time".into(), None, None, None)),
            );
            fields.insert(
                40,
                FitFieldType::new((40, "pages_enabled".into(), None, None, None)),
            );
            fields.insert(
                46,
                FitFieldType::new((46, "move_alert_enabled".into(), None, None, None)),
            );
            fields.insert(
                47,
                FitFieldType::new((47, "date_mode".into(), None, None, None)),
            );
            fields.insert(
                55,
                FitFieldType::new((55, "display_orientation".into(), None, None, None)),
            );
            fields.insert(
                56,
                FitFieldType::new((56, "mounting_side".into(), None, None, None)),
            );
            fields.insert(
                57,
                FitFieldType::new((57, "default_page".into(), None, None, None)),
            );
            fields.insert(
                58,
                FitFieldType::new((
                    58,
                    "autosync_min_steps".into(),
                    None,
                    None,
                    Some("steps".into()),
                )),
            );
            fields.insert(
                59,
                FitFieldType::new((
                    59,
                    "autosync_min_time".into(),
                    None,
                    None,
                    Some("minutes".into()),
                )),
            );
            fields.insert(
                80,
                FitFieldType::new((
                    80,
                    "lactate_threshold_autodetect_enabled".into(),
                    None,
                    None,
                    None,
                )),
            );
            fields.insert(
                86,
                FitFieldType::new((86, "ble_auto_upload_enabled".into(), None, None, None)),
            );
            fields.insert(
                89,
                FitFieldType::new((89, "auto_sync_frequency".into(), None, None, None)),
            );
            fields.insert(
                90,
                FitFieldType::new((90, "auto_activity_detect".into(), None, None, None)),
            );
            fields.insert(
                94,
                FitFieldType::new((94, "number_of_screens".into(), None, None, None)),
            );
            fields.insert(
                95,
                FitFieldType::new((
                    95,
                    "smart_notification_display_orientation".into(),
                    None,
                    None,
                    None,
                )),
            );
            fields.insert(
                134,
                FitFieldType::new((134, "tap_interface".into(), None, None, None)),
            );
            fields.insert(
                174,
                FitFieldType::new((174, "tap_sensitivity".into(), None, Some(1), None)),
            );
        }
        // Profile.xlsx row 96, user_profile/3
        3 => {
            name = "user_profile".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "friendly_name".into(), None, None, None)),
            );
            fields.insert(1, FitFieldType::new((1, "gender".into(), None, None, None)));
            fields.insert(
                2,
                FitFieldType::new((2, "age".into(), None, None, Some("years".to_owned()))),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "height".into(), Some(100), None, Some("m".into()))),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "weight".into(), Some(10), None, Some("kg".into()))),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "language".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "elev_setting".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "weight_setting".into(), None, None, None)),
            );
            fields.insert(
                8,
                FitFieldType::new((
                    8,
                    "resting_heart_rate".into(),
                    None,
                    None,
                    Some("bpm".into()),
                )),
            );
            fields.insert(
                9,
                FitFieldType::new((
                    9,
                    "default_max_running_heart_rate".into(),
                    None,
                    None,
                    Some("bpm".into()),
                )),
            );
            fields.insert(
                10,
                FitFieldType::new((
                    10,
                    "default_max_biking_heart_rate".into(),
                    None,
                    None,
                    Some("bpm".into()),
                )),
            );
            fields.insert(
                11,
                FitFieldType::new((
                    11,
                    "default_max_heart_rate".into(),
                    None,
                    None,
                    Some("bpm".into()),
                )),
            );
            fields.insert(
                12,
                FitFieldType::new((12, "hr_setting".into(), None, None, None)),
            );
            fields.insert(
                13,
                FitFieldType::new((13, "speed_setting".into(), None, None, None)),
            );
            fields.insert(
                14,
                FitFieldType::new((14, "dist_setting".into(), None, None, None)),
            );
            fields.insert(
                16,
                FitFieldType::new((16, "power_setting".into(), None, None, None)),
            );
            fields.insert(
                17,
                FitFieldType::new((17, "activity_class".into(), None, None, None)),
            );
            fields.insert(
                18,
                FitFieldType::new((18, "position_setting".into(), None, None, None)),
            );
            fields.insert(
                21,
                FitFieldType::new((21, "temperature_setting".into(), None, None, None)),
            );
            fields.insert(
                22,
                FitFieldType::new((22, "local_id".into(), None, None, None)),
            );
            fields.insert(
                23,
                FitFieldType::new((23, "global_id".into(), None, None, None)),
            );
            fields.insert(
                28,
                FitFieldType::new((28, "wake_time".into(), None, None, None)),
            );
            fields.insert(
                29,
                FitFieldType::new((29, "sleep_time".into(), None, None, None)),
            );
            fields.insert(
                30,
                FitFieldType::new((30, "height_setting".into(), None, None, None)),
            );
            fields.insert(
                31,
                FitFieldType::new((
                    31,
                    "user_running_step_length".into(),
                    Some(1000),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                32,
                FitFieldType::new((
                    32,
                    "user_walking_step_length".into(),
                    Some(1000),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                47,
                FitFieldType::new((47, "depth_setting".into(), None, None, None)),
            );
            fields.insert(
                49,
                FitFieldType::new((49, "dive_count".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 126, hrm_profile/4
        4 => {
            name = "hrm_profile".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "enabled".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "hrm_ant_id".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "log_hrv".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "hrm_ant_id_trans_type".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 133, sdm_profile/5
        5 => {
            name = "sdm_profile".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "enabled".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "sdm_ant_id".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "sdm_cal_factor".into(), Some(10), None, Some("%".into()))),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "odometer".into(), Some(100), None, Some("m".into()))),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "speed_source".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "sdm_ant_id_trans_type".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "odometer_rollover".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 143, bike_profile/6
        6 => {
            name = "bike_profile".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(0, FitFieldType::new((0, "name".into(), None, None, None)));
            fields.insert(1, FitFieldType::new((1, "sport".into(), None, None, None)));
            fields.insert(
                2,
                FitFieldType::new((2, "sub_sport".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "odometer".into(), Some(100), None, Some("m".into()))),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "bike_spd_ant_id".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "bike_cad_ant_id".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "bike_spdcad_ant_id".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "bike_power_ant_id".into(), None, None, None)),
            );
            fields.insert(
                8,
                FitFieldType::new((
                    8,
                    "custom_wheelsize".into(),
                    Some(1000),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                9,
                FitFieldType::new((
                    9,
                    "auto_wheelsize".into(),
                    Some(1000),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "bike_weight".into(), Some(10), None, Some("kg".into()))),
            );
            fields.insert(
                11,
                FitFieldType::new((
                    11,
                    "power_cal_factor".into(),
                    Some(10),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                12,
                FitFieldType::new((12, "auto_wheel_cal".into(), None, None, None)),
            );
            fields.insert(
                13,
                FitFieldType::new((13, "auto_power_zero".into(), None, None, None)),
            );
            fields.insert(14, FitFieldType::new((14, "id".into(), None, None, None)));
            fields.insert(
                15,
                FitFieldType::new((15, "spd_enabled".into(), None, None, None)),
            );
            fields.insert(
                16,
                FitFieldType::new((16, "cad_enabled".into(), None, None, None)),
            );
            fields.insert(
                17,
                FitFieldType::new((17, "spdcad_enabled".into(), None, None, None)),
            );
            fields.insert(
                18,
                FitFieldType::new((18, "power_enabled".into(), None, None, None)),
            );
            fields.insert(
                19,
                FitFieldType::new((
                    19,
                    "crank_length".into(),
                    Some(2),
                    Some(-110),
                    Some("mm".into()),
                )),
            );
            fields.insert(
                20,
                FitFieldType::new((20, "enabled".into(), None, None, None)),
            );
            fields.insert(
                21,
                FitFieldType::new((21, "bike_spd_ant_id_trans_type".into(), None, None, None)),
            );
            fields.insert(
                22,
                FitFieldType::new((22, "bike_cad_ant_id_trans_type".into(), None, None, None)),
            );
            fields.insert(
                23,
                FitFieldType::new((23, "bike_spdcad_ant_id_trans_type".into(), None, None, None)),
            );
            fields.insert(
                24,
                FitFieldType::new((24, "bike_power_ant_id_trans_type".into(), None, None, None)),
            );
            fields.insert(
                37,
                FitFieldType::new((37, "odometer_rollover".into(), None, None, None)),
            );
            fields.insert(
                38,
                FitFieldType::new((38, "front_gear_num".into(), None, None, None)),
            );
            fields.insert(
                39,
                FitFieldType::new((39, "front_gear".into(), None, None, None)),
            );
            fields.insert(
                40,
                FitFieldType::new((40, "rear_gear_num".into(), None, None, None)),
            );
            fields.insert(
                41,
                FitFieldType::new((41, "rear_gear".into(), None, None, None)),
            );
            fields.insert(
                44,
                FitFieldType::new((44, "shimano_di2_enabled".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 177, connectivity/127
        127 => {
            name = "connectivity".into();
            fields.insert(
                0,
                FitFieldType::new((0, "bluetooth_enabled".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "bluetooth_le_enabled".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "ant_enabled".into(), None, None, None)),
            );
            fields.insert(3, FitFieldType::new((3, "name".into(), None, None, None)));
            fields.insert(
                4,
                FitFieldType::new((4, "live_tracking_enabled".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "weather_conditions_enabled".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "weather_alerts_enabled".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "auto_activity_upload_enabled".into(), None, None, None)),
            );
            fields.insert(
                8,
                FitFieldType::new((8, "course_download_enabled".into(), None, None, None)),
            );
            fields.insert(
                9,
                FitFieldType::new((9, "workout_download_enabled".into(), None, None, None)),
            );
            fields.insert(
                10,
                FitFieldType::new((
                    10,
                    "gps_ephemeris_download_enabled".into(),
                    None,
                    None,
                    None,
                )),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "incident_detection_enabled".into(), None, None, None)),
            );
            fields.insert(
                12,
                FitFieldType::new((12, "grouptrack_enabled".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 191, watchface_settings/159
        159 => {
            name = "watchface_settings".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(0, FitFieldType::new((0, "mode".into(), None, None, None)));
            fields.insert(1, FitFieldType::new((1, "layout".into(), None, None, None)));
        }
        // Profile.xlsx row 197, ohr_settings/188
        188 => {
            name = "ohr_settings".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "enabled".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 201, zones_target/7
        7 => {
            name = "zones_target".into();
            fields.insert(
                1,
                FitFieldType::new((1, "max_heart_rate".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "threshold_heart_rate".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "functional_threshold_power".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "hr_calc_type".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "pwr_calc_type".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 208, sport/12
        12 => {
            name = "sport".into();
            fields.insert(0, FitFieldType::new((0, "sport".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "sub_sport".into(), None, None, None)),
            );
            fields.insert(3, FitFieldType::new((3, "name".into(), None, None, None)));
        }
        // Profile.xlsx row 212, hr_zone/8
        8 => {
            name = "hr_zone".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "high_bpm".into(), None, None, Some("bpm".to_owned()))),
            );
            fields.insert(2, FitFieldType::new((2, "name".into(), None, None, None)));
        }
        // Profile.xlsx row 217, speed_zone/53
        53 => {
            name = "speed_zone".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "high_value".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(1, FitFieldType::new((1, "name".into(), None, None, None)));
        }
        // Profile.xlsx row 222, cadence_zone/131
        131 => {
            name = "cadence_zone".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "high_value".into(), None, None, Some("rpm".into()))),
            );
            fields.insert(1, FitFieldType::new((1, "name".into(), None, None, None)));
        }
        // Profile.xlsx row 227, power_zone/9
        9 => {
            name = "power_zone".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "high_value".into(), None, None, Some("watts".into()))),
            );
            fields.insert(2, FitFieldType::new((2, "name".into(), None, None, None)));
        }
        // Profile.xlsx row 232, met_zone/10
        10 => {
            name = "met_zone".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "high_bpm".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((
                    2,
                    "calories".into(),
                    Some(10),
                    None,
                    Some("kcal / min".into()),
                )),
            );
            fields.insert(
                3,
                FitFieldType::new((
                    3,
                    "fat_calories".into(),
                    Some(10),
                    None,
                    Some("kcal / min".into()),
                )),
            );
        }
        // Profile.xlsx row 238, dive_settings/258
        258 => {
            name = "dive_settings".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(0, FitFieldType::new((0, "name".into(), None, None, None)));
            fields.insert(1, FitFieldType::new((1, "model".into(), None, None, None)));
            fields.insert(
                2,
                FitFieldType::new((2, "gf_low".into(), None, None, Some("percent".into()))),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "gf_high".into(), None, None, Some("percent".into()))),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "water_type".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "water_density".into(), None, None, Some("kg/m^3".into()))),
            );
            fields.insert(
                6,
                FitFieldType::new((
                    6,
                    "po2_warn".into(),
                    Some(100),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                7,
                FitFieldType::new((
                    7,
                    "po2_critical".into(),
                    Some(100),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                8,
                FitFieldType::new((
                    8,
                    "po2_deco".into(),
                    Some(100),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                9,
                FitFieldType::new((9, "safety_stop_enabled".into(), None, None, None)),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "bottom_depth".into(), None, None, None)),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "bottom_time".into(), None, None, None)),
            );
            fields.insert(
                12,
                FitFieldType::new((12, "apnea_countdown_enabled".into(), None, None, None)),
            );
            fields.insert(
                13,
                FitFieldType::new((13, "apnea_countdown_time".into(), None, None, None)),
            );
            fields.insert(
                14,
                FitFieldType::new((14, "backlight_mode".into(), None, None, None)),
            );
            fields.insert(
                15,
                FitFieldType::new((15, "backlight_brightness".into(), None, None, None)),
            );
            fields.insert(
                16,
                FitFieldType::new((16, "backlight_timeout".into(), None, None, None)),
            );
            fields.insert(
                17,
                FitFieldType::new((
                    17,
                    "repeat_dive_interval".into(),
                    Some(1),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                18,
                FitFieldType::new((
                    18,
                    "safety_stop_time".into(),
                    Some(1),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                19,
                FitFieldType::new((19, "heart_rate_source_type".into(), None, None, None)),
            );
            fields.insert(
                20,
                FitFieldType::new((20, "heart_rate_source".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 263, dive_alarm/262
        262 => {
            name = "dive_alarm".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "depth".into(), Some(1000), None, Some("m".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "time".into(), Some(1), None, Some("s".to_owned()))),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "enabled".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "alarm_type".into(), None, None, None)),
            );
            fields.insert(4, FitFieldType::new((4, "sound".into(), None, None, None)));
            fields.insert(
                5,
                FitFieldType::new((5, "dive_types".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 271, dive_gas/259
        259 => {
            name = "dive_gas".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((
                    0,
                    "helium_content".into(),
                    None,
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                1,
                FitFieldType::new((
                    1,
                    "oxygen_content".into(),
                    None,
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(2, FitFieldType::new((2, "status".into(), None, None, None)));
        }
        // Profile.xlsx row 277, goal/15
        15 => {
            name = "goal".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(0, FitFieldType::new((0, "sport".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "sub_sport".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "start_date".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "end_date".into(), None, None, None)),
            );
            fields.insert(4, FitFieldType::new((4, "type".into(), None, None, None)));
            fields.insert(5, FitFieldType::new((5, "value".into(), None, None, None)));
            fields.insert(6, FitFieldType::new((6, "repeat".into(), None, None, None)));
            fields.insert(
                7,
                FitFieldType::new((7, "target_value".into(), None, None, None)),
            );
            fields.insert(
                8,
                FitFieldType::new((8, "recurrence".into(), None, None, None)),
            );
            fields.insert(
                9,
                FitFieldType::new((9, "recurrence_value".into(), None, None, None)),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "enabled".into(), None, None, None)),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "source".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 293, activity/34
        34 => {
            name = "activity".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((
                    0,
                    "total_timer_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "num_sessions".into(), None, None, None)),
            );
            fields.insert(2, FitFieldType::new((2, "type".into(), None, None, None)));
            fields.insert(3, FitFieldType::new((3, "event".into(), None, None, None)));
            fields.insert(
                4,
                FitFieldType::new((4, "event_type".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "local_timestamp".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "event_group".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 303, session/18
        18 => {
            name = "session".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(0, FitFieldType::new((0, "event".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "event_type".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "start_time".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((
                    3,
                    "start_position_lat".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                4,
                FitFieldType::new((
                    4,
                    "start_position_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(5, FitFieldType::new((5, "sport".into(), None, None, None)));
            fields.insert(
                6,
                FitFieldType::new((6, "sub_sport".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((
                    7,
                    "total_elapsed_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                8,
                FitFieldType::new((
                    8,
                    "total_timer_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                9,
                FitFieldType::new((
                    9,
                    "total_distance".into(),
                    Some(100),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "total_cycles".into(), None, None, Some("cycles".into()))),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "total_calories".into(), None, None, Some("kcal".into()))),
            );
            fields.insert(
                13,
                FitFieldType::new((
                    13,
                    "total_fat_calories".into(),
                    None,
                    None,
                    Some("kcal".into()),
                )),
            );
            fields.insert(
                14,
                FitFieldType::new((14, "avg_speed".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(
                15,
                FitFieldType::new((15, "max_speed".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(
                16,
                FitFieldType::new((16, "avg_heart_rate".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                17,
                FitFieldType::new((17, "max_heart_rate".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                18,
                FitFieldType::new((18, "avg_cadence".into(), None, None, Some("rpm".into()))),
            );
            fields.insert(
                19,
                FitFieldType::new((19, "max_cadence".into(), None, None, Some("rpm".into()))),
            );
            fields.insert(
                20,
                FitFieldType::new((20, "avg_power".into(), None, None, Some("watts".into()))),
            );
            fields.insert(
                21,
                FitFieldType::new((21, "max_power".into(), None, None, Some("watts".into()))),
            );
            fields.insert(
                22,
                FitFieldType::new((22, "total_ascent".into(), None, None, Some("m".into()))),
            );
            fields.insert(
                23,
                FitFieldType::new((23, "total_descent".into(), None, None, Some("m".into()))),
            );
            fields.insert(
                24,
                FitFieldType::new((24, "total_training_effect".into(), Some(10), None, None)),
            );
            fields.insert(
                25,
                FitFieldType::new((25, "first_lap_index".into(), None, None, None)),
            );
            fields.insert(
                26,
                FitFieldType::new((26, "num_laps".into(), None, None, None)),
            );
            fields.insert(
                27,
                FitFieldType::new((27, "event_group".into(), None, None, None)),
            );
            fields.insert(
                28,
                FitFieldType::new((28, "trigger".into(), None, None, None)),
            );
            fields.insert(
                29,
                FitFieldType::new((29, "nec_lat".into(), None, None, Some("semicircles".into()))),
            );
            fields.insert(
                30,
                FitFieldType::new((
                    30,
                    "nec_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                31,
                FitFieldType::new((31, "swc_lat".into(), None, None, Some("semicircles".into()))),
            );
            fields.insert(
                32,
                FitFieldType::new((
                    32,
                    "swc_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                33,
                FitFieldType::new((33, "num_lengths".into(), None, None, Some("lengths".into()))),
            );
            fields.insert(
                34,
                FitFieldType::new((
                    34,
                    "normalized_power".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                35,
                FitFieldType::new((
                    35,
                    "training_stress_score".into(),
                    Some(10),
                    None,
                    Some("tss".into()),
                )),
            );
            fields.insert(
                36,
                FitFieldType::new((
                    36,
                    "intensity_factor".into(),
                    Some(1000),
                    None,
                    Some("if".into()),
                )),
            );
            fields.insert(
                37,
                FitFieldType::new((37, "left_right_balance".into(), None, None, None)),
            );
            fields.insert(
                41,
                FitFieldType::new((
                    41,
                    "avg_stroke_count".into(),
                    Some(10),
                    None,
                    Some("strokes/lap".into()),
                )),
            );
            fields.insert(
                42,
                FitFieldType::new((
                    42,
                    "avg_stroke_distance".into(),
                    Some(100),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                43,
                FitFieldType::new((
                    43,
                    "swim_stroke".into(),
                    None,
                    None,
                    Some("swim_stroke".into()),
                )),
            );
            fields.insert(
                44,
                FitFieldType::new((44, "pool_length".into(), Some(100), None, Some("m".into()))),
            );
            fields.insert(
                45,
                FitFieldType::new((
                    45,
                    "threshold_power".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                46,
                FitFieldType::new((46, "pool_length_unit".into(), None, None, None)),
            );
            fields.insert(
                47,
                FitFieldType::new((
                    47,
                    "num_active_lengths".into(),
                    None,
                    None,
                    Some("lengths".into()),
                )),
            );
            fields.insert(
                48,
                FitFieldType::new((48, "total_work".into(), None, None, Some("J".into()))),
            );
            fields.insert(
                49,
                FitFieldType::new((
                    49,
                    "avg_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                50,
                FitFieldType::new((
                    50,
                    "max_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                51,
                FitFieldType::new((51, "gps_accuracy".into(), None, None, Some("m".into()))),
            );
            fields.insert(
                52,
                FitFieldType::new((52, "avg_grade".into(), Some(100), None, Some("%".into()))),
            );
            fields.insert(
                53,
                FitFieldType::new((
                    53,
                    "avg_pos_grade".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                54,
                FitFieldType::new((
                    54,
                    "avg_neg_grade".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                55,
                FitFieldType::new((
                    55,
                    "max_pos_grade".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                56,
                FitFieldType::new((
                    56,
                    "max_neg_grade".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                57,
                FitFieldType::new((57, "avg_temperature".into(), None, None, Some("C".into()))),
            );
            fields.insert(
                58,
                FitFieldType::new((58, "max_temperature".into(), None, None, Some("C".into()))),
            );
            fields.insert(
                59,
                FitFieldType::new((
                    59,
                    "total_moving_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                60,
                FitFieldType::new((
                    60,
                    "avg_pos_vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                61,
                FitFieldType::new((
                    61,
                    "avg_neg_vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                62,
                FitFieldType::new((
                    62,
                    "max_pos_vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                63,
                FitFieldType::new((
                    63,
                    "max_neg_vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                64,
                FitFieldType::new((64, "min_heart_rate".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                65,
                FitFieldType::new((
                    65,
                    "time_in_hr_zone".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                66,
                FitFieldType::new((
                    66,
                    "time_in_speed_zone".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                67,
                FitFieldType::new((
                    67,
                    "time_in_cadence_zone".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                68,
                FitFieldType::new((
                    68,
                    "time_in_power_zone".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                69,
                FitFieldType::new((
                    69,
                    "avg_lap_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                70,
                FitFieldType::new((70, "best_lap_index".into(), None, None, None)),
            );
            fields.insert(
                71,
                FitFieldType::new((
                    71,
                    "min_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                82,
                FitFieldType::new((82, "player_score".into(), None, None, None)),
            );
            fields.insert(
                83,
                FitFieldType::new((83, "opponent_score".into(), None, None, None)),
            );
            fields.insert(
                84,
                FitFieldType::new((84, "opponent_name".into(), None, None, None)),
            );
            fields.insert(
                85,
                FitFieldType::new((85, "stroke_count".into(), None, None, Some("counts".into()))),
            );
            fields.insert(
                86,
                FitFieldType::new((86, "zone_count".into(), None, None, Some("counts".into()))),
            );
            fields.insert(
                87,
                FitFieldType::new((
                    87,
                    "max_ball_speed".into(),
                    Some(100),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                88,
                FitFieldType::new((
                    88,
                    "avg_ball_speed".into(),
                    Some(100),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                89,
                FitFieldType::new((
                    89,
                    "avg_vertical_oscillation".into(),
                    Some(10),
                    None,
                    Some("mm".into()),
                )),
            );
            fields.insert(
                90,
                FitFieldType::new((
                    90,
                    "avg_stance_time_percent".into(),
                    Some(100),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                91,
                FitFieldType::new((
                    91,
                    "avg_stance_time".into(),
                    Some(10),
                    None,
                    Some("ms".into()),
                )),
            );
            fields.insert(
                92,
                FitFieldType::new((
                    92,
                    "avg_fractional_cadence".into(),
                    Some(128),
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                93,
                FitFieldType::new((
                    93,
                    "max_fractional_cadence".into(),
                    Some(128),
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                94,
                FitFieldType::new((
                    94,
                    "total_fractional_cycles".into(),
                    Some(128),
                    None,
                    Some("cycles".into()),
                )),
            );
            fields.insert(
                95,
                FitFieldType::new((
                    95,
                    "avg_total_hemoglobin_conc".into(),
                    Some(100),
                    None,
                    Some("g/dL".into()),
                )),
            );
            fields.insert(
                96,
                FitFieldType::new((
                    96,
                    "min_total_hemoglobin_conc".into(),
                    Some(100),
                    None,
                    Some("g/dL".into()),
                )),
            );
            fields.insert(
                97,
                FitFieldType::new((
                    97,
                    "max_total_hemoglobin_conc".into(),
                    Some(100),
                    None,
                    Some("g/dL".into()),
                )),
            );
            fields.insert(
                98,
                FitFieldType::new((
                    98,
                    "avg_saturated_hemoglobin_percent".into(),
                    Some(10),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                99,
                FitFieldType::new((
                    99,
                    "min_saturated_hemoglobin_percent".into(),
                    Some(10),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                100,
                FitFieldType::new((
                    100,
                    "max_saturated_hemoglobin_percent".into(),
                    Some(10),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                101,
                FitFieldType::new((
                    101,
                    "avg_left_torque_effectiveness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                102,
                FitFieldType::new((
                    102,
                    "avg_right_torque_effectiveness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                103,
                FitFieldType::new((
                    103,
                    "avg_left_pedal_smoothness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                104,
                FitFieldType::new((
                    104,
                    "avg_right_pedal_smoothness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                105,
                FitFieldType::new((
                    105,
                    "avg_combined_pedal_smoothness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                111,
                FitFieldType::new((111, "sport_index".into(), None, None, None)),
            );
            fields.insert(
                112,
                FitFieldType::new((
                    112,
                    "time_standing".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                113,
                FitFieldType::new((113, "stand_count".into(), None, None, None)),
            );
            fields.insert(
                114,
                FitFieldType::new((114, "avg_left_pco".into(), None, None, Some("mm".into()))),
            );
            fields.insert(
                115,
                FitFieldType::new((115, "avg_right_pco".into(), None, None, Some("mm".into()))),
            );
            // fields.insert(116, FitFieldType::new((116, "avg_left_power_phase".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            // fields.insert(117, FitFieldType::new((117, "avg_left_power_phase_peak".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            // fields.insert(118, FitFieldType::new((118, "avg_right_power_phase".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            // fields.insert(119, FitFieldType::new((119, "avg_right_power_phase_peak".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            fields.insert(
                120,
                FitFieldType::new((
                    120,
                    "avg_power_position".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                121,
                FitFieldType::new((
                    121,
                    "max_power_position".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                122,
                FitFieldType::new((
                    122,
                    "avg_cadence_position".into(),
                    None,
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                123,
                FitFieldType::new((
                    123,
                    "max_cadence_position".into(),
                    None,
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                124,
                FitFieldType::new((
                    124,
                    "enhanced_avg_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                125,
                FitFieldType::new((
                    125,
                    "enhanced_max_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                126,
                FitFieldType::new((
                    126,
                    "enhanced_avg_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                127,
                FitFieldType::new((
                    127,
                    "enhanced_min_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                128,
                FitFieldType::new((
                    128,
                    "enhanced_max_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                129,
                FitFieldType::new((
                    129,
                    "avg_lev_motor_power".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                130,
                FitFieldType::new((
                    130,
                    "max_lev_motor_power".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                131,
                FitFieldType::new((
                    131,
                    "lev_battery_consumption".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                132,
                FitFieldType::new((
                    132,
                    "avg_vertical_ratio".into(),
                    Some(100),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                133,
                FitFieldType::new((
                    133,
                    "avg_stance_time_balance".into(),
                    Some(100),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                134,
                FitFieldType::new((
                    134,
                    "avg_step_length".into(),
                    Some(10),
                    None,
                    Some("mm".into()),
                )),
            );
            fields.insert(
                137,
                FitFieldType::new((
                    137,
                    "total_anaerobic_training_effect".into(),
                    Some(10),
                    None,
                    None,
                )),
            );
            fields.insert(
                139,
                FitFieldType::new((139, "avg_vam".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(
                181,
                FitFieldType::new((181, "total_grit".into(), None, None, Some("kGrit".into()))),
            );
            fields.insert(
                182,
                FitFieldType::new((182, "total_flow".into(), None, None, Some("Flow".into()))),
            );
            fields.insert(
                183,
                FitFieldType::new((183, "jump_count".into(), None, None, None)),
            );
            fields.insert(
                186,
                FitFieldType::new((186, "avg_grit".into(), None, None, Some("kGrit".into()))),
            );
            fields.insert(
                187,
                FitFieldType::new((187, "avg_flow".into(), None, None, Some("Flow".into()))),
            );
            fields.insert(
                199,
                FitFieldType::new((
                    199,
                    "total_fractional_ascent".into(),
                    Some(100),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                200,
                FitFieldType::new((
                    200,
                    "total_fractional_descent".into(),
                    Some(100),
                    None,
                    Some("m".into()),
                )),
            );
        }
        // Profile.xlsx row 435, lap/19
        19 => {
            name = "lap".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(0, FitFieldType::new((0, "event".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "event_type".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "start_time".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((
                    3,
                    "start_position_lat".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                4,
                FitFieldType::new((
                    4,
                    "start_position_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                5,
                FitFieldType::new((
                    5,
                    "end_position_lat".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                6,
                FitFieldType::new((
                    6,
                    "end_position_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                7,
                FitFieldType::new((
                    7,
                    "total_elapsed_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                8,
                FitFieldType::new((
                    8,
                    "total_timer_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                9,
                FitFieldType::new((
                    9,
                    "total_distance".into(),
                    Some(100),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "total_cycles".into(), None, None, Some("cycles".into()))),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "total_calories".into(), None, None, Some("kcal".into()))),
            );
            fields.insert(
                12,
                FitFieldType::new((
                    12,
                    "total_fat_calories".into(),
                    None,
                    None,
                    Some("kcal".into()),
                )),
            );
            fields.insert(
                13,
                FitFieldType::new((13, "avg_speed".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(
                14,
                FitFieldType::new((14, "max_speed".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(
                15,
                FitFieldType::new((15, "avg_heart_rate".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                16,
                FitFieldType::new((16, "max_heart_rate".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                17,
                FitFieldType::new((17, "avg_cadence".into(), None, None, Some("rpm".into()))),
            );
            fields.insert(
                18,
                FitFieldType::new((18, "max_cadence".into(), None, None, Some("rpm".into()))),
            );
            fields.insert(
                19,
                FitFieldType::new((19, "avg_power".into(), None, None, Some("watts".into()))),
            );
            fields.insert(
                20,
                FitFieldType::new((20, "max_power".into(), None, None, Some("watts".into()))),
            );
            fields.insert(
                21,
                FitFieldType::new((21, "total_ascent".into(), None, None, Some("m".into()))),
            );
            fields.insert(
                22,
                FitFieldType::new((22, "total_descent".into(), None, None, Some("m".into()))),
            );
            fields.insert(
                23,
                FitFieldType::new((23, "intensity".into(), None, None, None)),
            );
            fields.insert(
                24,
                FitFieldType::new((24, "lap_trigger".into(), None, None, None)),
            );
            fields.insert(
                25,
                FitFieldType::new((25, "sport".into(), None, None, None)),
            );
            fields.insert(
                26,
                FitFieldType::new((26, "event_group".into(), None, None, None)),
            );
            fields.insert(
                32,
                FitFieldType::new((32, "num_lengths".into(), None, None, Some("lengths".into()))),
            );
            fields.insert(
                33,
                FitFieldType::new((
                    33,
                    "normalized_power".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                34,
                FitFieldType::new((34, "left_right_balance".into(), None, None, None)),
            );
            fields.insert(
                35,
                FitFieldType::new((35, "first_length_index".into(), None, None, None)),
            );
            fields.insert(
                37,
                FitFieldType::new((
                    37,
                    "avg_stroke_distance".into(),
                    Some(100),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                38,
                FitFieldType::new((38, "swim_stroke".into(), None, None, None)),
            );
            fields.insert(
                39,
                FitFieldType::new((39, "sub_sport".into(), None, None, None)),
            );
            fields.insert(
                40,
                FitFieldType::new((
                    40,
                    "num_active_lengths".into(),
                    None,
                    None,
                    Some("lengths".into()),
                )),
            );
            fields.insert(
                41,
                FitFieldType::new((41, "total_work".into(), None, None, Some("J".into()))),
            );
            fields.insert(
                42,
                FitFieldType::new((
                    42,
                    "avg_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                43,
                FitFieldType::new((
                    43,
                    "max_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                44,
                FitFieldType::new((44, "gps_accuracy".into(), None, None, Some("m".into()))),
            );
            fields.insert(
                45,
                FitFieldType::new((45, "avg_grade".into(), Some(100), None, Some("%".into()))),
            );
            fields.insert(
                46,
                FitFieldType::new((
                    46,
                    "avg_pos_grade".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                47,
                FitFieldType::new((
                    47,
                    "avg_neg_grade".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                48,
                FitFieldType::new((
                    48,
                    "max_pos_grade".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                49,
                FitFieldType::new((
                    49,
                    "max_neg_grade".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                50,
                FitFieldType::new((50, "avg_temperature".into(), None, None, Some("C".into()))),
            );
            fields.insert(
                51,
                FitFieldType::new((51, "max_temperature".into(), None, None, Some("C".into()))),
            );
            fields.insert(
                52,
                FitFieldType::new((
                    52,
                    "total_moving_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                53,
                FitFieldType::new((
                    53,
                    "avg_pos_vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                54,
                FitFieldType::new((
                    54,
                    "avg_neg_vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                55,
                FitFieldType::new((
                    55,
                    "max_pos_vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                56,
                FitFieldType::new((
                    56,
                    "max_neg_vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                57,
                FitFieldType::new((
                    57,
                    "time_in_hr_zone".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                58,
                FitFieldType::new((
                    58,
                    "time_in_speed_zone".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                59,
                FitFieldType::new((
                    59,
                    "time_in_cadence_zone".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                60,
                FitFieldType::new((
                    60,
                    "time_in_power_zone".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                61,
                FitFieldType::new((61, "repetition_num".into(), None, None, None)),
            );
            fields.insert(
                62,
                FitFieldType::new((
                    62,
                    "min_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                63,
                FitFieldType::new((63, "min_heart_rate".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                71,
                FitFieldType::new((71, "wkt_step_index".into(), None, None, None)),
            );
            fields.insert(
                74,
                FitFieldType::new((74, "opponent_score".into(), None, None, None)),
            );
            fields.insert(
                75,
                FitFieldType::new((75, "stroke_count".into(), None, None, Some("counts".into()))),
            );
            fields.insert(
                76,
                FitFieldType::new((76, "zone_count".into(), None, None, Some("counts".into()))),
            );
            fields.insert(
                77,
                FitFieldType::new((
                    77,
                    "avg_vertical_oscillation".into(),
                    Some(10),
                    None,
                    Some("mm".into()),
                )),
            );
            fields.insert(
                78,
                FitFieldType::new((
                    78,
                    "avg_stance_time_percent".into(),
                    Some(100),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                79,
                FitFieldType::new((
                    79,
                    "avg_stance_time".into(),
                    Some(10),
                    None,
                    Some("ms".into()),
                )),
            );
            fields.insert(
                80,
                FitFieldType::new((
                    80,
                    "avg_fractional_cadence".into(),
                    Some(128),
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                81,
                FitFieldType::new((
                    81,
                    "max_fractional_cadence".into(),
                    Some(128),
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                82,
                FitFieldType::new((
                    82,
                    "total_fractional_cycles".into(),
                    Some(128),
                    None,
                    Some("cycles".into()),
                )),
            );
            fields.insert(
                83,
                FitFieldType::new((83, "player_score".into(), None, None, None)),
            );
            fields.insert(
                84,
                FitFieldType::new((
                    84,
                    "avg_total_hemoglobin_conc".into(),
                    Some(100),
                    None,
                    Some("g/dL".into()),
                )),
            );
            fields.insert(
                85,
                FitFieldType::new((
                    85,
                    "min_total_hemoglobin_conc".into(),
                    Some(100),
                    None,
                    Some("g/dL".into()),
                )),
            );
            fields.insert(
                86,
                FitFieldType::new((
                    86,
                    "max_total_hemoglobin_conc".into(),
                    Some(100),
                    None,
                    Some("g/dL".into()),
                )),
            );
            fields.insert(
                87,
                FitFieldType::new((
                    87,
                    "avg_saturated_hemoglobin_percent".into(),
                    Some(10),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                88,
                FitFieldType::new((
                    88,
                    "min_saturated_hemoglobin_percent".into(),
                    Some(10),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                89,
                FitFieldType::new((
                    89,
                    "max_saturated_hemoglobin_percent".into(),
                    Some(10),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                91,
                FitFieldType::new((
                    91,
                    "avg_left_torque_effectiveness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                92,
                FitFieldType::new((
                    92,
                    "avg_right_torque_effectiveness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                93,
                FitFieldType::new((
                    93,
                    "avg_left_pedal_smoothness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                94,
                FitFieldType::new((
                    94,
                    "avg_right_pedal_smoothness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                95,
                FitFieldType::new((
                    95,
                    "avg_combined_pedal_smoothness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                98,
                FitFieldType::new((
                    98,
                    "time_standing".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                99,
                FitFieldType::new((99, "stand_count".into(), None, None, None)),
            );
            fields.insert(
                100,
                FitFieldType::new((100, "avg_left_pco".into(), None, None, Some("mm".into()))),
            );
            fields.insert(
                101,
                FitFieldType::new((101, "avg_right_pco".into(), None, None, Some("mm".into()))),
            );
            // fields.insert(102, FitFieldType::new((102, "avg_left_power_phase".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            // fields.insert(103, FitFieldType::new((103, "avg_left_power_phase_peak".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            // fields.insert(104, FitFieldType::new((104, "avg_right_power_phase".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            // fields.insert(105, FitFieldType::new((105, "avg_right_power_phase_peak".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            fields.insert(
                106,
                FitFieldType::new((
                    106,
                    "avg_power_position".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                107,
                FitFieldType::new((
                    107,
                    "max_power_position".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                108,
                FitFieldType::new((
                    108,
                    "avg_cadence_position".into(),
                    None,
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                109,
                FitFieldType::new((
                    109,
                    "max_cadence_position".into(),
                    None,
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                110,
                FitFieldType::new((
                    110,
                    "enhanced_avg_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                111,
                FitFieldType::new((
                    111,
                    "enhanced_max_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                112,
                FitFieldType::new((
                    112,
                    "enhanced_avg_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                113,
                FitFieldType::new((
                    113,
                    "enhanced_min_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                114,
                FitFieldType::new((
                    114,
                    "enhanced_max_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                115,
                FitFieldType::new((
                    115,
                    "avg_lev_motor_power".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                116,
                FitFieldType::new((
                    116,
                    "max_lev_motor_power".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                117,
                FitFieldType::new((
                    117,
                    "lev_battery_consumption".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                118,
                FitFieldType::new((
                    118,
                    "avg_vertical_ratio".into(),
                    Some(100),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                119,
                FitFieldType::new((
                    119,
                    "avg_stance_time_balance".into(),
                    Some(100),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                120,
                FitFieldType::new((
                    120,
                    "avg_step_length".into(),
                    Some(10),
                    None,
                    Some("mm".into()),
                )),
            );
            fields.insert(
                121,
                FitFieldType::new((121, "avg_vam".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(
                149,
                FitFieldType::new((149, "total_grit".into(), None, None, Some("kGrit".into()))),
            );
            fields.insert(
                150,
                FitFieldType::new((150, "total_flow".into(), None, None, Some("Flow".into()))),
            );
            fields.insert(
                151,
                FitFieldType::new((151, "jump_count".into(), None, None, None)),
            );
            fields.insert(
                153,
                FitFieldType::new((153, "avg_grit".into(), None, None, Some("kGrit".into()))),
            );
            fields.insert(
                154,
                FitFieldType::new((154, "avg_flow".into(), None, None, Some("Flow".into()))),
            );
            fields.insert(
                156,
                FitFieldType::new((
                    156,
                    "total_fractional_ascent".into(),
                    Some(100),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                157,
                FitFieldType::new((
                    157,
                    "total_fractional_descent".into(),
                    Some(100),
                    None,
                    Some("m".into()),
                )),
            );
        }
        // Profile.xlsx row 553, length/101
        101 => {
            name = "length".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, None)),
            );
            fields.insert(0, FitFieldType::new((0, "event".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "event_type".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "start_time".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((
                    3,
                    "total_elapsed_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                4,
                FitFieldType::new((
                    4,
                    "total_timer_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                5,
                FitFieldType::new((
                    5,
                    "total_strokes".into(),
                    None,
                    None,
                    Some("strokes".into()),
                )),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "avg_speed".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(
                7,
                FitFieldType::new((
                    7,
                    "swim_stroke".into(),
                    None,
                    None,
                    Some("swim_stroke".into()),
                )),
            );
            fields.insert(
                9,
                FitFieldType::new((
                    9,
                    "avg_swimming_cadence".into(),
                    None,
                    None,
                    Some("strokes/min".into()),
                )),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "event_group".into(), None, None, None)),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "total_calories".into(), None, None, Some("kcal".into()))),
            );
            fields.insert(
                12,
                FitFieldType::new((12, "length_type".into(), None, None, None)),
            );
            fields.insert(
                18,
                FitFieldType::new((18, "player_score".into(), None, None, None)),
            );
            fields.insert(
                19,
                FitFieldType::new((19, "opponent_score".into(), None, None, None)),
            );
            fields.insert(
                20,
                FitFieldType::new((20, "stroke_count".into(), None, None, Some("counts".into()))),
            );
            fields.insert(
                21,
                FitFieldType::new((21, "zone_count".into(), None, None, Some("counts".into()))),
            );
        }
        // Profile.xlsx row 573, record/20
        20 => {
            name = "record".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((
                    0,
                    "position_lat".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                1,
                FitFieldType::new((
                    1,
                    "position_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "altitude".into(), Some(5), Some(500), Some("m".into()))),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "heart_rate".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "cadence".into(), None, None, Some("rpm".to_owned()))),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "distance".into(), Some(100), None, Some("m".into()))),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "speed".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "power".into(), None, None, Some("watts".to_owned()))),
            );
            // fields.insert(8, FitFieldType::new((8, "compressed_speed_distance".into(), Some(100,16), None, Some("m/s,m".to_owned()))));
            fields.insert(
                9,
                FitFieldType::new((9, "grade".into(), Some(100), None, Some("%".to_owned()))),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "resistance".into(), None, None, None)),
            );
            fields.insert(
                11,
                FitFieldType::new((
                    11,
                    "time_from_course".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                12,
                FitFieldType::new((12, "cycle_length".into(), Some(100), None, Some("m".into()))),
            );
            fields.insert(
                13,
                FitFieldType::new((13, "temperature".into(), None, None, Some("C".into()))),
            );
            fields.insert(
                17,
                FitFieldType::new((17, "speed_1s".into(), Some(16), None, Some("m/s".into()))),
            );
            fields.insert(
                18,
                FitFieldType::new((18, "cycles".into(), None, None, Some("cycles".into()))),
            );
            fields.insert(
                19,
                FitFieldType::new((19, "total_cycles".into(), None, None, Some("cycles".into()))),
            );
            fields.insert(
                28,
                FitFieldType::new((
                    28,
                    "compressed_accumulated_power".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                29,
                FitFieldType::new((
                    29,
                    "accumulated_power".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                30,
                FitFieldType::new((30, "left_right_balance".into(), None, None, None)),
            );
            fields.insert(
                31,
                FitFieldType::new((31, "gps_accuracy".into(), None, None, Some("m".into()))),
            );
            fields.insert(
                32,
                FitFieldType::new((
                    32,
                    "vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                33,
                FitFieldType::new((33, "calories".into(), None, None, Some("kcal".into()))),
            );
            fields.insert(
                39,
                FitFieldType::new((
                    39,
                    "vertical_oscillation".into(),
                    Some(10),
                    None,
                    Some("mm".into()),
                )),
            );
            fields.insert(
                40,
                FitFieldType::new((
                    40,
                    "stance_time_percent".into(),
                    Some(100),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                41,
                FitFieldType::new((41, "stance_time".into(), Some(10), None, Some("ms".into()))),
            );
            fields.insert(
                42,
                FitFieldType::new((42, "activity_type".into(), None, None, None)),
            );
            fields.insert(
                43,
                FitFieldType::new((
                    43,
                    "left_torque_effectiveness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                44,
                FitFieldType::new((
                    44,
                    "right_torque_effectiveness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                45,
                FitFieldType::new((
                    45,
                    "left_pedal_smoothness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                46,
                FitFieldType::new((
                    46,
                    "right_pedal_smoothness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                47,
                FitFieldType::new((
                    47,
                    "combined_pedal_smoothness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                48,
                FitFieldType::new((48, "time128".into(), Some(128), None, Some("s".into()))),
            );
            fields.insert(
                49,
                FitFieldType::new((49, "stroke_type".into(), None, None, None)),
            );
            fields.insert(50, FitFieldType::new((50, "zone".into(), None, None, None)));
            fields.insert(
                51,
                FitFieldType::new((51, "ball_speed".into(), Some(100), None, Some("m/s".into()))),
            );
            fields.insert(
                52,
                FitFieldType::new((52, "cadence256".into(), Some(256), None, Some("rpm".into()))),
            );
            fields.insert(
                53,
                FitFieldType::new((
                    53,
                    "fractional_cadence".into(),
                    Some(128),
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                54,
                FitFieldType::new((
                    54,
                    "total_hemoglobin_conc".into(),
                    Some(100),
                    None,
                    Some("g/dL".into()),
                )),
            );
            fields.insert(
                55,
                FitFieldType::new((
                    55,
                    "total_hemoglobin_conc_min".into(),
                    Some(100),
                    None,
                    Some("g/dL".into()),
                )),
            );
            fields.insert(
                56,
                FitFieldType::new((
                    56,
                    "total_hemoglobin_conc_max".into(),
                    Some(100),
                    None,
                    Some("g/dL".into()),
                )),
            );
            fields.insert(
                57,
                FitFieldType::new((
                    57,
                    "saturated_hemoglobin_percent".into(),
                    Some(10),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                58,
                FitFieldType::new((
                    58,
                    "saturated_hemoglobin_percent_min".into(),
                    Some(10),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                59,
                FitFieldType::new((
                    59,
                    "saturated_hemoglobin_percent_max".into(),
                    Some(10),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                62,
                FitFieldType::new((62, "device_index".into(), None, None, None)),
            );
            fields.insert(
                67,
                FitFieldType::new((67, "left_pco".into(), None, None, Some("mm".to_owned()))),
            );
            fields.insert(
                68,
                FitFieldType::new((68, "right_pco".into(), None, None, Some("mm".into()))),
            );
            // fields.insert(69, FitFieldType::new((69, "left_power_phase".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            // fields.insert(70, FitFieldType::new((70, "left_power_phase_peak".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            // fields.insert(71, FitFieldType::new((71, "right_power_phase".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            // fields.insert(72, FitFieldType::new((72, "right_power_phase_peak".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            fields.insert(
                73,
                FitFieldType::new((
                    73,
                    "enhanced_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                78,
                FitFieldType::new((
                    78,
                    "enhanced_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                81,
                FitFieldType::new((
                    81,
                    "battery_soc".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                82,
                FitFieldType::new((82, "motor_power".into(), None, None, Some("watts".into()))),
            );
            fields.insert(
                83,
                FitFieldType::new((
                    83,
                    "vertical_ratio".into(),
                    Some(100),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                84,
                FitFieldType::new((
                    84,
                    "stance_time_balance".into(),
                    Some(100),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                85,
                FitFieldType::new((85, "step_length".into(), Some(10), None, Some("mm".into()))),
            );
            fields.insert(
                91,
                FitFieldType::new((
                    91,
                    "absolute_pressure".into(),
                    None,
                    None,
                    Some("Pa".into()),
                )),
            );
            fields.insert(
                92,
                FitFieldType::new((92, "depth".into(), Some(1000), None, Some("m".into()))),
            );
            fields.insert(
                93,
                FitFieldType::new((
                    93,
                    "next_stop_depth".into(),
                    Some(1000),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                94,
                FitFieldType::new((94, "next_stop_time".into(), Some(1), None, Some("s".into()))),
            );
            fields.insert(
                95,
                FitFieldType::new((
                    95,
                    "time_to_surface".into(),
                    Some(1),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                96,
                FitFieldType::new((96, "ndl_time".into(), Some(1), None, Some("s".into()))),
            );
            fields.insert(
                97,
                FitFieldType::new((97, "cns_load".into(), None, None, Some("percent".into()))),
            );
            fields.insert(
                98,
                FitFieldType::new((98, "n2_load".into(), Some(1), None, Some("percent".into()))),
            );
            fields.insert(
                114,
                FitFieldType::new((114, "grit".into(), None, None, None)),
            );
            fields.insert(
                115,
                FitFieldType::new((115, "flow".into(), None, None, None)),
            );
            fields.insert(
                117,
                FitFieldType::new((
                    117,
                    "ebike_travel_range".into(),
                    None,
                    None,
                    Some("km".into()),
                )),
            );
            fields.insert(
                118,
                FitFieldType::new((
                    118,
                    "ebike_battery_level".into(),
                    None,
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                119,
                FitFieldType::new((
                    119,
                    "ebike_assist_mode".into(),
                    None,
                    None,
                    Some("depends on sensor".into()),
                )),
            );
            fields.insert(
                120,
                FitFieldType::new((
                    120,
                    "ebike_assist_level_percent".into(),
                    None,
                    None,
                    Some("percent".into()),
                )),
            );
        }
        // Profile.xlsx row 647, event/21
        21 => {
            name = "event".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(0, FitFieldType::new((0, "event".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "event_type".into(), None, None, None)),
            );
            fields.insert(2, FitFieldType::new((2, "data16".into(), None, None, None)));
            fields.insert(3, FitFieldType::new((3, "data".into(), None, None, None)));
            fields.insert(
                4,
                FitFieldType::new((4, "event_group".into(), None, None, None)),
            );
            fields.insert(7, FitFieldType::new((7, "score".into(), None, None, None)));
            fields.insert(
                8,
                FitFieldType::new((8, "opponent_score".into(), None, None, None)),
            );
            fields.insert(
                9,
                FitFieldType::new((9, "front_gear_num".into(), None, None, None)),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "front_gear".into(), None, None, None)),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "rear_gear_num".into(), None, None, None)),
            );
            fields.insert(
                12,
                FitFieldType::new((12, "rear_gear".into(), None, None, None)),
            );
            fields.insert(
                13,
                FitFieldType::new((13, "device_index".into(), None, None, None)),
            );
            fields.insert(
                21,
                FitFieldType::new((21, "radar_threat_level_max".into(), None, None, None)),
            );
            fields.insert(
                22,
                FitFieldType::new((22, "radar_threat_count".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 684, device_info/23
        23 => {
            name = "device_info".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "device_index".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "device_type".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "manufacturer".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "serial_number".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "product".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "software_version".into(), Some(100), None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "hardware_version".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "cum_operating_time".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                10,
                FitFieldType::new((
                    10,
                    "battery_voltage".into(),
                    Some(256),
                    None,
                    Some("V".into()),
                )),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "battery_status".into(), None, None, None)),
            );
            fields.insert(
                18,
                FitFieldType::new((18, "sensor_position".into(), None, None, None)),
            );
            fields.insert(
                19,
                FitFieldType::new((19, "descriptor".into(), None, None, None)),
            );
            fields.insert(
                20,
                FitFieldType::new((20, "ant_transmission_type".into(), None, None, None)),
            );
            fields.insert(
                21,
                FitFieldType::new((21, "ant_device_number".into(), None, None, None)),
            );
            fields.insert(
                22,
                FitFieldType::new((22, "ant_network".into(), None, None, None)),
            );
            fields.insert(
                25,
                FitFieldType::new((25, "source_type".into(), None, None, None)),
            );
            fields.insert(
                27,
                FitFieldType::new((27, "product_name".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 708, training_file/72
        72 => {
            name = "training_file".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, None)),
            );
            fields.insert(0, FitFieldType::new((0, "type".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "manufacturer".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "product".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "serial_number".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "time_created".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 718, hrv/78
        78 => {
            name = "hrv".into();
            fields.insert(
                0,
                FitFieldType::new((0, "time".into(), Some(1000), None, Some("s".to_owned()))),
            );
        }
        // Profile.xlsx row 721, weather_conditions/128
        128 => {
            name = "weather_conditions".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "weather_report".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "temperature".into(), None, None, Some("C".into()))),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "condition".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((
                    3,
                    "wind_direction".into(),
                    None,
                    None,
                    Some("degrees".into()),
                )),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "wind_speed".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "precipitation_probability".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((
                    6,
                    "temperature_feels_like".into(),
                    None,
                    None,
                    Some("C".into()),
                )),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "relative_humidity".into(), None, None, None)),
            );
            fields.insert(
                8,
                FitFieldType::new((8, "location".into(), None, None, None)),
            );
            fields.insert(
                9,
                FitFieldType::new((9, "observed_at_time".into(), None, None, None)),
            );
            fields.insert(
                10,
                FitFieldType::new((
                    10,
                    "observed_location_lat".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                11,
                FitFieldType::new((
                    11,
                    "observed_location_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                12,
                FitFieldType::new((12, "day_of_week".into(), None, None, None)),
            );
            fields.insert(
                13,
                FitFieldType::new((13, "high_temperature".into(), None, None, Some("C".into()))),
            );
            fields.insert(
                14,
                FitFieldType::new((14, "low_temperature".into(), None, None, Some("C".into()))),
            );
        }
        // Profile.xlsx row 738, weather_alert/129
        129 => {
            name = "weather_alert".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "report_id".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "issue_time".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "expire_time".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "severity".into(), None, None, None)),
            );
            fields.insert(4, FitFieldType::new((4, "type".into(), None, None, None)));
        }
        // Profile.xlsx row 745, gps_metadata/160
        160 => {
            name = "gps_metadata".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "timestamp_ms".into(), None, None, Some("ms".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((
                    1,
                    "position_lat".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                2,
                FitFieldType::new((
                    2,
                    "position_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                3,
                FitFieldType::new((
                    3,
                    "enhanced_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                4,
                FitFieldType::new((
                    4,
                    "enhanced_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "heading".into(), Some(100), None, Some("degrees".into()))),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "utc_timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "velocity".into(), Some(100), None, Some("m/s".into()))),
            );
        }
        // Profile.xlsx row 755, camera_event/161
        161 => {
            name = "camera_event".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "timestamp_ms".into(), None, None, Some("ms".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "camera_event_type".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "camera_file_uuid".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "camera_orientation".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 762, gyroscope_data/164
        164 => {
            name = "gyroscope_data".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "timestamp_ms".into(), None, None, Some("ms".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((
                    1,
                    "sample_time_offset".into(),
                    None,
                    None,
                    Some("ms".into()),
                )),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "gyro_x".into(), None, None, Some("counts".into()))),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "gyro_y".into(), None, None, Some("counts".into()))),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "gyro_z".into(), None, None, Some("counts".into()))),
            );
            fields.insert(
                5,
                FitFieldType::new((
                    5,
                    "calibrated_gyro_x".into(),
                    None,
                    None,
                    Some("deg/s".into()),
                )),
            );
            fields.insert(
                6,
                FitFieldType::new((
                    6,
                    "calibrated_gyro_y".into(),
                    None,
                    None,
                    Some("deg/s".into()),
                )),
            );
            fields.insert(
                7,
                FitFieldType::new((
                    7,
                    "calibrated_gyro_z".into(),
                    None,
                    None,
                    Some("deg/s".into()),
                )),
            );
        }
        // Profile.xlsx row 773, accelerometer_data/165
        165 => {
            name = "accelerometer_data".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "timestamp_ms".into(), None, None, Some("ms".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((
                    1,
                    "sample_time_offset".into(),
                    None,
                    None,
                    Some("ms".into()),
                )),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "accel_x".into(), None, None, Some("counts".into()))),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "accel_y".into(), None, None, Some("counts".into()))),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "accel_z".into(), None, None, Some("counts".into()))),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "calibrated_accel_x".into(), None, None, Some("g".into()))),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "calibrated_accel_y".into(), None, None, Some("g".into()))),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "calibrated_accel_z".into(), None, None, Some("g".into()))),
            );
            fields.insert(
                8,
                FitFieldType::new((
                    8,
                    "compressed_calibrated_accel_x".into(),
                    None,
                    None,
                    Some("mG".into()),
                )),
            );
            fields.insert(
                9,
                FitFieldType::new((
                    9,
                    "compressed_calibrated_accel_y".into(),
                    None,
                    None,
                    Some("mG".into()),
                )),
            );
            fields.insert(
                10,
                FitFieldType::new((
                    10,
                    "compressed_calibrated_accel_z".into(),
                    None,
                    None,
                    Some("mG".into()),
                )),
            );
        }
        // Profile.xlsx row 787, magnetometer_data/208
        208 => {
            name = "magnetometer_data".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "timestamp_ms".into(), None, None, Some("ms".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((
                    1,
                    "sample_time_offset".into(),
                    None,
                    None,
                    Some("ms".into()),
                )),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "mag_x".into(), None, None, Some("counts".to_owned()))),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "mag_y".into(), None, None, Some("counts".to_owned()))),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "mag_z".into(), None, None, Some("counts".to_owned()))),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "calibrated_mag_x".into(), None, None, Some("G".into()))),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "calibrated_mag_y".into(), None, None, Some("G".into()))),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "calibrated_mag_z".into(), None, None, Some("G".into()))),
            );
        }
        // Profile.xlsx row 798, barometer_data/209
        209 => {
            name = "barometer_data".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "timestamp_ms".into(), None, None, Some("ms".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((
                    1,
                    "sample_time_offset".into(),
                    None,
                    None,
                    Some("ms".into()),
                )),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "baro_pres".into(), None, None, Some("Pa".to_owned()))),
            );
        }
        // Profile.xlsx row 804, three_d_sensor_calibration/167
        167 => {
            name = "three_d_sensor_calibration".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "sensor_type".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "calibration_factor".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((
                    2,
                    "calibration_divisor".into(),
                    None,
                    None,
                    Some("counts".into()),
                )),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "level_shift".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "offset_cal".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "orientation_matrix".into(), Some(65535), None, None)),
            );
        }
        // Profile.xlsx row 815, one_d_sensor_calibration/210
        210 => {
            name = "one_d_sensor_calibration".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "sensor_type".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "calibration_factor".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((
                    2,
                    "calibration_divisor".into(),
                    None,
                    None,
                    Some("counts".into()),
                )),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "level_shift".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "offset_cal".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 824, video_frame/169
        169 => {
            name = "video_frame".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "timestamp_ms".into(), None, None, Some("ms".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "frame_number".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 829, obdii_data/174
        174 => {
            name = "obdii_data".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "timestamp_ms".into(), None, None, Some("ms".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "time_offset".into(), None, None, Some("ms".into()))),
            );
            fields.insert(2, FitFieldType::new((2, "pid".into(), None, None, None)));
            fields.insert(
                3,
                FitFieldType::new((3, "raw_data".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "pid_data_size".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "system_time".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "start_timestamp".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((
                    7,
                    "start_timestamp_ms".into(),
                    None,
                    None,
                    Some("ms".into()),
                )),
            );
        }
        // Profile.xlsx row 840, nmea_sentence/177
        177 => {
            name = "nmea_sentence".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "timestamp_ms".into(), None, None, Some("ms".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "sentence".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 845, aviation_attitude/178
        178 => {
            name = "aviation_attitude".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "timestamp_ms".into(), None, None, Some("ms".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "system_time".into(), None, None, Some("ms".into()))),
            );
            // fields.insert(2, FitFieldType::new((2, "pitch".into(), Some(10430,38), None, Some("radians".to_owned()))));
            // fields.insert(3, FitFieldType::new((3, "roll".into(), Some(10430,38), None, Some("radians".to_owned()))));
            fields.insert(
                4,
                FitFieldType::new((
                    4,
                    "accel_lateral".into(),
                    Some(100),
                    None,
                    Some("m/s^2".into()),
                )),
            );
            fields.insert(
                5,
                FitFieldType::new((
                    5,
                    "accel_normal".into(),
                    Some(100),
                    None,
                    Some("m/s^2".into()),
                )),
            );
            fields.insert(
                6,
                FitFieldType::new((
                    6,
                    "turn_rate".into(),
                    Some(1024),
                    None,
                    Some("radians/second".into()),
                )),
            );
            fields.insert(7, FitFieldType::new((7, "stage".into(), None, None, None)));
            fields.insert(
                8,
                FitFieldType::new((
                    8,
                    "attitude_stage_complete".into(),
                    None,
                    None,
                    Some("%".into()),
                )),
            );
            // fields.insert(9, FitFieldType::new((9, "track".into(), Some(10430,38), None, Some("radians".to_owned()))));
            fields.insert(
                10,
                FitFieldType::new((10, "validity".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 859, video/184
        184 => {
            name = "video".into();
            fields.insert(0, FitFieldType::new((0, "url".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "hosting_provider".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "duration".into(), None, None, Some("ms".to_owned()))),
            );
        }
        // Profile.xlsx row 864, video_title/185
        185 => {
            name = "video_title".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "message_count".into(), None, None, None)),
            );
            fields.insert(1, FitFieldType::new((1, "text".into(), None, None, None)));
        }
        // Profile.xlsx row 869, video_description/186
        186 => {
            name = "video_description".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "message_count".into(), None, None, None)),
            );
            fields.insert(1, FitFieldType::new((1, "text".into(), None, None, None)));
        }
        // Profile.xlsx row 874, video_clip/187
        187 => {
            name = "video_clip".into();
            fields.insert(
                0,
                FitFieldType::new((0, "clip_number".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "start_timestamp".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "start_timestamp_ms".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "end_timestamp".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "end_timestamp_ms".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "clip_start".into(), None, None, Some("ms".into()))),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "clip_end".into(), None, None, Some("ms".to_owned()))),
            );
        }
        // Profile.xlsx row 884, set/225
        225 => {
            name = "set".into();
            fields.insert(
                254,
                FitFieldType::new((254, "timestamp".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "duration".into(), Some(1000), None, Some("s".into()))),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "repetitions".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "weight".into(), Some(16), None, Some("kg".into()))),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "set_type".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "start_time".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "category".into(), None, None, None)),
            );
            fields.insert(
                8,
                FitFieldType::new((8, "category_subtype".into(), None, None, None)),
            );
            fields.insert(
                9,
                FitFieldType::new((9, "weight_display_unit".into(), None, None, None)),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "message_index".into(), None, None, None)),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "wkt_step_index".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 896, jump/285
        285 => {
            name = "jump".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "distance".into(), None, None, Some("m".to_owned()))),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "height".into(), None, None, Some("m".to_owned()))),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "rotations".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "hang_time".into(), None, None, Some("s".to_owned()))),
            );
            fields.insert(4, FitFieldType::new((4, "score".into(), None, None, None)));
            fields.insert(
                5,
                FitFieldType::new((
                    5,
                    "position_lat".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                6,
                FitFieldType::new((
                    6,
                    "position_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "speed".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(
                8,
                FitFieldType::new((
                    8,
                    "enhanced_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
        }
        // Profile.xlsx row 908, course/31
        31 => {
            name = "course".into();
            fields.insert(4, FitFieldType::new((4, "sport".into(), None, None, None)));
            fields.insert(5, FitFieldType::new((5, "name".into(), None, None, None)));
            fields.insert(
                6,
                FitFieldType::new((6, "capabilities".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "sub_sport".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 914, course_point/32
        32 => {
            name = "course_point".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "timestamp".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((
                    2,
                    "position_lat".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                3,
                FitFieldType::new((
                    3,
                    "position_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "distance".into(), Some(100), None, Some("m".into()))),
            );
            fields.insert(5, FitFieldType::new((5, "type".into(), None, None, None)));
            fields.insert(6, FitFieldType::new((6, "name".into(), None, None, None)));
            fields.insert(
                8,
                FitFieldType::new((8, "favorite".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 925, segment_id/148
        148 => {
            name = "segment_id".into();
            fields.insert(0, FitFieldType::new((0, "name".into(), None, None, None)));
            fields.insert(1, FitFieldType::new((1, "uuid".into(), None, None, None)));
            fields.insert(2, FitFieldType::new((2, "sport".into(), None, None, None)));
            fields.insert(
                3,
                FitFieldType::new((3, "enabled".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "user_profile_primary_key".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "device_id".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "default_race_leader".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "delete_status".into(), None, None, None)),
            );
            fields.insert(
                8,
                FitFieldType::new((8, "selection_type".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 936, segment_leaderboard_entry/149
        149 => {
            name = "segment_leaderboard_entry".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(0, FitFieldType::new((0, "name".into(), None, None, None)));
            fields.insert(1, FitFieldType::new((1, "type".into(), None, None, None)));
            fields.insert(
                2,
                FitFieldType::new((2, "group_primary_key".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "activity_id".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "segment_time".into(), Some(1000), None, Some("s".into()))),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "activity_id_string".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 945, segment_point/150
        150 => {
            name = "segment_point".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((
                    1,
                    "position_lat".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                2,
                FitFieldType::new((
                    2,
                    "position_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "distance".into(), Some(100), None, Some("m".into()))),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "altitude".into(), Some(5), Some(500), Some("m".into()))),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "leader_time".into(), Some(1000), None, Some("s".into()))),
            );
        }
        // Profile.xlsx row 953, segment_lap/142
        142 => {
            name = "segment_lap".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(0, FitFieldType::new((0, "event".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "event_type".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "start_time".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((
                    3,
                    "start_position_lat".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                4,
                FitFieldType::new((
                    4,
                    "start_position_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                5,
                FitFieldType::new((
                    5,
                    "end_position_lat".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                6,
                FitFieldType::new((
                    6,
                    "end_position_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                7,
                FitFieldType::new((
                    7,
                    "total_elapsed_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                8,
                FitFieldType::new((
                    8,
                    "total_timer_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                9,
                FitFieldType::new((
                    9,
                    "total_distance".into(),
                    Some(100),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "total_cycles".into(), None, None, Some("cycles".into()))),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "total_calories".into(), None, None, Some("kcal".into()))),
            );
            fields.insert(
                12,
                FitFieldType::new((
                    12,
                    "total_fat_calories".into(),
                    None,
                    None,
                    Some("kcal".into()),
                )),
            );
            fields.insert(
                13,
                FitFieldType::new((13, "avg_speed".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(
                14,
                FitFieldType::new((14, "max_speed".into(), Some(1000), None, Some("m/s".into()))),
            );
            fields.insert(
                15,
                FitFieldType::new((15, "avg_heart_rate".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                16,
                FitFieldType::new((16, "max_heart_rate".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                17,
                FitFieldType::new((17, "avg_cadence".into(), None, None, Some("rpm".into()))),
            );
            fields.insert(
                18,
                FitFieldType::new((18, "max_cadence".into(), None, None, Some("rpm".into()))),
            );
            fields.insert(
                19,
                FitFieldType::new((19, "avg_power".into(), None, None, Some("watts".into()))),
            );
            fields.insert(
                20,
                FitFieldType::new((20, "max_power".into(), None, None, Some("watts".into()))),
            );
            fields.insert(
                21,
                FitFieldType::new((21, "total_ascent".into(), None, None, Some("m".into()))),
            );
            fields.insert(
                22,
                FitFieldType::new((22, "total_descent".into(), None, None, Some("m".into()))),
            );
            fields.insert(
                23,
                FitFieldType::new((23, "sport".into(), None, None, None)),
            );
            fields.insert(
                24,
                FitFieldType::new((24, "event_group".into(), None, None, None)),
            );
            fields.insert(
                25,
                FitFieldType::new((25, "nec_lat".into(), None, None, Some("semicircles".into()))),
            );
            fields.insert(
                26,
                FitFieldType::new((
                    26,
                    "nec_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                27,
                FitFieldType::new((27, "swc_lat".into(), None, None, Some("semicircles".into()))),
            );
            fields.insert(
                28,
                FitFieldType::new((
                    28,
                    "swc_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(29, FitFieldType::new((29, "name".into(), None, None, None)));
            fields.insert(
                30,
                FitFieldType::new((
                    30,
                    "normalized_power".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                31,
                FitFieldType::new((31, "left_right_balance".into(), None, None, None)),
            );
            fields.insert(
                32,
                FitFieldType::new((32, "sub_sport".into(), None, None, None)),
            );
            fields.insert(
                33,
                FitFieldType::new((33, "total_work".into(), None, None, Some("J".into()))),
            );
            fields.insert(
                34,
                FitFieldType::new((
                    34,
                    "avg_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                35,
                FitFieldType::new((
                    35,
                    "max_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                36,
                FitFieldType::new((36, "gps_accuracy".into(), None, None, Some("m".into()))),
            );
            fields.insert(
                37,
                FitFieldType::new((37, "avg_grade".into(), Some(100), None, Some("%".into()))),
            );
            fields.insert(
                38,
                FitFieldType::new((
                    38,
                    "avg_pos_grade".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                39,
                FitFieldType::new((
                    39,
                    "avg_neg_grade".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                40,
                FitFieldType::new((
                    40,
                    "max_pos_grade".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                41,
                FitFieldType::new((
                    41,
                    "max_neg_grade".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                42,
                FitFieldType::new((42, "avg_temperature".into(), None, None, Some("C".into()))),
            );
            fields.insert(
                43,
                FitFieldType::new((43, "max_temperature".into(), None, None, Some("C".into()))),
            );
            fields.insert(
                44,
                FitFieldType::new((
                    44,
                    "total_moving_time".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                45,
                FitFieldType::new((
                    45,
                    "avg_pos_vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                46,
                FitFieldType::new((
                    46,
                    "avg_neg_vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                47,
                FitFieldType::new((
                    47,
                    "max_pos_vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                48,
                FitFieldType::new((
                    48,
                    "max_neg_vertical_speed".into(),
                    Some(1000),
                    None,
                    Some("m/s".into()),
                )),
            );
            fields.insert(
                49,
                FitFieldType::new((
                    49,
                    "time_in_hr_zone".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                50,
                FitFieldType::new((
                    50,
                    "time_in_speed_zone".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                51,
                FitFieldType::new((
                    51,
                    "time_in_cadence_zone".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                52,
                FitFieldType::new((
                    52,
                    "time_in_power_zone".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                53,
                FitFieldType::new((53, "repetition_num".into(), None, None, None)),
            );
            fields.insert(
                54,
                FitFieldType::new((
                    54,
                    "min_altitude".into(),
                    Some(5),
                    Some(500),
                    Some("m".into()),
                )),
            );
            fields.insert(
                55,
                FitFieldType::new((55, "min_heart_rate".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                56,
                FitFieldType::new((56, "active_time".into(), Some(1000), None, Some("s".into()))),
            );
            fields.insert(
                57,
                FitFieldType::new((57, "wkt_step_index".into(), None, None, None)),
            );
            fields.insert(
                58,
                FitFieldType::new((58, "sport_event".into(), None, None, None)),
            );
            fields.insert(
                59,
                FitFieldType::new((
                    59,
                    "avg_left_torque_effectiveness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                60,
                FitFieldType::new((
                    60,
                    "avg_right_torque_effectiveness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                61,
                FitFieldType::new((
                    61,
                    "avg_left_pedal_smoothness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                62,
                FitFieldType::new((
                    62,
                    "avg_right_pedal_smoothness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                63,
                FitFieldType::new((
                    63,
                    "avg_combined_pedal_smoothness".into(),
                    Some(2),
                    None,
                    Some("percent".into()),
                )),
            );
            fields.insert(
                64,
                FitFieldType::new((64, "status".into(), None, None, None)),
            );
            fields.insert(65, FitFieldType::new((65, "uuid".into(), None, None, None)));
            fields.insert(
                66,
                FitFieldType::new((
                    66,
                    "avg_fractional_cadence".into(),
                    Some(128),
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                67,
                FitFieldType::new((
                    67,
                    "max_fractional_cadence".into(),
                    Some(128),
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                68,
                FitFieldType::new((
                    68,
                    "total_fractional_cycles".into(),
                    Some(128),
                    None,
                    Some("cycles".into()),
                )),
            );
            fields.insert(
                69,
                FitFieldType::new((69, "front_gear_shift_count".into(), None, None, None)),
            );
            fields.insert(
                70,
                FitFieldType::new((70, "rear_gear_shift_count".into(), None, None, None)),
            );
            fields.insert(
                71,
                FitFieldType::new((
                    71,
                    "time_standing".into(),
                    Some(1000),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                72,
                FitFieldType::new((72, "stand_count".into(), None, None, None)),
            );
            fields.insert(
                73,
                FitFieldType::new((73, "avg_left_pco".into(), None, None, Some("mm".into()))),
            );
            fields.insert(
                74,
                FitFieldType::new((74, "avg_right_pco".into(), None, None, Some("mm".into()))),
            );
            // fields.insert(75, FitFieldType::new((75, "avg_left_power_phase".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            // fields.insert(76, FitFieldType::new((76, "avg_left_power_phase_peak".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            // fields.insert(77, FitFieldType::new((77, "avg_right_power_phase".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            // fields.insert(78, FitFieldType::new((78, "avg_right_power_phase_peak".into(), Some(0,7111111), None, Some("degrees".to_owned()))));
            fields.insert(
                79,
                FitFieldType::new((
                    79,
                    "avg_power_position".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                80,
                FitFieldType::new((
                    80,
                    "max_power_position".into(),
                    None,
                    None,
                    Some("watts".into()),
                )),
            );
            fields.insert(
                81,
                FitFieldType::new((
                    81,
                    "avg_cadence_position".into(),
                    None,
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                82,
                FitFieldType::new((
                    82,
                    "max_cadence_position".into(),
                    None,
                    None,
                    Some("rpm".into()),
                )),
            );
            fields.insert(
                83,
                FitFieldType::new((83, "manufacturer".into(), None, None, None)),
            );
            fields.insert(
                84,
                FitFieldType::new((84, "total_grit".into(), None, None, Some("kGrit".into()))),
            );
            fields.insert(
                85,
                FitFieldType::new((85, "total_flow".into(), None, None, Some("Flow".into()))),
            );
            fields.insert(
                86,
                FitFieldType::new((86, "avg_grit".into(), None, None, Some("kGrit".into()))),
            );
            fields.insert(
                87,
                FitFieldType::new((87, "avg_flow".into(), None, None, Some("Flow".into()))),
            );
            fields.insert(
                89,
                FitFieldType::new((
                    89,
                    "total_fractional_ascent".into(),
                    Some(100),
                    None,
                    Some("m".into()),
                )),
            );
            fields.insert(
                90,
                FitFieldType::new((
                    90,
                    "total_fractional_descent".into(),
                    Some(100),
                    None,
                    Some("m".into()),
                )),
            );
        }
        // Profile.xlsx row 1047, segment_file/151
        151 => {
            name = "segment_file".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "file_uuid".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "enabled".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "user_profile_primary_key".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "leader_type".into(), None, None, None)),
            );
            fields.insert(
                8,
                FitFieldType::new((8, "leader_group_primary_key".into(), None, None, None)),
            );
            fields.insert(
                9,
                FitFieldType::new((9, "leader_activity_id".into(), None, None, None)),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "leader_activity_id_string".into(), None, None, None)),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "default_race_leader".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1058, workout/26
        26 => {
            name = "workout".into();
            fields.insert(4, FitFieldType::new((4, "sport".into(), None, None, None)));
            fields.insert(
                5,
                FitFieldType::new((5, "capabilities".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "num_valid_steps".into(), None, None, None)),
            );
            fields.insert(
                8,
                FitFieldType::new((8, "wkt_name".into(), None, None, None)),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "sub_sport".into(), None, None, None)),
            );
            fields.insert(
                14,
                FitFieldType::new((14, "pool_length".into(), Some(100), None, Some("m".into()))),
            );
            fields.insert(
                15,
                FitFieldType::new((15, "pool_length_unit".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1067, workout_session/158
        158 => {
            name = "workout_session".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(0, FitFieldType::new((0, "sport".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "sub_sport".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "num_valid_steps".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "first_step_index".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "pool_length".into(), Some(100), None, Some("m".into()))),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "pool_length_unit".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1075, workout_step/27
        27 => {
            name = "workout_step".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "wkt_step_name".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "duration_type".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "duration_value".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "target_type".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "target_value".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "custom_target_value_low".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "custom_target_value_high".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "intensity".into(), None, None, None)),
            );
            fields.insert(8, FitFieldType::new((8, "notes".into(), None, None, None)));
            fields.insert(
                9,
                FitFieldType::new((9, "equipment".into(), None, None, None)),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "exercise_category".into(), None, None, None)),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "exercise_name".into(), None, None, None)),
            );
            fields.insert(
                12,
                FitFieldType::new((
                    12,
                    "exercise_weight".into(),
                    Some(100),
                    None,
                    Some("kg".into()),
                )),
            );
            fields.insert(
                13,
                FitFieldType::new((13, "weight_display_unit".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1117, exercise_title/264
        264 => {
            name = "exercise_title".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "exercise_category".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "exercise_name".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "wkt_step_name".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1124, schedule/28
        28 => {
            name = "schedule".into();
            fields.insert(
                0,
                FitFieldType::new((0, "manufacturer".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "product".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "serial_number".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "time_created".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "completed".into(), None, None, None)),
            );
            fields.insert(5, FitFieldType::new((5, "type".into(), None, None, None)));
            fields.insert(
                6,
                FitFieldType::new((6, "scheduled_time".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1136, totals/33
        33 => {
            name = "totals".into();
            fields.insert(
                254,
                FitFieldType::new((254, "message_index".into(), None, None, None)),
            );
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "timer_time".into(), None, None, Some("s".to_owned()))),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "distance".into(), None, None, Some("m".to_owned()))),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "calories".into(), None, None, Some("kcal".into()))),
            );
            fields.insert(3, FitFieldType::new((3, "sport".into(), None, None, None)));
            fields.insert(
                4,
                FitFieldType::new((4, "elapsed_time".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "sessions".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "active_time".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                9,
                FitFieldType::new((9, "sport_index".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1149, weight_scale/30
        30 => {
            name = "weight_scale".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "weight".into(), Some(100), None, Some("kg".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "percent_fat".into(), Some(100), None, Some("%".into()))),
            );
            fields.insert(
                2,
                FitFieldType::new((
                    2,
                    "percent_hydration".into(),
                    Some(100),
                    None,
                    Some("%".into()),
                )),
            );
            fields.insert(
                3,
                FitFieldType::new((
                    3,
                    "visceral_fat_mass".into(),
                    Some(100),
                    None,
                    Some("kg".into()),
                )),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "bone_mass".into(), Some(100), None, Some("kg".into()))),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "muscle_mass".into(), Some(100), None, Some("kg".into()))),
            );
            fields.insert(
                7,
                FitFieldType::new((
                    7,
                    "basal_met".into(),
                    Some(4),
                    None,
                    Some("kcal/day".into()),
                )),
            );
            fields.insert(
                8,
                FitFieldType::new((8, "physique_rating".into(), None, None, None)),
            );
            fields.insert(
                9,
                FitFieldType::new((
                    9,
                    "active_met".into(),
                    Some(4),
                    None,
                    Some("kcal/day".into()),
                )),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "metabolic_age".into(), None, None, Some("years".into()))),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "visceral_fat_rating".into(), None, None, None)),
            );
            fields.insert(
                12,
                FitFieldType::new((12, "user_profile_index".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1165, blood_pressure/51
        51 => {
            name = "blood_pressure".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((
                    0,
                    "systolic_pressure".into(),
                    None,
                    None,
                    Some("mmHg".into()),
                )),
            );
            fields.insert(
                1,
                FitFieldType::new((
                    1,
                    "diastolic_pressure".into(),
                    None,
                    None,
                    Some("mmHg".into()),
                )),
            );
            fields.insert(
                2,
                FitFieldType::new((
                    2,
                    "mean_arterial_pressure".into(),
                    None,
                    None,
                    Some("mmHg".into()),
                )),
            );
            fields.insert(
                3,
                FitFieldType::new((
                    3,
                    "map_3_sample_mean".into(),
                    None,
                    None,
                    Some("mmHg".into()),
                )),
            );
            fields.insert(
                4,
                FitFieldType::new((
                    4,
                    "map_morning_values".into(),
                    None,
                    None,
                    Some("mmHg".into()),
                )),
            );
            fields.insert(
                5,
                FitFieldType::new((
                    5,
                    "map_evening_values".into(),
                    None,
                    None,
                    Some("mmHg".into()),
                )),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "heart_rate".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "heart_rate_type".into(), None, None, None)),
            );
            fields.insert(8, FitFieldType::new((8, "status".into(), None, None, None)));
            fields.insert(
                9,
                FitFieldType::new((9, "user_profile_index".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1179, monitoring_info/103
        103 => {
            name = "monitoring_info".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "local_timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "activity_type".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((
                    3,
                    "cycles_to_distance".into(),
                    Some(5000),
                    None,
                    Some("m/cycle".into()),
                )),
            );
            fields.insert(
                4,
                FitFieldType::new((
                    4,
                    "cycles_to_calories".into(),
                    Some(5000),
                    None,
                    Some("kcal/cycle".into()),
                )),
            );
            fields.insert(
                5,
                FitFieldType::new((
                    5,
                    "resting_metabolic_rate".into(),
                    None,
                    None,
                    Some("kcal / day".into()),
                )),
            );
        }
        // Profile.xlsx row 1187, monitoring/55
        55 => {
            name = "monitoring".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "device_index".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "calories".into(), None, None, Some("kcal".into()))),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "distance".into(), Some(100), None, Some("m".into()))),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "cycles".into(), Some(2), None, Some("cycles".into()))),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "active_time".into(), Some(1000), None, Some("s".into()))),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "activity_type".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "activity_subtype".into(), None, None, None)),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "activity_level".into(), None, None, None)),
            );
            fields.insert(
                8,
                FitFieldType::new((8, "distance_16".into(), None, None, Some("100 * m".into()))),
            );
            fields.insert(
                9,
                FitFieldType::new((
                    9,
                    "cycles_16".into(),
                    None,
                    None,
                    Some("2 * cycles (steps)".into()),
                )),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "active_time_16".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "local_timestamp".into(), None, None, None)),
            );
            fields.insert(
                12,
                FitFieldType::new((12, "temperature".into(), Some(100), None, Some("C".into()))),
            );
            fields.insert(
                14,
                FitFieldType::new((
                    14,
                    "temperature_min".into(),
                    Some(100),
                    None,
                    Some("C".into()),
                )),
            );
            fields.insert(
                15,
                FitFieldType::new((
                    15,
                    "temperature_max".into(),
                    Some(100),
                    None,
                    Some("C".into()),
                )),
            );
            fields.insert(
                16,
                FitFieldType::new((
                    16,
                    "activity_time".into(),
                    None,
                    None,
                    Some("minutes".into()),
                )),
            );
            fields.insert(
                19,
                FitFieldType::new((
                    19,
                    "active_calories".into(),
                    None,
                    None,
                    Some("kcal".into()),
                )),
            );
            fields.insert(
                24,
                FitFieldType::new((
                    24,
                    "current_activity_type_intensity".into(),
                    None,
                    None,
                    None,
                )),
            );
            fields.insert(
                25,
                FitFieldType::new((25, "timestamp_min_8".into(), None, None, Some("min".into()))),
            );
            fields.insert(
                26,
                FitFieldType::new((26, "timestamp_16".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                27,
                FitFieldType::new((27, "heart_rate".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                28,
                FitFieldType::new((28, "intensity".into(), Some(10), None, None)),
            );
            fields.insert(
                29,
                FitFieldType::new((29, "duration_min".into(), None, None, Some("min".into()))),
            );
            fields.insert(
                30,
                FitFieldType::new((30, "duration".into(), None, None, Some("s".to_owned()))),
            );
            fields.insert(
                31,
                FitFieldType::new((31, "ascent".into(), Some(1000), None, Some("m".into()))),
            );
            fields.insert(
                32,
                FitFieldType::new((32, "descent".into(), Some(1000), None, Some("m".into()))),
            );
            fields.insert(
                33,
                FitFieldType::new((
                    33,
                    "moderate_activity_minutes".into(),
                    None,
                    None,
                    Some("minutes".into()),
                )),
            );
            fields.insert(
                34,
                FitFieldType::new((
                    34,
                    "vigorous_activity_minutes".into(),
                    None,
                    None,
                    Some("minutes".into()),
                )),
            );
        }
        // Profile.xlsx row 1220, hr/132
        132 => {
            name = "hr".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, None)),
            );
            fields.insert(
                0,
                FitFieldType::new((
                    0,
                    "fractional_timestamp".into(),
                    Some(32768),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "time256".into(), Some(256), None, Some("s".into()))),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "filtered_bpm".into(), None, None, Some("bpm".into()))),
            );
            fields.insert(
                9,
                FitFieldType::new((
                    9,
                    "event_timestamp".into(),
                    Some(1024),
                    None,
                    Some("s".into()),
                )),
            );
            // fields.insert(10, FitFieldType::new((10, "event_timestamp_12".into(), Some(1024,1024,1024,1024,1024,1024,1024,1024,1024,1024), None, Some("s".to_owned()))));
        }
        // Profile.xlsx row 1227, stress_level/227
        227 => {
            name = "stress_level".into();
            fields.insert(
                0,
                FitFieldType::new((0, "stress_level_value".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "stress_level_time".into(), None, None, Some("s".into()))),
            );
        }
        // Profile.xlsx row 1231, memo_glob/145
        145 => {
            name = "memo_glob".into();
            fields.insert(
                250,
                FitFieldType::new((250, "part_index".into(), None, None, None)),
            );
            fields.insert(0, FitFieldType::new((0, "memo".into(), None, None, None)));
            fields.insert(
                1,
                FitFieldType::new((1, "message_number".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "message_index".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1237, ant_channel_id/82
        82 => {
            name = "ant_channel_id".into();
            fields.insert(
                0,
                FitFieldType::new((0, "channel_number".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "device_type".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "device_number".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "transmission_type".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "device_index".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1243, ant_rx/80
        80 => {
            name = "ant_rx".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((
                    0,
                    "fractional_timestamp".into(),
                    Some(32768),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "mesg_id".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "mesg_data".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "channel_number".into(), None, None, None)),
            );
            fields.insert(4, FitFieldType::new((4, "data".into(), None, None, None)));
        }
        // Profile.xlsx row 1251, ant_tx/81
        81 => {
            name = "ant_tx".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((
                    0,
                    "fractional_timestamp".into(),
                    Some(32768),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "mesg_id".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "mesg_data".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "channel_number".into(), None, None, None)),
            );
            fields.insert(4, FitFieldType::new((4, "data".into(), None, None, None)));
        }
        // Profile.xlsx row 1258, exd_screen_configuration/200
        200 => {
            name = "exd_screen_configuration".into();
            fields.insert(
                0,
                FitFieldType::new((0, "screen_index".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "field_count".into(), None, None, None)),
            );
            fields.insert(2, FitFieldType::new((2, "layout".into(), None, None, None)));
            fields.insert(
                3,
                FitFieldType::new((3, "screen_enabled".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1264, exd_data_field_configuration/201
        201 => {
            name = "exd_data_field_configuration".into();
            fields.insert(
                0,
                FitFieldType::new((0, "screen_index".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "concept_field".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "field_id".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "concept_count".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "display_type".into(), None, None, None)),
            );
            fields.insert(5, FitFieldType::new((5, "title".into(), None, None, None)));
        }
        // Profile.xlsx row 1272, exd_data_concept_configuration/202
        202 => {
            name = "exd_data_concept_configuration".into();
            fields.insert(
                0,
                FitFieldType::new((0, "screen_index".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "concept_field".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "field_id".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "concept_index".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "data_page".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "concept_key".into(), None, None, None)),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "scaling".into(), None, None, None)),
            );
            fields.insert(
                8,
                FitFieldType::new((8, "data_units".into(), None, None, None)),
            );
            fields.insert(
                9,
                FitFieldType::new((9, "qualifier".into(), None, None, None)),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "descriptor".into(), None, None, None)),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "is_signed".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1284, field_description/206
        206 => {
            name = "field_description".into();
            fields.insert(
                0,
                FitFieldType::new((0, "developer_data_index".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "field_definition_number".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "fit_base_type_id".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "field_name".into(), None, None, None)),
            );
            fields.insert(4, FitFieldType::new((4, "array".into(), None, None, None)));
            fields.insert(
                5,
                FitFieldType::new((5, "components".into(), None, None, None)),
            );
            fields.insert(6, FitFieldType::new((6, "scale".into(), None, None, None)));
            fields.insert(7, FitFieldType::new((7, "offset".into(), None, None, None)));
            fields.insert(8, FitFieldType::new((8, "units".into(), None, None, None)));
            fields.insert(9, FitFieldType::new((9, "bits".into(), None, None, None)));
            fields.insert(
                10,
                FitFieldType::new((10, "accumulate".into(), None, None, None)),
            );
            fields.insert(
                13,
                FitFieldType::new((13, "fit_base_unit_id".into(), None, None, None)),
            );
            fields.insert(
                14,
                FitFieldType::new((14, "native_mesg_num".into(), None, None, None)),
            );
            fields.insert(
                15,
                FitFieldType::new((15, "native_field_num".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1300, developer_data_id/207
        207 => {
            name = "developer_data_id".into();
            fields.insert(
                0,
                FitFieldType::new((0, "developer_id".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "application_id".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "manufacturer_id".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "developer_data_index".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "application_version".into(), None, None, None)),
            );
        }
        // Profile.xlsx row 1306, dive_summary/268
        268 => {
            name = "dive_summary".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((0, "reference_mesg".into(), None, None, None)),
            );
            fields.insert(
                1,
                FitFieldType::new((1, "reference_index".into(), None, None, None)),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "avg_depth".into(), Some(1000), None, Some("m".into()))),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "max_depth".into(), Some(1000), None, Some("m".into()))),
            );
            fields.insert(
                4,
                FitFieldType::new((
                    4,
                    "surface_interval".into(),
                    Some(1),
                    None,
                    Some("s".into()),
                )),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "start_cns".into(), Some(1), None, Some("percent".into()))),
            );
            fields.insert(
                6,
                FitFieldType::new((6, "end_cns".into(), Some(1), None, Some("percent".into()))),
            );
            fields.insert(
                7,
                FitFieldType::new((7, "start_n2".into(), Some(1), None, Some("percent".into()))),
            );
            fields.insert(
                8,
                FitFieldType::new((8, "end_n2".into(), Some(1), None, Some("percent".into()))),
            );
            fields.insert(
                9,
                FitFieldType::new((9, "o2_toxicity".into(), None, None, Some("OTUs".into()))),
            );
            fields.insert(
                10,
                FitFieldType::new((10, "dive_number".into(), None, None, None)),
            );
            fields.insert(
                11,
                FitFieldType::new((11, "bottom_time".into(), Some(1000), None, Some("s".into()))),
            );
        }
        // Profile.xlsx row 1320, climb_pro/317
        317 => {
            name = "climb_pro".into();
            fields.insert(
                253,
                FitFieldType::new((253, "timestamp".into(), None, None, Some("s".into()))),
            );
            fields.insert(
                0,
                FitFieldType::new((
                    0,
                    "position_lat".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                1,
                FitFieldType::new((
                    1,
                    "position_long".into(),
                    None,
                    None,
                    Some("semicircles".into()),
                )),
            );
            fields.insert(
                2,
                FitFieldType::new((2, "climb_pro_event".into(), None, None, None)),
            );
            fields.insert(
                3,
                FitFieldType::new((3, "climb_number".into(), None, None, None)),
            );
            fields.insert(
                4,
                FitFieldType::new((4, "climb_category".into(), None, None, None)),
            );
            fields.insert(
                5,
                FitFieldType::new((5, "current_dist".into(), None, None, Some("m".into()))),
            );
        }
        _ => (),
    }

    FitMessageType {
        global_id,
        name,
        fields,
    }
}
// message types: 87
// empty rows:    58
// group rows:    15
// }
