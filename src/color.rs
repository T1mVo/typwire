use serde::{Deserialize, Serialize};

use crate::{Angle, Gradient, Ratio};

pub const BLACK: Color = Color::Luma(Luma::new(Ratio::new(0.0), Ratio::new(1.0)));
pub const GRAY: Color = Color::Luma(Luma::new(Ratio::new(170.0 / 255.0), Ratio::new(1.0)));
pub const SILVER: Color = Color::Luma(Luma::new(Ratio::new(221.0 / 255.0), Ratio::new(1.0)));
pub const WHITE: Color = Color::Luma(Luma::new(Ratio::new(1.0), Ratio::new(1.0)));

pub const NAVY: Color = Color::Rgb(Rgb::new(
    Ratio::new(0.0),
    Ratio::new(31.0 / 255.0),
    Ratio::new(63.0 / 255.0),
    Ratio::new(1.0),
));
pub const BLUE: Color = Color::Rgb(Rgb::new(
    Ratio::new(0.0),
    Ratio::new(116.0 / 255.0),
    Ratio::new(217.0 / 255.0),
    Ratio::new(1.0),
));
pub const AQUA: Color = Color::Rgb(Rgb::new(
    Ratio::new(127.0 / 255.0),
    Ratio::new(219.0 / 255.0),
    Ratio::new(1.0),
    Ratio::new(1.0),
));
pub const TEAL: Color = Color::Rgb(Rgb::new(
    Ratio::new(57.0 / 255.0),
    Ratio::new(204.0 / 255.0),
    Ratio::new(204.0 / 255.0),
    Ratio::new(1.0),
));
pub const EASTERN: Color = Color::Rgb(Rgb::new(
    Ratio::new(35.0 / 255.0),
    Ratio::new(157.0 / 255.0),
    Ratio::new(173.0 / 255.0),
    Ratio::new(1.0),
));
pub const PURPLE: Color = Color::Rgb(Rgb::new(
    Ratio::new(177.0 / 255.0),
    Ratio::new(13.0 / 255.0),
    Ratio::new(201.0 / 255.0),
    Ratio::new(1.0),
));
pub const FUCHSIA: Color = Color::Rgb(Rgb::new(
    Ratio::new(240.0 / 255.0),
    Ratio::new(18.0 / 255.0),
    Ratio::new(190.0 / 255.0),
    Ratio::new(1.0),
));
pub const MAROON: Color = Color::Rgb(Rgb::new(
    Ratio::new(133.0 / 255.0),
    Ratio::new(20.0 / 255.0),
    Ratio::new(75.0 / 255.0),
    Ratio::new(1.0),
));
pub const RED: Color = Color::Rgb(Rgb::new(
    Ratio::new(1.0),
    Ratio::new(65.0 / 255.0),
    Ratio::new(54.0 / 255.0),
    Ratio::new(1.0),
));
pub const ORANGE: Color = Color::Rgb(Rgb::new(
    Ratio::new(1.0),
    Ratio::new(133.0 / 255.0),
    Ratio::new(27.0 / 255.0),
    Ratio::new(1.0),
));
pub const YELLOW: Color = Color::Rgb(Rgb::new(
    Ratio::new(1.0),
    Ratio::new(220.0 / 255.0),
    Ratio::new(0.0),
    Ratio::new(1.0),
));
pub const OLIVE: Color = Color::Rgb(Rgb::new(
    Ratio::new(61.0 / 255.0),
    Ratio::new(153.0 / 255.0),
    Ratio::new(112.0 / 255.0),
    Ratio::new(1.0),
));
pub const GREEN: Color = Color::Rgb(Rgb::new(
    Ratio::new(46.0 / 255.0),
    Ratio::new(204.0 / 255.0),
    Ratio::new(64.0 / 255.0),
    Ratio::new(1.0),
));
pub const LIME: Color = Color::Rgb(Rgb::new(
    Ratio::new(1.0 / 255.0),
    Ratio::new(1.0),
    Ratio::new(112.0 / 255.0),
    Ratio::new(1.0),
));

/// A grayscale color with lightness and alpha (transparency).
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Luma {
    /// The lightness component of the color.
    pub lightness: Ratio,
    /// The alpha (transparency) component of the color.
    pub alpha: Ratio,
}

impl Luma {
    /// Creates a new Luma color with the given lightness and alpha.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::{Luma, Ratio};
    ///
    /// let luma = Luma::new(Ratio::new(0.5), Ratio::new(1.0));
    /// assert_eq!(luma.lightness, Ratio::new(0.5));
    /// assert_eq!(luma.alpha, Ratio::new(1.0));
    /// ```
    pub const fn new(lightness: Ratio, alpha: Ratio) -> Self {
        Self { lightness, alpha }
    }
}

/// A color in the OKLab color space.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Oklab {
    /// The lightness component of the color.
    pub lightness: Ratio,
    /// The 'a' component of the color.
    pub a: f64,
    /// The 'b' component of the color.
    pub b: f64,
    /// The alpha (transparency) component of the color.
    pub alpha: Ratio,
}

impl Oklab {
    /// Creates a new Oklab color with the given lightness, a, b, and alpha components.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::{Oklab, Ratio};
    ///
    /// let o = Oklab::new(
    ///     Ratio::new(0.5),
    ///     0.0,
    ///     0.0,
    ///     Ratio::new(1.0),
    /// );
    /// assert_eq!(o.lightness, Ratio::new(0.5));
    /// ```
    pub const fn new(lightness: Ratio, a: f64, b: f64, alpha: Ratio) -> Self {
        Self {
            lightness,
            a,
            b,
            alpha,
        }
    }
}

/// A color in the OKLCH color space.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Oklch {
    /// The lightness component of the color.
    pub lightness: Ratio,
    /// The chroma component of the color.
    pub chroma: f64,
    /// The hue component of the color.
    pub hue: Angle,
    /// The alpha (transparency) component of the color.
    pub alpha: Ratio,
}

impl Oklch {
    /// Creates a new Oklch color with the given lightness, chroma, hue, and alpha components.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::{Angle, Oklch, Ratio};
    ///
    /// let o = Oklch::new(
    ///     Ratio::new(0.6),
    ///     0.2,
    ///     Angle::new(0.0),
    ///     Ratio::new(1.0),
    /// );
    /// assert_eq!(o.chroma, 0.2);
    /// ```
    pub const fn new(lightness: Ratio, chroma: f64, hue: Angle, alpha: Ratio) -> Self {
        Self {
            lightness,
            chroma,
            hue,
            alpha,
        }
    }
}

/// A color in the linear RGB color space.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LinearRgb {
    /// The red component of the color.
    pub r: Ratio,
    /// The green component of the color.
    pub g: Ratio,
    /// The blue component of the color.
    pub b: Ratio,
    /// The alpha (transparency) component of the color.
    pub alpha: Ratio,
}

impl LinearRgb {
    /// Creates a new LinearRgb color with the given red, green, blue, and alpha components.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::{LinearRgb, Ratio};
    ///
    /// let lr = LinearRgb::new(
    ///     Ratio::new(0.1),
    ///     Ratio::new(0.2),
    ///     Ratio::new(0.3),
    ///     Ratio::new(1.0),
    /// );
    /// assert_eq!(lr.r, Ratio::new(0.1));
    /// ```
    pub const fn new(r: Ratio, g: Ratio, b: Ratio, alpha: Ratio) -> Self {
        Self { r, g, b, alpha }
    }
}

/// A color in the standard RGB color space.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Rgb {
    /// The red component of the color.
    pub r: Ratio,
    /// The green component of the color.
    pub g: Ratio,
    /// The blue component of the color.
    pub b: Ratio,
    /// The alpha (transparency) component of the color.
    pub alpha: Ratio,
}

impl Rgb {
    /// Creates a new Rgb color with the given red, green, blue, and alpha components.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::{Ratio, Rgb};
    ///
    /// let rgb = Rgb::new(
    ///     Ratio::new(1.0),
    ///     Ratio::new(0.0),
    ///     Ratio::new(0.0),
    ///     Ratio::new(1.0),
    /// );
    /// assert_eq!(rgb.r, Ratio::new(1.0));
    /// ```
    pub const fn new(r: Ratio, g: Ratio, b: Ratio, alpha: Ratio) -> Self {
        Self { r, g, b, alpha }
    }
}

/// A color in the CMYK color space.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Cmyk {
    /// The cyan component of the color.
    pub cyan: Ratio,
    /// The magenta component of the color.
    pub magenta: Ratio,
    /// The yellow component of the color.
    pub yellow: Ratio,
    /// The key (black) component of the color.
    pub key: Ratio,
}

impl Cmyk {
    /// Creates a new Cmyk color with the given cyan, magenta, yellow, and key (black) components.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::{Cmyk, Ratio};
    ///
    /// let c = Cmyk::new(
    ///     Ratio::new(0.0),
    ///     Ratio::new(0.0),
    ///     Ratio::new(0.0),
    ///     Ratio::new(1.0),
    /// );
    /// assert_eq!(c.key, Ratio::new(1.0));
    /// ```
    pub const fn new(cyan: Ratio, magenta: Ratio, yellow: Ratio, key: Ratio) -> Self {
        Self {
            cyan,
            magenta,
            yellow,
            key,
        }
    }
}

/// A color in the HSL (Hue, Saturation, Lightness) color space.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Hsl {
    /// The hue component of the color.
    pub hue: Angle,
    /// The saturation component of the color.
    pub saturation: Ratio,
    /// The lightness component of the color.
    pub lightness: Ratio,
    /// The alpha (transparency) component of the color.
    pub alpha: Ratio,
}

impl Hsl {
    /// Creates a new Hsl color with the given hue, saturation, lightness, and alpha components.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::{Angle, Hsl, Ratio};
    ///
    /// let h = Hsl::new(
    ///     Angle::new(0.0),
    ///     Ratio::new(1.0),
    ///     Ratio::new(0.5),
    ///     Ratio::new(1.0),
    /// );
    /// assert_eq!(h.saturation, Ratio::new(1.0));
    /// ```
    pub const fn new(hue: Angle, saturation: Ratio, lightness: Ratio, alpha: Ratio) -> Self {
        Self {
            hue,
            saturation,
            lightness,
            alpha,
        }
    }
}

/// A color in the HSV (Hue, Saturation, Value) color space.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Hsv {
    /// The hue component of the color.
    pub hue: Angle,
    /// The saturation component of the color.
    pub saturation: Ratio,
    /// The value (brightness) component of the color.
    pub value: Ratio,
    /// The alpha (transparency) component of the color.
    pub alpha: Ratio,
}

impl Hsv {
    /// Creates a new Hsv color with the given hue, saturation, value, and alpha components.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::{Angle, Hsv, Ratio};
    ///
    /// let h = Hsv::new(
    ///     Angle::new(0.0),
    ///     Ratio::new(1.0),
    ///     Ratio::new(1.0),
    ///     Ratio::new(1.0),
    /// );
    /// assert_eq!(h.value, Ratio::new(1.0));
    /// ```
    pub const fn new(hue: Angle, saturation: Ratio, value: Ratio, alpha: Ratio) -> Self {
        Self {
            hue,
            saturation,
            value,
            alpha,
        }
    }
}

/// Represents a color in various color spaces.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "typwire-type")]
pub enum Color {
    /// A grayscale color with lightness and alpha (transparency).
    #[serde(rename = "color-luma")]
    Luma(Luma),
    /// A color in the OKLab color space.
    #[serde(rename = "color-oklab")]
    Oklab(Oklab),
    /// A color in the OKLCH color space.
    #[serde(rename = "color-oklch")]
    Oklch(Oklch),
    /// A color in the linear RGB color space.
    #[serde(rename = "color-linear-rgb")]
    LinearRgb(LinearRgb),
    /// A color in the standard RGB color space.
    #[serde(rename = "color-rgb")]
    Rgb(Rgb),
    /// A color in the CMYK color space.
    #[serde(rename = "color-cmyk")]
    Cmyk(Cmyk),
    /// A color in the HSL (Hue, Saturation, Lightness) color space.
    #[serde(rename = "color-hsl")]
    Hsl(Hsl),
    /// A color in the HSV (Hue, Saturation, Value) color space.
    #[serde(rename = "color-hsv")]
    Hsv(Hsv),
}

// Conversions from structs to Color enum
impl From<Luma> for Color {
    fn from(luma: Luma) -> Self {
        Color::Luma(luma)
    }
}

impl From<Oklab> for Color {
    fn from(oklab: Oklab) -> Self {
        Color::Oklab(oklab)
    }
}

impl From<Oklch> for Color {
    fn from(oklch: Oklch) -> Self {
        Color::Oklch(oklch)
    }
}

impl From<LinearRgb> for Color {
    fn from(linear_rgb: LinearRgb) -> Self {
        Color::LinearRgb(linear_rgb)
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Color::Rgb(rgb)
    }
}

impl From<Cmyk> for Color {
    fn from(cmyk: Cmyk) -> Self {
        Color::Cmyk(cmyk)
    }
}

impl From<Hsl> for Color {
    fn from(hsl: Hsl) -> Self {
        Color::Hsl(hsl)
    }
}

impl From<Hsv> for Color {
    fn from(hsv: Hsv) -> Self {
        Color::Hsv(hsv)
    }
}

/// Represents either a single color or a gradient.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ColorGradient {
    /// A single color.
    Color(Color),
    /// A gradient of colors.
    Gradient(Gradient),
}
