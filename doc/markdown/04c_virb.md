### Garmin VIRB

Note that the Garmin VIRB Ultra 30 is no longer available for purchase. Garmin currently has no replacement product.

#### File structure

Example VIRB SDCard file structure:
```
├── DCIM
│   └── 100_VIRB
│       ├── VIRB0001-1.GLV           Recording session split up
│       ├── VIRB0001-1.MP4           as 10 minutes clips. Low (GLV)
│       ├── VIRB0001-2.GLV           and high (MP4) resolution clips.
│       └── VIRB0001-2.MP4
└── GMetrix
    ├── 2017-01-01-12-00-00.fit      Telemetry files, a.k.a "FIT-files".
    └── 2017-01-02-12-00-00.fit      May contain data, such as GPS-logs,
                                     for multiple recording sessions.
```

#### The VIRB and the FIT-format

To pair and match VIRB video clips belonging to the same recording sessions with a FIT-file unique identifiers (UUID) are embedded both within the original video clips and the FIT-files. Preserving these are key to synchronise and extract relevant GPS-data.

When synchronising and locating data, GeoELAN will sometimes list all sessions present in the FIT-file. As a help, the number of video clips and the _UUID for the first clip_ in each session is listed.

A single FIT-file may contain telemetry for multiple recording sessions. When the camera is turned on, it immediately starts logging data into a new FIT-file, regardless of a video being recorded or not. The camera will keep logging to this file until completely turned off. If turned on again, a new FIT-file will be created. All data points in a FIT-file are explicitly timestamped, which technically allows synchronisation against any data type in the file. Further, with the help of the built-in GPS, absolute timestamps can be derived for all data types. These can be used for documentation purposes or to synchronise against external data sources.

For geo-referenced annotations, GeoELAN always embeds absolute timestamps in the resulting KML-file.

The VIRB cameras split up recording sessions into video clips, each approximately 10 minutes in length, with no option to turn this off. To link VIRB video to its corresponding telemetry (e.g. coordinates logged by the GPS during the recording session), both the clips and the FIT-file contain UUIDs. When the user starts recording, a "video recording session start" message is logged to the current FIT-file together with the UUID embedded in the first clip, denoting the start of a recording session. Similarly, when recording ends, a "video recording session end" message is logged together with the UUID embedded in the last clip in the session. Since all logged FIT-data is timestamped, this creates a timeline for the session that can be related to any logged data in the FIT-file.

**Matching MP4 and FIT-files via embedded UUIDs**
```
                                           ╭─────╮
                    UUID                   │ MP4 │      VIRB001-1.MP4
                   ╭───────────────────>   │     │
                   │                       ╰─────╯
                   │
  "VIRBactioncameraULTRA30_Tall_2688_2016_29.9700       UUID (unique identifier)
   _3937280306_32eed236_1_17_2017-01-28-05-16-40.fit"
                   │
                   │                       ╭─────╮
                   ╰───────────────────>   │ FIT │      2019-01-03-14-23-54.fit
                   Session start/end       │     │
                   message containing UUID ╰─────╯
```


**Logging telemetry and boundaries for a recording session in a FIT-file**
```
   VIRB turned on    Recording session                    VIRB turned off
          │         ├─────────────────┤                          │
Time    ──┼─────────┼─────────────────┼──────────────────────────┼────>
          .         .                 .                          .
          .         .                 .                          .
          .         ╭─────┬─────┬─────╮                          .
          .         │ MP4 │ MP4 │ MP4 │   Video clips            .
          .         │     │     │     │   in recording session   .
          .         ╰─────┴─────┴─────╯                          .
          .         .  │     │     │  .                          .
          .   VIRB001-1.MP4  │     │  .                          .
          .         VIRB001-2.MP4  │  .                          .
          .         .     VIRB001-3.MP4                          .
          .         .                 .                          .
FIT-file  .         .                 .                          .
time span ├─────────┼─────────────────┼─────────────────────────>│
          │         │                 │                          │
       Logging   Session           Session                    Logging
       starts     start              end                       stops
          .      message           message                       .
          .                                                      .
          └──────────────────────────┬───────────────────────────┘
                                  ╭─────╮
                                  │ FIT │
                                  │     │
                                  ╰─────╯
                          2019-01-03-14-23-54.fit
```

The VIRB logs location, barometric pressure, and rotation among many other data types. Since the FIT-format is not a text based data format, and thus cannot be inspected using a text editor, the `inspect` command allows for some exploration of a FIT-file (see command _inspect_). GeoELAN will also help out with matching recording sessions to the corresponding FIT-files (see commands _virb2eaf_, and _locate_).

#### Preserving UUIDs

Concatenating or converting the video clips will usually discard the UUIDs, so the user is advised to save the original video clips. The `inspect` command can be used to display the UUID for a specific VIRB MP4-file, just run `geoelan inspect --video VIRBVIDEO.MP4` with no other options.

Most of the commands allow for selecting UUID from those present in the relevant FIT-file when matching files or geo-referencing annotations. The `locate` command can also be used to locate all files for a specific session.

#### Video file management and options

On the VIRB MicroSD card, the low-resolution clips have a `.GLV` extension. These are generated by the VIRB for quick viewing on the internal camera display. If available, GeoELAN will prefer to link these in the ELAN-file over the high-resolution video due to their smaller size (both resolutions will still be concatenated by default). GeoELAN will not be able to identify the low-resolution `.GLV` as such if renamed to `.MP4` and they may even be mistaken for the high-resolution versions. If you only require the low-resolution videos to be concatenated, use the `--low-res-only` flag when running `virb2eaf`. This will ignore the high-resolution `.MP4`-files as a concatenation target, with an option to copy these as-is (`--copy`) to the output directory (see the _virb2eaf_ section for further information).
