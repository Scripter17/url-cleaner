//! `/get-domain-suffix` stuff.

use serde::{Serialize, Deserialize, ser::{Serializer, Error as _}, de::{Visitor, Deserializer, Error}};
use thiserror::Error;

/// The errors `/get-domain-suffix`.
#[derive(Debug, Error)]
pub enum GetDomainSuffixError {
    /// Returned when a [`url::ParseError`] is encountered.
    #[error(transparent)]
    ParseHostError(#[from] url::ParseError),
    /// Returned when the host ins't a domain.
    #[error("The host wasn't a domain.")]
    HostIsNotDomain,
    /// Returned when the domain doesn't have a suffix.
    #[error("The domain didn't have a suffix.")]
    DomainDoesNotHaveSuffix
}

/// The [`Result`] returned by the `/get-domain-suffix` route.
pub type GetDomainSuffixResult<'a> = Result<&'a str, GetDomainSuffixError>;

/// Serde helper for deserializing [`GetDomainSuffixError`].
struct GetDomainSuffixErrorVisitor;

impl<'de> Visitor<'de> for GetDomainSuffixErrorVisitor {
    type Value = GetDomainSuffixError;

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(match v {
            "EmptyHost"                        => GetDomainSuffixError::ParseHostError(url::ParseError::EmptyHost),
            "IdnaError"                        => GetDomainSuffixError::ParseHostError(url::ParseError::IdnaError),
            "InvalidPort"                      => GetDomainSuffixError::ParseHostError(url::ParseError::InvalidPort),
            "InvalidIpv4Address"               => GetDomainSuffixError::ParseHostError(url::ParseError::InvalidIpv4Address),
            "InvalidIpv6Address"               => GetDomainSuffixError::ParseHostError(url::ParseError::InvalidIpv6Address),
            "InvalidDomainCharacter"           => GetDomainSuffixError::ParseHostError(url::ParseError::InvalidDomainCharacter),
            "RelativeUrlWithoutBase"           => GetDomainSuffixError::ParseHostError(url::ParseError::RelativeUrlWithoutBase),
            "RelativeUrlWithCannotBeABaseBase" => GetDomainSuffixError::ParseHostError(url::ParseError::RelativeUrlWithCannotBeABaseBase),
            "SetHostOnCannotBeABaseUrl"        => GetDomainSuffixError::ParseHostError(url::ParseError::SetHostOnCannotBeABaseUrl),
            "Overflow"                         => GetDomainSuffixError::ParseHostError(url::ParseError::Overflow),
            "HostIsNotDomain"                  => GetDomainSuffixError::HostIsNotDomain,
            "DomainDoesNotHaveSuffix"          => GetDomainSuffixError::DomainDoesNotHaveSuffix,
            _ => Err(E::custom("Unknown variant"))?
        })
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Expected a string.")
    }
}

impl<'de> Deserialize<'de> for GetDomainSuffixError {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(GetDomainSuffixErrorVisitor)
    }
}

impl Serialize for GetDomainSuffixError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            GetDomainSuffixError::ParseHostError(url::ParseError::EmptyHost)                        => "EmptyHost",
            GetDomainSuffixError::ParseHostError(url::ParseError::IdnaError)                        => "IdnaError",
            GetDomainSuffixError::ParseHostError(url::ParseError::InvalidPort)                      => "InvalidPort",
            GetDomainSuffixError::ParseHostError(url::ParseError::InvalidIpv4Address)               => "InvalidIpv4Address",
            GetDomainSuffixError::ParseHostError(url::ParseError::InvalidIpv6Address)               => "InvalidIpv6Address",
            GetDomainSuffixError::ParseHostError(url::ParseError::InvalidDomainCharacter)           => "InvalidDomainCharacter",
            GetDomainSuffixError::ParseHostError(url::ParseError::RelativeUrlWithoutBase)           => "RelativeUrlWithoutBase",
            GetDomainSuffixError::ParseHostError(url::ParseError::RelativeUrlWithCannotBeABaseBase) => "RelativeUrlWithCannotBeABaseBase",
            GetDomainSuffixError::ParseHostError(url::ParseError::SetHostOnCannotBeABaseUrl)        => "SetHostOnCannotBeABaseUrl",
            GetDomainSuffixError::ParseHostError(url::ParseError::Overflow)                         => "Overflow",
            GetDomainSuffixError::HostIsNotDomain                                                   => "HostIsNotDomain",
            GetDomainSuffixError::DomainDoesNotHaveSuffix                                           => "DomainDoesNotHaveSuffix",
            GetDomainSuffixError::ParseHostError(e)                                                 => Err(S::Error::custom(format!("New url::ParseError varaint: {e:?}")))?,
        }.serialize(serializer)
    }
}
