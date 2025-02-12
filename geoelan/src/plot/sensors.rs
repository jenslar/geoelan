//! Overview of which sensors each camera model has.

/// Print sensor table
pub fn print_table() -> std::io::Result<()> {
    let headers = r#"| Device   |  GPS   | Accelerometer | Gyroscope |   Gravity   |  Magnetometer |"#;
    let gopro = r#"
    --[ GoPro ]--
    Hero 5 Black  | Yes*  | Yes | Yes |   No   |     No      |
    Hero 6 Black  | Yes*  | Yes | Yes |   No   |     No      |
    Hero 7 Black  | Yes*  | Yes | Yes |   No   |     No      |
    Hero 8 Black  | Yes*  | Yes | Yes |   No   |     No      |
    Hero 9 Black  | Yes*  | Yes | Yes |   No   |     No      |
    Hero 10 Black | Yes*  | Yes | Yes |   No   |     No      |
    Hero 11 Black | Yes** | Yes | Yes |   No   |     No      |
    Hero 12 Black | No*** | Yes | Yes |   No   |     No      |
    Hero 13 Black | Yes** | Yes | Yes |   No   |     No      |

    *   18Hz GPS, individual points not timestamped, only 1-second cluster (GPS5).
    **  10Hz GPS, individual points timestamped (GPS9).
    *** Hero 12 Black has no GPS module and can not be used with GeoELAN
    "#;

    let virb = r#"
    --[ VIRB ]--
    VIRB Ultra 30 | Yes   | Yes | Yes |   N/A   | Yes |
    "#;

    println!("{headers}");
    println!("{gopro}");
    println!("{virb}");

    Ok(())
}
