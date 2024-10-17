### GoPro and Garmin telemetry formats

Only GoPro Hero 5 Black and later use GoPro's GPMF format, earlier models are not supported. There are significant differences between Garmin's FIT-format and GoPro's GPMF-format. Here are a few:

|       | Garmin FIT       | GoPro GPMF
| :---: | :--------------- | :--------
| Storage form | Separate file (binary) | Embedded in MP4 (binary)
| Time stamps | Explicit, absolute time stamps for each data point | Absolute time stamps for GPS log, otherwise mostly derived from MP4 timing
| GPS | Each point time stamped | 18Hz models: Logged once per 1-second cluster. 10Hz models: Timestamps for each point

GoPro 10Hz GPS models are Hero 11 Black and later (Hero 12 Black has no onboard GPS, but Hero 13 Black does). VIRB Ultra 30 logs at 10Hz, but fitness watches usually log at 1Hz. GPS data differs between devices using Garmin's FIT format.

Both GoPro GPMF and Garmin FIT are binary formats, and thus can't be viewed in a text editor.

#### Documentation and development

Support for GPMF (GoPro) and FIT (VIRB) formats were written from scratch for GeoELAN with the help of the official documentation for both formats.

- Garmin FIT development kit and documentation: <https://developer.garmin.com/fit/>
- GoPro GPMF documentation and example code: <https://github.com/gopro/gpmf-parser>