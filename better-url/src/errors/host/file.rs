//! [`FileHost`].

use thiserror::Error;

use crate::prelude::*;

/// Returned when attempting to parse an invalid [`FileHost`].
#[derive(Debug, Error)]
#[error("Attempted to parse an invalid FileHost.")]
pub struct InvalidFileHost;

impl From<InvalidDomainHost> for InvalidFileHost {fn from(_: InvalidDomainHost) -> Self {Self}}
impl From<InvalidIpv4Host  > for InvalidFileHost {fn from(_: InvalidIpv4Host  ) -> Self {Self}}
impl From<InvalidIpv6Host  > for InvalidFileHost {fn from(_: InvalidIpv6Host  ) -> Self {Self}}
impl From<InvalidIpHost    > for InvalidFileHost {fn from(_: InvalidIpHost    ) -> Self {Self}}
impl From<InvalidEmptyHost > for InvalidFileHost {fn from(_: InvalidEmptyHost ) -> Self {Self}}
