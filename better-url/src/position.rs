//! [`BetterPosition`].

use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Error as _}};
use url::Position;

use crate::prelude::*;

/// Wrapper around a [`Position`].
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct BetterPosition(pub Position);

#[allow(non_upper_case_globals, reason = "Simulating enum variants.")]
impl BetterPosition {
    /// [`Position::AfterFragment`].
    pub const AfterFragment : Self = Self(Position:: AfterFragment );
    /// [`Position::AfterHost`].
    pub const AfterHost     : Self = Self(Position:: AfterHost     );
    /// [`Position::AfterPassword`].
    pub const AfterPassword : Self = Self(Position:: AfterPassword );
    /// [`Position::AfterPath`].
    pub const AfterPath     : Self = Self(Position:: AfterPath     );
    /// [`Position::AfterPort`].
    pub const AfterPort     : Self = Self(Position:: AfterPort     );
    /// [`Position::AfterQuery`].
    pub const AfterQuery    : Self = Self(Position:: AfterQuery    );
    /// [`Position::AfterScheme`].
    pub const AfterScheme   : Self = Self(Position:: AfterScheme   );
    /// [`Position::AfterUsername`].
    pub const AfterUsername : Self = Self(Position:: AfterUsername );
    /// [`Position::BeforeFragment`].
    pub const BeforeFragment: Self = Self(Position:: BeforeFragment);
    /// [`Position::BeforeHost`].
    pub const BeforeHost    : Self = Self(Position:: BeforeHost    );
    /// [`Position::BeforePassword`].
    pub const BeforePassword: Self = Self(Position:: BeforePassword);
    /// [`Position::BeforePath`].
    pub const BeforePath    : Self = Self(Position:: BeforePath    );
    /// [`Position::BeforePort`].
    pub const BeforePort    : Self = Self(Position:: BeforePort    );
    /// [`Position::BeforeQuery`].
    pub const BeforeQuery   : Self = Self(Position:: BeforeQuery   );
    /// [`Position::BeforeScheme`].
    pub const BeforeScheme  : Self = Self(Position:: BeforeScheme  );
    /// [`Position::BeforeUsername`].
    pub const BeforeUsername: Self = Self(Position:: BeforeUsername);
}

impl BetterPosition {
    /// Make a [`Self`].
    /// # Errors
    /// If `s` is an invalid [`Self`] variant, returns the error [`InvalidBetterPosition`].
    pub fn parse(s: &str) -> Result<Self, InvalidBetterPosition> {
        match s {
            "BeforeScheme"   => Ok(Self::BeforeScheme  ),
            "AfterScheme"    => Ok(Self::AfterScheme   ),
            "BeforeUsername" => Ok(Self::BeforeUsername),
            "AfterUsername"  => Ok(Self::AfterUsername ),
            "BeforePassword" => Ok(Self::BeforePassword),
            "AfterPassword"  => Ok(Self::AfterPassword ),
            "BeforeHost"     => Ok(Self::BeforeHost    ),
            "AfterHost"      => Ok(Self::AfterHost     ),
            "BeforePort"     => Ok(Self::BeforePort    ),
            "AfterPort"      => Ok(Self::AfterPort     ),
            "BeforePath"     => Ok(Self::BeforePath    ),
            "AfterPath"      => Ok(Self::AfterPath     ),
            "BeforeQuery"    => Ok(Self::BeforeQuery   ),
            "AfterQuery"     => Ok(Self::AfterQuery    ),
            "BeforeFragment" => Ok(Self::BeforeFragment),
            "AfterFragment"  => Ok(Self::AfterFragment ),
            _ => Err(InvalidBetterPosition)
        }
    }

    /// Get this as a string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self(Position::BeforeScheme  ) => "BeforeScheme",
            Self(Position::AfterScheme   ) => "AfterScheme",
            Self(Position::BeforeUsername) => "BeforeUsername",
            Self(Position::AfterUsername ) => "AfterUsername",
            Self(Position::BeforePassword) => "BeforePassword",
            Self(Position::AfterPassword ) => "AfterPassword",
            Self(Position::BeforeHost    ) => "BeforeHost",
            Self(Position::AfterHost     ) => "AfterHost",
            Self(Position::BeforePort    ) => "BeforePort",
            Self(Position::AfterPort     ) => "AfterPort",
            Self(Position::BeforePath    ) => "BeforePath",
            Self(Position::AfterPath     ) => "AfterPath",
            Self(Position::BeforeQuery   ) => "BeforeQuery",
            Self(Position::AfterQuery    ) => "AfterQuery",
            Self(Position::BeforeFragment) => "BeforeFragment",
            Self(Position::AfterFragment ) => "AfterFragment",
        }
    }
}

impl PartialEq for BetterPosition {
    fn eq(&self, other: &Self) -> bool {
        *self == other.0
    }
}

impl Eq for BetterPosition {}

impl PartialEq<Position> for BetterPosition {
    fn eq(&self, other: &Position) -> bool {
        match (self.0, other) {
            (Position::AfterFragment , Position::AfterFragment ) => true,
            (Position::AfterHost     , Position::AfterHost     ) => true,
            (Position::AfterPassword , Position::AfterPassword ) => true,
            (Position::AfterPath     , Position::AfterPath     ) => true,
            (Position::AfterPort     , Position::AfterPort     ) => true,
            (Position::AfterQuery    , Position::AfterQuery    ) => true,
            (Position::AfterScheme   , Position::AfterScheme   ) => true,
            (Position::AfterUsername , Position::AfterUsername ) => true,
            (Position::BeforeFragment, Position::BeforeFragment) => true,
            (Position::BeforeHost    , Position::BeforeHost    ) => true,
            (Position::BeforePassword, Position::BeforePassword) => true,
            (Position::BeforePath    , Position::BeforePath    ) => true,
            (Position::BeforePort    , Position::BeforePort    ) => true,
            (Position::BeforeQuery   , Position::BeforeQuery   ) => true,
            (Position::BeforeScheme  , Position::BeforeScheme  ) => true,
            (Position::BeforeUsername, Position::BeforeUsername) => true,
            _ => false
        }
    }
}

impl std::hash::Hash for BetterPosition {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        (self.0 as u8).hash(hasher)
    }
}

impl FromStr for BetterPosition {
    type Err = InvalidBetterPosition;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl std::fmt::Display for BetterPosition {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

#[cfg(feature = "serde")]
impl Serialize for BetterPosition {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BetterPosition {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <&str>::deserialize(deserializer)?.parse().map_err(D::Error::custom)
    }
}

impl From<Position      > for BetterPosition {fn from(value: Position      ) -> Self {Self(value)}}
impl From<BetterPosition> for Position       {fn from(value: BetterPosition) -> Self {value.0}}

impl AsRef<Position> for BetterPosition {fn as_ref(&self    ) -> &Position     {&self.0    }}
impl AsMut<Position> for BetterPosition {fn as_mut(&mut self) -> &mut Position {&mut self.0}}

impl std::ops::Deref for BetterPosition {
    type Target = Position;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BetterPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
