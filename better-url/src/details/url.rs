//! [`UrlDetails`].

use crate::prelude::*;

/// The details of a [`BetterUrl`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UrlDetails {
    /** The `:` marking the scheme.                   **/ pub scheme_mark   : u32,
    /** The character after the username.             **/ pub username_after: Option<NonZero<u32>>,
    /** The start of the host.                        **/ pub host_start    : Option<NonZero<u32>>,
    /** The `:` marking the port.                     **/ pub port_mark     : Option<NonZero<u32>>,
    /** The start of the path.                        **/ pub path_start    : u32,
    /** The `?` marking the query.                    **/ pub query_mark    : Option<NonZero<u32>>,
    /** The `#` marking the fragment.                 **/ pub fragment_mark : Option<NonZero<u32>>,
    /** The [`SchemeDetails`].                        **/ pub scheme        : SchemeDetails,
    /** The [`HostDetails`].                          **/ pub host          : Option<HostDetails>,
    /** If [`Self::port_mark`] is [`Some`], the port. **/ pub port          : u16,
}
