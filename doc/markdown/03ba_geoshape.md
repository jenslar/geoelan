## The _geoshape_ option

Different geographical representations can be generated, including points and lines. Six possible `--geoshape` values are accepted:

| Option         | Description |
| :------------: | :-- |
| `point-all`    | All logged points exported (default if no option passed)
| `point-multi`  | Exported points correspond to marked/annotated events only
| `point-single` | A single, averaged point for each annotation
| `line-all`     | Polyline from all logged points
| `line-multi`   | Polyline, corresponds to marked/annotated events only
| `circle-2d`    | 2D polygon, corresponds to marked/annotated events only
| `circle-3d`    | 3D polygon, corresponds to marked/annotated events only

`--downsample` can be used with all these options, but will be ignored for `point-single`, `circle-2d`/`circle-3d` since these will only ever result in a single point per annotation. `circle-2d` and `circle-3d` allow for further customisation, such as radius and height (`circle-3d`, KML-only). The circle options work in the same way as `point-single` and are currently only a visual flair, since radius and height are not yet derived from ELAN annotation values.

### `point-all`
All points logged during the recording session will be exported. Any point that intersects with the time span of an annotation will inherit the annotation value as the coordinate description. Points that do not, will have no description.

```
ELAN-file

─────┼──────────┼──────────┼──────────┼──────>  ELAN time-line
 00:01:35   00:01:40   00:01:45   00:01:50
─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
  │ Dayum │           │ Chcuh       │           "Feature" tier
  ├───────┤           ├─────────────┤           with annotations
  │       │           │             │           to geo-reference
                      .             .
                      .             .
KML-file              .             .
                      .             .
  + + +         + + + .             .
  ┊     + + + +     ┊ +             + <──────── Logged
  ┊       ┊ ┊       ┊ ┊ +       + + ┊           point
  ┊       ┊ ┊       ┊ ┊   + + +     ┊
  └───┬───┘ └───┬───┘ └──────┬──────┘
      │         │            │
 Points logged  │            │
  within ELAN   │            └ Description for each point in section:
  annotation    │              "Chcuh" (placename)
  time span     │
      │         └ No descriptions for this section,
      │           since there is no corresponding annotation
      │
      └ Description for each point in section: "Dayum" (placename)
```

### `point-multi`

Only points that intersect with the time span of an annotation will be exported and will inherit the annotation text as the coordinate description. Points that have no corresponding annotation will be discarded. _Useful for including points corresponding to marked events only_.

```

  + + +
  ┊     + +           +             + <──────── Logged
  ┊       ┊           ┊ +       + + ┊           point
  ┊       ┊           ┊   + + +     ┊
  ┊       ┊           ┊             ┊
  └───┬───┘           └──────┬──────┘
 Points logged               │
  within ELAN                │
  annotation                 └ Description for each point in section:
  time span                    "Chcuh" (placename)
      │
      └ Description for each point in section:
        "Dayum" (placename)
```

### `point-single`

Only points that intersect with the time span of an annotation will be considered for export. The difference to `point-multi` is that each annotation will only generate a single point: an average of those logged within the annotation's time span. Note that a custom `--downsample` value will be ignored for `point-single` since it may affect the result negatively. `--downsample` also has little use here, since the number of points in the output will not be affected and will be quite low compared to the other options. _Useful for distilling marked events, such as place names, to a single point for each event_.

```

      +
      │
      │
  Average for               +
 points logged              │
  within ELAN               │
  annotation                └ Description for point:
  time span                   "Chcuh" (placename)
      │
      └ Description for point:
        "Dayum" (placename)

```

### `line-all`

All points logged during the recording session will be exported, resulting in a continuous polyline. Sub-sections that intersect with an annotation inherit the annotation value as a description, whereas those that do not will have no description.

```
ELAN-file

─────┼──────────┼──────────┼──────────┼──────>  ELAN time-line
 00:01:35   00:01:40   00:01:45   00:01:50
─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
  │ walk down-hill  │        │ walk up-hill │   "Feature" tier
  ├─────────────────┤        ├──────────────┤   with annotations
  │                 │        │              │   to geo-reference


KML-file
  ____           ___
  ┊   \         /   \                           Resulting polyline
  ┊    \_______/    ┊\           ____________   in KML-file, continuous:
  ┊                 ┊ \         /           ┊   only line-sections with
  ┊                 ┊  \_______/            ┊   corresponding annotations
  ┊                 ┊        ┊              ┊   have a description
  └────────┬────────┘        └───────┬──────┘
           │                         │          Description for
   'walk down-hill'           'walk up-hill'    line-section
                                                in KML
```

### `line-multi`

Only points that intersect with the time span of an annotation will be exported, resulting in a broken-up line. Each sub-section inherits the value of the annotation it intersects with. _Useful for representing paths corresponding to marked events only_.

```

ELAN-file

─────┼──────────┼──────────┼──────────┼──────>  ELAN time-line
 00:01:35   00:01:40   00:01:45   00:01:50
─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
  │ walk down-hill  │        │ walk up-hill │   "Feature" tier
  ├─────────────────┤        ├──────────────┤   with annotations
  │                 │        │              │   to geo-reference


KML-file
  ____           ___
  ┊   \         /   \                           Resulting polyline
  ┊    \_______/    ┊            ____________   in KML-file, broken-up:
  ┊                 ┊           /           ┊   line-sections with no
  ┊                 ┊        __/            ┊   corresponding annotation
  ┊                 ┊        ┊              ┊   are discarded
  └────────┬────────┘        └───────┬──────┘
           │                         │          Description for
   'walk down-hill'           'walk up-hill'    line-section
                                                in KML
```

### `circle-2d`, `circle-3d`

`circle-2d`, and `circle-3d` work almost exactly like `point-single` with the difference that a circle is generated around the calculated average point. It is mostly a visual flair and its shape is currently not affected by annotation values. `circle-2d` is flat against the ground, whereas `circle-3d` can take a height value to become a cylindrical 3D shape (only applies to KML, not GeoJSON). If circle output is specified, three more options become available:

| Option | Description | Default |
| :-: | :-- | :--
| `--height`       | Height relative to ground in meters (`circle-3d`) |
| `--radius`      | Radius in meters (`circle-2d`, `circle-3d`) | `2.0`
| `--vertices`     | Roundness, valid range 3 - 255 (3 will literally be triangle) | `40`
