//! [`NonSpecialHost`].

use thiserror::Error;

use crate::prelude::*;

/// Returned when attempting to parse an invalid [`NonSpecialHost`].
#[derive(Debug, Error)]
#[error("Attempted to parse an invalid NonSpecialHost.")]
pub struct InvalidNonSpecialHost;

impl From<InvalidIpv6Host  > for InvalidNonSpecialHost {fn from(_: InvalidIpv6Host  ) -> Self {Self}}
impl From<InvalidOpaqueHost> for InvalidNonSpecialHost {fn from(_: InvalidOpaqueHost) -> Self {Self}}
impl From<InvalidEmptyHost > for InvalidNonSpecialHost {fn from(_: InvalidEmptyHost ) -> Self {Self}}
