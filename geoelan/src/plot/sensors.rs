//! Overview of which sensors each camera model has.

/// Print sensor table
pub fn print_table() -> std::io::Result<()> {
    let headers = r#"| Device   | GPS | Accelerometer | Gyroscope |   Gravity   |  Magnetometer |"#;
    let gopro = r#"
    --[ GoPro ]--
    Hero 5 Black  | Yes*  | Accelerometer | Gyroscope |   N/A   |     N/A      |
    Hero 6 Black  | Yes*  | Accelerometer | Gyroscope |   N/A   |     N/A      |
    Hero 7 Black  | Yes*  | Accelerometer | Gyroscope |   N/A   |     N/A      |
    Hero 8 Black  | Yes*  | Accelerometer | Gyroscope | Gravity |     N/A      |
    Hero 9 Black  | Yes*  | Accelerometer | Gyroscope | Gravity |     N/A      |
    Hero 10 Black | Yes*  | Accelerometer | Gyroscope | Gravity |     N/A      |
    Hero 11 Black | Yes** | Accelerometer | Gyroscope | Gravity |     N/A      |

    *  18Hz GPS, individual points not timestamped, only 1-second cluster (GPS5).
    ** 10Hz GPS, individual points timestamped (GPS9).
    "#;

    let virb = r#"
    --[ VIRB ]--
    VIRB Ultra 30 | Yes   | Accelerometer | Gyroscope |   N/A   | Magnetometer |
    "#;

    println!("{headers}");
    println!("{gopro}");
    println!("{virb}");

    Ok(())
}