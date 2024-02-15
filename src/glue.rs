#[cfg(feature = "regex")]
mod regex;
#[cfg(feature = "glob")]
mod glob;
#[cfg(feature = "commands")]
mod command;
/// Serializing and deserialzing [`reqwest::header::HeaderMap`].
#[cfg(all(feature = "http", not(target_family = "wasm")))]
pub mod headermap;

#[cfg(feature = "regex")]
pub use regex::*;
#[cfg(feature = "glob")]
pub use glob::*;
#[cfg(feature = "commands")]
pub use command::*;

use std::{
    str::FromStr,
    marker::PhantomData,
    fmt
};
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

// https://serde.rs/string-or-struct.html
pub(crate) fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr,
    D: Deserializer<'de>,
    <T as FromStr>::Err: fmt::Debug
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr,
        <T as FromStr>::Err: fmt::Debug
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<T, E> {
            FromStr::from_str(value).map_err(|_| E::custom("The provided string could not be parsed."))
        }

        fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<T, M::Error> {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

pub(crate) fn optional_string_or_struct<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de> + FromStr,
    D: Deserializer<'de>,
    <T as FromStr>::Err: fmt::Debug
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct OptionalStringOrStruct<T>(PhantomData<fn() -> Option<T>>);

    impl<'de, T> Visitor<'de> for OptionalStringOrStruct<Option<T>>
    where
        T: Deserialize<'de> + FromStr,
        <T as FromStr>::Err: fmt::Debug
    {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Option<T>, E> {
            FromStr::from_str(value).map_err(|_| E::custom("The provided string could not be parsed.")).map(Some)
        }

        fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<Option<T>, M::Error> {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map)).map(Some)
        }

        fn visit_none<E: de::Error>(self) -> Result<Option<T>, E> {
            Ok(None)
        }
    }

    deserializer.deserialize_any(OptionalStringOrStruct(PhantomData))
}
