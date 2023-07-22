# GeoELAN 2.6
- `plot`: Plot data for the entire session (use `--session`) for both GoPro and VIRB (sensor data, altitude etc), axis titles added, internal fixes.
- `cam2eaf`: `--batch` flag added, allowing batch processing of of all recording sessions in `--indir`.
- General: Fixed stalling on older MP4 files and not finding GPMF offsets for Hero 5, and many other internal fixes.

# GeoELAN 2.5
- Merged `gopro2eaf` and `virb2eaf` into the single command `cam2eaf`.
- New command `plot`: rudimentary plotting of sensor data. Leverages `plotly.js` via [`plotly`](https://github.com/igiagkiozis/plotly).
- Many under-the-hood changes, such as better sorting of GoPro clips, independent of GPS and filename.

# GeoELAN 2.2
- Changed repository from <https://gitlab.com/rwaai/geoelan> to <https://github.com/jenslar/geoelan>
- Experimental: Possible to use coordinates imported into ELAN via the `--geotier` as export source for `eaf2geo`.
- `gopro2eaf`: Low-resolution video supported (`.LRV`), if found LRV-files are default when linking in ELAN (similar to VIRB GLV). Link high-res video by using `--link-high-res`
- The new `GPS9` data will be used for devices that log this (currently only Hero11)
- Locating and matching GoPro clips no longer depends on filenames or path:
	- MUID (Media Unique ID) or GUMI (Global Unique ID) is used for matching clips in session
	- A hash of the partial, "raw" GPMF stream is used to match correspondging high and low-resolution clips
	- Sorting clips in chronological order currently depends on GPS. (fallback to filename sorting not yet implemented)
	- Note: Due to limited access to various GoPro models, matching and grouping clips may not yet work as expected for Hero 8 - 10.
- Changed or removed command line arguments for some commands
- All internally developed Rust crates are updated and now separately located at <https://github.com/jenslar> (also specified as source in `Cargo.toml`)

# GeoELAN 2.1.1
- Changed average time to float based calculation for points in `geo::mod::point_cluster_average()`
- Changed format for EAF default date to comply with `xs:dateTime`.
- Fixed command line argument bug in command `eaf2geo`.
- Fixed errors and typos in documentation.
- `inspect`: `-v` can now be used for GoPro MP4 files as well with `--atoms` or `--meta` (use `--gpmf` for inspecting GPMF data)

# GeoELAN 2.1
- Support for printing MP4 atom structure, similar to [AtomicParsley](https://atomicparsley.sourceforge.net). `geoelan inspect --video VIDEO.mp4 --atoms`
- Very rudimentary support for inspecting GoPro JPEG-files (contains GPMF streams)
- Bug and typo fixes

# GeoELAN 2.0
- GoPro Hero 5 Black and later is now supported.
- With the added of GoPro support some commands have been changed and/or tweaked, refer to the manual if GeoELAN no longer works as expected.
- New `eaf2geo --geoshape` option `circle`. This mostly a flair/visual option for `point-single`, since the radius and height parameters currently have no connections to logged data or annotation content.
- All `geoshape` options can have a "height" parameter, which affects KML output. The feature will then "extrude" up, relative to ground, according to set height value. This causes `--gepshape circle` to become a cylinder.
- Most backend code has been re-written, including the FIT-crate.
- New GoPro GPMF-crate written from scratch.
- New ELAN/EAF-crate for reading/writing EAF-files.
- New MP4-crate for iterating over atoms in MP4-files.
- Note: None of the crates are yet on crates.io, but will be shortly.