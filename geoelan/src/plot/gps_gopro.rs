use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use gpmf_rs::{GoProSession, Gpmf};
use plotly::{
    color::NamedColor, common::{DashType, Fill, Title}, layout::{Shape, ShapeLine, ShapeType}, Bar, Scatter, Trace
};

use crate::geo::haversine;

pub(crate) fn gps2plot(
    args: &clap::ArgMatches,
) -> std::io::Result<(Title, Title, Title, Vec<Box<dyn Trace>>, Option<Shape>)> {
    let path = args.get_one::<PathBuf>("gpmf").unwrap(); // verified to exist already
    let y_axis = args.get_one::<String>("y-axis").unwrap(); // sensor type, required arg
    let x_axis = args.get_one::<String>("x-axis"); // optional, default to counts/index
    let fill_to_zero_y = match args.get_one::<bool>("fill").unwrap() {
        true => Fill::ToZeroY,
        false => Fill::None,
    };
    let session = *args.get_one::<bool>("session").unwrap();
    let gps5 = *args.get_one::<bool>("gps5").unwrap();
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
        true => {
            let session = GoProSession::from_path(&path, Some(&indir), false, true, true)?;

            println!("Located the following session files:");
            for (i, gopro_file) in session.iter().enumerate() {
                println!(
                    "{:4}. MP4: {}",
                    i + 1,
                    gopro_file
                        .mp4
                        .as_ref()
                        .and_then(|f| f.to_str())
                        .unwrap_or("High-resolution MP4 not set")
                );
                println!(
                    "      LRV: {}",
                    gopro_file
                        .lrv
                        .as_ref()
                        .and_then(|f| f.to_str())
                        .unwrap_or("Low-resolution MP4 not set")
                );
            }
            
            println!("Merging GPMF-data for {} files...", session.len());
            session.gpmf()?
        },
        false => Gpmf::new(&path, false)?,
    };

    // Gps5 may fail if not available. Currently, only Hero11 logs both
    // Removed filter/pruning on fix or dop
    let gps = match gps5 {
        true => gpmf.gps5(),
        false => gpmf.gps(),
    };

    println!("Done ({} points)", gps.len());

    println!("Generating plot...");

    // let mut bar_plot = false;

    let y_axis_units: Option<&str>;
    let y_axis_name: &str;
    // let mut bar_plot = false;
    let mut y_axis_static_line_height: Option<f64> = None;
    let y: Vec<f64> = match y_axis.as_str() {
        "lat" | "latitude" => {
            y_axis_units = Some("deg");
            y_axis_name = "Latitude";
            gps.iter().map(|p| p.latitude).collect()
        }
        "lon" | "longitude" => {
            y_axis_units = Some("deg");
            y_axis_name = "Longitude";
            gps.iter().map(|p| p.longitude).collect()
        }
        "alt" | "altitude" => {
            y_axis_units = Some("m");
            y_axis_name = "Altitude";
            gps.iter().map(|p| p.altitude).collect()
        }
        "s2d" | "speed2d" => {
            y_axis_units = Some("m/s");
            y_axis_name = "2D speed";
            gps.iter().map(|p| p.speed2d).collect()
        }
        "s3d" | "speed3d" => {
            y_axis_units = Some("m/s");
            y_axis_name = "3D speed";
            gps.iter().map(|p| p.speed3d).collect()
        }
        "dop" | "dilution-of-precision" => {
            // dilution of precision should ideally stay below 5.0
            y_axis_units = None;
            y_axis_name = "Dilution of precision";
            y_axis_static_line_height = Some(5.);
            // bar_plot = true;
            gps.iter().map(|p| p.dop).collect()
        }
        "fix" | "gpsfix" => {
            // satellite lock level/GPS fix, visualising lock level
            y_axis_units = None;
            y_axis_name = "Satellite lock level";
            y_axis_static_line_height = Some(3.);
            // bar_plot = true;
            gps.iter().map(|p| p.fix as f64).collect()
        }
        other => {
            let msg = format!("(!) '{other}' is not supported by GoPro or not yet implemented. Run 'geoelan inspect --gpmf {}' for a summary.",
                path.display()
            );
            return Err(std::io::Error::new(ErrorKind::Other, msg));
        }
    };

    println!("[Y-axis '{y_axis_name}'] {} values, units: {}", y.len(), y_axis_units.unwrap_or("N/A"));

    let x_axis_units: Option<&str>;
    let x_axis_name: &str;
    let x: Vec<f64> = match x_axis.map(|s| s.as_str()) {
        Some("t" | "time") => {
            x_axis_units = Some("seconds");
            x_axis_name = "Time";
            gps.iter().map(|g| g.time.as_seconds_f64()).collect()
        }
        Some("dst" | "distance") => {
            x_axis_units = Some("meters");
            x_axis_name = "Distance";
            // Generate increasing distance vector
            let mut dist: Vec<f64> = vec![0.];
            let mut d = 0.;
            for p in gps.0.windows(2) {
                d +=
                    haversine(p[0].latitude, p[0].longitude, p[1].latitude, p[1].longitude) * 1000.; // haversine returns km
                dist.push(d)
            }
            dist
        }
        Some("c" | "count") => {
            x_axis_units = None;
            x_axis_name = "Sample count";
            (0..gps.len())
                .into_iter()
                .map(|i| (i + 1) as f64)
                .collect::<Vec<_>>()
        }
        other => {
            let msg = format!("(!) Invalid X-axis data type '{}'. Implemented values are 'time', 'distance'. Run 'geoelan inspect --gpmf {}' for a summary.",
                other.unwrap_or("NONE"),
                path.display()
            );
            return Err(std::io::Error::new(ErrorKind::Other, msg));
        }
    };

    println!("[X-axis '{x_axis_name}'] {} values, units: {}", x.len(), x_axis_units.unwrap_or("N/A"));

    let static_line: Option<Shape> = match y_axis_static_line_height {
        Some(y) => {
            let x = x.last().cloned().expect("No X-value for static line.");
            Some(Shape::new()
                .shape_type(ShapeType::Line)
                .x0(0.)
                .y0(y)
                .x1(x)
                .y1(y)
                .line(
                    ShapeLine::new()
                        .color(NamedColor::DarkRed)
                        .width(2.)
                        .dash(DashType::Dot)
                ))
        },
        None => None,
    };

    match (x_axis_name, y_axis_name) {
        ("Distance", "Dilution of precision" | "Satellite lock level") => {
            let msg =
                format!("(!) X-axis '{x_axis_name}' can not be used with Y-axis '{y_axis_name}'.");
            return Err(std::io::Error::new(ErrorKind::Other, msg));
        }
        _ => (),
    }

    assert_eq!(x.len(), y.len(), "(!) X and Y differ in size.");

    let title_txt = format!(
        "GPS [{}]",
        path.file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap()
    );
    let title = Title::from(title_txt);
    let x_axis_label_txt = format!(
        "{x_axis_name}{}",
        x_axis_units.map(|u| format!(" ({u})")).unwrap_or_default()
    );
    let x_axis_label = Title::from(x_axis_label_txt);
    let y_axis_label_txt = format!(
        "{y_axis_name}{}",
        y_axis_units.map(|u| format!(" ({u})")).unwrap_or_default()
    );
    let y_axis_label = Title::from(y_axis_label_txt);

    println!("Done");

    let x_y_trace: Box<dyn Trace> = Scatter::new(x, y)
        .fill(fill_to_zero_y)
        .text(y_axis_units.unwrap_or_default());

    // let x_y_trace: Box<dyn Trace> = match bar_plot {
    //     true => Bar::new(x, y)
    //         .text(y_axis_units.unwrap_or_default()),
    //     false => Scatter::new(x, y)
    //         .fill(fill_to_zero_y)
    //         .text(y_axis_units.unwrap_or_default()),
    // };

    Ok((title, x_axis_label, y_axis_label, vec![x_y_trace], static_line))
}

enum XAxisType {
    Time,
    Distance,
    Count,
    Invalid,
}

impl From<&str> for XAxisType {
    fn from(value: &str) -> Self {
        match value {
            "t" | "time" => Self::Time,
            "dst" | "distance" => Self::Distance,
            "c" | "count" => Self::Count,
            _ => Self::Invalid,
        }
    }
}

enum YAxisType {
    Latitude,
    Longitude,
    Altitude,
    Speed2d,
    Speed3d,
    Dilution,
    Gpsfix,
    Invalid,
}

impl From<&str> for YAxisType {
    fn from(value: &str) -> Self {
        match value {
            "lat" | "latitude" => Self::Latitude,
            "lon" | "longitude" => Self::Longitude,
            "alt" | "altitude" => Self::Altitude,
            "s2d" | "speed2d" => Self::Speed2d,
            "s3d" | "speed3d" => Self::Speed3d,
            "dop" | "dilution" => Self::Dilution,
            "fix" | "gpsfix" => Self::Gpsfix,
            _ => Self::Invalid,
        }
    }
}

// fn valid_axis_match(x: &XAxisType, y: &YAxisType) {
//     match (x, y) {
//         (XAxisType::Time, YAxisType::Latitude) => todo!(),
//         (XAxisType::Time, YAxisType::Longitude) => todo!(),
//         (XAxisType::Time, YAxisType::Altitude) => todo!(),
//         (XAxisType::Time, YAxisType::Speed2d) => todo!(),
//         (XAxisType::Time, YAxisType::Speed3d) => todo!(),
//         (XAxisType::Time, YAxisType::Dilution) => todo!(),
//         (XAxisType::Time, YAxisType::Gpsfix) => todo!(),
//         (XAxisType::Time, YAxisType::Invalid) => todo!(),
//         (XAxisType::Distance, YAxisType::Latitude) => todo!(),
//         (XAxisType::Distance, YAxisType::Longitude) => todo!(),
//         (XAxisType::Distance, YAxisType::Altitude) => todo!(),
//         (XAxisType::Distance, YAxisType::Speed2d) => todo!(),
//         (XAxisType::Distance, YAxisType::Speed3d) => todo!(),
//         (XAxisType::Distance, YAxisType::Dilution) => todo!(),
//         (XAxisType::Distance, YAxisType::Gpsfix) => todo!(),
//         (XAxisType::Distance, YAxisType::Invalid) => todo!(),
//         (XAxisType::Count, YAxisType::Latitude) => todo!(),
//         (XAxisType::Count, YAxisType::Longitude) => todo!(),
//         (XAxisType::Count, YAxisType::Altitude) => todo!(),
//         (XAxisType::Count, YAxisType::Speed2d) => todo!(),
//         (XAxisType::Count, YAxisType::Speed3d) => todo!(),
//         (XAxisType::Count, YAxisType::Dilution) => todo!(),
//         (XAxisType::Count, YAxisType::Gpsfix) => todo!(),
//         (XAxisType::Count, YAxisType::Invalid) => todo!(),
//         (XAxisType::Invalid, YAxisType::Latitude) => todo!(),
//         (XAxisType::Invalid, YAxisType::Longitude) => todo!(),
//         (XAxisType::Invalid, YAxisType::Altitude) => todo!(),
//         (XAxisType::Invalid, YAxisType::Speed2d) => todo!(),
//         (XAxisType::Invalid, YAxisType::Speed3d) => todo!(),
//         (XAxisType::Invalid, YAxisType::Dilution) => todo!(),
//         (XAxisType::Invalid, YAxisType::Gpsfix) => todo!(),
//         (XAxisType::Invalid, YAxisType::Invalid) => todo!(),
//     }
// }

// "lat" | "latitude
// "lon" | "longitude
// "alt" | "altitude
// "s2d" | "speed2d
// "s3d" | "speed3d
// "dop" | "dilution
// "fix" | "gpsfix
