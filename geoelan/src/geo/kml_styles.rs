//! KML styles.

use kml::types::Element;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub enum KmlStyleType {
    // KmlIconStyle(KmlIconStyle),
    // KmlLabelStyle(KmlLabelStyle),
    KmlLineStyle(KmlLineStyle),
    KmlPolyStyle(KmlPolyStyle),
}

impl KmlStyleType {
    fn to_element(&self) -> Element {
        match &self {
            // Self::KmlIconStyle(s) => s.to_element(),
            // Self::KmlLabelStyle(s) => s.to_element(),
            Self::KmlLineStyle(s) => s.to_element(),
            Self::KmlPolyStyle(s) => s.to_element(),
        }
    }
}

#[derive(Debug)]
pub struct KmlStyle {
    pub id: String,
    pub styles: Vec<KmlStyleType>,
}

impl Default for KmlStyle {
    fn default() -> Self {
        Self {
            id: "defaultStyle".to_owned(),
            styles: Vec::new(),
        }
    }
}

impl KmlStyle {
    pub fn to_element(&self) -> Element {
        let mut styles = Element::default();
        styles.name = "Style".to_owned();
        styles.attrs.insert("id".to_owned(), self.id.to_owned());

        for style in self.styles.iter() {
            styles.children.push(style.to_element());
        }

        styles
    }
}

pub struct KmlIconStyle {
    pub color: Rgba,
    pub href: String, // <Icon><href>path</href></Icon>
    pub scale: f32, // 1.0 = 100%
    pub heading: f32
}

pub struct KmlLabelStyle {
    pub color: Rgba,
    /// Scale 1.0 = 100%
    pub scale: f32
}

impl Default for KmlLabelStyle {
    fn default() -> Self {
        Self {
            color: Rgba::default(),
            scale: 1.0
        }
    }
}

/// ```xml
/// <LineStyle id="ID">
///   <!-- inherited from ColorStyle -->
///   <color>ffffffff</color>            <!-- kml:color -->
///   <colorMode>normal</colorMode>      <!-- colorModeEnum: normal or random -->
///   <!-- specific to LineStyle -->
///   <width>1</width>                            <!-- float -->
///   <gx:outerColor>ffffffff</gx:outerColor>     <!-- kml:color -->
///   <gx:outerWidth>0.0</gx:outerWidth>          <!-- float -->
///   <gx:physicalWidth>0.0</gx:physicalWidth>    <!-- float -->
///   <gx:labelVisibility>0</gx:labelVisibility>  <!-- boolean -->
/// </LineStyle>
/// ```
#[derive(Debug, Clone)]
pub struct KmlLineStyle {
    pub id: Option<String>,
    pub color: Rgba,
    pub color_mode: KmlColorMode,
    // Width in pixels
    pub width: f32,
    // pub outer_color: Option<Rgba>,
    // 0.0 - 1.0, proportion that uses `outer_color`
    // pub outer_width: Option<f32>,
    // pub physical_width: Option<f32>, // width in meters
}

impl Default for KmlLineStyle {
    fn default() -> Self {
        Self {
            id: None,
            color: Rgba::default(),
            color_mode: KmlColorMode::Normal,
            // outer_color: None,
            width: 4.0,
            // outer_width: None,
            // physical_width: None,
        }
    }
}

impl KmlLineStyle {
    pub fn to_element(&self) -> Element {
        let mut line_style = Element::default();
        line_style.name = "LineStyle".to_owned();

        if let Some(id) = &self.id {
            line_style.attrs.insert("id".to_owned(), id.to_owned());
        }

        let mut color = Element::default();
        color.name = "color".to_owned();
        color.content = Some(self.color.to_kml());
        line_style.children.push(color);
        
        let mut width = Element::default();
        width.name = "width".to_owned();
        width.content = Some(self.width.to_string());
        line_style.children.push(width);
        
        // let mut outer_color = Element::default();
        // outer_color.name = "outerColor".to_owned();
        // outer_color.content = self.outer_color.as_ref().map(|c| c.to_kml());
        // line_style.children.push(outer_color);
        
        // let mut outer_width = Element::default();
        // outer_width.name = "outerWidth".to_owned();
        // outer_width.content = self.outer_width.map(|w| w.to_string());
        // line_style.children.push(outer_width);

        line_style
    }
}

#[derive(Debug, Clone)]
pub struct KmlPolyStyle {
    pub id: Option<String>,
    pub color: Rgba,
    pub color_mode: KmlColorMode,
    pub fill: bool,
    /// Uses line style for outline color.
    pub outline: bool,
}

impl Default for KmlPolyStyle {
    fn default() -> Self {
        Self {
            id: None,
            color: Rgba::default(),
            color_mode: KmlColorMode::Normal,
            fill: true,
            outline: true
        }
    }
}

impl KmlPolyStyle {
    pub fn to_element(&self) -> Element {
        let mut poly_style = Element::default();
        poly_style.name = "PolyStyle".to_owned();

        if let Some(id) = &self.id {
            poly_style.attrs.insert("id".to_owned(), id.to_owned());
        }

        let mut color = Element::default();
        color.name = "color".to_owned();
        color.content = Some(self.color.to_kml());
        poly_style.children.push(color);

        let mut color_mode = Element::default();
        color_mode.name = "colorMode".to_owned();
        color_mode.content = Some(self.color_mode.to_string());
        poly_style.children.push(color_mode);

        let mut fill = Element::default();
        fill.name = "fill".to_owned();
        let fill_value = if self.fill == true {1} else {0};
        fill.content = Some(fill_value.to_string());
        poly_style.children.push(fill);

        let mut outline = Element::default();
        outline.name = "outline".to_owned();
        let outline_value = if self.outline == true {1} else {0};
        outline.content = Some(outline_value.to_string());
        poly_style.children.push(outline);

        poly_style
    }
}

#[derive(Debug, Clone)]
pub enum KmlColorMode {
    Normal,
    Random
}

impl KmlColorMode {
    pub fn to_string(&self) -> String {
        match self {
            KmlColorMode::Normal => "normal".to_owned(),
            KmlColorMode::Random => "random".to_owned()
        }
    }
}

#[derive(Debug, Clone)]
/// Rgba color. Red, green blue, alpha.
pub struct Rgba(u8, u8, u8, u8);

impl Default for Rgba {
    /// Default, solid white.
    fn default() -> Self {
        Rgba::white()
    }
}

impl Rgba{
    /// Generate hexadecimal string.
    pub fn to_hex(&self) -> String {
        format!("{:02x?}{:02x?}{:02x?}{:02x?}", self.0, self.1, self.2, self.3)
    }

    /// Generate CSS style hexadecimal string, prefixed with `#`.
    pub fn to_css(&self) -> String {
        format!("#{:02x?}{:02x?}{:02x?}{:02x?}", self.0, self.1, self.2, self.3)
    }

    /// Generate KML style hexadecimal string: alpha, blue, green, red.
    pub fn to_kml(&self) -> String {
        format!("{:02x?}{:02x?}{:02x?}{:02x?}", self.3, self.2, self.1, self.0)
    }

    /// Random color with optional transparency.
    pub fn random(alpha: Option<u8>) -> Self {
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();
        let a = alpha.unwrap_or(255);

        Rgba(r, g, b, a)
    }

    pub fn with_alpha(&self, alpha: u8) -> Self {
        Rgba(
            self.0,
            self.1,
            self.2,
            alpha,
        )
    }

    /// Solid red.
    pub fn red() -> Self {
        Rgba(255, 0, 0, 255)
    }

    /// Solid green.
    pub fn green() -> Self {
        Rgba(0, 255, 0, 255)
    }

    /// Solid blue.
    pub fn blue() -> Self {
        Rgba(0, 0, 255, 255)
    }

    /// Solid black.
    pub fn black() -> Self {
        Rgba(0, 0, 0, 255)
    }

    /// Solid white.
    pub fn white() -> Self {
        Rgba(255, 255, 255, 255)
    }
}