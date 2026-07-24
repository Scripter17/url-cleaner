//! [`Host`].

use thiserror::Error;

use crate::prelude::*;

/// Returned when attempting to parse an invalid [`Host`].
#[derive(Debug, Error)]
#[error("Attempted to parse an invalid Host.")]
pub struct InvalidHost;

impl From<InvalidFileHost          > for InvalidHost {fn from(_: InvalidFileHost          ) -> Self {Self}}
impl From<InvalidSpecialNotFileHost> for InvalidHost {fn from(_: InvalidSpecialNotFileHost) -> Self {Self}}
impl From<InvalidNonSpecialHost    > for InvalidHost {fn from(_: InvalidNonSpecialHost    ) -> Self {Self}}

impl From<InvalidDomainHost> for InvalidHost {fn from(_: InvalidDomainHost) -> Self {Self}}
impl From<InvalidIpv4Host  > for InvalidHost {fn from(_: InvalidIpv4Host  ) -> Self {Self}}
impl From<InvalidIpv6Host  > for InvalidHost {fn from(_: InvalidIpv6Host  ) -> Self {Self}}
impl From<InvalidIpHost    > for InvalidHost {fn from(_: InvalidIpHost    ) -> Self {Self}}
impl From<InvalidOpaqueHost> for InvalidHost {fn from(_: InvalidOpaqueHost) -> Self {Self}}
impl From<InvalidEmptyHost > for InvalidHost {fn from(_: InvalidEmptyHost ) -> Self {Self}}
