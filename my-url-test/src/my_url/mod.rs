//! [`MyUrl`].

use crate::prelude::*;

mod new;
mod join;

mod canon;
mod scheme;
mod userinfo;
mod host;
mod host_port;
mod port;
mod path;
mod query;
mod fragment;

/// The enum of errors that can occur when trying to parse an invalid URL.
#[derive(Debug, Error)]
pub enum InvalidUrl {
    /// [`InvalidScheme`].
    #[error(transparent)]
    InvalidScheme(#[from] InvalidScheme),
    /// [`InvalidHost`].
    #[error(transparent)]
    InvalidHost(#[from] InvalidHost),
    /// [`InvalidPort`].
    #[error(transparent)]
    InvalidPort(#[from] InvalidPort),
    /// [`TooLong`].
    #[error(transparent)]
    TooLong(#[from] TooLong),
    /// Returned when attempting to parse a URL with no schene.
    #[error("Attempted to parse a URL with no scheme.")]
    MissingScheme,
    /// Returned when attempting to parse a URL with an empty host and a userinfo and/or port.
    #[error("Attempted to parse a URL with an empty host and a userinfo and/or port.")]
    EmptyHostCantHaveUserinfoOrPort,
}

/// [`MyUrl::join`].
#[derive(Debug, Error)]
pub enum InvalidJoin {
    /** [`InvalidUrl`].       **/ #[error(transparent)] InvalidUrl      (#[from] InvalidUrl       ),
    /** [`InvalidScheme`].    **/ #[error(transparent)] InvalidScheme   (#[from] InvalidScheme    ),
    /** [`SetPathError`].     **/ #[error(transparent)] SetPathError    (#[from] SetPathError     ),
    /** [`SetQueryError`].    **/ #[error(transparent)] SetQueryError   (#[from] SetQueryError    ),
    /** [`SetFragmentError`]. **/ #[error(transparent)] SetFragmentError(#[from] SetFragmentError ),
    /** [`CannotBeABase`].    **/ #[error(transparent)] CannotBeABase   (#[from] CannotBeABase    ),
    /// Returned when attempting to join a non-relative URL with something other than just a fragment or a whole URL.
    #[error("Attempted to join a non-relative URL with something other than a fragment or a whole URL.")]
    MissingSchemeNonRelativeUrl
}

/// Returned when attempting to join more than the fragment on a cannot-be-a-base URL.
#[derive(Debug, Error)]
#[error("Attempted to join more than the fragment on a cannot-be-a-base URL.")]
pub struct CannotBeABase;

/// A URL type decoupled from [`url`].
#[derive(Debug, Clone)]
pub struct MyUrl {
    /// The serialization.
    serialization : String,
    /// The end of the scheme.
    scheme_mark  : u32,
    /// The end of the username.
    username_after: Option<NonZero<u32>>,
    /// The start of the host.
    host_start    : Option<NonZero<u32>>,
    /// The `:` marking the port.
    port_mark     : Option<NonZero<u32>>,
    /// If [`Self::port_mark`] is [`Some`], the port.
    port          : u16,
    /// The start of the path.
    path_start    : u32,
    /// The `?` marking the query.
    query_mark    : Option<NonZero<u32>>,
    /// The `#` marking the fragment.
    fragment_mark : Option<NonZero<u32>>,
    /// The [`UrlDetails`].
    details       : UrlDetails,
}

impl MyUrl {
    /// The URL as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.serialization
    }

    /// If the URL cannot be a base.
    pub fn cannot_be_a_base(&self) -> bool {
        !self.serialization[self.scheme_mark as usize..].starts_with(":/")
    }

    /// The length.
    pub fn len(&self) -> usize {
        self.serialization.len()
    }
}
