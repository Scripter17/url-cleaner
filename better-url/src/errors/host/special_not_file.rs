//! [`SpecialNotFileHost`].

use thiserror::Error;

use crate::prelude::*;

/// Returned when attempting to parse an invalid [`SpecialNotFileHost`].
#[derive(Debug, Error)]
#[error("Attempted to parse an invalid SpecialNotFileHost.")]
pub struct InvalidSpecialNotFileHost;

impl From<InvalidDomainHost> for InvalidSpecialNotFileHost {fn from(_: InvalidDomainHost) -> Self {Self}}
impl From<InvalidIpv4Host  > for InvalidSpecialNotFileHost {fn from(_: InvalidIpv4Host  ) -> Self {Self}}
impl From<InvalidIpv6Host  > for InvalidSpecialNotFileHost {fn from(_: InvalidIpv6Host  ) -> Self {Self}}
impl From<InvalidIpHost    > for InvalidSpecialNotFileHost {fn from(_: InvalidIpHost    ) -> Self {Self}}
