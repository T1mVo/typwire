use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{Angle, Center, Ratio, Stop};

/// Represents different types of gradients with specific parameters for each type.
///
/// # Examples
/// ```
/// use typwire::{Angle, Center, ColorSpace, Gradient, Ratio, Stop};
///
/// let linear_gradient = Gradient::linear(vec![], Angle::new(45.0), ColorSpace::Oklab);
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "typwire-type", rename_all_fields = "kebab-case")]
pub enum Gradient {
    #[serde(rename = "gradient-linear")]
    Linear {
        stops: Vec<Stop>,
        angle: Angle,
        space: ColorSpace,
    },
    #[serde(rename = "gradient-radial")]
    Radial {
        stops: Vec<Stop>,
        center: Center,
        radius: Ratio,
        focal_center: Center,
        focal_radius: Ratio,
        space: ColorSpace,
    },
    #[serde(rename = "gradient-conic")]
    Conic {
        stops: Vec<Stop>,
        angle: Angle,
        center: Center,
        space: ColorSpace,
    },
}

impl Gradient {
    /// Creates a new linear gradient with the specified stops, angle, and color space.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Angle, ColorSpace, Gradient, Stop};
    ///
    /// let gradient = Gradient::linear(vec![], Angle::new(45.0), ColorSpace::Oklab);
    /// ```
    pub const fn linear(stops: Vec<Stop>, angle: Angle, space: ColorSpace) -> Self {
        Self::Linear {
            stops,
            angle,
            space,
        }
    }

    /// Returns a builder for creating a linear gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::Gradient;
    ///
    /// let builder = Gradient::linear_builder();
    /// ```
    pub fn linear_builder() -> LinearGradientBuilder {
        LinearGradientBuilder {
            stops: vec![],
            angle: None,
            space: ColorSpace::default(),
        }
    }

    /// Creates a new radial gradient using specified stops, centers, radii, and color space.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Center, ColorSpace, Gradient, Ratio, Stop};
    ///
    /// let gradient = Gradient::radial(
    ///     vec![],
    ///     Center::new(Ratio::new(0.5), Ratio::new(0.5)),
    ///     Ratio::new(1.0),
    ///     Center::new(Ratio::new(0.5), Ratio::new(0.5)),
    ///     Ratio::new(0.5),
    ///     ColorSpace::Oklab,
    /// );
    /// ```
    pub const fn radial(
        stops: Vec<Stop>,
        center: Center,
        radius: Ratio,
        focal_center: Center,
        focal_radius: Ratio,
        space: ColorSpace,
    ) -> Self {
        Self::Radial {
            stops,
            center,
            radius,
            focal_center,
            focal_radius,
            space,
        }
    }

    /// Returns a builder for creating a radial gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::Gradient;
    ///
    /// let builder = Gradient::radial_builder();
    /// ```
    pub fn radial_builder() -> RadialGradientBuilder {
        RadialGradientBuilder {
            stops: vec![],
            center: None,
            radius: None,
            focal_center: None,
            focal_radius: None,
            space: ColorSpace::default(),
        }
    }

    /// Creates a new conic gradient using specified stops, an angle, a center, and a color space.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Angle, Center, ColorSpace, Gradient, Ratio, Stop};
    ///
    /// let gradient = Gradient::conic(
    ///     vec![],
    ///     Angle::new(45.0),
    ///     Center::new(Ratio::new(0.5), Ratio::new(0.5)),
    ///     ColorSpace::Oklab,
    /// );
    /// ```
    pub const fn conic(stops: Vec<Stop>, angle: Angle, center: Center, space: ColorSpace) -> Self {
        Self::Conic {
            stops,
            angle,
            center,
            space,
        }
    }

    /// Returns a builder for creating a conic gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Center, Gradient, Ratio};
    ///
    /// let builder = Gradient::conic_builder();
    /// ```
    pub fn conic_builder() -> ConicGradientBuilder {
        ConicGradientBuilder {
            stops: vec![],
            angle: None,
            center: None,
            space: ColorSpace::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ColorSpace {
    Luma,
    #[default]
    Oklab,
    Oklch,
    LinearRgb,
    Rgb,
    Cmyk,
    Hsl,
    Hsv,
}

/// A builder for creating linear gradients.
///
/// # Examples
/// ```
/// use typwire::{Angle, ColorSpace, Gradient, Ratio, Stop, color};
///
/// let gradient = Gradient::linear_builder()
///     .stops(vec![
///         Stop::new(color::BLACK, Ratio::new(0.0)),
///         Stop::new(color::WHITE, Ratio::new(1.0)),
///     ])
///     .angle(Angle::new(45.0))
///     .build()
///     .unwrap();
/// ```
pub struct LinearGradientBuilder {
    stops: Vec<Stop>,
    angle: Option<Angle>,
    space: ColorSpace,
}

impl LinearGradientBuilder {
    /// Sets the stops for the gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Gradient, Stop};
    ///
    /// let builder = Gradient::linear_builder().stops(vec![]);
    /// ```
    pub fn stops(mut self, stops: Vec<Stop>) -> Self {
        self.stops = stops;

        self
    }

    /// Adds a single stop to the gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Gradient, Ratio, Stop, color};
    ///
    /// let builder = Gradient::linear_builder().stop(Stop::new(color::BLACK, Ratio::new(0.0)));
    /// ```
    pub fn stop(mut self, stop: Stop) -> Self {
        self.stops.push(stop);

        self
    }

    /// Sets the angle for the linear gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Angle, Gradient};
    ///
    /// let builder = Gradient::linear_builder().angle(Angle::new(45.0));
    /// ```
    pub const fn angle(mut self, angle: Angle) -> Self {
        self.angle = Some(angle);

        self
    }

    /// Sets the color space for the gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{ColorSpace, Gradient};
    ///
    /// let builder = Gradient::linear_builder().space(ColorSpace::Rgb);
    /// ```
    pub const fn space(mut self, space: ColorSpace) -> Self {
        self.space = space;

        self
    }

    /// Builds the linear gradient.
    ///
    /// # Errors
    /// Returns a `GradientBuilderError` if required fields are missing.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Angle, Gradient, Ratio, Stop, color};
    ///
    /// let gradient = Gradient::linear_builder()
    ///     .stops(vec![
    ///         Stop::new(color::BLACK, Ratio::new(0.0)),
    ///         Stop::new(color::WHITE, Ratio::new(1.0)),
    ///     ])
    ///     .angle(Angle::new(45.0))
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<Gradient, GradientBuilderError> {
        let stops = if self.stops.len() >= 2 {
            self.stops
        } else {
            return Err(GradientBuilderError::MissingField("stops"));
        };

        let Some(angle) = self.angle else {
            return Err(GradientBuilderError::MissingField("angle"));
        };

        Ok(Gradient::linear(stops, angle, self.space))
    }
}

/// A builder for creating radial gradients.
///
/// # Examples
/// ```
/// use typwire::{Center, Gradient, Ratio, Stop, color};
///
/// let gradient = Gradient::radial_builder()
///     .stops(vec![
///         Stop::new(color::BLACK, Ratio::new(0.0)),
///         Stop::new(color::WHITE, Ratio::new(1.0)),
///     ])
///     .center(Center::new(Ratio::new(0.5), Ratio::new(0.5)))
///     .radius(Ratio::new(1.0))
///     .focal_center(Center::new(Ratio::new(0.5), Ratio::new(0.5)))
///     .focal_radius(Ratio::new(0.5))
///     .build()
///     .unwrap();
/// ```
pub struct RadialGradientBuilder {
    stops: Vec<Stop>,
    center: Option<Center>,
    radius: Option<Ratio>,
    focal_center: Option<Center>,
    focal_radius: Option<Ratio>,
    space: ColorSpace,
}

impl RadialGradientBuilder {
    /// Sets the stops for the gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Gradient, Stop};
    ///
    /// let builder = Gradient::radial_builder().stops(vec![]);
    /// ```
    pub fn stops(mut self, stops: Vec<Stop>) -> Self {
        self.stops = stops;

        self
    }

    /// Adds a single stop to the gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Gradient, Ratio, Stop, color};
    ///
    /// let builder = Gradient::radial_builder().stop(Stop::new(color::BLACK, Ratio::new(0.0)));
    /// ```
    pub fn stop(mut self, stop: Stop) -> Self {
        self.stops.push(stop);

        self
    }

    /// Sets the center point for the radial gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Center, Gradient, Ratio};
    ///
    /// let builder = Gradient::radial_builder().center(Center::new(Ratio::new(0.5), Ratio::new(0.5)));
    /// ```
    pub const fn center(mut self, center: Center) -> Self {
        self.center = Some(center);

        self
    }

    /// Sets the radius for the radial gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Gradient, Ratio};
    ///
    /// let builder = Gradient::radial_builder().radius(Ratio::new(1.0));
    /// ```
    pub const fn radius(mut self, radius: Ratio) -> Self {
        self.radius = Some(radius);

        self
    }

    /// Sets the focal center for the radial gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Center, Gradient, Ratio};
    ///
    /// let builder =
    ///     Gradient::radial_builder().focal_center(Center::new(Ratio::new(0.5), Ratio::new(0.5)));
    /// ```
    pub const fn focal_center(mut self, focal_center: Center) -> Self {
        self.focal_center = Some(focal_center);

        self
    }

    /// Sets the focal radius for the radial gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Gradient, Ratio};
    ///
    /// let builder = Gradient::radial_builder().focal_radius(Ratio::new(0.5));
    /// ```
    pub const fn focal_radius(mut self, focal_radius: Ratio) -> Self {
        self.focal_radius = Some(focal_radius);

        self
    }

    /// Sets the color space for the gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{ColorSpace, Gradient};
    ///
    /// let builder = Gradient::radial_builder().space(ColorSpace::Rgb);
    /// ```
    pub const fn space(mut self, space: ColorSpace) -> Self {
        self.space = space;

        self
    }

    /// Builds the radial gradient.
    ///
    /// # Errors
    /// Returns a `GradientBuilderError` if required fields are missing.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Center, Gradient, Ratio, Stop, color};
    ///
    /// let gradient = Gradient::radial_builder()
    ///     .stops(vec![
    ///         Stop::new(color::BLACK, Ratio::new(0.0)),
    ///         Stop::new(color::WHITE, Ratio::new(1.0)),
    ///     ])
    ///     .center(Center::new(Ratio::new(0.5), Ratio::new(0.5)))
    ///     .radius(Ratio::new(1.0))
    ///     .focal_center(Center::new(Ratio::new(0.5), Ratio::new(0.5)))
    ///     .focal_radius(Ratio::new(0.5))
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<Gradient, GradientBuilderError> {
        let stops = if self.stops.len() >= 2 {
            self.stops
        } else {
            return Err(GradientBuilderError::MissingField("stops"));
        };

        let Some(center) = self.center else {
            return Err(GradientBuilderError::MissingField("center"));
        };

        let Some(radius) = self.radius else {
            return Err(GradientBuilderError::MissingField("radius"));
        };

        let Some(focal_center) = self.focal_center else {
            return Err(GradientBuilderError::MissingField("focal_center"));
        };

        let Some(focal_radius) = self.focal_radius else {
            return Err(GradientBuilderError::MissingField("focal_radius"));
        };

        Ok(Gradient::radial(
            stops,
            center,
            radius,
            focal_center,
            focal_radius,
            self.space,
        ))
    }
}

/// A builder for creating conic gradients.
///
/// # Examples
/// ```
/// use typwire::{Angle, Center, Gradient, Ratio, Stop, color};
///
/// let gradient = Gradient::conic_builder()
///     .stops(vec![
///         Stop::new(color::BLACK, Ratio::new(0.0)),
///         Stop::new(color::WHITE, Ratio::new(0.0)),
///     ])
///     .angle(Angle::new(45.0))
///     .center(Center::new(Ratio::new(0.5), Ratio::new(0.5)))
///     .build()
///     .unwrap();
/// ```
pub struct ConicGradientBuilder {
    stops: Vec<Stop>,
    angle: Option<Angle>,
    center: Option<Center>,
    space: ColorSpace,
}

impl ConicGradientBuilder {
    /// Sets the stops for the gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Gradient, Stop};
    ///
    /// let builder = Gradient::conic_builder().stops(vec![]);
    /// ```
    pub fn stops(mut self, stops: Vec<Stop>) -> Self {
        self.stops = stops;

        self
    }

    /// Adds a single stop to the gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Gradient, Ratio, Stop, color};
    ///
    /// let builder = Gradient::conic_builder().stop(Stop::new(color::BLACK, Ratio::new(0.0)));
    /// ```
    pub fn stop(mut self, stop: Stop) -> Self {
        self.stops.push(stop);

        self
    }

    /// Sets the angle for the conic gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Angle, Gradient};
    ///
    /// let builder = Gradient::conic_builder().angle(Angle::new(45.0));
    /// ```
    pub const fn angle(mut self, angle: Angle) -> Self {
        self.angle = Some(angle);

        self
    }

    /// Sets the center point for the conic gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Center, Gradient, Ratio};
    ///
    /// let builder = Gradient::conic_builder().center(Center::new(Ratio::new(0.5), Ratio::new(0.5)));
    /// ```
    pub const fn center(mut self, center: Center) -> Self {
        self.center = Some(center);

        self
    }

    /// Sets the color space for the gradient.
    ///
    /// # Examples
    /// ```
    /// use typwire::{ColorSpace, Gradient};
    ///
    /// let builder = Gradient::conic_builder().space(ColorSpace::Rgb);
    /// ```
    pub const fn space(mut self, space: ColorSpace) -> Self {
        self.space = space;

        self
    }

    /// Builds the conic gradient.
    ///
    /// # Errors
    /// Returns a `GradientBuilderError` if required fields are missing.
    ///
    /// # Examples
    /// ```
    /// use typwire::{Angle, Center, Gradient, Ratio, Stop, color};
    ///
    /// let gradient = Gradient::conic_builder()
    ///     .stops(vec![
    ///         Stop::new(color::BLACK, Ratio::new(0.0)),
    ///         Stop::new(color::WHITE, Ratio::new(1.0)),
    ///     ])
    ///     .angle(Angle::new(45.0))
    ///     .center(Center::new(Ratio::new(0.5), Ratio::new(0.5)))
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<Gradient, GradientBuilderError> {
        let stops = if self.stops.len() >= 2 {
            self.stops
        } else {
            return Err(GradientBuilderError::MissingField("stops"));
        };

        let Some(angle) = self.angle else {
            return Err(GradientBuilderError::MissingField("angle"));
        };

        let Some(center) = self.center else {
            return Err(GradientBuilderError::MissingField("center"));
        };

        Ok(Gradient::conic(stops, angle, center, self.space))
    }
}

/// Represents errors that can occur while building a gradient.
#[derive(Error, Debug)]
pub enum GradientBuilderError {
    #[error("builder missing required field: {0}")]
    MissingField(&'static str),
}
