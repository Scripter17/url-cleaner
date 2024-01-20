use url::{Url, ParseError};
use std::str::FromStr;

use serde::{
    Serialize,
    ser::Serializer,
    {de::Error as DeError, Deserialize, Deserializer}
};

/// The method [`crate::rules::conditions::Condition::DomainCondition`] should use.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum DomainConditionRule {
    /// Use the specified URL. If the source of the URL being cleaned is a link on a webpage then this should contain the URL of that webpage.
    #[serde(serialize_with = "serialize_url", deserialize_with = "deserialize_url")]
    Url(Url),
    /// Makes [`crate::rules::conditions::Condition::DomainCondition`] always pass.
    Always,
    /// Makes [`crate::rules::conditions::Condition::DomainCondition`] always fail.
    Never,
    /// Similar to [`DomainConditionRule::Url`] except the contained URL would always be the URL being cleaned.
    /// This is the default as I assume it's the one that works most of the time.
    #[default]
    UseUrlBeingCleaned
}

fn deserialize_url<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Url, D::Error> {
    let x: &'de str=Deserialize::deserialize(deserializer)?;
    Url::parse(x).map_err(|e| D::Error::custom(format!("{e:?}: {x:?}")))
}
fn serialize_url<S: Serializer>(value: &Url, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(value.as_str())
}

impl FromStr for DomainConditionRule {
    type Err=ParseError;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        Ok(match x {
            "Always"             => DomainConditionRule::Always,
            "Never"              => DomainConditionRule::Never,
            "UseUrlBeingCleaned" => DomainConditionRule::UseUrlBeingCleaned,
            _                    => DomainConditionRule::Url(Url::parse(x)?)
        })
    }
}

impl ToString for DomainConditionRule {
    fn to_string(&self) -> String {
        match self {
            Self::Url(url)           => url.to_string(),
            Self::Always             => "Always".to_string(),
            Self::Never              => "Never".to_string(),
            Self::UseUrlBeingCleaned => "UseUrlBeingCleaned".to_string()
        }
    }
}
