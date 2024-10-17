## inspect

> - *Command/alias:* `inspect` / `i`
> - *Help:* `geoelan inspect --help`
> - *Basic usage:*
>   - GoPro: `geoelan inspect --gpmf GH010026.MP4`
>   - VIRB: `geoelan inspect --fit 2017-01-28-05-16-40.fit`
>   - MP4: `geoelan inspect --video VideoFile.MP4`

`inspect` can print telemetry contents of a GoPro MP4 or a Garmin FIT-file. Options include filtering to a sub-set of the telemetry, such as GPS-data, and general MP4 structure (any MP4 file can be specified). `inspect` is more of a technical aid to, for example, verify that the GPS really did log coordinates. KML or GeoJSON files can also be generated.

**Flags**

| Short | Long                | Description
| :---: | :---------- | :--------------------------------
|       | `--debug`   | Print FIT definitions and data while parsing
|       | `--kml`     | Generate a KML-file
|       | `--ikml`    | Generate an indexed KML-file
|       | `--json`    | Generate a GeoJSON-file.
|       | `--verbose` | Print raw data
|       | `--gps`     | Print processed GPS log
|       | `--meta`    | Print MP4 custom user data (`udta` atom)
|       | `--atoms`   | Print MP4 atom hierarchy
| `-s`  | `--session` | GoPro: Merge session data. VIRB: Select from a list.

**Options**

| Short | Long           | Description                       |  Required
| :---: | :------------- | :-------------------------------- |  :------:
| `-t`  | `--type`       | Data type to print                |
| `-v`  | `--video`      | MP4-file                          | unless `-g`, `-f`
| `-o`  | `--offsets`    | Print byte offsets for specified track |
| `-g`  | `--gpmf`       | \[GoPro\]-file (MP4 or raw GPMF-file) |  unless `-f`, `-v`
| `-f`  | `--fit`        | \[VIRB\]FIT-file                      |  unless `-g`, `-v`

Note that `--type` takes a string for GoPro and a numerical identifier for VIRB. `--video` accepts any MP4-file. See the sections below.