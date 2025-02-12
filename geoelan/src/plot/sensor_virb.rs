use std::{io::ErrorKind, path::PathBuf};

use fit_rs::{Fit, SensorType};
use plotly::{common::Title, layout::Shape, Scatter, Trace};

use crate::files::virb::select_session;

pub(crate) fn sensor2plot(
    args: &clap::ArgMatches,
// ) -> std::io::Result<(Title, Title, Title, Vec<Box<Scatter<f64, f64>>>)> {
) -> std::io::Result<(Title, Title, Title, Vec<Box<dyn Trace>>, Option<Shape>)> {
    let path = args.get_one::<PathBuf>("fit").unwrap();
    let y_axis = args.get_one::<String>("y-axis").unwrap(); // sensor type, required arg
    let x_axis = args.get_one::<String>("x-axis"); // optional, default to counts/index
    let session = *args.get_one::<bool>("session").unwrap();

    println!("Compiling data...");

    let (fit, range) = match session {
        true => {
            let f = Fit::new(path)?;
            let r = select_session(&f)?.range();
            (f, Some(r))
        }
        false => (Fit::new(path)?, None),
    };

    let sensor_type = match fit_rs::SensorType::from_str(&y_axis) {
        Some(s) => s,
        None => {
            let msg = format!("(!) '{y_axis}' is not supported by the FIT format or not yet implemented. Run Run 'geoelan inspect --fit {}' for a summary.", path.display());
            return Err(std::io::Error::new(ErrorKind::Other, msg));
        }
    };

    let sensor_data = match fit.sensor(&sensor_type, range.as_ref()) {
        Ok(s) => s,
        Err(err) => return Err(err.into()),
    };

    println!("Done");

    println!("Generating plot...");

    // Compile x, y, z Vec:s
    let y_axis_units = sensor_type.units();
    let (y_axis_x, y_axis_y, y_axis_z): (Vec<f64>, Vec<f64>, Vec<f64>);
    match sensor_type {
        SensorType::Barometer => {
            // 1D sensor
            y_axis_x = sensor_data
                .iter()
                .cloned()
                .flat_map(|s| s.calibrated_x.into_iter())
                .collect();
            (y_axis_y, y_axis_z) = (Vec::new(), Vec::new());
        }
        _ => {
            // 3D sensors
            ((y_axis_x, y_axis_y), y_axis_z) = sensor_data
                .iter()
                .cloned()
                // Flatten values into Vec<((f64, f64), f64)>...
                .flat_map(|s| {
                    s.calibrated_x
                        .into_iter()
                        .zip(s.calibrated_y.into_iter())
                        .zip(s.calibrated_z.into_iter())
                })
                // ...then unzip to get separate x, y, z Vec:s
                .unzip();
        }
    };

    // x-axis values
    let x_axis_name: &str;
    let x_axis_units: &str;
    let x_axis: Vec<f64> = match x_axis.map(|s| s.as_str()) {
        Some("t" | "time") => {
            x_axis_units = " (seconds)";
            x_axis_name = "Time";
            sensor_data
                .iter()
                .flat_map(|s| {
                    // add millisecond offset for each sample to record timestamp (sec + ms), return ms
                    s.sample_time_offset.iter().map(|o| {
                        *o as f64 / 1000. + s.timestamp as f64 + s.timestamp_ms as f64 / 1000.
                    })
                })
                .collect()
        }
        Some("c" | "count") => {
            x_axis_units = "";
            x_axis_name = "Sample count";
            (0..y_axis_x.len())
                .into_iter()
                .map(|i| (i + 1) as f64)
                .collect::<Vec<_>>()
        }
        other => {
            let msg = format!("(!) Invalid X-axis data type '{}'. Implemented values are 'time', 'count'. Run 'geoelan inspect --gpmf {}' for a summary.",
                other.unwrap_or("NONE"),
                path.display()
            );
            return Err(std::io::Error::new(ErrorKind::Other, msg));
        }
    };

    // Plot title: DATA [FILENAME]
    let title_txt = format!(
        "{} [{}]",
        sensor_type.to_string(),
        path.file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap()
    );
    let title = Title::from(title_txt);
    let x_axis_label_txt = format!("{x_axis_name}{x_axis_units}");
    let x_axis_label = Title::from(x_axis_label_txt);
    let y_axis_label_txt = format!(
        "{} ({})",
        sensor_type.quantifier(),
        sensor_type.units()
    );
    let y_axis_label = Title::from(y_axis_label_txt);

    println!("Done");

    return Ok((
        title,
        x_axis_label,
        y_axis_label,
        vec![
            Scatter::new(x_axis.to_owned(), y_axis_x)
                .name("x")
                .text(&y_axis_units), // TODO add units to x-axis
            Scatter::new(x_axis.to_owned(), y_axis_y)
                .name("y")
                .text(&y_axis_units),
            Scatter::new(x_axis, y_axis_z).name("z").text(&y_axis_units),
        ],
        None
    ));
}
