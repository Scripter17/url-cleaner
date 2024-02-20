#[cfg(feature = "regex")]
mod regex;
#[cfg(feature = "glob")]
mod glob;
#[cfg(feature = "commands")]
mod command;
/// Serializing and deserializing [`reqwest::header::HeaderMap`].
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

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
            FromStr::from_str(value).map_err(|_| E::custom("The provided string could not be parsed."))
        }

        fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
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
    struct OptionalStringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for OptionalStringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr,
        <T as FromStr>::Err: fmt::Debug
    {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or null or map")
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
            FromStr::from_str(value).map_err(|_| E::custom("The provided string could not be parsed.")).map(Some)
        }

        fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map)).map(Some)
        }
    }

    deserializer.deserialize_any(OptionalStringOrStruct(PhantomData))
}

pub(crate) fn box_string_or_struct<'de, T, D>(deserializer: D) -> Result<Box<T>, D::Error>
where
    T: Deserialize<'de> + FromStr,
    D: Deserializer<'de>,
    <T as FromStr>::Err: fmt::Debug
{
    struct OptionalStringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for OptionalStringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr,
        <T as FromStr>::Err: fmt::Debug
    {
        type Value = Box<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
            FromStr::from_str(value).map_err(|_| E::custom("The provided string could not be parsed.")).map(Box::new)
        }

        fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map)).map(Box::new)
        }
    }

    deserializer.deserialize_any(OptionalStringOrStruct(PhantomData))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, dead_code)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[cfg(feature = "string-source")]
    #[test]
    fn optional_string_or_struct_test() {
        #[derive(Deserialize)]
        struct A {
            #[serde(deserialize_with = "optional_string_or_struct")]
            a: Option<crate::types::StringSource>
        }
        serde_json::from_str::<A>(r#"{"a": null}"#).unwrap();
        serde_json::from_str::<A>(r#"{"a": "/"}"# ).unwrap();
        serde_json::from_str::<A>(r#"{"a": {"String": "/"}}"#).unwrap();
        serde_json::from_str::<A>(r#"{"a": {"Part": "Path"}}"#).unwrap();
        serde_json::from_str::<A>(r#"{"a": {"Var": "path"}}"#).unwrap();
    }

    #[cfg(feature = "string-source")]
    #[test]
    fn box_string_or_struct_test() {
        #[derive(Deserialize)]
        struct B {
            #[serde(deserialize_with = "box_string_or_struct")]
            b: Box<crate::types::StringSource>
        }
        serde_json::from_str::<B>(r#"{"b": "/"}"# ).unwrap();
        serde_json::from_str::<B>(r#"{"b": {"String": "/"}}"#).unwrap();
        serde_json::from_str::<B>(r#"{"b": {"Part": "Path"}}"#).unwrap();
        serde_json::from_str::<B>(r#"{"b": {"Var": "path"}}"#).unwrap();
    }
}
