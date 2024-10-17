use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use gpmf_rs::{GoProSession, Gpmf};
use plotly::{common::Title, Scatter, Trace};

pub(crate) fn sensor2plot(
    args: &clap::ArgMatches,
    // ) -> std::io::Result<(Title, Title, Title, Vec<Box<Scatter<f64, f64>>>)> {
) -> std::io::Result<(Title, Title, Title, Vec<Box<dyn Trace>>)> {
    let path = args.get_one::<PathBuf>("gpmf").unwrap();
    let y_axis = args.get_one::<String>("y-axis").unwrap(); // sensor type, required arg
    let x_axis = args.get_one::<String>("x-axis"); // optional, default to counts/index
    let session = *args.get_one::<bool>("session").unwrap();
    let average = *args.get_one::<bool>("average").unwrap();
    let indir = match args.get_one::<PathBuf>("input-directory") {
        Some(p) => p.to_owned(),
        None => match path.parent() {
            Some(d) => {
                if d == Path::new("") {
                    PathBuf::from(".")
                } else {
                    d.to_owned()
                }
            }
            None => {
                let msg = "(!) Failed to determine input directory";
                return Err(std::io::Error::new(ErrorKind::Other, msg));
            }
        },
    };

    println!("Compiling data...");

    let gpmf = match session {
        true => GoProSession::from_path(&path, Some(&indir), false, true, true)?.gpmf()?,
        false => Gpmf::new(&path, false)?,
    };

    // y-axis values
    let sensor_type = gpmf_rs::SensorType::from(y_axis.as_str());
    let sensor_data = gpmf.sensor(&sensor_type);

    println!("Done");

    println!("Generating plot...");

    if sensor_data.len() == 0 {
        let device = gpmf
            .device_name()
            .first()
            .cloned()
            .unwrap_or(String::from("Unknown model"));
        let msg = format!("(!) No '{}' data found. Either it is not supported by {device} or not yet implemented. Run 'geoelan inspect --gpmf {}' for a summary.",
            sensor_type.to_string(),
            path.display()
        );
        return Err(std::io::Error::new(ErrorKind::Other, msg));
    }

    let y_axis_units = sensor_type.units();
    let y_axis_quantifier = sensor_type.quantifier();

    // Compile x, y, z Vec:s
    // let ((y_axis_x, y_axis_y), y_axis_z): ((Vec<f64>, Vec<f64>), Vec<f64>) = sensor_data.iter()
    //     // Flatten values into Vec<((f64, f64), f64)>...
    //     .flat_map(|s| s.x().into_iter()
    //         .zip(s.y().into_iter())
    //         .zip(s.z().into_iter()))
    //     // ...then unzip to get separate x, y, z Vec:s
    //     .unzip();

    let (y_axis_x, y_axis_y, y_axis_z) = match average {
        false => (
            sensor_data.iter().flat_map(|s| s.x()).collect::<Vec<f64>>(),
            sensor_data.iter().flat_map(|s| s.y()).collect::<Vec<f64>>(),
            sensor_data.iter().flat_map(|s| s.z()).collect::<Vec<f64>>(),
        ),
        // A single average value for each sensor cluster
        true => (
            sensor_data.iter().map(|s| s.x_avg()).collect::<Vec<f64>>(),
            sensor_data.iter().map(|s| s.y_avg()).collect::<Vec<f64>>(),
            sensor_data.iter().map(|s| s.z_avg()).collect::<Vec<f64>>(),
        ),
    };

    // x-axis values
    let x_axis_name: &str;
    let x_axis_units: &str;
    // !!! check whether unwraps are ok for gpmf sensor implementation
    let (total, duration) = sensor_data
        .last()
        .map(|s| (s.total, s.timestamp.unwrap() + s.duration.unwrap()))
        .unwrap();
    let x_axis: Vec<f64> = match x_axis.map(|s| s.as_str()) {
        Some("t" | "time") => {
            x_axis_units = " (seconds)";
            x_axis_name = "Time";
            let sample_rate = total as f64 / duration.as_seconds_f64();
            let t_incr = 1. / sample_rate;
            (0..total)
                .into_iter()
                .map(|i| i as f64 * t_incr)
                .collect::<Vec<_>>()
        }
        Some("c" | "count") => {
            x_axis_units = "";
            x_axis_name = "Sample count";
            (0..total)
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
    let y_axis_label_txt = format!("{y_axis_quantifier} ({y_axis_units})");
    let y_axis_label = Title::from(y_axis_label_txt);

    println!("Done");

    return Ok((
        title,
        x_axis_label,
        y_axis_label,
        vec![
            Scatter::new(x_axis.to_owned(), y_axis_x)
                .name("x")
                .text(y_axis_units),
            Scatter::new(x_axis.to_owned(), y_axis_y)
                .name("y")
                .text(y_axis_units),
            Scatter::new(x_axis, y_axis_z).name("z").text(y_axis_units),
        ],
    ));
}
