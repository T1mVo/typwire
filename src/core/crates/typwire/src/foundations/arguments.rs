use std::borrow::Cow;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::Any;
use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "arguments";

/// Captured arguments to a function, either positional or named.
///
/// Mirrors Typst's `.pos()`/`.named()` split; order is only kept within each
/// group.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/arguments/).
///
/// # Examples
///
/// ```
/// use typwire::{Arguments, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn count(args: Arguments) -> Result<i64, String> {
///     Ok((args.pos.len() + args.named.len()) as i64)
/// }
/// ```
#[derive(Deserialize, Clone, Debug)]
#[serde(from = "ArgumentsTypst<'static>")]
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
struct ArgumentsTypst<'a> {
    typwire_type: Tag<Arguments>,
    pos: Cow<'a, [Any]>,
    named: Cow<'a, BTreeMap<String, Any>>,
}

impl Tagged for Arguments {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize through a borrowing ArgumentsTypst -- `#[serde(into = ...)]`
// would clone the whole value first.
impl Serialize for Arguments {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ArgumentsTypst::from(self).serialize(serializer)
    }
}

impl<'a> From<&'a Arguments> for ArgumentsTypst<'a> {
    fn from(value: &'a Arguments) -> Self {
        Self {
            typwire_type: Tag::new(),
            pos: Cow::Borrowed(&value.pos),
            named: Cow::Borrowed(&value.named),
        }
    }
}

// Deserialize from ArgumentsTypst
impl From<ArgumentsTypst<'_>> for Arguments {
    fn from(value: ArgumentsTypst<'_>) -> Self {
        Self::new(value.pos.into_owned(), value.named.into_owned())
    }
}
