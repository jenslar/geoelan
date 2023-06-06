use std::{io::ErrorKind, path::PathBuf};

use fit_rs::Fit;
use plotly::{common::Title, Scatter};

pub(crate) fn virb2plot(
    args: &clap::ArgMatches,
) -> std::io::Result<(Title, Vec<Box<Scatter<f64, f64>>>)> {
    let path = args.get_one::<PathBuf>("fit").unwrap();
    let y_axis = args.get_one::<String>("y-axis").unwrap(); // sensor type, required arg
    let x_axis = args.get_one::<String>("x-axis"); // optional, default to counts/index

    let fit = Fit::new(path)?;
    let sensor_type = match fit_rs::SensorType::from_str(&y_axis) {
        Some(t) => t,
        None => {
            let msg = format!("(!) '{y_axis}' is not supported by the FIT format or not yet implemented. Run Run 'geoelan inspect --fit {}' for a summary.", path.display());
            return Err(std::io::Error::new(ErrorKind::Other, msg))
        }
    };

    // TODO add range via uuid or select session
    let sensor_data = match fit.sensor(&sensor_type, None) {
        Ok(s) => s,
        Err(err) => return Err(err.into()),
    };

    // Compile x, y, z Vec:s
    let ((x, y), z): ((Vec<f64>, Vec<f64>), Vec<f64>) = sensor_data.iter()
        .cloned()
        // Flatten values into Vec<((f64, f64), f64)>...
        .flat_map(|s| s.calibrated_x.into_iter()
            .zip(s.calibrated_y.into_iter())
            .zip(s.calibrated_z.into_iter()))
        // ...then unzip to get separate x, y, z Vec:s
        .unzip();


    // X-axis: timestamps in seconds
    let t: Vec<_> = sensor_data
        .iter()
        .flat_map(|s| {
            // add millisecond offset for each sample to record timestamp (sec + ms), return ms
            s.sample_time_offset
                .iter()
                .map(|o| *o as f64 / 1000. + s.timestamp as f64 + s.timestamp_ms as f64 / 1000.)
        })
        .collect();

    let units = sensor_type.units();
    let title = Title::new(&format!("{} ({})", sensor_type.to_string(), units));

    return Ok((
        title,
        vec![
            Scatter::new(t.to_owned(), x).name("x"), // TODO add units to x-axis
            Scatter::new(t.to_owned(), y).name("y"),
            Scatter::new(t, z).name("z"),
        ],
    ));
}
