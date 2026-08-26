use serde::{Deserialize, Serialize};

use crate::length::Length;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "RadiusCbor", into = "RadiusCbor", rename_all = "kebab-case")]
pub struct Radius {
    pub top_left: Option<Length>,
    pub top_right: Option<Length>,
    pub bottom_left: Option<Length>,
    pub bottom_right: Option<Length>,
}

impl Radius {
    pub const fn new(
        top_left: Option<Length>,
        top_right: Option<Length>,
        bottom_left: Option<Length>,
        bottom_right: Option<Length>,
    ) -> Self {
        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }

    pub const fn builder() -> RadiusBuilder {
        RadiusBuilder {
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
            top: None,
            bottom: None,
            left: None,
            right: None,
            rest: None,
        }
    }
}

pub struct RadiusBuilder {
    top_left: Option<Length>,
    top_right: Option<Length>,
    bottom_left: Option<Length>,
    bottom_right: Option<Length>,
    top: Option<Length>,
    bottom: Option<Length>,
    left: Option<Length>,
    right: Option<Length>,
    rest: Option<Length>,
}

impl RadiusBuilder {
    pub const fn top_left(mut self, value: Length) -> Self {
        self.top_left = Some(value);
        self
    }

    pub const fn top_right(mut self, value: Length) -> Self {
        self.top_right = Some(value);
        self
    }

    pub const fn bottom_left(mut self, value: Length) -> Self {
        self.bottom_left = Some(value);
        self
    }

    pub const fn bottom_right(mut self, value: Length) -> Self {
        self.bottom_right = Some(value);
        self
    }

    pub const fn top(mut self, value: Length) -> Self {
        self.top = Some(value);
        self
    }

    pub const fn bottom(mut self, value: Length) -> Self {
        self.bottom = Some(value);
        self
    }

    pub const fn left(mut self, value: Length) -> Self {
        self.left = Some(value);
        self
    }

    pub const fn right(mut self, value: Length) -> Self {
        self.right = Some(value);
        self
    }

    pub const fn rest(mut self, value: Length) -> Self {
        self.rest = Some(value);
        self
    }

    pub fn build(self) -> Radius {
        Radius::new(
            self.top_left
                .or_else(|| self.left.clone())
                .or_else(|| self.top.clone())
                .or_else(|| self.rest.clone()),
            self.top_right
                .or(self.top)
                .or_else(|| self.right.clone())
                .or_else(|| self.rest.clone()),
            self.bottom_left
                .or(self.left)
                .or_else(|| self.bottom.clone())
                .or_else(|| self.rest.clone()),
            self.bottom_right
                .or(self.right)
                .or(self.bottom)
                .or(self.rest),
        )
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct RadiusCbor {
    top_left: Option<Length>,
    top_right: Option<Length>,
    bottom_left: Option<Length>,
    bottom_right: Option<Length>,
    left: Option<Length>,
    top: Option<Length>,
    right: Option<Length>,
    bottom: Option<Length>,
    rest: Option<Length>,
}

impl From<Radius> for RadiusCbor {
    fn from(value: Radius) -> Self {
        Self {
            top_left: value.top_left,
            top_right: value.top_right,
            bottom_left: value.bottom_left,
            bottom_right: value.bottom_right,
            left: None,
            top: None,
            right: None,
            bottom: None,
            rest: None,
        }
    }
}

impl From<RadiusCbor> for Radius {
    fn from(value: RadiusCbor) -> Self {
        Self::new(
            value
                .top_left
                .or_else(|| value.left.clone())
                .or_else(|| value.top.clone())
                .or_else(|| value.rest.clone()),
            value
                .top_right
                .or(value.top)
                .or_else(|| value.right.clone())
                .or_else(|| value.rest.clone()),
            value
                .bottom_left
                .or(value.left)
                .or_else(|| value.bottom.clone())
                .or_else(|| value.rest.clone()),
            value
                .bottom_right
                .or(value.right)
                .or(value.bottom)
                .or(value.rest),
        )
    }
}
