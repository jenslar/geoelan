## Example walkthrough

This section describes how GeoELAN can be used to geo-reference ELAN-annotations. Please refer to the detailed sections if you get stuck. Remember that all input video clips must be the unprocessed, original MP4 (GoPro + VIRB) and FIT-files (VIRB). The so-called FIT-files mentioned throughout this manual are where the VIRB logs GPS-data and other kinds of telemetry during a recording session. These need to be matched to the corresponding video recording. GeoELAN will help with all of this, with the exception of annotating your data.

Note that some commands differ slightly between GoPro and VIRB.

The basic steps are:

1. Record video with a recent GoPro or VIRB.
2. Use GeoELAN to concatenate the video clips and generate an ELAN-file.
3. Annotate spatially interesting sections in ELAN.
4. Use GeoELAN to geo-reference the annotations, resulting in annotated KML and GeoJSON files.

Input files (example file names, naming convention may differ sligtly depending on model):
- **GoPro**:
    - `GH010026.MP4`, any clip in a recording session (remaining clips located automatically)
- **VIRB**:
    - `VIRB0001-1.MP4`, any clip in a recording session (remaining clips located automatically)
    - FIT-file with corresponding GPS-data (located automatically)

Output files:
- **GoPro + VIRB**:
    - KML and GeoJSON files with ELAN annotation content synchronised and mapped to the corresponding points as descriptions.
