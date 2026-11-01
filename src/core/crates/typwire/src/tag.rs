//! The `typwire-type` tag every native type's wire dictionary carries.
//!
//! Both helpers here exist for speed, not shape -- the wire bytes are the
//! same as a plain `String` field would produce. A `String` tag costs one
//! allocation per tagged value on decode (plus one on encode, for
//! `TYPE_NAME.to_string()`), which adds up fast for something like a
//! gradient, where every stop carries a tagged color. [`Tag`] checks the
//! tag against a `&'static str` straight from the borrowed input instead.

use std::fmt;
use std::marker::PhantomData;

use serde::de::{self, DeserializeSeed, IntoDeserializer, MapAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The key every tagged wire dictionary carries its type name under.
pub(crate) const TAG_KEY: &str = "typwire-type";

/// A native type with a fixed `typwire-type` tag.
pub(crate) trait Tagged {
    const TYPE_NAME: &'static str;
}

/// A zero-sized `typwire-type` field for a `…Typst` shadow struct: encodes
/// as `T::TYPE_NAME` and only decodes from exactly that string.
pub(crate) struct Tag<T>(PhantomData<T>);

impl<T> Tag<T> {
    pub(crate) const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Tagged> Serialize for Tag<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(T::TYPE_NAME)
    }
}

impl<'de, T: Tagged> Deserialize<'de> for Tag<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer
            .deserialize_str(TagVisitor(&[T::TYPE_NAME]))
            .map(|_| Self::new())
    }
}

/// Matches a tag string against a fixed set of type names, returning the
/// matching `&'static str` -- so callers can dispatch on it without keeping
/// (or allocating) the input string.
struct TagVisitor(&'static [&'static str]);

impl<'de> Visitor<'de> for TagVisitor {
    type Value = &'static str;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            [name] => write!(formatter, "the typwire-type `{name}`"),
            names => write!(formatter, "one of the typwire-types {names:?}"),
        }
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        self.0
            .iter()
            .copied()
            .find(|name| *name == value)
            .ok_or_else(|| E::invalid_value(Unexpected::Str(value), &self))
    }
}

impl<'de> DeserializeSeed<'de> for TagVisitor {
    type Value = &'static str;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_str(self)
    }
}

/// Accepts only the map key `typwire-type`.
struct TagKey;

impl<'de> Visitor<'de> for TagKey {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "`{TAG_KEY}` as the first key")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        if value == TAG_KEY {
            Ok(())
        } else {
            Err(E::invalid_value(Unexpected::Str(value), &self))
        }
    }
}

impl<'de> DeserializeSeed<'de> for TagKey {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_str(self)
    }
}

/// Read a tagged dictionary's leading `typwire-type` entry, returning
/// which of `names` it is. The rest of `map` is left for the caller to
/// hand to the matching type, e.g. through
/// `serde::de::value::MapAccessDeserializer`.
///
/// This is what lets a multi-variant type like `Color` dispatch on its tag
/// in a single pass. serde's own `#[serde(tag = "...")]` can't, since it
/// doesn't know where in the map the tag will be: it buffers the whole
/// value into an intermediate tree first and deserializes that a second
/// time. The cost is a wire rule: the tag must be the dictionary's
/// **first** key. Both sides always write it first (Typst keeps a
/// dictionary's insertion order through `cbor.encode`/`json.encode`, and
/// every encoder puts the tag first; serde writes an internally tagged
/// enum's tag first, too).
pub(crate) fn next_tag<'de, A: MapAccess<'de>>(
    map: &mut A,
    names: &'static [&'static str],
) -> Result<&'static str, A::Error> {
    if map.next_key_seed(TagKey)?.is_none() {
        return Err(de::Error::missing_field(TAG_KEY));
    }
    map.next_value_seed(TagVisitor(names))
}

/// Replays an already-read leading `typwire-type` entry in front of the
/// rest of `map` -- for a caller that had to read the tag itself to pick a
/// type (`Any`), but whose chosen type's own `Deserialize` expects to read
/// the tag too.
pub(crate) struct WithTag<'t, A> {
    tag: &'t str,
    replayed: Replayed,
    map: A,
}

#[derive(PartialEq)]
enum Replayed {
    Nothing,
    Key,
    Entry,
}

impl<'t, A> WithTag<'t, A> {
    pub(crate) fn new(tag: &'t str, map: A) -> Self {
        Self {
            tag,
            replayed: Replayed::Nothing,
            map,
        }
    }
}

impl<'de, A: MapAccess<'de>> MapAccess<'de> for WithTag<'_, A> {
    type Error = A::Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        if self.replayed == Replayed::Nothing {
            self.replayed = Replayed::Key;
            return seed
                .deserialize(IntoDeserializer::<A::Error>::into_deserializer(TAG_KEY))
                .map(Some);
        }
        self.map.next_key_seed(seed)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        if self.replayed == Replayed::Key {
            self.replayed = Replayed::Entry;
            return seed.deserialize(IntoDeserializer::<A::Error>::into_deserializer(self.tag));
        }
        self.map.next_value_seed(seed)
    }

    fn size_hint(&self) -> Option<usize> {
        let pending = usize::from(self.replayed == Replayed::Nothing);
        self.map.size_hint().map(|len| len + pending)
    }
}

#[cfg(test)]
mod tests {
    use cbor4ii::core::Value;

    use crate::{Color, Length, decode};

    fn text(value: &str) -> Value {
        Value::Text(value.to_owned())
    }

    fn cbor(entries: Vec<(&str, Value)>) -> Vec<u8> {
        let map = Value::Map(
            entries
                .into_iter()
                .map(|(key, value)| (text(key), value))
                .collect(),
        );
        cbor4ii::serde::to_vec(Vec::new(), &map).unwrap()
    }

    #[test]
    fn tag_must_be_the_first_key() {
        let tag_first = cbor(vec![
            ("typwire-type", text("color-luma")),
            ("lightness", Value::Float(0.5)),
            ("alpha", Value::Float(1.0)),
        ]);
        let tag_last = cbor(vec![
            ("lightness", Value::Float(0.5)),
            ("alpha", Value::Float(1.0)),
            ("typwire-type", text("color-luma")),
        ]);

        assert!(decode::<Color>(&tag_first).is_ok());
        assert!(decode::<Color>(&tag_last).is_err());
    }

    #[test]
    fn wrong_tag_is_rejected() {
        let angle = cbor(vec![
            ("typwire-type", text("angle")),
            ("points", Value::Float(1.0)),
        ]);
        assert!(decode::<Length>(&angle).is_err());
        assert!(decode::<Color>(&angle).is_err());
    }
}
