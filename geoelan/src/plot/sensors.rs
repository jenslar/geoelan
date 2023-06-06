//! Overview of which sensors each camera model has.

pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    let headers = r#"| Model   | GPS | Accelerometer | Gyroscope |   Gravity   |  Magnetometer |"#;
    let gopro = r#"
    --[ GoPro ]--
    Hero 5 Black  | GPS | Accelerometer | Gyroscope |   N/A   |     N/A      |
    Hero 6 Black  | GPS | Accelerometer | Gyroscope |   N/A   |     N/A      |
    Hero 7 Black  | GPS | Accelerometer | Gyroscope |   N/A   |     N/A      |
    Hero 8 Black  | GPS | Accelerometer | Gyroscope | Gravity |     N/A      |
    Hero 9 Black  | GPS | Accelerometer | Gyroscope | Gravity |     N/A      |
    Hero 10 Black | GPS | Accelerometer | Gyroscope | Gravity |     N/A      |
    Hero 11 Black | GPS | Accelerometer | Gyroscope | Gravity |     N/A      |
    "#;

    let virb = r#"
    --[ VIRB ]--
    VIRB Ultra 30 | GPS | Accelerometer | Gyroscope |   N/A   | Magnetometer |
    "#;

    Ok(())
}