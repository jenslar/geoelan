## plot

> - *Command/alias:* `plot` / `p`
> - *Help:* `geoelan plot --help`
> - *Basic usage:*
>   - GoPro: `geoelan plot --gpmf GH010026.MP4 --y-axis accelerometer --x-axis time`
>   - VIRB: `geoelan plot --fit 2017-01-28-05-16-40.fit --y-axis accelerometer --x-axis time`

`plot` can plot some of the telemetry in a semi-interactive web view, such as sensor data (accelerometer, gyroscope over time or sample count), and GPS data (latitude, longitude, altitude over time or distance - as a plot only, no maps).

**Flags:**

| Short | Long        | Description
| :---: | :---------- | :-----
| `-s`  | `--session` | Compile telemetry for a recording session.
|       | `--fill`    | Fill area under plot.
| `-a`  | `--average` | Generate a linear average for each sensor data cluster
|       | `--gps5`    | \[GoPro\] Force the use of GPS5 for Hero 11


**Options:**

| Short | Long                | Description
| :---: | :------------------ | :-----
| `-y`  | `--y-axis <y-axis>` | Data to plot on Y-axis.
| `-x`  | `--x-axis <x-axis>` | Data to plot on X-axis. Default: count
| `-g`  | `--gpmf <gpmf>`     | \[GoPro\] Unedited GoPro MP4-file, or extracted GPMF-track.
| `-i`  | `--indir`           | \[GoPro\] Input directory for locating GoPro clips.
| `-f`  | `--fit <fit>`       | \[VIRB\] Garmin FIT-file.

Possible Y-axis values:

- `acc`, `accelerometer`
- `gyr`, `gyroscope`
- `grv`, `gravity`
- `bar`, `barometer`
- `mag`, `magnetometer`
- `lat`, `latitude`
- `lon`, `longitude`
- `alt`, `altitude`
- `s2d`, `speed2d`
- `s3d`, `speed3d` (scalar only)
- `dop`, `dilution` (dilution of precision)
- `fix`, `gpsfix` (satellite lock level)

Possible X-axis value:

- `c`, `count`
- `t`, `time`
- `dst`, `distance`
