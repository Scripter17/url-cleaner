//! [`DomainPart`].

use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// A part of a domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DomainPart {
    /// The segments below [`Self::Middle`].
    Prefix,
    /// The dot between [`Self::Prefix`] and [`Self::Middle`].
    Predot,
    /// The segment just below [`Self::Suffix`].
    Middle,
    /// The dot between [`Self::Middle`] and [`Self::Suffix`].
    Middot,
    /// The suffix as determined by the [`psl`].
    Suffix,
    /// The FQDN dot.
    Fqddot,
    /// [`Self::Middle`] to [`Self::Suffix`], inclusive.
    Origin,
    /// Everything except [`Self::Fqddot`].
    Labels,
    /// If [`Self::Prefix`] is `www`, [`Self::Origin`]. Otherwise [`Self::Labels`].
    Normal,
}

impl DomainPart {
    /// Make a [`DomainPart`].
    /// # Errors
    /// If `s` is an invalid [`DomainPart`], returns the error [`InvalidDomainPart`].
    pub fn parse(s: &str) -> Result<Self, InvalidDomainPart> {
        match s {
            "Prefix" => Ok(Self::Prefix),
            "Predot" => Ok(Self::Predot),
            "Middle" => Ok(Self::Middle),
            "Middot" => Ok(Self::Middot),
            "Suffix" => Ok(Self::Suffix),
            "Fqddot" => Ok(Self::Fqddot),
            "Origin" => Ok(Self::Origin),
            "Labels" => Ok(Self::Labels),
            "Normal" => Ok(Self::Normal),
            _ => Err(InvalidDomainPart)
        }
    }

    /// Get this as a string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Prefix => "Prefix",
            Self::Predot => "Predot",
            Self::Middle => "Middle",
            Self::Middot => "Middot",
            Self::Suffix => "Suffix",
            Self::Fqddot => "Fqddot",
            Self::Origin => "Origin",
            Self::Labels => "Labels",
            Self::Normal => "Normal",
        }
    }
}

impl FromStr for DomainPart {
    type Err = InvalidDomainPart;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for DomainPart {
    type Error = InvalidDomainPart;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl std::fmt::Display for DomainPart {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}
