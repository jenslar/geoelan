### FFmpeg

The `cam2eaf` command requires [FFmpeg](https://www.ffmpeg.org) for joining MP4-clips and to extract the audio track as a WAV-file (required to display a wave form in ELAN while annotating).

The video and audio streams are by default only concatenated, not converted, to avoid data loss and to save time, but note that **VIRB UUID and GoPro telemetry will still be discarded - save the original files**.

There are two main options for installing FFmpeg:
1. Download the _static build_ of FFmpeg, and specify its path using the `--ffmpeg` option
2. Install via a _package manager_. FFmpeg will be automatically available to `cam2eaf` in this case.

> **Static build:**
>
> The _static build_ option means that the relevant media codecs are included in a single, executable file that can be used as is. The [FFmpeg download page](https://ffmpeg.org/download.html) provides links to static builds for macOS, Windows and Linux. Put the downloaded `ffmpeg`-file in a convenient location and use the `--ffmpeg` option when running `cam2eaf`. Optionally moving or [symlinking](https://en.wikipedia.org/wiki/Symbolic_link) this file to a directory in [PATH](https://en.wikipedia.org/wiki/PATH_(variable)) will yield the same result as using a package manager below.
>
> **Package manager:**
>
> Installing via a _package manager_ means the `ffmpeg` command can be executed from anywhere in a terminal. Linux distributions usually come with one pre-installed. For macOS [Homebrew](https://brew.sh) is a popular choice, whereas Windows has [Chocolatey](https://chocolatey.org) (or [WSL](https://docs.microsoft.com/en-us/windows/wsl/)). This option means you do not have to specify the location of `ffmpeg` each time `cam2eaf` is run. If a package manager is not for you, go with the _static build_ for your platform.
