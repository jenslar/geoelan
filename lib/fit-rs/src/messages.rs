#![allow(dead_code)]

pub mod message_types {

    pub fn get_messagetype(global_id: u16) -> String {
        match global_id {
            0 => String::from("file_id"),
            1 => String::from("capabilities"),
            2 => String::from("device_settings"),
            3 => String::from("user_profile"),
            4 => String::from("hrm_profile"),
            5 => String::from("sdm_profile"),
            6 => String::from("bike_profile"),
            7 => String::from("zones_target"),
            8 => String::from("hr_zone"),
            9 => String::from("power_zone"),
            10 => String::from("met_zone"),
            12 => String::from("sport"),
            15 => String::from("goal"),
            18 => String::from("session"),
            19 => String::from("lap"),
            20 => String::from("record"),
            21 => String::from("event"),
            23 => String::from("device_info"),
            26 => String::from("workout"),
            27 => String::from("workout_step"),
            28 => String::from("schedule"),
            30 => String::from("weight_scale"),
            31 => String::from("course"),
            32 => String::from("course_point"),
            33 => String::from("totals"),
            34 => String::from("activity"),
            35 => String::from("software"),
            37 => String::from("file_capabilities"),
            38 => String::from("mesg_capabilities"),
            39 => String::from("field_capabilities"),
            49 => String::from("file_creator"),
            51 => String::from("blood_pressure"),
            53 => String::from("speed_zone"),
            55 => String::from("monitoring"),
            72 => String::from("training_file"),
            78 => String::from("hrv"),
            80 => String::from("ant_rx"),
            81 => String::from("ant_tx"),
            82 => String::from("ant_channel_id"),
            101 => String::from("length"),
            103 => String::from("monitoring_info"),
            105 => String::from("pad"),
            106 => String::from("slave_device"),
            127 => String::from("connectivity"),
            128 => String::from("weather_conditions"),
            129 => String::from("weather_alert"),
            131 => String::from("cadence_zone"),
            132 => String::from("hr"),
            142 => String::from("segment_lap"),
            145 => String::from("memo_glob"),
            148 => String::from("segment_id"),
            149 => String::from("segment_leaderboard_entry"),
            150 => String::from("segment_point"),
            151 => String::from("segment_file"),
            158 => String::from("workout_session"),
            159 => String::from("watchface_settings"),
            160 => String::from("gps_metadata"),
            161 => String::from("camera_event"),
            162 => String::from("timestamp_correlation"),
            164 => String::from("gyroscope_data"),
            165 => String::from("accelerometer_data"),
            167 => String::from("three_d_sensor_calibration"),
            169 => String::from("video_frame"),
            174 => String::from("obdii_data"),
            177 => String::from("nmea_sentence"),
            178 => String::from("aviation_attitude"),
            184 => String::from("video"),
            185 => String::from("video_title"),
            186 => String::from("video_description"),
            187 => String::from("video_clip"),
            188 => String::from("ohr_settings"),
            200 => String::from("exd_screen_configuration"),
            201 => String::from("exd_data_field_configuration"),
            202 => String::from("exd_data_concept_configuration"),
            206 => String::from("field_description"),
            207 => String::from("developer_data_id"),
            208 => String::from("magnetometer_data"),
            209 => String::from("barometer_data"),
            210 => String::from("one_d_sensor_calibration"),
            225 => String::from("set"),
            227 => String::from("stress_level"),
            258 => String::from("dive_settings"),
            259 => String::from("dive_gas"),
            262 => String::from("dive_alarm"),
            264 => String::from("exercise_title"),
            268 => String::from("dive_summary"),
            _ => format!("UNDEFINED_MESSAGE_TYPE_{}", global_id),
        }
    }
}

pub mod field_types {

    pub fn get_fieldtype(global_id: u16, field_definition_number: u8) -> String {
        match global_id {
            0 => {
                // file_id
                match field_definition_number {
                    0 => String::from("type"),         // file
                    1 => String::from("manufacturer"), // manufacturer/enum/u16: garmin = 1 (long list...)
                    2 => String::from("product"),      // uint16
                    // garmin_product	garmin_product // 7 ???
                    3 => String::from("serial_number"), // uint32z
                    4 => String::from("time_created"),  // date_time
                    5 => String::from("number"),        // uint16
                    8 => String::from("product_name"),  // string NOT IN VIRB DATA
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            2 => {
                // device_settings
                match field_definition_number {
                    0 => String::from("active_time_zone"),          // uint8
                    1 => String::from("utc_offset"),                // uint32
                    2 => String::from("time_offset"),               // uint32	[N]
                    4 => String::from("time_mode"),                 // time_mode	[N]
                    5 => String::from("time_zone_offset"),          // sint8	[N]
                    12 => String::from("backlight_mode"),           // backlight_mode
                    36 => String::from("activity_tracker_enabled"), // bool
                    39 => String::from("clock_time"),               // date_time
                    40 => String::from("pages_enabled"),            // uint16	[N]
                    46 => String::from("move_alert_enabled"),       // bool
                    47 => String::from("date_mode"),                // date_mode
                    55 => String::from("display_orientation"),      // display_orientation
                    56 => String::from("mounting_side"),            // side
                    57 => String::from("default_page"),             // uint16	[N]
                    58 => String::from("autosync_min_steps"),       // uint16
                    59 => String::from("autosync_min_time"),        // uint16
                    80 => String::from("lactate_threshold_autodetect_enabled"), // bool
                    86 => String::from("ble_auto_upload_enabled"),  // bool
                    89 => String::from("auto_sync_frequency"),      // auto_sync_frequency
                    90 => String::from("auto_activity_detect"),     // auto_activity_detect
                    94 => String::from("number_of_screens"),        // uint8
                    95 => String::from("smart_notification_display_orientation"), // display_orientation
                    134 => String::from("tap_interface"),                         // switch
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            3 => {
                match field_definition_number {
                    254 => String::from("message_index"),    // message_index
                    0 => String::from("friendly_name"),      // string
                    1 => String::from("gender"),             // gender
                    2 => String::from("age"),                // uint8
                    3 => String::from("height"),             // uint8
                    4 => String::from("weight"),             // uint16
                    5 => String::from("language"),           // language
                    6 => String::from("elev_setting"),       // display_measure
                    7 => String::from("weight_setting"),     // display_measure
                    8 => String::from("resting_heart_rate"), // uint8
                    9 => String::from("default_max_running_heart_rate"), // uint8
                    10 => String::from("default_max_biking_heart_rate"), // uint8
                    11 => String::from("default_max_heart_rate"), // uint8
                    12 => String::from("hr_setting"),        // display_heart
                    13 => String::from("speed_setting"),     // display_measure
                    14 => String::from("dist_setting"),      // display_measure
                    16 => String::from("power_setting"),     // display_power
                    17 => String::from("activity_class"),    // activity_class
                    18 => String::from("position_setting"),  // display_position
                    21 => String::from("temperature_setting"), // display_measure
                    22 => String::from("local_id"),          // user_local_id
                    23 => String::from("global_id"),         // byte [6]
                    28 => String::from("wake_time"),         // localtime_into_day
                    29 => String::from("sleep_time"),        // localtime_into_day
                    30 => String::from("height_setting"),    // display_measure
                    31 => String::from("user_running_step_length"), // uint16
                    32 => String::from("user_walking_step_length"), // uint16
                    47 => String::from("depth_setting"),     // display_measure
                    49 => String::from("dive_count"),        // uint32
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            18 => {
                // session
                match field_definition_number {
                    254 => String::from("message_index"),
                    253 => String::from("timestamp"),
                    0 => String::from("event"),
                    1 => String::from("event_type"),
                    2 => String::from("start_time"),
                    3 => String::from("start_position_lat"),
                    4 => String::from("start_position_long"),
                    5 => String::from("sport"),
                    6 => String::from("sub_sport"),
                    7 => String::from("total_elapsed_time"),
                    8 => String::from("total_timer_time"),
                    9 => String::from("total_distance"),
                    10 => String::from("total_cycles"),
                    // => String::from("total_strides"), // ???
                    11 => String::from("total_calories"),
                    13 => String::from("total_fat_calories"),
                    14 => String::from("avg_speed"),
                    15 => String::from("max_speed"),
                    16 => String::from("avg_heart_rate"),
                    17 => String::from("max_heart_rate"),
                    18 => String::from("avg_cadence"),
                    // => String::from("avg_running_cadence"), // ???
                    19 => String::from("max_cadence"),
                    // => String::from("max_running_cadence"), // ???
                    20 => String::from("avg_power"),
                    21 => String::from("max_power"),
                    22 => String::from("total_ascent"),
                    23 => String::from("total_descent"),
                    24 => String::from("total_training_effect"),
                    25 => String::from("first_lap_index"),
                    26 => String::from("num_laps"),
                    27 => String::from("event_group"),
                    28 => String::from("trigger"),
                    29 => String::from("nec_lat"),
                    30 => String::from("nec_long"),
                    31 => String::from("swc_lat"),
                    32 => String::from("swc_long"),
                    34 => String::from("normalized_power"),
                    35 => String::from("training_stress_score"),
                    36 => String::from("intensity_factor"),
                    37 => String::from("left_right_balance"),
                    41 => String::from("avg_stroke_count"),
                    42 => String::from("avg_stroke_distance"),
                    43 => String::from("swim_stroke"),
                    44 => String::from("pool_length"),
                    45 => String::from("threshold_power"),
                    46 => String::from("pool_length_unit"),
                    47 => String::from("num_active_lengths"),
                    48 => String::from("total_work"),
                    49 => String::from("avg_altitude"),
                    50 => String::from("max_altitude"),
                    51 => String::from("gps_accuracy"),
                    52 => String::from("avg_grade"),
                    53 => String::from("avg_pos_grade"),
                    54 => String::from("avg_neg_grade"),
                    55 => String::from("max_pos_grade"),
                    56 => String::from("max_neg_grade"),
                    57 => String::from("avg_temperature"),
                    58 => String::from("max_temperature"),
                    59 => String::from("total_moving_time"),
                    60 => String::from("avg_pos_vertical_speed"),
                    61 => String::from("avg_neg_vertical_speed"),
                    62 => String::from("max_pos_vertical_speed"),
                    63 => String::from("max_neg_vertical_speed"),
                    64 => String::from("min_heart_rate"),
                    65 => String::from("time_in_hr_zone"),
                    66 => String::from("time_in_speed_zone"),
                    67 => String::from("time_in_cadence_zone"),
                    68 => String::from("time_in_power_zone"),
                    69 => String::from("avg_lap_time"),
                    70 => String::from("best_lap_index"),
                    71 => String::from("min_altitude"),
                    82 => String::from("player_score"),
                    83 => String::from("opponent_score"),
                    84 => String::from("opponent_name"),
                    85 => String::from("stroke_count"),
                    86 => String::from("zone_count"),
                    87 => String::from("max_ball_speed"),
                    88 => String::from("avg_ball_speed"),
                    89 => String::from("avg_vertical_oscillation"),
                    90 => String::from("avg_stance_time_percent"),
                    91 => String::from("avg_stance_time"),
                    92 => String::from("avg_fractional_cadence"),
                    93 => String::from("max_fractional_cadence"),
                    94 => String::from("total_fractional_cycles"),
                    95 => String::from("avg_total_hemoglobin_conc"),
                    96 => String::from("min_total_hemoglobin_conc"),
                    97 => String::from("max_total_hemoglobin_conc"),
                    98 => String::from("avg_saturated_hemoglobin_percent"),
                    99 => String::from("min_saturated_hemoglobin_percent"),
                    100 => String::from("max_saturated_hemoglobin_percent"),
                    101 => String::from("avg_left_torque_effectiveness"),
                    102 => String::from("avg_right_torque_effectiveness"),
                    103 => String::from("avg_left_pedal_smoothness"),
                    104 => String::from("avg_right_pedal_smoothness"),
                    105 => String::from("avg_combined_pedal_smoothness"),
                    111 => String::from("sport_index"),
                    112 => String::from("time_standing"),
                    113 => String::from("stand_count"),
                    114 => String::from("avg_left_pco"),
                    115 => String::from("avg_right_pco"),
                    116 => String::from("avg_left_power_phase"),
                    117 => String::from("avg_left_power_phase_peak"),
                    118 => String::from("avg_right_power_phase"),
                    119 => String::from("avg_right_power_phase_peak"),
                    120 => String::from("avg_power_position"),
                    121 => String::from("max_power_position"),
                    122 => String::from("avg_cadence_position"),
                    123 => String::from("max_cadence_position"),
                    124 => String::from("enhanced_avg_speed"),
                    125 => String::from("enhanced_max_speed"),
                    126 => String::from("enhanced_avg_altitude"),
                    127 => String::from("enhanced_min_altitude"),
                    128 => String::from("enhanced_max_altitude"),
                    129 => String::from("avg_lev_motor_power"),
                    130 => String::from("max_lev_motor_power"),
                    131 => String::from("lev_battery_consumption"),
                    132 => String::from("avg_vertical_ratio"),
                    133 => String::from("avg_stance_time_balance"),
                    134 => String::from("avg_step_length"),
                    137 => String::from("total_anaerobic_training_effect"),
                    139 => String::from("avg_vam"),
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            19 => {
                // lap
                match field_definition_number {
                    254 => String::from("message_index"),     // message_index
                    253 => String::from("timestamp"),         // date_time
                    0 => String::from("event"),               // event
                    1 => String::from("event_type"),          // event_type
                    2 => String::from("start_time"),          // date_time
                    3 => String::from("start_position_lat"),  // sint32
                    4 => String::from("start_position_long"), // sint32
                    5 => String::from("end_position_lat"),    // sint32
                    6 => String::from("end_position_long"),   // sint32
                    7 => String::from("total_elapsed_time"),  // uint32
                    8 => String::from("total_timer_time"),    // uint32
                    9 => String::from("total_distance"),      // uint32
                    10 => String::from("total_cycles"),       // uint32
                    // total_strides	uint32
                    11 => String::from("total_calories"), // uint16
                    12 => String::from("total_fat_calories"), // uint16
                    13 => String::from("avg_speed"),      // uint16
                    14 => String::from("max_speed"),      // uint16
                    15 => String::from("avg_heart_rate"), // uint8
                    16 => String::from("max_heart_rate"), // uint8
                    17 => String::from("avg_cadence"),    // uint8
                    // avg_running_cadence	uint8
                    18 => String::from("max_cadence"), // uint8
                    // max_running_cadence	uint8
                    19 => String::from("avg_power"),        // uint16
                    20 => String::from("max_power"),        // uint16
                    21 => String::from("total_ascent"),     // uint16
                    22 => String::from("total_descent"),    // uint16
                    23 => String::from("intensity"),        // intensity
                    24 => String::from("lap_trigger"),      // lap_trigger
                    25 => String::from("sport"),            // sport
                    26 => String::from("event_group"),      // uint8
                    32 => String::from("num_lengths"),      // uint16
                    33 => String::from("normalized_power"), // uint16
                    34 => String::from("left_right_balance"), // left_right_balance_100
                    35 => String::from("first_length_index"), // uint16
                    37 => String::from("avg_stroke_distance"), // uint16
                    38 => String::from("swim_stroke"),      // swim_stroke
                    39 => String::from("sub_sport"),        // sub_sport
                    40 => String::from("num_active_lengths"), // uint16
                    41 => String::from("total_work"),       // uint32
                    42 => String::from("avg_altitude"),     // uint16
                    43 => String::from("max_altitude"),     // uint16
                    44 => String::from("gps_accuracy"),     // uint8
                    45 => String::from("avg_grade"),        // sint16
                    46 => String::from("avg_pos_grade"),    // sint16
                    47 => String::from("avg_neg_grade"),    // sint16
                    48 => String::from("max_pos_grade"),    // sint16
                    49 => String::from("max_neg_grade"),    // sint16
                    50 => String::from("avg_temperature"),  // sint8
                    51 => String::from("max_temperature"),  // sint8
                    52 => String::from("total_moving_time"), // uint32
                    53 => String::from("avg_pos_vertical_speed"), // sint16
                    54 => String::from("avg_neg_vertical_speed"), // sint16
                    55 => String::from("max_pos_vertical_speed"), // sint16
                    56 => String::from("max_neg_vertical_speed"), // sint16
                    57 => String::from("time_in_hr_zone"),  // uint32	[N]
                    58 => String::from("time_in_speed_zone"), // uint32	[N]
                    59 => String::from("time_in_cadence_zone"), // uint32	[N]
                    60 => String::from("time_in_power_zone"), // uint32	[N]
                    61 => String::from("repetition_num"),   // uint16
                    62 => String::from("min_altitude"),     // uint16
                    63 => String::from("min_heart_rate"),   // uint8
                    71 => String::from("wkt_step_index"),   // message_index
                    74 => String::from("opponent_score"),   // uint16
                    75 => String::from("stroke_count"),     // uint16	[N]
                    76 => String::from("zone_count"),       // uint16	[N]
                    77 => String::from("avg_vertical_oscillation"), // uint16
                    78 => String::from("avg_stance_time_percent"), // uint16
                    79 => String::from("avg_stance_time"),  // uint16
                    80 => String::from("avg_fractional_cadence"), // uint8
                    81 => String::from("max_fractional_cadence"), // uint8
                    82 => String::from("total_fractional_cycles"), // uint8
                    83 => String::from("player_score"),     // uint16
                    84 => String::from("avg_total_hemoglobin_conc"), // uint16	[N]
                    85 => String::from("min_total_hemoglobin_conc"), // uint16	[N]
                    86 => String::from("max_total_hemoglobin_conc"), // uint16	[N]
                    87 => String::from("avg_saturated_hemoglobin_percent"), // uint16	[N]
                    88 => String::from("min_saturated_hemoglobin_percent"), // uint16	[N]
                    89 => String::from("max_saturated_hemoglobin_percent"), // uint16	[N]
                    91 => String::from("avg_left_torque_effectiveness"), // uint8
                    92 => String::from("avg_right_torque_effectiveness"), // uint8
                    93 => String::from("avg_left_pedal_smoothness"), // uint8
                    94 => String::from("avg_right_pedal_smoothness"), // uint8
                    95 => String::from("avg_combined_pedal_smoothness"), // uint8
                    98 => String::from("time_standing"),    // uint32
                    99 => String::from("stand_count"),      // uint16
                    100 => String::from("avg_left_pco"),    // sint8
                    101 => String::from("avg_right_pco"),   // sint8
                    102 => String::from("avg_left_power_phase"), // uint8	[N]
                    103 => String::from("avg_left_power_phase_peak"), // uint8	[N]
                    104 => String::from("avg_right_power_phase"), // uint8	[N]
                    105 => String::from("avg_right_power_phase_peak"), // uint8	[N]
                    106 => String::from("avg_power_position"), // uint16	[N]
                    107 => String::from("max_power_position"), // uint16	[N]
                    108 => String::from("avg_cadence_position"), // uint8	[N]
                    109 => String::from("max_cadence_position"), // uint8	[N]
                    110 => String::from("enhanced_avg_speed"), // uint32
                    111 => String::from("enhanced_max_speed"), // uint32
                    112 => String::from("enhanced_avg_altitude"), // uint32
                    113 => String::from("enhanced_min_altitude"), // uint32
                    114 => String::from("enhanced_max_altitude"), // uint32
                    115 => String::from("avg_lev_motor_power"), // uint16
                    116 => String::from("max_lev_motor_power"), // uint16
                    117 => String::from("lev_battery_consumption"), // uint8
                    118 => String::from("avg_vertical_ratio"), // uint16
                    119 => String::from("avg_stance_time_balance"), // uint16
                    120 => String::from("avg_step_length"), // uint16
                    121 => String::from("avg_vam"),         // uint16
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            20 => {
                // record (contains no gps_metadata until satellite sync)
                match field_definition_number {
                    253 => String::from("timestamp"),                       // date_time
                    0 => String::from("position_lat"),                      // sint32
                    1 => String::from("position_long"),                     // sint32
                    2 => String::from("altitude"),                          // uint16
                    3 => String::from("heart_rate"),                        // uint8
                    4 => String::from("cadence"),                           // uint8
                    5 => String::from("distance"),                          // uint32
                    6 => String::from("speed"),                             // uint16
                    7 => String::from("power"),                             // uint16
                    8 => String::from("compressed_speed_distance"),         // byte [3]
                    9 => String::from("grade"),                             // sint16
                    10 => String::from("resistance"),                       // uint8
                    11 => String::from("time_from_course"),                 // sint32
                    12 => String::from("cycle_length"),                     // uint8
                    13 => String::from("temperature"),                      // sint8
                    17 => String::from("speed_1s"),                         // uint8 [N]
                    18 => String::from("cycles"),                           // uint8
                    19 => String::from("total_cycles"),                     // uint32
                    28 => String::from("compressed_accumulated_power"),     // uint16
                    29 => String::from("accumulated_power"),                // uint32
                    30 => String::from("left_right_balance"),               // left_right_balance
                    31 => String::from("gps_accuracy"),                     // uint8
                    32 => String::from("vertical_speed"),                   // sint16
                    33 => String::from("calories"),                         // uint16
                    39 => String::from("vertical_oscillation"),             // uint16
                    40 => String::from("stance_time_percent"),              // uint16
                    41 => String::from("stance_time"),                      // uint16
                    42 => String::from("activity_type"),                    // activity_type
                    43 => String::from("left_torque_effectiveness"),        // uint8
                    44 => String::from("right_torque_effectiveness"),       // uint8
                    45 => String::from("left_pedal_smoothness"),            // uint8
                    46 => String::from("right_pedal_smoothness"),           // uint8
                    47 => String::from("combined_pedal_smoothness"),        // uint8
                    48 => String::from("time128"),                          // uint8
                    49 => String::from("stroke_type"),                      // stroke_type
                    50 => String::from("zone"),                             // uint8
                    51 => String::from("ball_speed"),                       // uint16
                    52 => String::from("cadence256"),                       // uint16
                    53 => String::from("fractional_cadence"),               // uint8
                    54 => String::from("total_hemoglobin_conc"),            // uint16
                    55 => String::from("total_hemoglobin_conc_min"),        // uint16
                    56 => String::from("total_hemoglobin_conc_max"),        // uint16
                    57 => String::from("saturated_hemoglobin_percent"),     // uint16
                    58 => String::from("saturated_hemoglobin_percent_min"), // uint16
                    59 => String::from("saturated_hemoglobin_percent_max"), // uint16
                    62 => String::from("device_index"),                     // device_index
                    67 => String::from("left_pco"),                         // sint8
                    68 => String::from("right_pco"),                        // sint8
                    69 => String::from("left_power_phase"),                 // uint8 [N]
                    70 => String::from("left_power_phase_peak"),            // uint8 [N]
                    71 => String::from("right_power_phase"),                // uint8 [N]
                    72 => String::from("right_power_phase_peak"),           // uint8 [N]
                    73 => String::from("enhanced_speed"),                   // uint32
                    78 => String::from("enhanced_altitude"),                // uint32
                    81 => String::from("battery_soc"),                      // uint8
                    82 => String::from("motor_power"),                      // uint16
                    83 => String::from("vertical_ratio"),                   // uint16
                    84 => String::from("stance_time_balance"),              // uint16
                    85 => String::from("step_length"),                      // uint16
                    91 => String::from("absolute_pressure"),                // uint32
                    92 => String::from("depth"),                            // uint32
                    93 => String::from("next_stop_depth"),                  // uint32
                    94 => String::from("next_stop_time"),                   // uint32
                    95 => String::from("time_to_surface"),                  // uint32
                    96 => String::from("ndl_time"),                         // uint32
                    97 => String::from("cns_load"),                         // uint8
                    98 => String::from("n2_load"),                          // uint16
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            21 => {
                // event
                match field_definition_number {
                    253 => String::from("timestamp"),    // date_time
                    0 => String::from("event"),          // event
                    1 => String::from("event_type"),     // event_type
                    2 => String::from("data16"),         // uint16
                    3 => String::from("data"),           // uint32
                    4 => String::from("event_group"),    // uint8
                    7 => String::from("score"),          // uint16
                    8 => String::from("opponent_score"), // uint16
                    9 => String::from("front_gear_num"), // uint8z
                    10 => String::from("front_gear"),    // uint8z
                    11 => String::from("rear_gear_num"), // uint8z
                    12 => String::from("rear_gear"),     // uint8z
                    13 => String::from("device_index"),  // device_index
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number), // NOT IN VIRB DATA/NO NUMERICAL ID IN PROFILE.XLSX:
                                                                                 // timer_trigger	timer_trigger
                                                                                 // course_point_index	message_index
                                                                                 // battery_level	uint16
                                                                                 // virtual_partner_speed	uint16
                                                                                 // hr_high_alert	uint8
                                                                                 // hr_low_alert	uint8
                                                                                 // speed_high_alert	uint32
                                                                                 // speed_low_alert	uint32
                                                                                 // cad_high_alert	uint16
                                                                                 // cad_low_alert	uint16
                                                                                 // power_high_alert	uint16
                                                                                 // power_low_alert	uint16
                                                                                 // time_duration_alert	uint32
                                                                                 // distance_duration_alert	uint32
                                                                                 // calorie_duration_alert	uint32
                                                                                 // fitness_equipment_state	fitness_equipment_state
                                                                                 // sport_point	uint32
                                                                                 // gear_change_data	uint32
                                                                                 // rider_position	rider_position_type
                                                                                 // comm_timeout	comm_timeout_type
                }
            }
            // 22 => {}, // not in Profile.xlsx
            23 => {
                // device_info
                match field_definition_number {
                    253 => String::from("timestamp"),            // date_time
                    0 => String::from("device_index"),           // device_index
                    1 => String::from("device_type"),            // uint8
                    2 => String::from("manufacturer"),           // manufacturer
                    3 => String::from("serial_number"),          // uint32z
                    4 => String::from("product"),                // uint16
                    5 => String::from("software_version"),       // uint16
                    6 => String::from("hardware_version"),       // uint8
                    7 => String::from("cum_operating_time"),     // uint32
                    10 => String::from("battery_voltage"),       // uint16
                    11 => String::from("battery_status"),        // battery_status
                    18 => String::from("sensor_position"),       // body_location
                    19 => String::from("descriptor"),            // string
                    20 => String::from("ant_transmission_type"), // uint8z but basetype = STRING in ViRB data???
                    21 => String::from("ant_device_number"),     // uint16z
                    22 => String::from("ant_network"),           // ant_network
                    25 => String::from("source_type"),           // source_type
                    27 => String::from("product_name"),          // string
                    // NOT IN VIRB DATA/NO NUMERICAL ID IN PROFILE.XLSX:
                    // antplus_device_type	antplus_device_type
                    // ant_device_type	uint8
                    // garmin_product	garmin_product
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            34 => {
                // activity
                match field_definition_number {
                    253 => String::from("timestamp"),      // date_time
                    0 => String::from("total_timer_time"), // uint32
                    1 => String::from("num_sessions"),     // uint16
                    2 => String::from("type"),             // activity
                    3 => String::from("event"),            // event
                    4 => String::from("event_type"),       // event_type
                    5 => String::from("local_timestamp"),  // local_date_time
                    6 => String::from("event_group"),      // uint8
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            49 => {
                // file_creator
                match field_definition_number {
                    0 => String::from("software_version"), // uint16
                    1 => String::from("hardware_version"), // uint8
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            160 => {
                // gps_metadata
                match field_definition_number {
                    253 => String::from("timestamp"),       // date_time
                    0 => String::from("timestamp_ms"),      // uint16
                    1 => String::from("position_lat"),      // sint32
                    2 => String::from("position_long"),     // sint32
                    3 => String::from("enhanced_altitude"), // uint32
                    4 => String::from("enhanced_speed"),    // uint32
                    5 => String::from("heading"),           // uint16
                    6 => String::from("utc_timestamp"),     // date_time
                    7 => String::from("velocity"),          // sint16 [3]
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            161 => {
                // camera_event
                match field_definition_number {
                    253 => String::from("timestamp"),        // seconds/u32
                    0 => String::from("timestamp_ms"),       // milliseconds/u16
                    1 => String::from("camera_event_type"),  // enum/u8
                    2 => String::from("camera_file_uuid"),   // String
                    3 => String::from("camera_orientation"), // enum/u8
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            162 => {
                // timestamp_correlation
                match field_definition_number {
                    253 => String::from("timestamp"), // seconds/u32 (date_time)
                    0 => String::from("fractional_timestamp"), // uint16
                    1 => String::from("system_timestamp"), // date_time/u32
                    2 => String::from("fractional_system_timestamp"), // uint16
                    3 => String::from("local_timestamp"), // local_date_time/u32
                    4 => String::from("timestamp_ms"), // milliseconds/uint16
                    5 => String::from("system_timestamp_ms"), // uint16
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            164 => {
                // gyroscope_data
                match field_definition_number {
                    253 => String::from("timestamp"),        // seconds, date_time/u32
                    0 => String::from("timestamp_ms"),       // milliseconds/u16
                    1 => String::from("sample_time_offset"), // (array) milliseconds/u16
                    2 => String::from("gyro_x"),             // (array) u16
                    3 => String::from("gyro_y"),             // (array) u16
                    4 => String::from("gyro_z"),             // (array) u16
                    5 => String::from("calibrated_gyro_x"),  // (array) f32 not in virb data
                    6 => String::from("calibrated_gyro_y"),  // (array) f32 not in virb data
                    7 => String::from("calibrated_gyro_z"),  // (array) f32 not in virb data
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            165 => {
                // accelerometer_data
                match field_definition_number {
                    253 => String::from("timestamp"),        // seconds, date_time/u32
                    0 => String::from("timestamp_ms"),       // milliseconds/u16
                    1 => String::from("sample_time_offset"), // (array) milliseconds/u16
                    2 => String::from("accel_x"),            // (array) u16
                    3 => String::from("accel_y"),            // (array) u16
                    4 => String::from("accel_z"),            // (array) u16
                    5 => String::from("calibrated_accel_x"), // (array) f32
                    6 => String::from("calibrated_accel_y"), // (array) f32
                    7 => String::from("calibrated_accel_z"), // (array) f32
                    8 => String::from("compressed_calibrated_accel_x"), // (array) i16 NOT IN VIRB DATA
                    9 => String::from("compressed_calibrated_accel_y"), // (array) i16 NOT IN VIRB DATA
                    10 => String::from("compressed_calibrated_accel_z"), // (array) i16 NOT IN VIRB DATA
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            167 => {
                // three_d_sensor_calibration
                match field_definition_number {
                    253 => String::from("timestamp"),         // date_time
                    0 => String::from("sensor_type"),         // sensor_type
                    1 => String::from("calibration_factor"),  // uint32
                    2 => String::from("calibration_divisor"), // uint32
                    3 => String::from("level_shift"),         // uint32
                    4 => String::from("offset_cal"),          // sint32 [3]
                    5 => String::from("orientation_matrix"),  // sint32 [9]
                    // accel_cal_factor	uint32
                    // gyro_cal_factor	uint32
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            206 => {
                // field_description
                match field_definition_number {
                    0 => String::from("developer_data_index"),    // uint8
                    1 => String::from("field_definition_number"), // uint8
                    2 => String::from("fit_base_type_id"),        // fit_base_type
                    3 => String::from("field_name"),              // string
                    4 => String::from("array"),                   // uint8
                    5 => String::from("components"),              // string
                    6 => String::from("scale"),                   // uint8
                    7 => String::from("offset"),                  // sint8
                    8 => String::from("units"),                   // string
                    9 => String::from("bits"),                    // string
                    10 => String::from("accumulate"),             // string
                    13 => String::from("fit_base_unit_id"),       // fit_base_unit
                    14 => String::from("native_mesg_num"),        // mesg_num
                    15 => String::from("native_field_num"),       // uint8
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            207 => {
                match field_definition_number {
                    // developer_data_id
                    0 => String::from("developer_id"),    // byte
                    1 => String::from("application_id"),  // byte
                    2 => String::from("manufacturer_id"), // manufacturer
                    3 => String::from("developer_data_index"), // uint8
                    4 => String::from("application_version"), // uint32
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            208 => {
                // magnetometer_data
                match field_definition_number {
                    253 => String::from("timestamp"),        // date_time
                    0 => String::from("timestamp_ms"),       // u16
                    1 => String::from("sample_time_offset"), // milliseconds/u16
                    2 => String::from("mag_x"),              // (array) u16
                    3 => String::from("mag_y"),              // (array) u16
                    4 => String::from("mag_z"),              // (array) u16
                    5 => String::from("calibrated_mag_x"),   // (array) f32 NOT IN VIRB DATA
                    6 => String::from("calibrated_mag_y"),   // (array) f32 NOT IN VIRB DATA
                    7 => String::from("calibrated_mag_z"),   // (array) f32 NOT IN VIRB DATA
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            209 => {
                // barometer_data
                match field_definition_number {
                    253 => String::from("timestamp"),        // date_time
                    0 => String::from("timestamp_ms"),       // uint16
                    1 => String::from("sample_time_offset"), // uint16 [N]
                    2 => String::from("baro_pres"),          // uint32 [N]
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            210 => {
                match field_definition_number {
                    253 => String::from("timestamp"),         // date_time
                    0 => String::from("sensor_type"),         // sensor_type
                    1 => String::from("calibration_factor"),  // uint32
                    2 => String::from("calibration_divisor"), // uint32
                    3 => String::from("level_shift"),         // uint32
                    4 => String::from("offset_cal"),          // sint32
                    // baro_cal_factor	uint32
                    _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
                }
            }
            _ => format!("UNDEFINED_FIELD_{}", field_definition_number),
        }
    }

    // pub fn get_enum(global_id: u16, field_id: u8, field_data: u8) -> String {
    pub fn get_enum(global_id: u16, field: &crate::structs::DataField) -> String {
        let field_definition_number = field.field_definition_number;
        let field_data = match &field.data {
            crate::structs::BaseType::ENUM(val) => val[0],
            _ => return String::from(""), // if not enum, easier to use for `check` command
        };
        match global_id {
            21 => {
                match field_definition_number {
                    0 => {
                        // event
                        match field_data {
                            0 => String::from("timer"),
                            3 => String::from("workout"),
                            4 => String::from("workout_step"),
                            5 => String::from("power_down"),
                            6 => String::from("power_up"),
                            7 => String::from("off_course"),
                            8 => String::from("session"),
                            9 => String::from("lap"),
                            10 => String::from("course_point"),
                            11 => String::from("battery"),
                            12 => String::from("virtual_partner_pace"),
                            13 => String::from("hr_high_alert"),
                            14 => String::from("hr_low_alert"),
                            15 => String::from("speed_high_alert"),
                            16 => String::from("speed_low_alert"),
                            17 => String::from("cad_high_alert"),
                            18 => String::from("cad_low_alert"),
                            19 => String::from("power_high_alert"),
                            20 => String::from("power_low_alert"),
                            21 => String::from("recovery_hr"),
                            22 => String::from("battery_low"),
                            23 => String::from("time_duration_alert"),
                            24 => String::from("distance_duration_alert"),
                            25 => String::from("calorie_duration_alert"),
                            26 => String::from("activity"),
                            27 => String::from("fitness_equipment"),
                            28 => String::from("length"),
                            32 => String::from("user_marker"),
                            33 => String::from("sport_point"),
                            36 => String::from("calibration"),
                            42 => String::from("front_gear_change"),
                            43 => String::from("rear_gear_change"),
                            44 => String::from("rider_position_change"),
                            45 => String::from("elev_high_alert"),
                            46 => String::from("elev_low_alert"),
                            47 => String::from("comm_timeout"),
                            75 => String::from("radar_threat_alert"),
                            _ => format!(
                                "UNDEFINED_ENUM_{}_FIELD_TYPE_{}",
                                field_data, field_definition_number
                            ),
                        }
                    }
                    1 => {
                        // event_type
                        match field_data {
                            0 => String::from("start"),
                            1 => String::from("stop"),
                            2 => String::from("consecutive_depreciated"),
                            3 => String::from("marker"),
                            4 => String::from("stop_all"),
                            5 => String::from("begin_depreciated"),
                            6 => String::from("end_depreciated"),
                            7 => String::from("end_all_depreciated"),
                            8 => String::from("stop_disable"),
                            9 => String::from("stop_disable_all"),
                            _ => format!(
                                "UNDEFINED_ENUM_{}_FIELD_TYPE_{}",
                                field_data, field_definition_number
                            ),
                        }
                    }
                    _ => format!(
                        "NO_ENUM_FOR_FIELD_TYPE_{}_GLOBAL_{}",
                        field_definition_number, global_id
                    ),
                }
            }
            161 => {
                match field_definition_number {
                    1 => {
                        // camera_event_type
                        match field_data {
                            0 => String::from("video_start"),
                            1 => String::from("video_split"),
                            2 => String::from("video_end"),
                            3 => String::from("photo_taken"),
                            4 => String::from("video_second_stream_start"),
                            5 => String::from("video_second_stream_split"),
                            6 => String::from("video_second_stream_end"),
                            7 => String::from("video_split_start"),
                            8 => String::from("video_second_stream_split_start"),
                            11 => String::from("video_pause"),
                            12 => String::from("video_second_stream_pause"),
                            13 => String::from("video_resume"),
                            14 => String::from("video_second_stream_resume"),
                            _ => format!(
                                "UNDEFINED_ENUM_{}_FIELD_TYPE_{}",
                                field_data, field_definition_number
                            ),
                        }
                    }
                    3 => {
                        // camera_orientation_type
                        match field_data {
                            0 => String::from("camera_orientation_0"),
                            1 => String::from("camera_orientation_90"),
                            2 => String::from("camera_orientation_180"),
                            3 => String::from("camera_orientation_270"),
                            _ => format!(
                                "UNDEFINED_ENUM_{}_FIELD_TYPE_{}",
                                field_data, field_definition_number
                            ),
                        }
                    }
                    _ => format!(
                        "NO_ENUM_FOR_FIELD_TYPE_{}_GLOBAL_{}",
                        field_definition_number, global_id
                    ),
                }
            }
            167 => {
                match field_definition_number {
                    0 => {
                        match field_data {
                            0 => String::from("accelerometer"),
                            1 => String::from("gyroscope"),
                            2 => String::from("compass"), // Magnetometer
                            3 => String::from("barometer"),
                            _ => format!(
                                "UNDEFINED_ENUM_{}_FIELD_TYPE_{}",
                                field_data, field_definition_number
                            ),
                        }
                    }

                    _ => format!(
                        "NO_ENUM_FOR_FIELD_TYPE_{}_GLOBAL_{}",
                        field_definition_number, global_id
                    ),
                }
            }
            _ => format!("ENUM_NOT_IMPLEMENTED_FOR_GLOBAL_ID_{}", global_id),
        }
    }
}
