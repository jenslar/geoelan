//! Plot GoPro and VIRB sensor time series data as a interactive HTML or a static image
//! using <https://lib.rs/crates/plotly>.
//! Export to CSV and import into ELAN as time series, e.g. to find sections with
//! altitude changes as annotation targets.
//! 
//! Currently only does a time series 2D plot, e.g. air pressure (VIRB) over time.

use std::io::ErrorKind;

mod sensors;
mod plot_gopro;
use plot_gopro::gopro2plot;
mod plot_virb;
use plot_virb::virb2plot;

// https://lib.rs/crates/plotly
use plotly::{Plot, Scatter, Layout, common::{Title, Marker}, layout::{LayoutGrid, GridPattern}, Scatter3D};

// Quick check for if requested data is sensor data or not.
fn is_sensor(value: &str) -> bool {
    match value {
        "gyr" | "gyroscrope"
        | "acc" | "accelerometer"
        | "mag" | "magnetometer" // VIRB magnetometer (+ Fusion, MAX)
        | "grv" | "gravity"     // GoPro gravtiy vector
        | "bar" | "baro" | "barometer" => true,
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
    // let y_axis = args.get_one::<String>("y-axis").unwrap(); // sensor type, required arg
    let is_gopro = args.contains_id("gpmf");
    let is_fit = args.contains_id("fit");

    // Data in tuples (DATA, SECONDS) as [(f64, f64), ...]

    let title: Title;
    let traces: Vec<Box<Scatter<f64, f64>>>;

    // GoPro
    if is_gopro {
        (title, traces) = gopro2plot(&args)?;
    // FIT, VIRB
    } else if is_fit {
        (title, traces) = virb2plot(args)?;
    } else {
        let msg = "(!) No data file specified.";
        return Err(std::io::Error::new(ErrorKind::Other, msg))
    }

    let layout = Layout::new()
        .height(600)
        .title(title);
    let mut plot = Plot::new();
    plot.set_layout(layout);
    for trace in traces.into_iter() {
        plot.add_trace(trace)
    }
    // let layout = Layout::new()
    //     .title(title);
    //     // .title(title)
    //     // !!! grid/subplots do not work as described?
    //     // .grid(LayoutGrid::new()
    //     //     .columns(3)
    //     //     .rows(3) // x, y, z
    //     //     .pattern(GridPattern::Independent)).title(title);

    // plot.set_layout(layout);
    plot.show();

    Ok(())
}