**GeoELAN v2.7.5 2025-02-12**

**Important:**
> **GoPro Hero 12 Black is not supported** since it does not have a GPS module.
>
> **GoPro Hero 13 Black is/will be supported** as soon as I get hold of some raw/unedited sample footage.
>
> **Garmin VIRB Ultra 30 is discontinued**. GeoELAN will continue to support these and the FIT-format.

## Introduction

GeoELAN is a command-line tool that geo-references time-aligned text-annotations of observed phenomena in audiovisual recordings, captured with a recent GoPro or Garmin VIRB action camera, see [Larsson et al 2021](https://doi.org/10.1080/13645579.2020.1763705). In other words, GeoELAN is used for annotating action camera GPS logs with the help of the free annotation tool [ELAN](https://archive.mpi.nl/tla/elan).

Requirements:
- GoPro Hero 5 Black - GoPro Hero 11 Black, Hero 13 Black (Hero 12 Black is not supported, but some data can still be inspected and plotted)
- Garmin VIRB
- [FFmpeg](http://ffmpeg.org) (in `PATH` preferred, but custom path can also be set when running GeoELAN)

> GeoELAN is multi-functional command-line tool that can
> - **geo-reference** ELAN-annotations of GoPro and VIRB footage (i.e. annotate GPS logs) and **generate annotated points, lines, or circles**.
> - **inspect** the raw content of your GoPro GPMF data, or Garmin FIT-files.
> - **locate and match** all relevant files belonging to the same recording session irrespective of file name (clips, telemetry-files).
> - automatically **join clips** (requires FFmpeg) for a specific recording session, and **generate an ELAN-file** with linked media.

Any ELAN annotation - be it an on-site utterance, or a plant in view - can be geo-referenced as long as the GPS logged coordinates within the annotation's timespan. The nature of the workflow also means consultants not physically present at the the time of recording can evaluate and annotate sections to be geo-referenced post-collection. As the name implies, the annotation tool [ELAN](https://archive.mpi.nl/tla/elan) plays a central role and is required to annotate events. The output can be points, polylines, or polygons (circles), as [KML](https://www.ogc.org/standards/kml/) and [GeoJSON-files](https://geojson.org). "GoPro" refers to a GoPro Hero 5 Black - Hero11 Black, and "VIRB" to the Garmin VIRB Ultra 30.

> While GeoELAN functionality differs slightly between Garmin and GoPro due to differences in formats and file structure,
> the general workflow and the final output are the same.

## Acknowledgments

GeoELAN was developed with support from the [Bank of Sweden Tercentenary Foundation](https://www.rj.se/en/) (Grant nos [NHS14-1665:1](https://www.rj.se/en/grants/2015/language-as-key-to-perceptual-diversity-an-interdisciplinary-approach-to-the-senses/) and [IN17-0183:1](https://www.rj.se/en/grants/2017/digital-multimedia-archive-of-austroasiatic-intangible-heritage-phase-ii-seeding-multidisciplinary-workspaces/)).

We would also like to acknowledge the [The Language Archive](https://archive.mpi.nl/tla/), Max Planck Institute for Psycholinguistics in Nijmegen for their tireless efforts in developing [ELAN](https://archive.mpi.nl/tla/elan), and making it available for free.

## References

ELAN (Version 6.8) [Computer software]. 2024. Nijmegen: Max Planck Institute for Psycholinguistics, The Language Archive. Retrieved from https://archive.mpi.nl/tla/elan

Larsson, J. 2024. GeoELAN Manual. Lund: Lund University Humanities Lab. <https://github.com/jenslar/geoelan>

Larsson et al, 2021. Integrating behavioral and geospatial data on the timeline: towards new dimensions of analysis, *International Journal of Social Research Methodology*, 24:1, 1-13, DOI: [10.1080/13645579.2020.1763705](https://doi.org/10.1080/13645579.2020.1763705)