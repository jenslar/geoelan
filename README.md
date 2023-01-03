## GeoELAN README

[GeoELAN](https://gitlab.com/rwaai/geoelan) is a tool for annotating action-camera GPS logs via the [ELAN](https://archive.mpi.nl/tla/elan) annotation software, see [Larsson et al 2021](https://doi.org/10.1080/13645579.2020.1763705).

Supported action cameras are **GoPro** (Hero 5 Black and later) and **Garmin VIRB** (VIRB Ultra 30).

By annotating a section representing an on-site utterance, a plant that is in view, or anything else that was captured, it can be automatically linked to the corresponding coordinates. The nature of the workflow also means consultants not physically present at the the time of recording may evaluate observed phenomena to be geo-referenced post-collection. As the name implies, the free [ELAN](https://archive.mpi.nl/tla/elan) annotation software plays a central role and is required to annotate events. The final output can be points, polylines, or polygons (circles), in the form of annotated [KML](https://www.ogc.org/standards/kml/) and [GeoJSON-files](https://geojson.org). Henceforth, "GoPro" refers to a GoPro Hero 5 Black or later, and "VIRB" to the Garmin VIRB Ultra 30. Note that while GeoELAN functionality differs slightly between Garmin and GoPro due to differences in formats and file structure, its main purpose is intact for either brand.

> GeoELAN is multi-functional tool that can
> - geo-reference ELAN-annotations of GoPro and VIRB footage (i.e. annotate GPS logs) and output these as annotated points, lines. or circles.
> - inspect the content of your GoPro GPMF data, or Garmin FIT-files.
> - locate and match all relevant files belonging to the same recording session (clips, telemetry-files).
> - automatically concatenate clips for a specific recording, and generate an ELAN-file with linked media.

### Manual
- See the `doc` directory for the full manual and a brief A4-guide.

### Installation
- See the `bin` directory for pre-compiled executables for macOS (Intel x86 + Apple Silicon), and Windows (Intel x86). Version 2.1 binaries for Linux are not yet added, but you can always compile on your own in the meantime.

### Compile and install from source

If you wish, you can compile GeoELAN yourself. Depending on your operating system, this may require installing additional software, and a basic understanding of working in a terminal. The basic steps are:

1. Install [the Rust toolchain](https://www.rust-lang.org)
2. Get the GeoELAN source from <https://gitlab.com/rwaai/geoelan> (via `git` or the zip-file)
3. `cd geoelan` (you should be in the folder containing `Cargo.toml`)
4. `cargo build --release`
5. `cargo install --path .` (optional, makes `geoelan` [a global command](https://doc.rust-lang.org/cargo/commands/cargo-install.html))

## Requirements

- An action camera with a built-in GPS. Supported devices are:
    -  [GoPro](https://gopro.com) Hero Black 5 or newer (Max, and Fusion cameras have not been tested)
    -  [Garmin VIRB Ultra 30](https://buy.garmin.com/en-US/US/p/522869/pn/010-01529-03) ([documentation](https://support.garmin.com/en-US/?partNumber=010-01529-03&tab=manuals))
- [ELAN](https://archive.mpi.nl/tla/elan) ([documentation](https://archive.mpi.nl/tla/elan/documentation))
- [FFmpeg](https://www.ffmpeg.org) (for concatenating video)
- [Rust toolchain](https://www.rust-lang.org) (optional, only required for compiling GeoELAN from source)

### Quick help
- Usage: `geoelan COMMAND OPTIONS`. E.g. to geo-reference an ELAN-file:
  - GoPro: `geoelan eaf2geo --eaf MyElanFile.eaf --video GH01000.MP4`
  - VIRB: `geoelan eaf2geo --eaf MyElanFile.eaf --fit MyFitFile.fit`
- Running `geoelan` with no options will display an overview.
- Running `geoelan COMMAND --help` displays an overview for that command, e.g.:
  - `geoelan eaf2geo --help`.
- Available commands: `gopro2eaf`, `virb2eaf`, `eaf2geo`, `locate`, `inspect`, `manual`
- The `geoelan` executable contains the full PDF manual for convenience: `geoelan manual --pdf`

## Example walkthrough

This section describes an example of how GeoELAN can be used to geo-reference ELAN-annotations. Please refer to the detailed sections if you get stuck. Note that all input video clips must be the unprocessed, original MP4 (GoPro + VIRB) and FIT-files (VIRB). The so-called FIT-files mentioned throughout this manual are where the VIRB logs GPS-data and other kinds of telemetry during a recording session. These need to be matched to the corresponding video recording (see _The FIT-format and the Garmin VIRB_ for further information). GeoELAN will help with all of this, with the exception of annotating your data.

As noted below, some of the commands differ slightly between GoPro and VIRB due to differences in file and data structures.

![Annotating in ELAN](doc/img/elan.jpg "Annotating in ELAN")

The basic steps are:

1. Record video with a recent GoPro or Garmin VIRB action camera.
2. Use GeoELAN to concatenate the video clips and generate an ELAN-file.
3. Annotate spatially interesting sections in ELAN using the pre-generated ELAN-file.
4. Use GeoELAN to geo-reference the annotations, resulting in annotated KML and GeoJSON files.

### Input files

**VIRB**
- `VIRB0001-1.MP4`, any clip in a recording session (remaining clips located automatically)
- FIT-file with corresponding GPS-data (located automatically)

**GoPro**
- `GH010026.MP4`, any clip in a recording session (remaining clips located automatically)

### Output files

**VIRB + GoPro**
- KML and GeoJSON files with ELAN annotation content synchronised and mapped to the corresponding points as descriptions. See the command _eaf2geo_ for other options.

## Step 1/3: Generate an ELAN-file with linked media files

In step 1 we will locate all video clips (GoPro + VIRB) and FIT-files (VIRB) that belong to a specific recording session, process these, and generate an ELAN-file with linked media files.

### VIRB

**Command**
```sh
geoelan virb2eaf --video INDIR/VIRB0001-1.MP4 --indir INDIR/ --outdir OUTDIR/
```

**Output files**
```
OUTDIR/VIRB0001-1/
├── VIRB0001-1.mp4     High-resolution video (concatenated)
├── VIRB0001-1_LO.mp4  Low-resolution video for ELAN (concatenated)
├── VIRB0001-1.wav     Extracted audio for ELAN (concatenated)
├── VIRB0001-1.eaf     ELAN-file with pre-linked media files
├── VIRB0001-1.kml     Overview KML-file with all points logged during the recording session
└── VIRB0001-1.txt     FFmpeg concatenation file, paths to input clips
```

### GoPro
**Command**
```sh
geoelan gopro2eaf --gpmf INDIR/GH010026.MP4 --indir INDIR/ --outdir OUTDIR/
```

**Output files**
```
OUTDIR/GH010026/
├── GH010026.mp4     High-resolution video (concatenated)
├── GH010026_LO.mp4  Low-resolution video for ELAN (concatenated)
├── GH010026.wav     Extracted audio for ELAN (concatenated)
├── GH010026.eaf     ELAN-file with pre-linked media files
├── GH010026.kml     Overview KML-file with all points logged during the recording session
└── GH010026.txt     FFmpeg concatenation file, paths to input clips
```


### Explanation of the command
GeoELAN locates and concatenates all clips belonging to the recording session starting with `VIRB0001-1.MP4`/`GH010026.MP4`, then generates an ELAN-file with the resulting audio and video files pre-linked.

The relevant sub-commands are `virb2eaf`/`gopro2eaf`, depending on camera.
By specifying any clip in the recording session  via `--video` (VIRB), or `--gpmf` (GoPro), the remaining clips, including the corresponding FIT-file (VIRB), will be automatically located as long as these exist somewhere in the specified input directory (`--indir`). Sub-directories will be searched as well. The result, including the corresponding FIT-file for VIRB cameras, will be saved to the specified outout directory (`--outdir`).

If low-resolution clips (`.GLV`/`.LRV`) are located, a concatenated low-resolution video will be linked in the ELAN-file. If not, the concatenated high-resolution video will be linked instead.

GeoELAN defaults to _not_ insert a tier with geo-data in the ELAN-file due to the effect this may have on performance. To do so, see _Geo-data in ELAN_. The result, including the FIT-file (VIRB), is copied to a folder named after the first clip in the session under the specified output directory (`--outdir`).

## Step 2/3: Annotate events in ELAN

In step 2 the user annotates events in ELAN (as per normal) they wish to geo-reference using the generated ELAN-file.

GeoELAN will geo-reference annotations from a single tier, selectable in step 3. Thus, if you want to generate a KML-file with e.g. indigenous place names mentioned on-site during the recording, all place names must be limited to a single tier. When the annotations are geo-referenced in step 3, their textual content will be used as descriptions for the corresponding points in the KML and GeoJSON-files. Points corresponding to unannotated sections of the ELAN-file will either be discarded or have no description, depending on which options you use in step 3.

An annotated event can relate to anything observed in the recording and can be represented as either points or polylines in the output KML-file. If you are unsure which best applies to what you have in mind for your data, or how this may affect how you annotate, here are a few ideas for each kind.

> **Points** could concern documenting:
> - **the location of a plant or a geographical feature**, e.g. annotate the the timespan either is visible in the video.
> - **an uttered place name or an animal cry**, e.g. annotate the timespan it is uttered or heard on-site.
> 
> For these specific cases, the exact time spans of the annotations are not that important. It should be enough to ensure the annotation lasts for the duration of the place name being uttered, or for as long as the plant is visible. If unsure, add a another second to the annotation timespan. An average coordinate will be calculated for those that were logged within each annotation's time span, so as long as the camera wearer does not stray too far from the observation point, the result should be accurate enough.
> 
> **Lines** could concern documenting:
> - various **types of movement through the landscape**. To annotate the movement of "walking up-hill" as it is observed visually in the recording set the annotation's start time at the bottom of the hill and its end at the top, or for as long as the motion can be observed.
> - a **narrative reflecting on the immediate surroundings** as they change over time. E.g. comments on visible landscape features, or perhaps the re-construction of an historical event as it unfolded over space and time.

If you wish to geo-reference several categories of phenomena in a single ELAN-file for a specific recording session, create a separate tier for each category. In step 3 you can then re-run GeoELAN as many times as required, then select a different tier and/or options on each run.

## Step 3/3: Generate a KML-file from geo-referenced ELAN annotations

Now that we have a few annotations, we can geo-referenence these by determining which points were logged within each annotation's timespan. Note the different commands between GoPro and VIRB.

This is where you choose the approriate geographical representations for your annotated phenomena. Here are suggestions for the examples in step 2.

> **Points**:
> - **the location of a plant or a geographical feature**
> - **an uttered place name or an animal cry**
> 
> To get a single, average coordinate for each annotation in the KML and GeoJSON-files, use the `--geoshape point-single` option.
> 
> **Lines**:
> - **types of movement through the landscape**
> - **narrative reflecting on the immediate surroundings**
> 
> Two line options may apply to the above. To get a continuous polyline alternating between marked (annotated) and unmarked (un-annotated) events, use the option `--geoshape line-all`. To get a broken-up polyline representing marked events only, use the option `--geoshape line-multi`.
> 

There are other options, such as _circle_ output. It is similar to point output, for which radius and height can be specified (all circles will have the same size). For a more detailed overview of the possibilities, see the `--geoshape` option for the command _eaf2geo_. Experiment! If you realise one representation is not appropriate after all, re-run GeoELAN with a different option.

### VIRB

**Command**
```sh
geoelan eaf2geo --eaf VIRB0001-1.eaf --fit 2003-01-02-12-00-00.fit --geoshape point-single
```

**Output files**
```
OUTDIR/VIRB0001-1/
├── ...                              Existing files
└── VIRB0001-1_point-single.kml      New KML-file, one point per ELAN-annotation in the selected tier
└── VIRB0001-1_point-single.geojson  New GeoJSON-file, one point per ELAN-annotation in the selected tier
```

### GoPro

**Command**
```sh
geoelan eaf2geo --eaf GH010026.eaf --gpmf INDIR/GH010026.MP4  --geoshape point-single
```

> **Important:** `GH010026.MP4` must be the original video generated by the camera, indicated by `INDIR` from step 1/3.

**Output files**
```
OUTDIR/GH010026/
├── ...                            Existing files
└── GH010026_point-single.kml      New KML-file, one point per ELAN-annotation in the selected tier
└── GH010026_point-single.geojson  New GeoJSON-file, one point per ELAN-annotation in the selected tier
```

### Explanation of the command
GeoELAN geo-references all annotations in a single ELAN-tier (selectable from a list) for the specified ELAN-file and generates an annotated KML-file where each point represents a single annotation.

The relevant command is `eaf2geo`. By specifying an ELAN-file (`--eaf`) and the corresponding GoPro MP4-file (`--gpmf`) or VIRB FIT-file (`--fit`), GeoELAN will synchronise the annotations with the coordinates contained within the MP4/FIT-file. This process is usually completely automatic.

`--geoshape point-single` lets GeoELAN know that each, respective annotation should be distilled into a single point, meaning that the generated KML-file will contain as many points as there are annotations in the selected tier. Each point inherits the corresponding annotation text for the selected tier as its description. The KML-file is named according to the selected `--geoshape` option, in this case `VIRB0001-1_point-single.kml`/`GH010026_point-single.kml`.

If the process fails for VIRB footage, the user will be presented with a list of recording sessions present in the FIT-file (see _The FIT-format and the Garmin VIRB_). GoPro MP4-files lack the appropriate metadata to display such as list.

Annotating placename utterances recorded on-site
![Annotating placename utterances in ELAN](doc/img/elan_placename.jpg "Annotating placename utterances recorded on-site")

Using GeoELAN to geo-reference ELAN annotations
![Using GeoELAN to geo-reference ELAN annotations](doc/img/map_placename.jpg "Using GeoELAN to geo-reference ELAN annotations")

If the proces fails for VIRB footage, the user will be presented with a list of recording sessions present in the FIT-file (see _The FIT-format and the Garmin VIRB_). GoPro MP4-files lack the appropriate metadata to display such as list.

## References

Larsson, Jens, Niclas Burenhult, Nicole Kruspe, Ross. S Purves, Mikael Rothstein and Peter Sercombe. 2020. Integrating behavioral and geospatial data on the timeline: towards new dimensions of analysis. _International Journal of Social Research Methodology_. doi: [10.1080/13645579.2020.1763705](https://doi.org/10.1080/13645579.2020.1763705)

ELAN (Version 6.4) [Computer software]. 2022. Nijmegen: Max Planck Institute for Psycholinguistics. Retrieved from https://archive.mpi.nl/tla/elan

## Acknowledgemnts

GeoELAN was developed with support from the [Bank of Sweden Tercentenary Foundation](https://www.rj.se/en/) (Grant nos [NHS14-1665:1](https://www.rj.se/en/grants/2015/language-as-key-to-perceptual-diversity-an-interdisciplinary-approach-to-the-senses/) and [IN17-0183:1](https://www.rj.se/en/grants/2017/digital-multimedia-archive-of-austroasiatic-intangible-heritage-phase-ii-seeding-multidisciplinary-workspaces/)).

We would also like to acknowledge the [The Language Archive](https://archive.mpi.nl/tla/), Max Planck Institute for Psycholinguistics in Nijmegen for their tireless efforts in developing [ELAN](https://archive.mpi.nl/tla/elan), and making it available for free.
