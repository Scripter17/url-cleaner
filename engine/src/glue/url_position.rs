//! Wrapper around a [`url::Position`].

use serde::{Serialize, Deserialize, ser::Serializer, de::{Deserializer, Visitor}};
use url::Position;

use crate::util::*;

/// Wrapper around a [`url::Position`].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Suitability)]
#[suitable(always)]
#[serde(transparent)]
pub struct BetterUrlPosition(#[serde(serialize_with = "serialize", deserialize_with = "deserialize")] pub url::Position);

#[allow(non_upper_case_globals, reason = "Simulating enum variants.")]
impl BetterUrlPosition {
    /// [`url::Position::AfterFragment`].
    pub const AfterFragment : Self = Self(url::Position:: AfterFragment );
    /// [`url::Position::AfterHost`].
    pub const AfterHost     : Self = Self(url::Position:: AfterHost     );
    /// [`url::Position::AfterPassword`].
    pub const AfterPassword : Self = Self(url::Position:: AfterPassword );
    /// [`url::Position::AfterPath`].
    pub const AfterPath     : Self = Self(url::Position:: AfterPath     );
    /// [`url::Position::AfterPort`].
    pub const AfterPort     : Self = Self(url::Position:: AfterPort     );
    /// [`url::Position::AfterQuery`].
    pub const AfterQuery    : Self = Self(url::Position:: AfterQuery    );
    /// [`url::Position::AfterScheme`].
    pub const AfterScheme   : Self = Self(url::Position:: AfterScheme   );
    /// [`url::Position::AfterUsername`].
    pub const AfterUsername : Self = Self(url::Position:: AfterUsername );
    /// [`url::Position::BeforeFragment`].
    pub const BeforeFragment: Self = Self(url::Position:: BeforeFragment);
    /// [`url::Position::BeforeHost`].
    pub const BeforeHost    : Self = Self(url::Position:: BeforeHost    );
    /// [`url::Position::BeforePassword`].
    pub const BeforePassword: Self = Self(url::Position:: BeforePassword);
    /// [`url::Position::BeforePath`].
    pub const BeforePath    : Self = Self(url::Position:: BeforePath    );
    /// [`url::Position::BeforePort`].
    pub const BeforePort    : Self = Self(url::Position:: BeforePort    );
    /// [`url::Position::BeforeQuery`].
    pub const BeforeQuery   : Self = Self(url::Position:: BeforeQuery   );
    /// [`url::Position::BeforeScheme`].
    pub const BeforeScheme  : Self = Self(url::Position:: BeforeScheme  );
    /// [`url::Position::BeforeUsername`].
    pub const BeforeUsername: Self = Self(url::Position:: BeforeUsername);
}

impl PartialEq for BetterUrlPosition {
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

impl Eq for BetterUrlPosition {}

/// Serializer for [`BetterUrlPosition`].
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

/// Visitor for [`BetterUrlPosition`].
struct PositionVisitor;

impl Visitor<'_> for PositionVisitor {
    type Value = Position;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A valid url::Position.")
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
            _ => Err(E::custom("Expected a valid url::Position"))?
        })
    }
}

/// Deserializer for [`BetterUrlPosition`].
fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Position, D::Error> {
    deserializer.deserialize_any(PositionVisitor)
}

impl From<url::Position> for BetterUrlPosition {
    fn from(value: url::Position) -> Self {
        Self(value)
    }
}

impl From<BetterUrlPosition> for url::Position {
    fn from(value: BetterUrlPosition) -> Self {
        value.0
    }
}

impl AsRef<url::Position> for BetterUrlPosition {
    fn as_ref(&self) -> &url::Position {
        &self.0
    }
}

impl AsMut<url::Position> for BetterUrlPosition {
    fn as_mut(&mut self) -> &mut url::Position {
        &mut self.0
    }
}

impl std::ops::Deref for BetterUrlPosition {
    type Target = url::Position;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BetterUrlPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for BetterUrlPosition {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(match self.0 {
            url::Position::BeforeScheme   => "BeforeScheme",
            url::Position::AfterScheme    => "AfterScheme",
            url::Position::BeforeUsername => "BeforeUsername",
            url::Position::AfterUsername  => "AfterUsername",
            url::Position::BeforePassword => "BeforePassword",
            url::Position::AfterPassword  => "AfterPassword",
            url::Position::BeforeHost     => "BeforeHost",
            url::Position::AfterHost      => "AfterHost",
            url::Position::BeforePort     => "BeforePort",
            url::Position::AfterPort      => "AfterPort",
            url::Position::BeforePath     => "BeforePath",
            url::Position::AfterPath      => "AfterPath",
            url::Position::BeforeQuery    => "BeforeQuery",
            url::Position::AfterQuery     => "AfterQuery",
            url::Position::BeforeFragment => "BeforeFragment",
            url::Position::AfterFragment  => "AfterFragment",
        })
    }
}
