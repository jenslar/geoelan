use std::{io::ErrorKind, path::PathBuf};

use gpmf_rs::Gpmf;
use plotly::{common::Title, Scatter};

pub(crate) fn gopro2plot(
    args: &clap::ArgMatches,
) -> std::io::Result<(Title, Vec<Box<Scatter<f64, f64>>>)> {
    let path = args.get_one::<PathBuf>("gpmf").unwrap();
    let y_axis = args.get_one::<String>("y-axis").unwrap(); // sensor type, required arg
    // X-axis currently defaults to count/index for GP. May use distance if altitude is set to Y-axis.
    let x_axis = args.get_one::<String>("x-axis"); // optional, default to counts/index

    let gpmf = Gpmf::new(&path, false)?;
    let sensor_type = gpmf_rs::SensorType::from(y_axis.as_str());
    let sensor_data = gpmf.sensor(&sensor_type);

    if sensor_data.len() == 0 {
        let device = gpmf
            .device_name()
            .first()
            .cloned()
            .unwrap_or(String::from("Unknown model"));
        let msg = format!("(!) '{}' is not supported by {device} or not yet implemented. Run 'geoelan inspect --gpmf {}' for a summary.",
            sensor_type.to_string(),
            path.display()
        );
        return Err(std::io::Error::new(ErrorKind::Other, msg))
    }

    let sensor_descr = sensor_data
        .first()
        .map(|s| {
            format!(
                "{} ({})",
                s.sensor.to_string(),
                s.units
                    .to_owned()
                    .unwrap_or_else(|| String::from("Unspecified"))
            )
        })
        .unwrap_or_else(|| String::from("Sensor data"));

    let x: Vec<_> = sensor_data.iter().flat_map(|s| s.x()).collect();
    let y: Vec<_> = sensor_data.iter().flat_map(|s| s.y()).collect();
    let z: Vec<_> = sensor_data.iter().flat_map(|s| s.z()).collect();

    // TODO use time/distance instead, index should be last fallback (or expl specified)
    let count: Vec<_> = (0..x.len()).into_iter().map(|i| (i + 1) as f64).collect();

    let title = Title::new(&sensor_descr);

    return Ok((
        title,
        vec![
            Scatter::new(count.to_owned(), x).name("x"),
            Scatter::new(count.to_owned(), y).name("y"),
            Scatter::new(count, z).name("z"),
        ],
    ));
}
