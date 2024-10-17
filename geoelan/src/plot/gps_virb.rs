use std::{io::ErrorKind, path::PathBuf};

use fit_rs::{Fit, FitPoint};
use plotly::{
    common::{Fill, Title},
    Scatter, Trace,
};

use crate::{files::virb::select_session, geo::haversine};

pub(crate) fn gps2plot(
    args: &clap::ArgMatches,
// ) -> std::io::Result<(Title, Title, Title, Vec<Box<Scatter<f64, f64>>>)> {
) -> std::io::Result<(Title, Title, Title, Vec<Box<dyn Trace>>)> {
    let path = args.get_one::<PathBuf>("fit").unwrap(); // verified to exist already
    let y_axis = args.get_one::<String>("y-axis").unwrap(); // sensor type, required arg
    let x_axis = args.get_one::<String>("x-axis"); // optional, default to counts/index
    let fill = *args.get_one::<bool>("fill").unwrap();
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

    // Convert to easier to use form
    let gps: Vec<FitPoint> = fit
        .gps(range.as_ref())?
        .iter()
        .map(|g| g.to_point())
        .collect();

    println!("Done");

    println!("Generating plot...");

    let x_axis_units: &str;
    let x_axis_name: &str;
    let x: Vec<f64> = match x_axis.map(|s| s.as_str()) {
        Some("t" | "time") => {
            x_axis_units = "seconds";
            x_axis_name = "Time";
            gps.iter().map(|g| g.time.as_seconds_f64()).collect()
        }
        Some("dst" | "distance") => {
            x_axis_units = "meters";
            x_axis_name = "Distance";
            let mut dist: Vec<f64> = vec![0.];
            let mut d = 0.;
            for p in gps.windows(2) {
                d += haversine(p[0].latitude, p[0].longitude, p[1].latitude, p[1].longitude);
                dist.push(d)
            }
            dist
        }
        other => {
            let msg = format!(
                "(!) Invalid X-axis data type '{}'. Run 'geoelan inspect --fit {}' for a summary.",
                other.unwrap_or("NONE"),
                path.display()
            );
            return Err(std::io::Error::new(ErrorKind::Other, msg));
        }
    };

    let y_axis_units: &str;
    let y_axis_name: &str;
    let y: Vec<f64> = match y_axis.as_str() {
        "lat" | "latitude" => {
            y_axis_units = "deg";
            y_axis_name = "Latitude";
            gps.iter().map(|p| p.latitude).collect()
        }
        "lon" | "longitude" => {
            y_axis_units = "deg";
            y_axis_name = "Longitude";
            gps.iter().map(|p| p.longitude).collect()
        }
        "alt" | "altitude" => {
            y_axis_units = "m";
            y_axis_name = "Altitude";
            gps.iter().map(|p| p.altitude).collect()
        }
        "s2d" | "speed2d" => {
            y_axis_units = "m/s";
            y_axis_name = "2D speed";
            gps.iter().map(|p| p.speed2d).collect()
        }
        "s3d" | "speed3d" => {
            y_axis_units = "m/s";
            y_axis_name = "3D speed";
            gps.iter().map(|p| p.speed3d).collect()
        }
        other => {
            let msg = format!("(!) '{other}' is not supported by VIRB or not yet implemented. Run 'geoelan inspect --fit {}' for a summary.",
                path.display()
            );
            return Err(std::io::Error::new(ErrorKind::Other, msg));
        }
    };

    let title_txt = format!(
        "GPS [{}]",
        path.file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap()
    );
    let title = Title::from(title_txt);
    let x_axis_label_txt = format!("{x_axis_name} ({x_axis_units})");
    let x_axis_label = Title::from(x_axis_label_txt);
    let y_axis_label_txt = format!("{y_axis_name} ({y_axis_units})");
    let y_axis_label = Title::from(y_axis_label_txt);

    let x_y_scatter = if fill {
        // Fill, would be better to have an arbitrary Y value to give more height to data
        Scatter::new(x, y).fill(Fill::ToZeroY).text(y_axis_units)
    } else {
        Scatter::new(x, y).text(y_axis_units)
    };

    println!("Done");

    Ok((title, x_axis_label, y_axis_label, vec![x_y_scatter]))
}
