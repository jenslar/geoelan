## cam2eaf

> - *Command/alias:* `cam2eaf` / `c2e`
> - *Help:* `geoelan cam2eaf --help`
> - *Basic usage:* `geoelan cam2eaf --indir INDIR/ --video GH010006.MP4 --outdir OUTDIR/`

`cam2eaf` generates an ELAN-file with pre-linked media files. All clips in the specified recording session will be automatically located, grouped, and concatenated. A WAV-file from the full video is also extracted. By default the low-resolution footage is used (if found), use the `--link-high-res` flag to link the high-resolution footage. The corresponding coordinates can optionally be added a tier.

**Flags**

| Short | Long               | Description
| :---: | :----------------- | :----------
|       | `--dryrun`         | Show results but do not process or copy files
|       | `--fullgps`        | Use the full-res GPS log for the ELAN geotier
|       | `--geotier`        | Insert tier with synchronised coordinates in ELAN-file
|       | `--link-high-res`  | Link high-resolution video in ELAN-file
| `-l`  | `--low-res-only`   | Only concatenate low-res clips (`.LRV`/`.GLV`), ignores high-res clips
|       | `--single`         | Only use the specified clip, ignore remaining clips in session
|       | `--verify`         | \[GoPro\] Verify GPMF data, ignore corrupt clips

**Options**

| Short | Long              | Description                                      | Default   | Required
| :---: | :---------------- | :------------------------------------            | :-------: | :------:
|       | `--ffmpeg`        | Custom path to FFmpeg                            | `ffmpeg`  |
| `-i`  | `--indir`         | Input path for locating files                    |           | yes
| `-o`  | `--outdir`        | Output path for resulting files                  | `geoelan` |
| `-t`  | `--time-offset`   | Time offset in +/- hours                         | `0`       |
| `-v`  | `--video`         | Clip in the relevant session                     |           | unless `-f` or `-u`
|       | `--gpsfix`        | \[GoPro\] Minimum satellite lock                 | `3`       |
| `-f`  | `--fit`           | \[VIRB\] FIT-file                                |           | unless `-u` or `-v`
| `-u`  | `--uuid`          | \[VIRB\] UUID for a clip in the relevant session |           | unless `-f` or `-v`

### Example GoPro

**GoPro example**

| | | | | | |
| :-------: | :--------: | :-----------------: | :---------: | :----------: | :---------:
| `geoelan` | `cam2eaf` | `-v GH010026.MP4` | `-i INDIR/` | `-o OUTDIR/` | `--geotier`
| | command | clip in session | input directory | output directory | insert coordinate tier

**Result:** Locates all clips for the recording session containing the clip `GH010026.MP4` (`-g`) in the input directory `INDIR/` (`-i`). These will be concatenated, and the audio track exported as a WAV for use in ELAN. The resulting files are then copied to the output directory `OUTDIR/` (`-o`). The generated ELAN-file will also have synchronised coordinates inserted as a tier (`--geotier`).

### Examples VIRB

> ❓Recording session can be specified using one of `--fit`, `--uuid`, `--video`. These options are mutually exclusive. `--fit` returns a list of sessions present in the FIT-file, from which the user can select the relevant one. `--uuid` and `--video` require no further user input. UUID is the unique VIRB clip identifier and can be retreived by running `geoelan inspect --video VIRB0001-1.MP4`.

> ❗Using `--fullgps` (together with `--geotier`) may slow down ELAN considerably.

**VIRB example 1**

| | | | | | |
| :-------: | :--------: | :-----------------: | :---------: | :----------: | :---------:
| `geoelan` | `cam2eaf` | `-v VIRB0001-1.MP4` | `-i INDIR/` | `-o OUTDIR/` | `--geotier`
| | command | clip in session | input directory | output directory | insert coordinate tier

**Result:** Locates all clips for the recording session containing the clip `VIRB0001-1.MP4` (`-v`) in the input directory `INDIR/` (`-i`). These will be concatenated, and the audio track exported as a WAV for use in ELAN. The resulting files are then copied together with the corresponding FIT-file to the output directory `OUTDIR/` (`-o`). The generated ELAN-file will also have synchronised coordinates inserted as a tier (`--geotier`).

**VIRB example 2**

| | | | | | |
| :-------: | :--------: | :-----------------: | :---------: | :----------: | :---------:
|`geoelan` |`cam2eaf` |`-f 2017-01-28-05-16-40.FIT` |`-i INDIR/` |`-o OUTDIR/` | `-l`
| | command |FIT-file  | input directory | output directory | ignore hi-res MP4

**Result**: Recording session is specified via the FIT-file `2017-01-28-05-16-40.fit` (`-f`). The user will be prompted to select session from a list, allowing GeoELAN to locate the corresponding clips in the input directory `INDIR/` (`-i`). Only the low-resolution clips (`--low-res-only`) will be concatenated. All resulting files are then copied together with the corresponding FIT-file to the output directory `OUTDIR/` (`-o`).

> ❓If you are unsure of the whereabouts of the FIT-file, make the search wider. Specifying the root of an external hard drive as input directory (`--indir`) will make the search process take slightly longer, but should work well. Otherwise, just specify the FIT-file separately (`--fit`), which can be useful if it is located outside of the input directory.