## eaf2geo

> - *Command/alias:* `eaf2geo` / `e2g`
> - *Help:* `geoelan eaf2geo --help`
> - *Basic usage:* `geoelan eaf2geo --eaf VIRB0001-1.eaf --fit 2017-01-28-05-16-40.fit`

`eaf2geo` generates KML and GeoJSON files by geo-referencing all annotations in the specified tier. The user is presented with a list of all tiers in the ELAN-file to select from. Referred tiers are fine, but tokenized tiers can not be used, since these lack meaningful time stamps. Several output options exist via the `--geoshape` option, such as points or polylines (see below). In the resulting KML and GeoJSON files, any point that intersects with an annotation's timespan will inherit the annotation value as a description.

**Flags**

| Short | Long      | Description
| :---: | :-------: | :---------:
|       | `--cdata` | KML-option, added visuals in Google Earth

**Options**

|Short  | Long              | Description                       | Default       | Possible | Required
| :---: | :---------------: | :-------------------------------- | :-----------: | :------: | :------:
| `-d`  | `--downsample`    | Downsample factor for coordinates | `1`           |   |
| `-e`  | `--eaf`           | ELAN-file                         |               |   | yes
| `-f`  | `--fit`           | \[VIRB\] FIT-file                     |               |   | unless `-g`
| `-g`  | `--gpmf`          | \[GoPro\] MP4-file                    |               |   | unless `-f`
|       | `--geoshape`      | Output options for KML-file       | `point-all`  | `point-all`, `point-multi`, `point-single`, `line-all`, `line-multi`, `circle-2d`, `circle-3d` |
|       | `--height`        | Circle height (`circle-3d`) | `10.0`         |   |
|       | `--radius`        | Circle radius (`circle-2d`, `circle-3d`) | `2.0`         |   |
| `-t`  | `--time-offset`   | Time offset, +/- hours            | `0`           |   |
|       | `--vertices`      | Circle vertices/roundness ('circle-2d', 'circle-3d') | `40`         |   |

**GoPro example**

| | | | | |
| :-------: | :--------: | :-----------------: | :---------: | :----------: |
| `geoelan` | `eaf2geo`   | `-g GH010026.MP4` | `-e GH010026.eaf` | `--geoshape line-all`
| | command | original GoPro MP4-file | ELAN-file            | output option

**Result**: Geo-references annotations in the ELAN-file `GH010026.eaf` (`-e`) and generates KML and GeoJSON files with a continous poly-line, alternating between marked (annotated) and unmarked (un-annotated) sections (`--geoshape line-all`).
****

**VIRB example**

| | | | | |
| :-------: | :--------: | :-----------------: | :---------: | :----------: |
| `geoelan` | `eaf2geo`  | `-f 2017-01-28-05-16-40.fit` | `-e VIRB0001-1.eaf` | `--geoshape point-single`
| | command | FIT-file   | ELAN-file            | output option

**Result**: Geo-references annotations in the ELAN-file `VIRB0001-1.eaf` (`-e`) and generates KML and GeoJSON files with a single point per annotation (`--geoshape point-single`). Since no original VIRB clip is specified, the user will be presented with a list of clip UUIDs in the specified FIT-file `2017-01-28-05-16-40.fit` (`-f`) to choose from. It should be fairly straight forward to guess which session is relevant.
****
