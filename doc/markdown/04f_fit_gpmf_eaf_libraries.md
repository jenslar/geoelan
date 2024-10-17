## GeoELAN Rust crates

GeoELAN is written in [Rust](https://www.rust-lang.org) and uses four custom libraries (aka crates) that were developed in parallel with the tool itself.

Since these Rust crates are still in development they are not yet available on [crates.io](https://crates.io), but can be specified as a git resource in `Cargo.toml` (see the respective repository URLs).

Crates were developed for GeoELAN:
- `eaf-rs`
    - Read, write, and process EAF-files. Uses [quick-xml](https://github.com/tafia/quick-xml) and its serialization support via [serde](https://serde.rs).
    - Repository: <https://github.com/jenslar/eaf-rs>
- `gpmf-rs`:
    - Read GoPro MP/GPMF-files.
    - Repository: <https://github.com/jenslar/gpmf-rs>
- `fit-rs`:
    - Read Garmin FIT-files. Supports custom developer messages.
    - Repository: <https://github.com/jenslar/fit-rs>
- `mp4iter`:
    - Crate to find tracks, move around, find sections, and read values in an MP4 file (does not and will not support any kind of media en/decoding).
    - Repository: <https://github.com/jenslar/mp4iter>

Data extracted with both `gpmf-rs` and `fit-rs` will mostly require further processing. Support for this is built-in for some data types (e.g. GPS data, since this is fundamental for GeoELAN, some processing of sensor data as well), but for others you will have to develop and expand on this yourself. A first pass, extracting and parsing data, should always work for both crates. GeoELAN's `inspect` command with the `--verbose` flag or `--type` option prints data in this "raw" form.
