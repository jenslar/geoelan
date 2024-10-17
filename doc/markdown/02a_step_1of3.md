## Step 1/3: Generate an ELAN-file with linked media files

In step 1 we will locate all video clips (GoPro + VIRB) and FIT-files (VIRB) that belong to a specific recording session. Video clips are then joined, and linked in the resulting ELAN-file.

### Command

**Command**
```sh
geoelan cam2eaf --video INDIR/VIRB_OR_GOPRO_CLIP.MP4 --indir INDIR/ --outdir OUTDIR/
```

**Output files GoPro**
```
OUTDIR/GH010026/
├── GH010026.mp4             High-resolution video (concatenated)
├── GH010026_LO.mp4          Low-resolution video for ELAN (concatenated)
├── GH010026.wav             Extracted audio for ELAN (concatenated)
├── GH010026.eaf             ELAN-file with pre-linked media files
├── GH010026.kml             Overview KML-file with all points logged during the recording session
├── GH010026.json            Overview GeoJSON-file with all points logged during the recording session
└── GH010026.txt             FFmpeg concatenation file, paths to input clips
```

**Output files VIRB**
```
OUTDIR/VIRB0001-1/
├── 2017-05-29-13-05-42.fit  FIT-file with corresponding telemetry
├── VIRB0001-1.mp4           High-resolution video (concatenated)
├── VIRB0001-1_LO.mp4        Low-resolution video for ELAN (concatenated)
├── VIRB0001-1.wav           Extracted audio for ELAN (concatenated)
├── VIRB0001-1.eaf           ELAN-file with pre-linked media files
├── VIRB0001-1.kml           Overview KML-file with all points logged during the recording session
├── VIRB0001-1.json          Overview GeoJSON-file with all points logged during the recording session
└── VIRB0001-1.txt           FFmpeg concatenation file, paths to input clips
```



### Explanation of the command

The relevant sub-command is `cam2eaf`. Run `geoelan cam2eaf --help` for an overview.

By specifying any clip in the recording session  via `--video`, the remaining clips (GoPro + VIRB), including the corresponding FIT-file (VIRB), will be automatically located and joined, if they exist in the input directory `INDIR/`, including sub-directories. The result, including an ELAN-file with linked media files, will be saved to the output directory `OUTDIR/`.

If low-resolution clips (`.GLV`/`.LRV`) are located, these will be linked in the ELAN-file. If not, the high-resolution video will be linked instead.

GeoELAN defaults to _not_ insert a tier with geo-data in the ELAN-file due to the effect this may have on performance. To do so, use the `--geotier` flag (see _Geo-data in ELAN_).

> **TIP:** For longer recording sessions or when batching, resulting in many video clips, step 1 is usually much faster if `--indir` and `--outdir` is not on the same physical hard drive. Those with an [SSD](https://en.wikipedia.org/wiki/Solid-state_drive) (standard on most modern laptops) should be fine running step 1. on a single drive however.