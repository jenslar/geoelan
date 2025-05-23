## Commands

| Command   | Alias | Description |
| :-------: | :---: | :---------- |
| `cam2eaf` | `g2e` | Generate an ELAN-file, and link concatenated media files |
| `eaf2geo` | `e2g` | Geo-reference ELAN-annotations and generate annotated KML/GeoJSON |
| `locate`  | `l`   | Locate and match video clips and/or FIT-files |
| `inspect` | `i`   | Inspect the telemetry of a GoPro MP4-file or any Garmin FIT-file |
| `plot`    | `p`   | Plot the telemetry of a GoPro MP4-file or any Garmin FIT-file |
| `manual`  | `m`   | View or save this manual to disk |

Run `geoelan --help` for a general overview, or `geoelan <COMMAND> --help`, for an overview of a specific command.

The most relevant commands are probably `cam2eaf` and `eaf2geo`. `locate` is there to help with locating and matching video clips and/or FIT-files that belong to the same recording session, but this functionality partly exists in `cam2eaf` as well. `inspect` can be used to print various kinds of data in a GoPro MP4/Garmin FIT-file, but will do so in an unprocessed form. It is intended more as a technical aid for troubleshooting or to verify the contents of MP4/FIT-files. `plot` is used to plot sensor data and some of the GPS data, such as altitude over time. `manual` is for viewing or saving the full manual.

> Note that some parameters in the following sections may only be valid for e.g. GoPro cameras, not VIRB, and vice versa. The description column will be prefixed \[GoPro\] or \[VIRB\] to denote this.

### Set GoPro satellite lock (`--gpsfix`) and dilution of precision (`--gpsdop`) thresholdsldosfhds

GoPro cameras log how well they can see satellites.

If no satellite is in line of sight, the camera will log dummy coordinates. GeoELAN will ignore these by default, and for `cam2eaf` a '3D lock' (altitude is included) is the default. In cases where only 2D lock could be achieved, one can manually set minimum "lock level" via `--gpsfix`. Valid values are `0` (no lock), `2` (2D lock), and `3` (3D lock). Setting to `0` will result in unusable data for `eaf2geo` if most coordinates are bad.

Similarly, [dilution of precision](https://en.wikipedia.org/wiki/Dilution_of_precision_(navigation)) (DOP) is a value that represent how tightly clustered the satellites are. A lower value is better. Ideally, it should be below 5.0. There is no default value set, but if coordinates seem erratic, the maximum DOP value can be manually set via `--gpsdop`. E.g. perhaps try 10.0 and gradually go lower.

### Time adjustment with `--time-offset`

If the action camera has not adjusted for the current time zone, several commands have a `--time-offset` option. It takes a +/- value in hours that will be applied to all timestamps in the output, e.g. `--time-offset 7` will add seven hours to all timestamps.

### Reducing the number of coordinates with `--downsample`

The command `eaf2geo` outputs coordinates as KML and GeoJSON files. Since supported cameras log at either 10 or 18Hz, a 2 hour recording may contain more than 70 000 logged points. The `--downsample` parameter can be used to reduce the number of coordinates exported. Google Earth does not cope well with a large amount of points, whereas dedicated GIS software such as QGIS, usually will.

`--downsample` takes a positive numerical value that is effectively a divisor: `--downsample 10` means an average coordinate will be calculated for every cluster of 10 points. For 70 000 logged points, a value of 100 means the output will contain 700 averaged points and so on. If the user sets `--downsample` to a value that exceeds the total number of points logged by the GPS, it will be changed to the largest applicable value (resulting in a single point for the entire recording as opposed to none at all).

> Extreme values may affect the result in unexpected ways, depending on gaps in and/or quality of the GPS-data.

VIRB Ultra 30 logs at 10Hz, and GoPro logs at 10 or 18Hz depending on model. Only VIRB Ultra 30 and GoPro Hero 11 (10Hz) and later timestamp each individual point, whereas earlier models only timestamp a cluster of points. In the latter case, GeoELAN average each cluster to a single, timestamped point, resulting in roughly 1 point/second.

### If 'cam2eaf' or 'eaf2geo' return errors

Try the `inspect` command on problematic MP4/FIT-files. This way you can verify whether points were actually logged or not. If the file is corrupt the error message will also be printed.

### FFmpeg

The command `cam2eaf` requires [FFmpeg](https://ffmpeg.org). See the [appendix under _FFmpeg_](./04d_ffmpeg.md#ffmpeg) on how to install. If you intend to use the _static build_, point to it using `--ffmpeg PATH/TO/FFMPEG/ffmpeg` (`ffmpeg.exe` on Windows). If the `--ffmpeg` option is not used, `geoelan` will assume `ffmpeg` is available as a global command and complain accordingly if it is not.

> **TIP:** GeoELAN will never overwrite existing files without permission. Should you accidentally delete the generated ELAN-file with the output media files intact, just re-run the `cam2eaf` command. It will automatically skip concatenating videos, but still generate a new ELAN-file.

> **TIP:** In the tables for the respective command sections, arguments listed under 'Flags' do not take a value, whereas those listed under 'Options' do. If a `default` value is listed, it will be automatically set, unless the user specifies otherwise.
