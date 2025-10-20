//! Wrapper around a [`Position`].

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Visitor}};
use url::Position;

/// Wrapper around a [`Position`].
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BetterPosition(#[cfg_attr(feature = "serde", serde(serialize_with = "serialize", deserialize_with = "deserialize"))] pub Position);

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

impl PartialEq for BetterPosition {
    fn eq(&self, other: &Self) -> bool {
        match (self.0, other.0) {
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

impl Eq for BetterPosition {}

/// Serializer for [`BetterPosition`].
#[cfg(feature = "serde")]
fn serialize<S: Serializer>(value: &Position, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(match value {
        Position::BeforeScheme   => "BeforeScheme",
        Position::AfterScheme    => "AfterScheme",
        Position::BeforeUsername => "BeforeUsername",
        Position::AfterUsername  => "AfterUsername",
        Position::BeforePassword => "BeforePassword",
        Position::AfterPassword  => "AfterPassword",
        Position::BeforeHost     => "BeforeHost",
        Position::AfterHost      => "AfterHost",
        Position::BeforePort     => "BeforePort",
        Position::AfterPort      => "AfterPort",
        Position::BeforePath     => "BeforePath",
        Position::AfterPath      => "AfterPath",
        Position::BeforeQuery    => "BeforeQuery",
        Position::AfterQuery     => "AfterQuery",
        Position::BeforeFragment => "BeforeFragment",
        Position::AfterFragment  => "AfterFragment",
    })
}

/// Visitor for [`BetterPosition`].
#[cfg(feature = "serde")]
struct PositionVisitor;

#[cfg(feature = "serde")]
impl Visitor<'_> for PositionVisitor {
    type Value = Position;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A valid Position.")
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(match value {
            "BeforeScheme"   => Position::BeforeScheme,
            "AfterScheme"    => Position::AfterScheme,
            "BeforeUsername" => Position::BeforeUsername,
            "AfterUsername"  => Position::AfterUsername,
            "BeforePassword" => Position::BeforePassword,
            "AfterPassword"  => Position::AfterPassword,
            "BeforeHost"     => Position::BeforeHost,
            "AfterHost"      => Position::AfterHost,
            "BeforePort"     => Position::BeforePort,
            "AfterPort"      => Position::AfterPort,
            "BeforePath"     => Position::BeforePath,
            "AfterPath"      => Position::AfterPath,
            "BeforeQuery"    => Position::BeforeQuery,
            "AfterQuery"     => Position::AfterQuery,
            "BeforeFragment" => Position::BeforeFragment,
            "AfterFragment"  => Position::AfterFragment,
            _ => Err(E::custom("Expected a valid Position"))?
        })
    }
}

/// Deserializer for [`BetterPosition`].
#[cfg(feature = "serde")]
fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Position, D::Error> {
    deserializer.deserialize_any(PositionVisitor)
}

impl From<Position> for BetterPosition {
    fn from(value: Position) -> Self {
        Self(value)
    }
}

impl From<BetterPosition> for Position {
    fn from(value: BetterPosition) -> Self {
        value.0
    }
}

impl AsRef<Position> for BetterPosition {
    fn as_ref(&self) -> &Position {
        &self.0
    }
}

impl AsMut<Position> for BetterPosition {
    fn as_mut(&mut self) -> &mut Position {
        &mut self.0
    }
}

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

impl std::fmt::Display for BetterPosition {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(match self.0 {
            Position::BeforeScheme   => "BeforeScheme",
            Position::AfterScheme    => "AfterScheme",
            Position::BeforeUsername => "BeforeUsername",
            Position::AfterUsername  => "AfterUsername",
            Position::BeforePassword => "BeforePassword",
            Position::AfterPassword  => "AfterPassword",
            Position::BeforeHost     => "BeforeHost",
            Position::AfterHost      => "AfterHost",
            Position::BeforePort     => "BeforePort",
            Position::AfterPort      => "AfterPort",
            Position::BeforePath     => "BeforePath",
            Position::AfterPath      => "AfterPath",
            Position::BeforeQuery    => "BeforeQuery",
            Position::AfterQuery     => "AfterQuery",
            Position::BeforeFragment => "BeforeFragment",
            Position::AfterFragment  => "AfterFragment",
        })
    }
}

