//! Extract.

use crate::prelude::*;

/// Extract.
#[derive(Debug, Parser)]
pub struct Args {
    /// The part to extract.
    pub part: JsonThing<UrlPart>,
}

/// Deserialize from a JSON value or a JSON string of the input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JsonThing<T>(pub T);

impl<T: for <'de> Deserialize<'de>> FromStr for JsonThing<T> {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(match s.as_bytes() {
            [b'[' | b'{' | b'"'] => serde_json::from_str(s)?,
            _                    => serde_json::from_str(&serde_json::to_string(s)?)?
        }))
    }
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let part = self.part.0;

        for line in std::io::stdin().lines().map(Result::unwrap) {
            match line.parse() {
                Ok (url) => println!("{}", serde_json::to_string(&part.get(&url)).expect("???")),
                Err(e  ) => println!("-{e:?}"),
            }
        }
    }
}
