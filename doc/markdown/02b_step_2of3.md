## Step 2/3: Annotate events in ELAN

Next, use ELAN with the ELAN-file from step 1 to annotate events that should be geo-referenced in step 3. Feel free to create any tier structure you may need. Tokenized tiers can not be geo-referenced, but otherwise any tier is fine, including deeply nested, referred tiers.

GeoELAN will geo-reference annotations from a single tier (selectable in step 3). Thus, if you want to generate a KML/GeoJSON-file with e.g. indigenous place names mentioned on-site during the recording, those place names must be limited to a single tier. If there are other spatial categories or groupings you wish to explore, simply create a new tier for each. In step 3 you can then re-run GeoELAN as many times as required, then select a different tier and/or options on each run.

When the annotations are geo-referenced in step 3, the annotation values in the selected tier will be used as descriptions for the synchronized, corresponding points in the KML and GeoJSON-files. Points corresponding to unannotated sections of the ELAN-file will either be discarded or have no description, depending on which options you use in step 3.

An annotated event can relate to anything observed in the recording and can be represented as either points or polylines in the output KML-file. If you are unsure which best applies to what you have in mind for your data, or how this may affect how you annotate, here are a few ideas for each kind.

> **Points** could concern documenting:
> - **the location of a plant or a geographical feature**, e.g. annotate the timespan either is visible in the video.
> - **an uttered place name or an animal cry**, e.g. annotate the timespan of the on-site utterance or cry.
>
> For these specific cases, the exact time spans of the annotations are not that important. It should be enough to ensure the annotation lasts for the duration of the place name being uttered, or for as long as the plant is visible. If unsure, add a another second to the annotation timespan. An average coordinate will be calculated for those that were logged within each annotation's time span, so as long as the camera wearer does not stray too far from the observation point, the result should be accurate enough.
>
> **Lines** could concern documenting:
> - various **types of movement through the landscape**. To annotate the movement of "walking up-hill" as it is observed visually in the recording, set the annotation's start time at the bottom of the hill and its end at the top, or for as long as the motion can be observed.
> - a **narrative reflecting on the immediate surroundings** as they change over time. E.g. comments on visible landscape features, or perhaps the re-construction of an historical event as it unfolded over space and time.
