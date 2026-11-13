use serde::{Deserialize, Serialize};

use crate::{Color, Ratio};

/// A structure representing a color stop in a gradient with a specified color and offset.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stop {
    /// The color at this stop.
    pub color: Color,
    /// The position of this stop along the gradient.
    pub offset: Ratio,
}

impl Stop {
    /// Creates a new `Stop` instance with the specified color and offset.
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the stop in the gradient.
    /// * `offset` - The offset of the stop, represented as a ratio.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::{Ratio, Stop, color};
    ///
    /// let stop = Stop::new(color::BLACK, Ratio::new(0.5));
    /// assert_eq!(stop.color, color::BLACK);
    /// assert_eq!(stop.offset, Ratio::new(0.5));
    /// ```
    pub const fn new(color: Color, offset: Ratio) -> Self {
        Self { color, offset }
    }
}
