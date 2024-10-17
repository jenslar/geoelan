## Installation

- Download the zip-file from <https://github.com/jenslar/geoelan> or use `git clone https://github.com/jenslar/geoelan.git`.
- See the `bin` directory for pre-compiled executables for Linux, macOS, and Windows.

### Compile and install from source

You can also compile GeoELAN from source. Depending on your operating system, this may require installing additional software, and some understanding of working in a terminal. The basic steps are:

1. Install [the Rust programming language](https://www.rust-lang.org)
2. Get the GeoELAN source from <https://github.com/jenslar/geoelan>
3. `cd geoelan` (you should be in the folder containing `Cargo.toml`)
4. `cargo build --release`
5. `cargo install --path .` (optional, makes `geoelan` [a global command](https://doc.rust-lang.org/cargo/commands/cargo-install.html))