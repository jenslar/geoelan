**GeoELAN v2.7 2024-10-16**

> **Important:**
> - **GoPro Hero 12 Black does not have a GPS module and is not supported**. GoPro Hero 13 Black once again has a GPS and should be compatible.
> - **Garmin has discontinued the VIRB Ultra 30**. Use a GoPro with a GPS module instead.
> We still have and use VIRBs, so GeoELAN will continue to support these and the FIT-format.

Annotate action camera GPS logs with the help of the free annotation tool [ELAN](https://archive.mpi.nl/tla/elan).

GeoELAN is multi-functional command-line tool that can
- **geo-reference** ELAN-annotations of GoPro and VIRB footage (i.e. annotate GPS logs) and **generate annotated points, lines, or circles**.
- **inspect** the raw content of your GoPro GPMF data, or Garmin FIT-files.
- **locate and match** all relevant files belonging to the same recording session (clips, telemetry-files).
- automatically **join clips** for a specific recording session, and **generate an ELAN-file** with linked media (requires FFmpeg).
- be useful even if all you want to do is to inspect GoPro and Garmin telemetry and/or join action camera clips automatically

The `geoelan` executable contains the full PDF manual for convenience: `geoelan manual --pdf`.

List available sub-commands with `geoelan --help`. List parameters for each sub-command with `geoelan SUBCOMMAND --help`, e.g. `geoelan locate --help`.

# Introduction

GeoELAN is a command-line tool that geo-references time-aligned text-annotations of observed phenomena in audiovisual recordings, captured with a recent GoPro or Garmin VIRB action camera, see [Larsson et al 2021](https://doi.org/10.1080/13645579.2020.1763705). In other words, GeoELAN is used for annotating action camera GPS logs with the help of the free annotation tool [ELAN](https://archive.mpi.nl/tla/elan).

Requirements:
- GoPro Hero 5 Black - GoPro Hero 11 Black, GoPro Hero 13 Black (Hero12 Black does not have a GPS module)
- Garmin VIRB (VIRB Ultra 30 tested)
- [ELAN](https://archive.mpi.nl/tla/elan) ([documentation](https://archive.mpi.nl/tla/elan/documentation))
- [FFmpeg](http://ffmpeg.org) (in `PATH` preferred, but custom path can also be set when running GeoELAN)

---

# Install

See [releases](https://github.com/jenslar/geoelan/releases) to the right for pre-compiled binaries for Windows (x86), macOS (Apple Silicon, x86), and Linux (x86).

Note that some operating systems (most notably macOS) will not run unsigned binaries without user intervention, see https://support.apple.com/guide/mac-help/mh40616/mac for more information.

## Compile and install from source

You can also compile GeoELAN yourself. The full build requires [Pandoc](https://pandoc.org) and [Asciidoctor](https://asciidoctor.org) to be present in path to compile the GeoELAN documentation. Remove `build.rs` or change its name to skip this step.

The basic steps are:

1. Install the [Rust programming langugage toolchain](https://www.rust-lang.org)
2. Get the source: `git clone https://github.com/jenslar/geoelan`
3. `cd geoelan` (you should be in the folder containing `Cargo.toml`)
4. `cargo build --release`
5. `cargo install --path .` (optional, makes `geoelan` [a global command](https://doc.rust-lang.org/cargo/commands/cargo-install.html))

---

## Filenames

GeoELAN will find the correct high/low-resolution clips regardless of file name as long as the extension is `.MP4`, `.LRV`, `.GLV` (upper or lower case). I.e. a low-resolution GoPro clip named `GL010019.LRV` by the camera, can be renamed to e.g. `foraging_trip1.mp4` and GeoELAN will still tag it as low-resolution and find the remaining clips in that recording session.

## Examples

Locate all low-resolution GoPro clips (`.LRV`) in `~/Desktop/` for session the containing the high-resolution clip `GX020006.MP4`, join them and generate an ELAN-file with a tier containing coordinates:

```sh
geoelan cam2eaf --video ~/Desktop/gopro/GX010006.MP4 --indir ~/Desktop/ --geotier --low-res-only --outdir ~/Desktop/
```

(if `--indir` is not specified, GeoELAN will search the parent folder for the specified clip)

Prompts the user to select a tier to geo-reference, then generates a KML and GeoJSON files with a continuous poly-line,
alternating between annotated and un-annotated sections. Relevant GoPro files are automatically located.

```sh
geoelan eaf2geo --eaf ~/Desktop/gopro/MYELANFILE.eaf --gpmf ~/Desktop/gopro/GX010006.MP4 --indir ~/Desktop/ --geoshape line-all
```

Specified `--gpmf` file must be an original GoPro clip. Joined recording sessions (done via `geoelan cam2eaf ...` or using FFmpeg directly) will not
contain any telemetry, such as GPS log. If there is a solution to preserve telemetry when joining clips, such as mapping tracks with FFmpeg, please let me know.

Locate all GoPro clips and group them according to recording session:

```sh
geoelan locate --indir ~/Desktop/ --kind gopro
```

Locate remaining clips in session containing `GX020006.MP4` (does not have to be the first clip in the session):

```sh
geoelan locate --indir ~/Desktop/ --video ~/Desktop/gopro/GX020006.MP4
```

Generate a KML-file from the merged GPS-log for session containing `GX020006.MP4`:

```sh
geoelan inspect --gpmf ~/Desktop/gopro/GX010006.MP4 --session --indir ~/Desktop/ --kml
```

`--gpmf VIDEOFILE` tells GeoELAN to extract and refers to GoPro's "[GoPro Metadata Format](https://github.com/gopro/gpmf-parser)" which is how all telemetry is logged on GoPro cameras. It is embedded within the video files.

Print MP4 atom layout (similar to AtomicParsely):

```sh
geoelan inspect --video ~/Desktop/gopro/GX010006.MP4 --atoms
```

Print MP4 user data (`udta` atom), including GPMF data if encountered (this is different to the timed GPMF telemetry and contains device specific information):

```sh
geoelan inspect --video ~/Desktop/gopro/GX010006.MP4 --meta
```

`--video VIDEOFILE` tells GeoELAN to inspect the file as an MP4 video, ignoring e.g. timed GPMF telemetry.

Plot the accelerometer data in a GoPro MP4 file (opens in default browser):

```sh
geoelan plot --gpmf ~/Desktop/gopro/GX010006.MP4 --y-axis accelerometer
```

---

Annotating placename utterances in ELAN, to be geo-referenced by GeoELAN
![Annotating placename utterances in ELAN](doc/img/elan_placename.jpg "Annotating placename utterances recorded on-site")

Using GeoELAN to geo-reference ELAN annotations
![Using GeoELAN to geo-reference ELAN annotations](doc/img/map_placename.jpg "Using GeoELAN to geo-reference ELAN annotations")

---

# Example walkthrough

This section describes how GeoELAN can be used to geo-reference ELAN-annotations. Please refer to the detailed sections if you get stuck. Remember that all input video clips must be the unprocessed, original MP4 (GoPro + VIRB) and FIT-files (VIRB). The so-called FIT-files mentioned throughout this manual are where the VIRB logs GPS-data and other kinds of telemetry during a recording session. These need to be matched to the corresponding video recording. GeoELAN will help with all of this, with the exception of annotating your data.

Note that some commands differ slightly between GoPro and VIRB.

The basic steps are:

1. Record video with a recent GoPro or VIRB.
2. Use GeoELAN to concatenate the video clips and generate an ELAN-file.
3. Annotate spatially interesting sections in ELAN.
4. Use GeoELAN to geo-reference the annotations, resulting in annotated KML and GeoJSON files.

Input files:
- **GoPro**:
    - `GH010026.MP4`, any clip in a recording session (remaining clips located automatically)
- **VIRB**:
    - `VIRB0001-1.MP4`, any clip in a recording session (remaining clips located automatically)
    - FIT-file with corresponding GPS-data (located automatically)

Output files:
- **GoPro + VIRB**:
    - KML and GeoJSON files with ELAN annotation content synchronised and mapped to the corresponding points as descriptions.

## Step 1/3: Generate an ELAN-file with linked media files

In step 1 we will locate all video clips (GoPro + VIRB) and FIT-files (VIRB) that belong to a specific recording session, process these, join clips, and generate an ELAN-file with linked media files.

**Command**
```sh
geoelan cam2eaf --video INDIR/VIRB_OR_GOPRO_CLIP_.MP4 --indir INDIR/ --outdir OUTDIR/
```

**Output files GoPro**
```
OUTDIR/GH010026/
├── GH010026.mp4             High-resolution video (concatenated)
├── GH010026_LO.mp4          Low-resolution video for ELAN (concatenated)
├── GH010026.wav             Extracted audio for ELAN (concatenated)
├── GH010026.eaf             ELAN-file with pre-linked media files
├── GH010026.kml             Overview KML-file with all points logged during the recording session
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
└── VIRB0001-1.txt           FFmpeg concatenation file, paths to input clips
```

By specifying any clip in the recording session  via `--video`, remaining files will be automatically located, if they exist in the input directory `INDIR/`. The result, including an ELAN-file with linked media files, will be saved to the output directory `OUTDIR/`. The default behaviour is to link low-resolution clips (`.GLV`/`.LRV`) in the ELAN-file.

## Step 2/3: Annotate events in ELAN

Next, use ELAN with the ELAN-file from step 1 to annotate events that should be geo-referenced in step 3. Feel free to create any tier structure you may need. Tokenized tiers can not be geo-referenced, but otherwise any tier is fine, including deeply nested, referred tiers.

GeoELAN will geo-reference annotations from a single tier (selectable in step 3). Thus, if you want to generate a KML-file with e.g. indigenous place names mentioned on-site during the recording, those place names must be limited to a single tier. If there are other spatial categories or groupings you wish to explore, simply create a new tier for each. In step 3 you can then re-run GeoELAN as many times as required, then select a different tier and/or options on each run.

When the annotations are geo-referenced in step 3, the annotation values in the selected tier will be used as descriptions for the synchronized, corresponding points in the KML and GeoJSON-files. Points corresponding to unannotated sections of the ELAN-file will either be discarded or have no description, depending on which options you use in step 3.

An annotated event can relate to anything observed in the recording and can be represented as either points or polylines in the output KML-file. If you are unsure which best applies to what you have in mind for your data, or how this may affect how you annotate, here are a few ideas for each kind.

> **Points** could concern documenting:
> - **the location of a plant or a geographical feature**, e.g. annotate the timespan either is visible in the video.
> - **an uttered place name or an animal cry**, e.g. annotate the timespan of the on-site utterance or cry.
>
> **Lines** could concern documenting:
> - various **types of movement through the landscape**. To annotate the movement of "walking up-hill" as it is observed visually in the recording, set the annotation's start time at the bottom of the hill and its end at the top, or for as long as the motion can be observed.
> - a **narrative reflecting on the immediate surroundings** as they change over time. E.g. comments on visible landscape features, or perhaps the re-construction of an historical event as it unfolded over space and time.


## Step 3/3: Generate a KML-file from geo-referenced ELAN annotations

Now that we have a few annotations, GeoELAN will geo-referenence these by determining which points were logged within each annotation's timespan. Note the different commands between GoPro and VIRB.

This is where you choose the approriate geographical representations for your annotated phenomena. Here are suggestions for the examples in step 2.

> **Points**:
> - the location of a plant or a geographical feature
> - an uttered place name or an animal cry
>
> To get a single, average coordinate for each annotation, use the `--geoshape point-single` option.
>
> **Lines**:
> - types of movement through the landscape
> - narrative reflecting on the immediate surroundings

There are other options, such as _circle_ output. It is the same as point output with the difference that radius and height can be specified (all circles will have the same size). For a more detailed overview of the possibilities, see the `--geoshape` option for the command _eaf2geo_. Experiment! If you realise one representation is not appropriate after all, re-run GeoELAN with a different option.

### VIRB

**Command**
```sh
geoelan eaf2geo --eaf VIRB0001-1.eaf --fit 2003-01-02-12-00-00.fit --geoshape point-single
```

**Output files**
```
OUTDIR/VIRB0001-1/
├── ...                              Existing files
├── VIRB0001-1_point-single.kml      New KML-file, one point per annotation in the selected tier
└── VIRB0001-1_point-single.json  New GeoJSON-file, one point per annotation in the selected tier
```

### GoPro

**Command**
```sh
geoelan eaf2geo --eaf GH010026.eaf --gpmf INDIR/GH010026.MP4  --geoshape point-single
```

> **Important:** `GH010026.MP4` **must be an unedited GoPro clip from the recording session**, as it was generated by the camera, **not** the video linked in your ELAN file. E.g. the same one specified in step 1.

**Output files**
```
OUTDIR/GH010026/
├── ...                            Existing files
├── GH010026_point-single.kml      New KML-file, one point per annotation in the selected tier
└── GH010026_point-single.json  New GeoJSON-file, one point per annotation in the selected tier
```

### Explanation of the command

GeoELAN geo-references all annotations in a single ELAN-tier (you will be prompted to select from a list) for the specified ELAN-file, then generates annotated KML and GeoJSON files where each point represents a single annotation.

By specifying an ELAN-file (`--eaf`) and an original, unedited GoPro MP4-clip (`--gpmf`) or VIRB FIT-file (`--fit`), GeoELAN will synchronise the annotations with the coordinates contained within the MP4/FIT-file. Similar to step 1, all files will be automatically located.

`--geoshape point-single` lets GeoELAN know that each, respective annotation should be distilled into a single point, meaning that the generated KML-file will contain as many points as there are annotations in the selected tier. Each point inherits the corresponding annotation value as its description. The KML-file is named according to the selected `--geoshape` option, in this case `GH010026_point-single.kml`/`VIRB0001-1_point-single.kml`.

For the example command for VIRB, the user will be presented with a list of recording sessions present in the FIT-file (see _The FIT-format and the Garmin VIRB_). For GoPro, specifying an original clip, e.g. the same one specified in step 1, is enough.

## References

Larsson, Jens, Niclas Burenhult, Nicole Kruspe, Ross. S Purves, Mikael Rothstein and Peter Sercombe. 2020. Integrating behavioral and geospatial data on the timeline: towards new dimensions of analysis. _International Journal of Social Research Methodology_. doi: [10.1080/13645579.2020.1763705](https://doi.org/10.1080/13645579.2020.1763705)

ELAN (Version 6.7) [Computer software]. 2024. Nijmegen: Max Planck Institute for Psycholinguistics. Retrieved from https://archive.mpi.nl/tla/elan

---
---

# Acknowledgements

GeoELAN was developed with support from the [Bank of Sweden Tercentenary Foundation](https://www.rj.se/en/) (Grant nos [NHS14-1665:1](https://www.rj.se/en/grants/2015/language-as-key-to-perceptual-diversity-an-interdisciplinary-approach-to-the-senses/) and [IN17-0183:1](https://www.rj.se/en/grants/2017/digital-multimedia-archive-of-austroasiatic-intangible-heritage-phase-ii-seeding-multidisciplinary-workspaces/)).

We would also like to acknowledge the [The Language Archive](https://archive.mpi.nl/tla/), Max Planck Institute for Psycholinguistics in Nijmegen for their tireless efforts in developing [ELAN](https://archive.mpi.nl/tla/elan), and making it available for free.