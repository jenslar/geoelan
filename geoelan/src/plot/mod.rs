//! Plot GoPro and VIRB sensor time series data as a interactive HTML or a static image
//! using <https://lib.rs/crates/plotly>.
//! Export to CSV and import into ELAN as time series, e.g. to find sections with
//! altitude changes as annotation targets.
//!
//! Currently only does a time series 2D plot, e.g. air pressure (VIRB) over time.

use std::io::ErrorKind;

mod gps_gopro;
mod gps_virb;
mod sensor_gopro;
mod sensor_virb;
mod sensors;

// https://lib.rs/crates/plotly
use plotly::{
    color::Rgb,
    common::{HoverInfo, Label, Line, LineShape, Title},
    layout::{Axis, HoverMode},
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
    // 'kind': what sensor data to plot:
    // - 'gyro' / 'gyroscope' (GP/VIRB)
    // - 'accl' / 'accelerometer' (GP/VIRB)
    // - 'baro' / 'barometer' (VIRB), seemingly pascal (normal around 100 kPa), check Profile.xslx
    // - 'alt' / 'altitude' - GPS altitude (GP/VIRB)
    // - 'sp2d' / 'speed2d' - GPS 2D speed (GP/VIRB)
    // - 'sp3d' / 'speed3d' - GPS 3D speed (GP/VIRB)
    // - 'hdg' / 'heading' - GPS heading (VIRB - GP N/Y but possible via accelerometer)
    // - 'fix' / 'gpsfix' - GPS satellite lock/fix (GP - may exist in VIRB undocumented fields?)
    // - 'dop' / 'dilution' - GPS dilution of position (GP - may exist in VIRB undocumented fields?)
    let y_axis = args.get_one::<String>("y-axis").unwrap(); // sensor type, required arg
    let is_gopro = args.contains_id("gpmf");
    let is_fit = args.contains_id("fit");
    // let print_sensor_table = *args.get_one::<bool>("sensor-table").unwrap();

    // if print_sensor_table {
    //     return print_table()
    // }

    // Data in tuples (DATA, SECONDS) as [(f64, f64), ...]

    let title: Title;
    let x_axis_label: Title;
    let y_axis_label: Title;
    // let traces: Vec<Box<Scatter<f64, f64>>>;
    let traces: Vec<Box <dyn Trace>>;

    // GoPro
    if is_gopro {
        (title, x_axis_label, y_axis_label, traces) = match y_axis.as_str() {
            "acc" | "accelerometer"
            | "gyr" | "gyroscope"
            | "grv" | "gravity"
            | "bar" | "barometer"
            | "mag" | "magnetometer" => sensor_gopro::sensor2plot(args)?,
            _ => gps_gopro::gps2plot(&args)?,
        }
    // FIT, VIRB
    } else if is_fit {
        (title, x_axis_label, y_axis_label, traces) = match y_axis.as_str() {
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
    let layout = Layout::new()
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
    plot.set_layout(layout);

    // Add traces to plot canvas
    for trace in traces.into_iter() {
        // plot.add_trace(trace.hover_text("some text"))
        plot.add_trace(trace)
    }

    plot.show();

    Ok(())
}
