//! Plot GoPro and VIRB sensor time series data as a interactive HTML or a static image
//! using <https://lib.rs/crates/plotly>.
//! Export to CSV and import into ELAN as time series, e.g. to find sections with
//! altitude changes as annotation targets.
//!
//! Currently only does a time series 2D plot, e.g. air pressure (VIRB) over time.

use std::{io::ErrorKind, path::PathBuf};

mod gps_gopro;
mod gps_virb;
mod sensor_gopro;
mod sensor_virb;
mod sensors;

// https://lib.rs/crates/plotly
use plotly::{
    color::Rgb,
    common::{HoverInfo, Label, Line, LineShape, Title},
    layout::{Axis, HoverMode, Shape},
    Layout, Plot, Scatter, Trace,
};

use self::sensors::print_table;

// Quick check for if requested data is sensor data or not.
fn is_sensor(value: &str) -> bool {
    match value {
        "gyr" | "gyroscope"
        | "acc" | "accelerometer"
        | "mag" | "magnetometer"       // VIRB magnetometer (+ Fusion, MAX, but not implemented)
        | "grv" | "gravity"            // GoPro gravity vector
        | "bar" | "barometer" => true, // VIRB barometer
        _ => false
    }
}

pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {
    // y-axis sensor data to plot:
    // - 'gyro' / 'gyroscope'     GP/VIRB
    // - 'accl' / 'accelerometer' GP/VIRB
    // - 'baro' / 'barometer'     VIRB, seemingly pascal (normal around 100 kPa), check Profile.xslx
    // - 'alt'  / 'altitude'      (GPS) altitude, GP/VIRB
    // - 'sp2d' / 'speed2d'       (GPS) 2D speed, GP/VIRB
    // - 'sp3d' / 'speed3d'       (GPS) 3D speed, GP/VIRB
    // - 'hdg'  / 'heading'       (GPS) heading, VIRB. GP N/A but possible via accelerometer)
    // - 'fix'  / 'gpsfix'        (GPS) satellite lock/fix, GP - VIRB may exist in undocumented gps_metadata fields?
    // - 'dop'  / 'dilution'      (GPS) dilution of precision, GP - VIRB may exist in undocumented gps_metadata fields?
    let y_axis = args.get_one::<String>("y-axis").unwrap(); // sensor type, required arg
    let is_gopro = args.contains_id("gpmf");
    let is_fit = args.contains_id("fit");

    // File path or dir (use fit/gpmf filestem if dir)
    let export_path = args.get_one::<PathBuf>("export");
    // Extract file stem if dir
    let export_file = match (args.get_one::<PathBuf>("fit"), args.get_one::<PathBuf>("gpmf")) {
        (Some(p), None) | (None, Some(p)) => {
            if let Some(path) = export_path {
                match path.is_dir() {
                    // User specified dir -> generate file name using fit/gpmf file stem
                    true => p.file_stem().map(|p| path.join(p).with_extension("html")),
                    // User specified file path, ensure extension is html
                    false => Some(path.with_extension("html")),
                }
            } else {
                None
            }
        },
        _ => None
    };
    let width = *args.get_one::<usize>("width").unwrap_or(&1920);
    let height = *args.get_one::<usize>("height").unwrap_or(&1440);

    // Data in tuples (DATA, SECONDS) as [(f64, f64), ...]

    let title: Title;
    let x_axis_label: Title;
    let y_axis_label: Title;
    // let traces: Vec<Box<Scatter<f64, f64>>>;
    let traces: Vec<Box <dyn Trace>>;
    let static_line: Option<Shape>;

    // GoPro
    if is_gopro {
        (title, x_axis_label, y_axis_label, traces, static_line) = match y_axis.as_str() {
            "acc" | "accelerometer"
            | "gyr" | "gyroscope"
            | "grv" | "gravity"
            | "bar" | "barometer"
            | "mag" | "magnetometer" => sensor_gopro::sensor2plot(args)?,
            _ => gps_gopro::gps2plot(&args)?,
        }
    // FIT, VIRB
    } else if is_fit {
        (title, x_axis_label, y_axis_label, traces, static_line) = match y_axis.as_str() {
            "acc" | "accelerometer"
            | "gyr" | "gyroscope"
            | "grv" | "gravity"
            | "bar" | "barometer"
            | "mag" | "magnetometer" => sensor_virb::sensor2plot(args)?,
            _ => gps_virb::gps2plot(args)?,
        };
    } else {
        let msg = "(!) No data file specified.";
        return Err(std::io::Error::new(ErrorKind::Other, msg));
    }

    // Create plot canvas
    let mut plot = Plot::new();
    let mut layout = Layout::new()
        .height(600)
        .x_axis(
            Axis::new()
                .title(x_axis_label)
                .grid_color(Rgb::new(255, 255, 255)),
        )
        .y_axis(
            Axis::new()
                .title(y_axis_label)
                .grid_color(Rgb::new(255, 255, 255)),
        )
        .plot_background_color(Rgb::new(229, 229, 229))
        .hover_mode(HoverMode::XUnified)
        .title(title);
    if let Some(line) = static_line {
        layout.add_shape(line);
    }
    plot.set_layout(layout);

    // Add traces to plot canvas
    for trace in traces.into_iter() {
        // plot.add_trace(trace.hover_text("some text"))
        plot.add_trace(trace)
    }

    if let Some(path) = export_file {
        plot.write_html(&path);
        println!("Exported HTML to {}", path.display());

        let png_path = path.with_extension("png");
        plot.write_image(&png_path, plotly::ImageFormat::PNG, width, height, 1.0);
        println!("Exported PNG to {}", png_path.display());

        let svg_path = path.with_extension("svg");
        plot.write_image(&svg_path, plotly::ImageFormat::SVG, width, height, 1.0);
        println!("Exported SVG to {}", svg_path.display());
    } else {
        plot.show();
    }

    Ok(())
}
