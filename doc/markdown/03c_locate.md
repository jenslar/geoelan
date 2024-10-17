## locate

> - *Command/alias:* `locate` / `l`
> - *Help:* `geoelan locate --help`
> - *Basic usage:* `geoelan locate --indir INDIR/ --kind gopro`

`locate` will locate and match original GoPro and VIRB clips in the input folder. For VIRB, corresponding FIT-file/s will also be located. By optionally specifying a UUID (`--uuid`, `--fit`) or a clip (`--video`) in a specific session, only the files in that recording session will be returned. If you are unsure of the location of all relevant files, use an input path closer to the root, such as the root of an external hard drive. If duplicate files are found, the last one encountered will be returned.

**Flags**

| Short | Long           | Description
| :---: | :------------: | :----------
|       | `--quiet`      | Do not print file-by-file search progress

**Options**

| Short | Long          | Description                                   | Possible | Required
| :---: | :-----------: | :-------------------------------------------- | :---: | :------:
| `-i`  | `--indir`     | Input path for locating files                 | | yes
| `-k`  | `--kind`      | Camera brand                                  | `virb`, `gopro` | unless `-v`, `-u`, `-f`
| `-v`  | `--video`     | Clip in relevant session                  | |
|       | `--verify`    | \[GoPro\] Verify GPMF data, ignore corrupt files | |
| `-f`  | `--fit`       | \[VIRB\] FIT-file for selecting session           | |
| `-u`  | `--uuid`      | \[VIRB\] UUID for clip in session         | |

**Example 1**

|  |  |  |  |
| :-: | :-: | :-: | :-:
| `geoelan` | `locate`       | `-i INDIR/`       | `--kind gopro`
|           | sub-command   | input directory  | consider GoPro files

**Result:** Locates all GoPro clips in `INDIR/` (`-i`) and groups them in recording sessions.

**Example 2**

|  |  |  |  |
| :-: | :-: | :-: | :-:
| `geoelan` | `locate`     | `-i INDIR/` | `-v VIRB0001-1.MP4`
|           | sub-command | input directory | clip in relevant session

**Result:** Camera brand is detected automatically (in this case VIRB). Locates all clips in `INDIR/` (`-i`) for the recording session that contains `VIRB0001-1.MP4` (`-v`) together with the corresponding FIT-file.
