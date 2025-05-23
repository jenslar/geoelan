### Inspecting telemetry and MP4 files

`inspect` will mostly print raw values - down to a list of bytes for some kinds of data - that require further processing to be of use. The exact nature of this data differs between GoPro and Garmin. For GPS data, the flag `--gps` can be used for either device to print a processed GPS-log showing coordinates in decimal degrees etc. Sensor data can also be printed via `--sensor <SENSOR_TYPE>`. Other GeoELAN commands, such as `eaf2geo`, always convert data to the relevant forms.

If a GoPro MP4 or a Garmin FIT-file can not be properly parsed, GeoELAN will often return an error message that may hint at the issue. Try `inspect` on files that raise errors with the other commands.

#### GoPro

GoPro cameras embed all logged telemetry inside the MP4-files. In contrast to Garmin FIT, data types have no numerical identifier (see below) so internally, text descriptions are used instead.

To list all data types logged in a GoPro MP4-file, run:

```sh
geoelan inspect --gpmf GOPROVIDEO.MP4
```

This will list all data streams:

```
Unique data stream types (1018 DEVC streams in total):
    Accelerometer
    Average luminance
    Exposure time (shutter speed)
    Face Coordinates and details
    GPS (Lat., Long., Alt., 2D speed, 3D speed)
    Gyroscope
    Image uniformity
    Predominant hue[[hue, weight], ...]
    Scene classification[[CLASSIFIER_FOUR_CC,prob], ...]
    Sensor ISO
    Sensor read out time
    White Balance RGB gains
    White Balance temperature (Kelvin)
```

Use the data names in the list to print raw data for a specific type (note the citation marks):

```sh
geoelan inspect --gpmf GOPROVIDEO.MP4 --type "GPS (Lat., Long., Alt., 2D speed, 3D speed)"
```

Earlier GoPro models list GPS data as `GPS (Lat., Long., Alt., 2D speed, 3D speed)`, whereas Hero 11 Black and later models log more data for each point and use `GPS (Lat., Long., Alt., 2D, 3D, days, secs, DOP, fix)`. Hero 11 Black logs both the old and the new variants, whereas Hero 13 Black only logs to the newer format. Hero 12 Black does not have a GPS module.

Print the GPS log in a more conventional form:

```sh
geoelan inspect --gpmf GOPROVIDEO.MP4 --gps
```

Export the GPS log as a KML or GeoJSON file:

```sh
geoelan inspect --gpmf GOPROVIDEO.MP4 --kml
geoelan inspect --gpmf GOPROVIDEO.MP4 --json
```

##### GPMF byte offsets

GoPro telemetry is stored as samples, interleaved between audio and video samples (and other tracks' samples). To list the sample locations and sizes, run:

```sh
geoelan inspect --video GOPROVIDEO.MP4 --offsets "GoPro MET"
```

`GoPro MET` is the name of the MP4 track holding timed GPMF data.

This returns a table listing the samples' byte offsets (e.g. `@2026761919`), their sizes in bytes, and durations:

```
...
[ 359 GoPro MET/4] @2026761919 size: 7252   duration: 1s1ms
[ 360 GoPro MET/4] @2031934877 size: 7444   duration: 1s1ms
[ 361 GoPro MET/4] @2037379676 size: 7380   duration: 1s1ms
[ 362 GoPro MET/4] @2043168135 size: 7348   duration: 1s1ms
...
```

Similarly, you can print raw sample data for a track:

```sh
geoelan inspect --video GOPROVIDEO.MP4 --samples "GoPro MET"
```

 Save all track samples as a file (similar to FFmpeg's track export):

```sh
geoelan inspect --video GOPROVIDEO.MP4 --dump "GoPro MET"
```

Note that the video data may be many GB in size. GeoELAN will list the total size and prompt the user before saving to disk.

##### Images

Original GoPro JPEG-images can also be inspected. These will contain much less GPMF data than the MP4-files, and are currently not used elsewhere in GeoELAN's workflow. If no named data shows up in the summary, try  `geoelan inspect --gpmf GOPROIMAGE.JPG --verbose` to print the raw data. Early GoPro models do not embed GPMF data in JPEG-images.

#### Garmin FIT

The FIT-format is quite different to GoPro's GPMF, apart from being a separate file. There is among other things, additional information about VIRB recording sessions. The VIRB starts logging to a FIT-file the moment the camera is turned on, and only stops when it is turned off. This means that a single FIT-file may contain data for multiple recording sessions. Data is logged continuously - even between recordings.

Inside a FIT-file, data is identified by a numerical identifier. For example, GPS data is `160`, also referred to as `gps_metadata` in the [FIT Software Development Kit](https://developer.garmin.com/fit/download/) (FIT SDK). `inspect` lists both identifiers in the summary table, but only the numerical identifier is logged inside the FIT-file.

List all data types logged in a VIRB FIT-file:

```sh
geoelan inspect --fit FITFILE.FIT
```

This will return a table:

```
 Global ID | Message type                 | Count
...................................................
         0 | file_id                      |      1
        18 | session                      |      1
        19 | lap                          |      1
        20 | record                       |   6209
        21 | event                        |      1
        22 | UNKNOWN_TYPE_22              |      2
        23 | device_info                  |      3
        34 | activity                     |      1
        49 | file_creator                 |      1
       104 | UNKNOWN_TYPE_104             |    104
       160 | gps_metadata                 |  60114
       161 | camera_event                 |     24
       162 | timestamp_correlation        |      1
       164 | gyroscope_data               |  20405
       165 | accelerometer_data           |  20405
       167 | three_d_sensor_calibration   |     59
       208 | magnetometer_data            |  20405
       209 | barometer_data               |   6209
       210 | one_d_sensor_calibration     |      1
       219 | UNKNOWN_TYPE_219             |      1
...................................................
                                    Total:  133948
```

Find "Global ID" for the data type you wish to inspect further. To print GPS data in its "raw" form, run:

```sh
geoelan inspect --fit FITFILE.FIT --type 160
```

Print the GPS log in a more conventional form:

```sh
geoelan inspect --fit FITFILE.FIT --gps
```

Save the full GPS log as a KML or GeoJSON file:

```sh
geoelan inspect --fit FITFILE.FIT --kml
geoelan inspect --fit FITFILE.FIT --json
```

Print a single type of data for a specific recording session:

```sh
geoelan inspect --fit FITFILE.FIT --type 160 --session
```

This will return a table listing all VIRB recording sessions:

```
 Session | Clips | First UUID in session
............................................................................................
  1.     |  1    | VIRBactioncameraULTRA30_Tall_2688_2016_29..._1_17_2017-01-28-05-16-40.fit
  2.     |  1    | VIRBactioncameraULTRA30_Tall_2688_2016_29..._1_18_2017-01-28-05-16-40.fit
  3.     |  3    | VIRBactioncameraULTRA30_Tall_2688_2016_29..._1_19_2017-01-28-05-16-40.fit
         |       | VIRBactioncameraULTRA30_Tall_2688_2016_29..._2_19_2017-01-28-05-16-40.fit
         |       | VIRBactioncameraULTRA30_Tall_2688_2016_29..._3_19_2017-01-28-05-16-40.fit
  4.     |  1    | VIRBactioncameraULTRA30_Tall_2688_2016_29..._1_20_2017-01-28-05-16-40.fit
  5.     |  1    | VIRBactioncameraULTRA30_Tall_2688_2016_29..._1_21_2017-01-28-05-16-40.fit
............................................................................................
Select session:
```

Type the number in the "Session" column for the relevant session. The output will now be limited to the selected recording session. KML and GeoJSON files can be filtered this way as well.

You could also specify recording session via a VIRB MP4-file to achieve the same result:

```sh
geoelan inspect --video VIRBVIDEO.MP4 --fit FITFILE.MP4
```

To find out the embedded UUID of a VIRB MP4-file, run:
```sh
geoelan inspect --video VIRBVIDEO.MP4
```

This will return the embedded UUID:
```
UUID: VIRBactioncameraULTRA30_Expansive_1920_1440_29.9700_3937280306_3af2a648_1_299_2021-05-03-14-23-23.fit
```

Most FIT-files, from e.g. watches, bike computers, will work with `inspect`. Custom developer data is also supported (such fields will be prefixed '`DEV`' when inspecting). However, some FIT features are exclusive to VIRB, such as UUID and selecting sessions.

Compressed timestamp headers are not supported. In such cases, the tool will report the error and exit. Missing features may or may not be implemented in future versions.

> ❗For those who wish to dig deeper, the [Garmin FIT Software Development Kit](https://developer.garmin.com/fit/download/) contains a spreadsheet, `Profile.xlsx`, which lists the kinds of data a FIT-file may contain. Not all of those apply to every device however, and undocumented data types exist.

#### Video/MP4-files

Some options apply to any MP4-file. Access these by using the `--video` option.

The `--meta` flag will show raw (i.e. bytes) content for the so-called user data section (a.k.a. `udta` atom), where some cameras log custom data. GoPro embeds undocumented GPMF data in this section, which will also be listed. Garmin logs a unique identifier here (the "UUID" mentioned above).

List tracks and information for any MP4 file (GoPro and VIRB files list additional information, such as the unique identifers used for grouping clips into recording sessions):

```sh
geoelan inspect --video VIDEOFILE.MP4
```

List sample byte offsets for a track in any MP4 file:

```sh
geoelan inspect --video VIDEOFILE.MP4 --offsets <TRACK_ID>
```

List atom structure in any MP4 file:

```sh
geoelan inspect --video VIDEOFILE.MP4 --atoms
```
