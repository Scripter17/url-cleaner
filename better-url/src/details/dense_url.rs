//! [`DenseUrlDetails`].

use crate::prelude::*;

/// A [`UrlDetails`] that optimizes for space.
///
/// Specifically, instead of storing a [`HostDetails`], this stores a [`HostType`] and [`HostData`].
///
/// This lets [`Self::scheme`] and [`Self::port`] live in the 3 bytes of padding in [`UrlDetails::host`].
///
/// This then saves 1 more byte of padding, making this 4 bytes smaller than [`UrlDetails`].
///
/// However, as of Rust 1.98.1, the codegen of [`Self::host_details`] is significantly worse than I think it should be.
///
/// [`Self::domain_details`] and whatnot are still fast.
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// assert_eq!(std::mem::size_of::<DenseUrlDetails>(), 48);
/// assert_eq!(std::mem::size_of::<     UrlDetails>(), 52);
/// ```
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "UrlDetails", into = "UrlDetails"))]
pub struct DenseUrlDetails {
    /** The `:` marking the scheme.        **/ pub scheme_mark   : u32                 ,
    /** The `:` or `@` after the username. **/ pub username_after: Option<NonZero<u32>>,
    /** The start of the host.             **/ pub host_start    : Option<NonZero<u32>>,
    /** The `:` marking the port.          **/ pub port_mark     : Option<NonZero<u32>>,
    /** The start of the path.             **/ pub path_start    : u32                 ,
    /** The `?` marking the query.         **/ pub query_mark    : Option<NonZero<u32>>,
    /** The `#` marking the fragment.      **/ pub fragment_mark : Option<NonZero<u32>>,
    /** The [`SchemeDetails`].             **/ pub scheme        : SchemeDetails       ,
    /** The [`HostType`].                  **/     host_type     : Option<HostType>    ,
    /** The [`HostData`].                  **/     host_data     : HostData            ,
    /** The port.                          **/ pub port          : u16                 ,
}

/// The variant of a [`HostDetails`].
///
/// Used in [`DenseUrlDetails`] to avoid 4 bytes of padding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HostType {
    /** [`HostDetails::Domain`]. **/ Domain,
    /** [`HostDetails::Ipv4`].   **/ Ipv4  ,
    /** [`HostDetails::Ipv6`].   **/ Ipv6  ,
    /** [`HostDetails::Opaque`]. **/ Opaque,
    /** [`HostDetails::Empty`].  **/ Empty ,
}

/// The data for a [`HostDetails`].
///
/// Used in [`DenseUrlDetails`] to avoid 4 bytes of padding.
#[derive(Clone, Copy)]
pub union HostData {
    /** [`DomainHostDetails`]. **/ pub domain: DomainHostDetails,
    /** [`Ipv4HostDetails`].   **/ pub ipv4  : Ipv4HostDetails  ,
    /** [`Ipv6HostDetails`].   **/ pub ipv6  : Ipv6HostDetails  ,
    /** [`OpaqueHostDetails`]. **/ pub opaque: OpaqueHostDetails,
    /** [`EmptyHostDetails`].  **/ pub empty : EmptyHostDetails ,
}

impl std::fmt::Debug for HostData {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("HostData")
    }
}

impl DenseUrlDetails {
    /// The [`HostType`].
    pub fn host_type(self) -> Option<HostType> {
        self.host_type
    }

    /// The [`HostData`].
    pub fn host_data(self) -> HostData {
        self.host_data
    }

    /// The [`HostDetails`].
    pub fn host_details(self) -> Option<HostDetails> {
        unsafe {
            HostDetails::from_option_parts(self.host_type, self.host_data)
        }
    }

    /// Set [`Self::host_type`] and [`Self::host_data`].
    pub fn set_host_details(&mut self, details: Option<HostDetails>) {
        let (host_type, host_data) = HostDetails::into_option_parts(details);

        self.host_type = host_type;
        self.host_data = host_data;
    }

    /** The [`DomainHostDetails`]. **/ pub fn domain_details(self) -> Option<DomainHostDetails> {(self.host_type == Some(HostType::Domain)).then_some(unsafe {self.host_data.domain})}
    /** The [`Ipv4HostDetails`].   **/ pub fn ipv4_details  (self) -> Option<Ipv4HostDetails  > {(self.host_type == Some(HostType::Ipv4  )).then_some(unsafe {self.host_data.ipv4  })}
    /** The [`Ipv6HostDetails`].   **/ pub fn ipv6_details  (self) -> Option<Ipv6HostDetails  > {(self.host_type == Some(HostType::Ipv6  )).then_some(unsafe {self.host_data.ipv6  })}
    /** The [`OpaqueHostDetails`]. **/ pub fn opaque_details(self) -> Option<OpaqueHostDetails> {(self.host_type == Some(HostType::Opaque)).then_some(unsafe {self.host_data.opaque})}
    /** The [`EmptyHostDetails`].  **/ pub fn empty_details (self) -> Option<EmptyHostDetails > {(self.host_type == Some(HostType::Empty )).then_some(unsafe {self.host_data.empty })}

    /** If the host is [`DomainHost`]. **/ pub fn host_is_domain(&self) -> bool {self.host_type == Some(HostType::Domain)}
    /** If the host is [`Ipv4Host`].   **/ pub fn host_is_ipv4  (&self) -> bool {self.host_type == Some(HostType::Ipv4  )}
    /** If the host is [`Ipv6Host`].   **/ pub fn host_is_ipv6  (&self) -> bool {self.host_type == Some(HostType::Ipv6  )}
    /** If the host is [`OpaqueHost`]. **/ pub fn host_is_opaque(&self) -> bool {self.host_type == Some(HostType::Opaque)}
    /** If the host is [`EmptyHost`].  **/ pub fn host_is_empty (&self) -> bool {self.host_type == Some(HostType::Empty )}

    /// If the host is [`Ipv4Host`] or [`Ipv6Host`].
    pub fn host_is_ip(&self) -> bool {
        matches!(self.host_type, Some(HostType::Ipv4 | HostType::Ipv6))
    }
}

impl std::fmt::Debug for DenseUrlDetails {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DenseUrlDetails")
            .field("scheme_mark"   , &self.scheme_mark   )
            .field("username_after", &self.username_after)
            .field("host_start"    , &self.host_start    )
            .field("port_mark"     , &self.port_mark     )
            .field("path_start"    , &self.path_start    )
            .field("query_mark"    , &self.query_mark    )
            .field("fragment_mark" , &self.fragment_mark )
            .field("scheme"        , &self.scheme        )
            .field("host"          , &self.host_details())
            .field("port"          , &self.port          )
            .finish()
    }
}

impl From<UrlDetails> for DenseUrlDetails {
    fn from(value: UrlDetails) -> Self {
        let (host_type, host_data) = HostDetails::into_option_parts(value.host);

        Self {
            scheme_mark   : value.scheme_mark   ,
            username_after: value.username_after,
            host_start    : value.host_start    ,
            port_mark     : value.port_mark     ,
            path_start    : value.path_start    ,
            query_mark    : value.query_mark    ,
            fragment_mark : value.fragment_mark ,
            scheme        : value.scheme        ,
            host_type,
            host_data,
            port          : value.port          ,
        }
    }
}

impl From<DenseUrlDetails> for UrlDetails {
    fn from(value: DenseUrlDetails) -> Self {
        let host = unsafe {HostDetails::from_option_parts(value.host_type, value.host_data)};

        Self {
            scheme_mark   : value.scheme_mark   ,
            username_after: value.username_after,
            host_start    : value.host_start    ,
            port_mark     : value.port_mark     ,
            path_start    : value.path_start    ,
            query_mark    : value.query_mark    ,
            fragment_mark : value.fragment_mark ,
            scheme        : value.scheme        ,
            host,
            port          : value.port          ,
        }
    }
}
