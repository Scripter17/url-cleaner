//! [`Host`].

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::prelude::*;

/// A host.
/// # Examples
/// ```
/// use better_url::prelude::*;
///
/// assert_eq!(Host::new_special_not_file("example.com"  ).unwrap().domain().unwrap(), "example.com");
///
/// // Accepts percent encoded domain/IPv4 hosts.
/// assert_eq!(Host::new_special_not_file("example%2Ecom").unwrap().domain().unwrap(), "example.com");
///
/// // Accepts the stupid bullshit the IPv4 host parser accepts.
/// assert_eq!(Host::new_special_not_file("0x12.034"     ).unwrap().ipv4  ().unwrap(), "18.0.0.28"  );
/// ```
#[derive(Debug, Clone)]
pub enum Host<'a> {
    /** [`DomainHost`]. **/ Domain(DomainHost<'a>),
    /** [`Ipv4Host`].   **/ Ipv4  (Ipv4Host  <'a>),
    /** [`Ipv6Host`].   **/ Ipv6  (Ipv6Host  <'a>),
    /** [`OpaqueHost`]. **/ Opaque(OpaqueHost<'a>),
    /** [`EmptyHost`].  **/ Empty (EmptyHost <'a>),
}

impl<'a> Host<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be a valid [`Self`] literal and `details` must be its details.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: T, details: HostDetails) -> Self {
        unsafe {
            match details {
                HostDetails::Domain(x) => DomainHost::new_unchecked(value, x).into(),
                HostDetails::Ipv4  (x) => Ipv4Host  ::new_unchecked(value, x).into(),
                HostDetails::Ipv6  (x) => Ipv6Host  ::new_unchecked(value, x).into(),
                HostDetails::Opaque(x) => OpaqueHost::new_unchecked(value, x).into(),
                HostDetails::Empty (x) => EmptyHost ::new_unchecked(value, x).into(),
            }
        }
    }

    /// Make a new [`Self`] for the specified [`SchemeType`].
    /// # Errors
    /// If the call to [`Self::new_file`] returns an error, returns the error [`InvalidHost`].
    ///
    /// If the call to [`Self::new_special_not_file`] returns an error, returns the error [`InvalidHost`].
    ///
    /// If the call to [`Self::new_non_special`] returns an error, returns the error [`InvalidHost`].
    pub fn new<T: TryInto<FileHost<'a>> + TryInto<SpecialNotFileHost<'a>> + TryInto<NonSpecialHost<'a>>>(value: T, scheme_type: SchemeType) -> Result<Self, InvalidHost>
        where InvalidHost: From<<T as TryInto<FileHost<'a>>>::Error> + From<<T as TryInto<SpecialNotFileHost<'a>>>::Error> + From<<T as TryInto<NonSpecialHost<'a>>>::Error>
    {
        Ok(match scheme_type {
            SchemeType::File           => Self::new_file            (value)?,
            SchemeType::SpecialNotFile => Self::new_special_not_file(value)?,
            SchemeType::NonSpecial     => Self::new_non_special     (value)?,
        })
    }

    /// Make from a new [`FileHost`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_file<T: TryInto<FileHost<'a>>>(value: T) -> Result<Self, T::Error> {
        Ok(value.try_into()?.into())
    }

    /// Make from a new [`SpecialNotFileHost`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_special_not_file<T: TryInto<SpecialNotFileHost<'a>>>(value: T) -> Result<Self, T::Error> {
        Ok(value.try_into()?.into())
    }

    /// Make from a new [`NonSpecialHost`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_non_special<T: TryInto<NonSpecialHost<'a>>>(value: T) -> Result<Self, T::Error> {
        Ok(value.try_into()?.into())
    }



    /// Make a new [`Self::Domain`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_domain<T: TryInto<DomainHost<'a>>>(value: T) -> Result<Self, T::Error> {
        value.try_into().map(Into::into)
    }

    /// Make a new [`Self::Ipv4`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_ipv4<T: TryInto<DomainHost<'a>>>(value: T) -> Result<Self, T::Error> {
        value.try_into().map(Into::into)
    }

    /// Make a new [`Self::Ipv6`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_ipv6<T: TryInto<Ipv6Host<'a>>>(value: T) -> Result<Self, T::Error> {
        value.try_into().map(Into::into)
    }

    /// Make a new [`Self::Ipv4`] or [`Self::Ipv6`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_ip<T: TryInto<IpHost<'a>>>(value: T) -> Result<Self, T::Error> {
        value.try_into().map(Into::into)
    }

    /// Make a new [`Self::Opaque`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_opaque<T: TryInto<OpaqueHost<'a>>>(value: T) -> Result<Self, T::Error> {
        value.try_into().map(Into::into)
    }

    /// Make a new [`Self::Empty`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new_empty<T: TryInto<EmptyHost<'a>>>(value: T) -> Result<Self, T::Error> {
        value.try_into().map(Into::into)
    }

    /// The host.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Domain(x) => x.as_str(),
            Self::Ipv4  (x) => x.as_str(),
            Self::Ipv6  (x) => x.as_str(),
            Self::Opaque(x) => x.as_str(),
            Self::Empty (x) => x.as_str(),
        }
    }

    /// The [`HostDetails`].
    pub fn details(&self) -> HostDetails {
        match self {
            Self::Domain(x) => x.details().into(),
            Self::Ipv4  (x) => x.details().into(),
            Self::Ipv6  (x) => x.details().into(),
            Self::Opaque(x) => x.details().into(),
            Self::Empty (x) => x.details().into(),
        }
    }

    /// Turn into the inner [`Cow`] and [`HostDetails`].
    pub fn into_parts(self) -> (Cow<'a, str>, HostDetails) {
        match self {
            Self::Domain(x) => {let (host, details) = x.into_parts(); (host, details.into())},
            Self::Ipv4  (x) => {let (host, details) = x.into_parts(); (host, details.into())},
            Self::Ipv6  (x) => {let (host, details) = x.into_parts(); (host, details.into())},
            Self::Opaque(x) => {let (host, details) = x.into_parts(); (host, details.into())},
            Self::Empty (x) => {let (host, details) = x.into_parts(); (host, details.into())},
        }
    }



    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Host<'static> {
        match self {
            Self::Domain(x) => x.into_owned().into(),
            Self::Ipv4  (x) => x.into_owned().into(),
            Self::Ipv6  (x) => x.into_owned().into(),
            Self::Opaque(x) => x.into_owned().into(),
            Self::Empty (x) => x.into_owned().into(),
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Host<'_> {
        match self {
            Self::Domain(x) => x.borrowed().into(),
            Self::Ipv4  (x) => x.borrowed().into(),
            Self::Ipv6  (x) => x.borrowed().into(),
            Self::Opaque(x) => x.borrowed().into(),
            Self::Empty (x) => x.borrowed().into(),
        }
    }



    /** If it's [`Self::Domain`]. **/ pub fn is_domain(&self) -> bool {matches!(self, Self::Domain(_))}
    /** If it's [`Self::Ipv4`].   **/ pub fn is_ipv4  (&self) -> bool {matches!(self, Self::Ipv4  (_))}
    /** If it's [`Self::Ipv6`].   **/ pub fn is_ipv6  (&self) -> bool {matches!(self, Self::Ipv6  (_))}
    /** If it's [`Self::Opaque`]. **/ pub fn is_opaque(&self) -> bool {matches!(self, Self::Opaque(_))}

    // [`Self::is_empty`] is added by [`as_str_impls`].



    /** Borrow the [`DomainHost`]. **/ pub fn as_domain(&self) -> Option<DomainHost<'_>> {self.borrowed().domain()}
    /** Borrow the [`Ipv4Host`].   **/ pub fn as_ipv4  (&self) -> Option<Ipv4Host  <'_>> {self.borrowed().ipv4  ()}
    /** Borrow the [`Ipv6Host`].   **/ pub fn as_ipv6  (&self) -> Option<Ipv6Host  <'_>> {self.borrowed().ipv6  ()}
    /** Borrow the [`OpaqueHost`]. **/ pub fn as_opaque(&self) -> Option<OpaqueHost<'_>> {self.borrowed().opaque()}
    /** Borrow the [`EmptyHost`].  **/ pub fn as_empty (&self) -> Option<EmptyHost <'_>> {self.borrowed().empty ()}


    /** The [`DomainHost`]. **/ pub fn domain(self) -> Option<DomainHost<'a>> {self.try_into().ok()}
    /** The [`Ipv4Host`].   **/ pub fn ipv4  (self) -> Option<Ipv4Host  <'a>> {self.try_into().ok()}
    /** The [`Ipv6Host`].   **/ pub fn ipv6  (self) -> Option<Ipv6Host  <'a>> {self.try_into().ok()}
    /** The [`OpaqueHost`]. **/ pub fn opaque(self) -> Option<OpaqueHost<'a>> {self.try_into().ok()}
    /** The [`EmptyHost`].  **/ pub fn empty (self) -> Option<EmptyHost <'a>> {self.try_into().ok()}


    /** The [`DomainHost::prefix_str`]. **/ pub fn domain_prefix_str(&self) -> Option<&str> {Some(&self.as_str()[self.as_domain()?.details().prefix_range()?])}
    /** The [`DomainHost::middle_str`]. **/ pub fn domain_middle_str(&self) -> Option<&str> {Some(&self.as_str()[self.as_domain()?.details().middle_range()?])}
    /** The [`DomainHost::suffix_str`]. **/ pub fn domain_suffix_str(&self) -> Option<&str> {Some(&self.as_str()[self.as_domain()?.details().suffix_range() ])}
    /** The [`DomainHost::labels_str`]. **/ pub fn domain_labels_str(&self) -> Option<&str> {Some(&self.as_str()[self.as_domain()?.details().labels_range() ])}
    /** The [`DomainHost::origin_str`]. **/ pub fn domain_origin_str(&self) -> Option<&str> {Some(&self.as_str()[self.as_domain()?.details().origin_range()?])}
    /** The [`DomainHost::normal_str`]. **/ pub fn domain_normal_str(&self) -> Option<&str> {Some(&self.as_str()[self.as_domain()?.details().normal_range() ])}

    /** The [`DomainHost::prefix`]. **/ pub fn domain_prefix(&self) -> Option<DomainSegments<'_>> {Some(DomainSegments(self.domain_prefix_str()?.into()))}
    /** The [`DomainHost::middle`]. **/ pub fn domain_middle(&self) -> Option<DomainSegment <'_>> {Some(DomainSegment (self.domain_middle_str()?.into()))}
    /** The [`DomainHost::suffix`]. **/ pub fn domain_suffix(&self) -> Option<DomainSegments<'_>> {Some(DomainSegments(self.domain_suffix_str()?.into()))}
    /** The [`DomainHost::labels`]. **/ pub fn domain_labels(&self) -> Option<DomainSegments<'_>> {Some(DomainSegments(self.domain_labels_str()?.into()))}
    /** The [`DomainHost::origin`]. **/ pub fn domain_origin(&self) -> Option<DomainSegments<'_>> {Some(DomainSegments(self.domain_origin_str()?.into()))}
    /** The [`DomainHost::normal`]. **/ pub fn domain_normal(&self) -> Option<DomainSegments<'_>> {Some(DomainSegments(self.domain_normal_str()?.into()))}
}

impl<'a> From<DomainHost<'a>> for Host<'a> {fn from(value: DomainHost<'a>) -> Self {Self::Domain(value)}}
impl<'a> From<Ipv4Host  <'a>> for Host<'a> {fn from(value: Ipv4Host  <'a>) -> Self {Self::Ipv4  (value)}}
impl<'a> From<Ipv6Host  <'a>> for Host<'a> {fn from(value: Ipv6Host  <'a>) -> Self {Self::Ipv6  (value)}}
impl<'a> From<OpaqueHost<'a>> for Host<'a> {fn from(value: OpaqueHost<'a>) -> Self {Self::Opaque(value)}}
impl<'a> From<EmptyHost <'a>> for Host<'a> {fn from(value: EmptyHost <'a>) -> Self {Self::Empty (value)}}

impl<'a> From<IpHost<'a>> for Host<'a> {
    fn from(value: IpHost<'a>) -> Self {
        match value {
            IpHost::V4(x) => x.into(),
            IpHost::V6(x) => x.into(),
        }
    }
}

impl<'a> From<FileHost<'a>> for Host<'a> {
    fn from(value: FileHost<'a>) -> Self {
        match value {
            FileHost::Domain(x) => x.into(),
            FileHost::Ipv4  (x) => x.into(),
            FileHost::Ipv6  (x) => x.into(),
            FileHost::Empty (x) => x.into(),
        }
    }
}

impl<'a> From<SpecialNotFileHost<'a>> for Host<'a> {
    fn from(value: SpecialNotFileHost<'a>) -> Self {
        match value {
            SpecialNotFileHost::Domain(x) => x.into(),
            SpecialNotFileHost::Ipv4  (x) => x.into(),
            SpecialNotFileHost::Ipv6  (x) => x.into(),
        }
    }
}

impl<'a> From<NonSpecialHost<'a>> for Host<'a> {
    fn from(value: NonSpecialHost<'a>) -> Self {
        match value {
            NonSpecialHost::Ipv6  (x) => x.into(),
            NonSpecialHost::Opaque(x) => x.into(),
            NonSpecialHost::Empty (x) => x.into(),
        }
    }
}

impl From<Ipv4Addr> for Host<'static> {fn from(value: Ipv4Addr) -> Self {Self::Ipv4(value.into())}}
impl From<Ipv6Addr> for Host<'static> {fn from(value: Ipv6Addr) -> Self {Self::Ipv6(value.into())}}

impl From<IpAddr> for Host<'static> {
    fn from(value: IpAddr) -> Self {
        match value {
            IpAddr::V4(x) => x.into(),
            IpAddr::V6(x) => x.into(),
        }
    }
}
