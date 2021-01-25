#![allow(dead_code)]

pub mod write {
    use chrono::Local;

    fn head() -> String {
        let ver = "2.2";
        format!(
            r#"<?xml version='1.0' encoding='utf-8'?>
<kml xmlns="http://www.opengis.net/kml/{0}" xmlns:gx="http://www.google.com/kml/ext/{0}">
    <Document>
"#,
            ver
        )
    }

    pub fn tail() -> String {
        String::from(
            r#"
    </Document>
</kml>"#,
        )
    }

    fn meta(uuid: &str, device: &str, linestyle: bool) -> String {
        // let t = Local::now().format("%+").to_string(); // ISO with time zone offset
        let t = Local::now().format("%Y-%m-%dT%H:%M:%S%.3f").to_string();
        let style = match linestyle {
            true => {
                r#"
        <Style id="marked_event">
            <LineStyle>
                <color>7fff0000</color>
                <width>4</width>
            </LineStyle>
        </Style>
"#
            }
            false => "",
        };
        format!(
            r#"        <name>{0}</name>
        <description>uuid:{1}</description>
        <TimeStamp><when>{2}</when></TimeStamp>{3}
"#,
            device, uuid, t, style
        )
    }

    fn point(
        t0: chrono::DateTime<chrono::Utc>,
        id: usize,
        point: &crate::structs::Point,
        cdata: bool,
    ) -> String {
        // using six decimal digits (around 11mm...)
        // see: https://gis.stackexchange.com/questions/8650/measuring-accuracy-of-latitude-and-longitude

        let description = if let Some(t) = &point.text {
            if cdata {
                format!(
                    r#"
                    <tr><td>Description: {}</td></tr>
"#,
                    t
                )
            } else {
                format!("<description>{}</description>", t)
            }
        } else {
            if cdata {
                String::from("")
            } else {
                String::from("<description/>")
            }
        };

        if cdata {
            format!(
                r#"    <Placemark>
        <name>ID{0}</name>
        <snippet/>
        <description>
            <![CDATA[
            <table>{1}
            <tr><td>Longitude:   {3:.6}</td></tr>
            <tr><td>Latitude:    {2:.6}</td></tr>
            <tr><td>Altitude:    {4:.1}</td></tr>
            <tr><td>Heading:     {5:.1}</td></tr>
            <tr><td>Time:        {6}</td></tr>
            </table>
            ]]>
        </description>
        <LookAt>
            <longitude>{3:.6}</longitude>
            <latitude>{2:.6}</latitude>
            <altitude>{4:.1}</altitude>
            <tilt>66</tilt>
        </LookAt>
        <TimeStamp><when>{6}</when></TimeStamp>
        <styleUrl/>
        <Point><coordinates>{3:.6},{2:.6},{4:.1}</coordinates></Point>
    </Placemark>"#,
                id,
                description,
                point.latitude,
                point.longitude,
                point.altitude,
                point.heading,
                (t0 + point.time)
                    .format("%Y-%m-%dT%H:%M:%S%.3f")
                    .to_string()
            )
        // (t0+point.time).format("%+").to_string()) // ISO with time zone offset
        } else {
            format!(
                r#"    <Placemark>
        <name>ID{0}</name>
        {1}
        <TimeStamp><when>{5}</when></TimeStamp>
        <Point><coordinates>{3:.6},{2:.6},{4:.1}</coordinates></Point>
    </Placemark>
    "#,
                id,
                description,
                point.latitude,
                point.longitude,
                point.altitude,
                (t0 + point.time)
                    .format("%Y-%m-%dT%H:%M:%S%.3f")
                    // .format("%+").to_string()) // ISO with time zone offset
                    .to_string()
            )
        }
    }

    fn polyline(
        t0: chrono::DateTime<chrono::Utc>,
        id: usize,
        line: &[crate::structs::Point],
        cdata: bool,
    ) -> String {
        // NOTE: input = coordindates for single line, not mulitple
        let mut polystr: Vec<String> = Vec::new();
        let mut text: Option<String> = None;

        let t1 = (t0 + line.first().expect("Polyline: No points?").time)
            .format("%Y-%m-%dT%H:%M:%S%.3f")
            .to_string();
        let t2 = (t0 + line.last().expect("Polyline: No points?").time)
            .format("%Y-%m-%dT%H:%M:%S%.3f")
            // .format("%+").to_string(), // ISO with time zone offset
            .to_string();

        for point in line {
            if text.is_none() && point.text.is_some() {
                text = match cdata {
                    true => Some(format!(
                        "
            <![CDATA[
                <table>
                <tr><td>Description: {}</td></tr>
                <tr><td>Longitude:   {:.6}</td></tr>
                <tr><td>Latitude:    {:.6}</td></tr>
                <tr><td>Time Start:  {}</td></tr>
                <tr><td>Time End:    {}</td></tr>
                </table>
            ]]>
        ",
                        point.text.to_owned().unwrap(),
                        point.longitude,
                        point.latitude,
                        t1,
                        t2
                    )),
                    false => point.text.to_owned(),
                };
            }
            polystr.push(
                [
                    format!("{:.6}", point.longitude),
                    format!("{:.6}", point.latitude),
                    format!("{:.1}", point.altitude),
                ]
                .join(","),
            );
        }

        let description = if let Some(t) = text {
            format!(
                "<description>{}</description>
            <TimeSpan>
                <begin>{}</begin>
                <end>{}</end>
            </TimeSpan>
        <styleUrl>#marked_event</styleUrl>",
                t, t1, t2
            )
        } else {
            String::from("<description/>")
        };

        format!(
            r#"    <Placemark>
        <name>{}</name>
        {}
        <LineString>
            <coordinates>
            {}
            </coordinates>
        </LineString>
    </Placemark>"#,
            id,
            description,
            polystr.join("\n\t\t\t")
        )
    }

    pub fn build(
        data: &crate::structs::GeoType,
        t0: &chrono::DateTime<chrono::Utc>,
        uuid: &[String],
        device: &str,
        cdata: bool,
    ) -> String {
        let mut kml_data: Vec<String> = Vec::new();

        // KML CONTENT
        let mut linestyle = false;
        match data {
            crate::structs::GeoType::POINT(points) => {
                for (point_count, p) in points.iter().enumerate() {
                    kml_data.push(point(*t0, point_count + 1, p, cdata));
                }
            }
            crate::structs::GeoType::LINE(lines) => {
                linestyle = true;
                for (line_count, l) in lines.iter().enumerate() {
                    kml_data.push(polyline(*t0, line_count + 1, l, cdata));
                }
            }
        }

        // KML, BUILD DOC
        let mut kml_doc = String::new();

        let kml_head = head();
        let kml_meta = meta(&uuid.join(";"), device, linestyle);
        let kml_tail = tail();

        kml_doc.push_str(&kml_head[..]);
        kml_doc.push_str(&kml_meta[..]);
        kml_doc.push_str(&kml_data.join(""));
        kml_doc.push_str(&kml_tail[..]);

        kml_doc
    }
}
