## Before you start

### Practical advice
- **Keep the original files**. GeoELAN can only use unedited files, since these contain all telemetry (GPS log, sensor data etc), and identifiers used for synchronisation.
    - Pre-maturely **converting videos will discard both telemetry and identifiers**, which means there is no data for GeoELAN to work with.
    - Low-resolution clips have a `.LRV` extension for GoPro, and `.GLV` for VIRB (these are normal MP4-files).
    - GeoELAN will automaticall use low-resolution clips for linking in ELAN. Run `geoelan cam2eaf --link-high-res` to link high-resolution video instead.
- **You can rename files**. GeoELAN does not use file name when matching files as long as the file extension for video is either `.MP4`, `.LRV`, or `.GLV` (case ignored).
- **Data locations**:
    - **GoPro**: All telemetry is embedded inside the MP4-files.
    - **VIRB**: All telemetry, such as the GPS-log, is stored as a separate FIT-file.
    - **Keep all files on the microSD** unless you are absolutely certain which files are relevant.

### Running GeoELAN

- GeoELAN is a command line tool and has no graphical user interface.
- [FFmpeg](https://www.ffmpeg.org) is required to concatenate clips. (`cam2eaf`)
- If you use macOS and GeoELAN does not run, see <https://support.apple.com/en-us/HT202491>.

### Device compatibility

- GoPro: Only "main line" Hero cameras with GPS have been tested, but Max and Fusion cameras may still work.
- Garmin: Only VIRB Ultra 30 has been tested extensively, but earlier VIRB models may still work.

### GPS

Make sure the GPS is turned on and has acquired a satellite lock. This may take a couple of minutes or longer, especially if you have not used the camera for a while or have traveled far between uses.

Verifying a satellite lock:
- For **VIRB**, the GPS-icon should be steady, not blinking (it may log coordinates while the icon is still blinking, but do not rely on this being the norm).
- For **GoPro**, the GPS-icon should be white, not gray. The icon only shows under settings, not on the main screen.

> It may be difficult to acquire a satellite lock and/or reliably log position in areas with heavy overhead vegetation or dense cities with very tall buildings. Using a headstrap, instead of a cheststrap, sometimes helps.

GPS logging behaviour:
- **GoPro** logs dummy coordinates if no lock has been acquired. GeoELAN will not use these.
    - Verify lock by running: `geoelan inspect --gpmf PATH/TO/GOPRO.MP4 --gps` which will list number of bad points.
- **VIRB** seems not to log position at all until a satellite lock has been acquired.

### Annotating in ELAN

- It is best to limit each kind of observed phenomena you wish to geo-reference to a single ELAN-tier, so...
- ...to keep e.g. place names and plant sightings within the same ELAN-file, make a separate tier for each (see the example walkthrough in the next section). Then you can just re-run GeoELAN on the same ELAN-file and select another tier to geo-reference along with changing other output options as required.