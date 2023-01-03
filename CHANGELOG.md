# GeoELAN v2.1
- Support for printing MP4 atom structure, similar to [AtomicParsley](https://atomicparsley.sourceforge.net)
- Support for extracting GPMF from GoPro JPEG-files
- Bug and typo fixes

# GeoELAN v2.0
- GoPro Hero 5 Black and later is now supported.
- With the added of GoPro support some commands have been changed and/or tweaked, refer to the manual if GeoELAN no longer works as expected.
- New `eaf2geo --geoshape` option `circle`. This mostly a flair/visual option for `point-single`, since the radius and height parameters currently have no connections to logged data or annotation content.
- All `geoshape` options can have a "height" parameter, which affects KML output. The feature will then "extrude" up, relative to ground, according to set height value. This causes `--gepshape circle` to become a cylinder.
- Most backend code has been re-written, including the FIT-crate.
- New GoPro GPMF-crate written from scratch.
- New ELAN/EAF-crate for reading/writing EAF-files.
- New MP4-crate for iterating over atoms in MP4-files.
- Note: None of the crates are yet on crates.io, but will be shortly.
