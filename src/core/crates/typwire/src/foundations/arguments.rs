use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::Any;

const TYPE_NAME: &str = "arguments";

/// A structure representing Typst's captured `arguments` type.
///
/// `pos` and `named` mirror Typst's own `.pos()`/`.named()` split; order is
/// only meaningful within each group, not between them.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(try_from = "ArgumentsTypst", into = "ArgumentsTypst")]
pub struct Arguments {
    pub pos: Vec<Any>,
    pub named: BTreeMap<String, Any>,
}

impl Arguments {
    /// Creates a new `Arguments` instance from positional and named values.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Arguments;
    ///
    /// let args = Arguments::new(vec![1_i64.into()], Default::default());
    /// assert_eq!(args.pos.len(), 1);
    /// ```
    pub const fn new(pos: Vec<Any>, named: BTreeMap<String, Any>) -> Self {
        Self { pos, named }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ArgumentsTypst {
    typwire_type: String,
    pos: Vec<Any>,
    named: BTreeMap<String, Any>,
}

// Serialize into ArgumentsTypst
impl From<Arguments> for ArgumentsTypst {
    fn from(value: Arguments) -> Self {
        Self {
            typwire_type: TYPE_NAME.to_string(),
            pos: value.pos,
            named: value.named,
        }
    }
}

// Deserialize from ArgumentsTypst
impl TryFrom<ArgumentsTypst> for Arguments {
    type Error = String;

    fn try_from(value: ArgumentsTypst) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Arguments: {}",
                value.typwire_type
            ));
        }

        Ok(Self::new(value.pos, value.named))
    }
}
