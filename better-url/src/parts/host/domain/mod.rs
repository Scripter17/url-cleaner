//! [`DomainHost`].

use crate::prelude::*;

mod segment;
mod segments;

pub use segment::*;
pub use segments::*;

#[cfg(feature = "psl")]
mod prefix;
#[cfg(feature = "psl")]
mod middle;
#[cfg(feature = "psl")]
mod suffix;
mod fqddot;
mod labels;
#[cfg(feature = "psl")]
mod origin;
#[cfg(feature = "psl")]
mod normal;

/// A domain host.
///
/// # Parts
///
/// There are 6 parts:
///
/// ```
/// use better_url::prelude::*;
///
/// #[cfg(feature = "psl")]
/// {
///     // The prefix, the segments below the middle.
///     assert_eq!(DomainHost::new("abc.def.example.co.uk" ).unwrap().prefix_str(), Some("abc.def"                ));
///     assert_eq!(DomainHost::new("abc.def.example.co.uk.").unwrap().prefix_str(), Some("abc.def"                ));
///
///     // The middle, the segment just below the suffix.
///     assert_eq!(DomainHost::new("abc.def.example.co.uk" ).unwrap().middle_str(), Some(        "example"        ));
///     assert_eq!(DomainHost::new("abc.def.example.co.uk.").unwrap().middle_str(), Some(        "example"        ));
///
///     // The suffix, as defined by the PSL.
///     assert_eq!(DomainHost::new("abc.def.example.co.uk" ).unwrap().suffix_str(),                      "co.uk"   );
///     assert_eq!(DomainHost::new("abc.def.example.co.uk.").unwrap().suffix_str(),                      "co.uk"   );
///
///     // The labels (better name pending), everything but the FQDDot.
///     assert_eq!(DomainHost::new(                     ".").unwrap().labels_str(),                      "");
///     assert_eq!(DomainHost::new(                "co.uk.").unwrap().labels_str(),                 "co.uk");
///     assert_eq!(DomainHost::new(        "example.co.uk.").unwrap().labels_str(),         "example.co.uk");
///     assert_eq!(DomainHost::new(    "www.example.co.uk.").unwrap().labels_str(),     "www.example.co.uk");
///     assert_eq!(DomainHost::new("abc.def.example.co.uk.").unwrap().labels_str(), "abc.def.example.co.uk");
///
///     // The origin, the suffix and the middle.
///     // Also called the registerable domain but that doesn't have a 6 letter equivalent.
///     assert_eq!(DomainHost::new(                "co.uk.").unwrap().origin_str(), None                 );
///     assert_eq!(DomainHost::new(        "example.co.uk.").unwrap().origin_str(), Some("example.co.uk"));
///     assert_eq!(DomainHost::new("abc.def.example.co.uk.").unwrap().origin_str(), Some("example.co.uk"));
///
///     // The normal, the labels (better name still pending), excluding the prefix if it's `www`.
///     assert_eq!(DomainHost::new(                     ".").unwrap().normal_str(),                      "");
///     assert_eq!(DomainHost::new(                "co.uk.").unwrap().normal_str(),                 "co.uk");
///     assert_eq!(DomainHost::new(        "example.co.uk.").unwrap().normal_str(),         "example.co.uk");
///     assert_eq!(DomainHost::new(    "www.example.co.uk.").unwrap().normal_str(),         "example.co.uk");
///     assert_eq!(DomainHost::new("abc.def.example.co.uk.").unwrap().normal_str(), "abc.def.example.co.uk");
/// }
/// ```
///
/// # Setters
///
/// For performance reasons, setters don't check that the part and value match.
///
/// ```
/// use better_url::prelude::*;
///
/// #[cfg(feature = "psl")]
/// {
///     let mut domain = DomainHost::new("example.com").unwrap();
///
///     domain.set_suffix(Some("example.com")).unwrap();
///
///     assert_eq!(domain         , "example.example.com");
///     assert_eq!(domain.suffix(),                 "com");
/// }
/// ```
///
/// However, they do ensure that the result is a valid domain host.
///
/// ```
/// use better_url::prelude::*;
///
/// #[cfg(feature = "psl")]
/// {
///     // Domains can't "end in a number".
///     DomainHost::new("example.com" ).unwrap().set_suffix(Some("123")).unwrap_err();
///     DomainHost::new("example.com.").unwrap().set_suffix(Some("123")).unwrap_err();
///
///     // Unicode segments are IDNA'd.
///     let mut domain = DomainHost::new("example.com").unwrap();
///     domain.set_suffix(Some("δοκιμή")).unwrap();
///     assert_eq!(domain, "example.xn--jxalpdlp");
///
///     // IDNA segment literals are accepted.
///     let mut domain = DomainHost::new("example.com").unwrap();
///     domain.set_suffix(Some("xn--jxalpdlp")).unwrap();
///     assert_eq!(domain, "example.xn--jxalpdlp");
/// }
/// ```
///
/// Additionally, attempting to set the suffix of a non-FQDN domain to end in an empty segment returns an error.
///
/// ```
/// use better_url::prelude::*;
///
/// #[cfg(feature = "psl")]
/// {
///     DomainHost::new("example.com").unwrap().set_suffix(Some(    "")).unwrap_err();
///     DomainHost::new("example.com").unwrap().set_suffix(Some(   ".")).unwrap_err();
///     DomainHost::new("example.com").unwrap().set_suffix(Some("com.")).unwrap_err();
///
///     DomainHost::new("example..com").unwrap().set_suffix(None::<&str>).unwrap_err();
///
///     let mut domain = DomainHost::new("example.com.").unwrap();
///     domain.set_suffix(Some("com.")).unwrap();
///     assert_eq!(domain, "example.com..");
/// }
/// ```
///
/// This is to ensure that if you have a domain with X segments and replace a range of Y segments with Z segments (and the setter returns [`Ok`]), you always end up with a domain with X-Y+Z segments.
#[derive(Debug, Clone)]
pub struct DomainHost<'a> {
    /// The host string.
    host: Cow<'a, str>,
    /// The [`DomainHostDetails`].
    details: DomainHostDetails,
}

impl<'a> DomainHost<'a> {
    /// Make a new [`Self`] from a percent decoded value.
    /// # Safety
    /// Requires that `value` be percent decoded.
    /// # Errors
    /// If [`Self::new_normalized`] returns an error, that error is returned.
    pub unsafe fn new_percent_decoded<T: Into<Cow<'a, str>>>(value: T) -> Result<Self, InvalidDomainHost> {
        let (_, value, class) = uts46_classify_map_normalize(value);

        Ok(if class & 0b0001_0100 == 0b0001_0000 {
            Err(InvalidDomainHost)?
        } else if class & 0b0000_0110 == 0b0000_0000 && !value.is_empty() {
            unsafe {Self::new_raw(value)}
        } else {
            unsafe {Self::new_not_eian(value)}?
        })
    }

    /// Make a new [`Self`] from a percent decoded and UTS46 map normalized value.
    /// # Safety
    /// Requires that `value` be percent decoded and UTS46 map normalized.
    /// # Errors
    /// If [`ends_in_a_number`] returns [`true`], returns the error [`InvalidDomainHost`].
    pub unsafe fn new_normalized<T: Into<Cow<'a, str>>>(value: T) -> Result<Self, InvalidDomainHost> {
        let value = value.into();

        if ends_in_a_number(&value) {
            Err(InvalidDomainHost)?;
        }

        unsafe {
            Self::new_not_eian(value)
        }
    }

    /// Make a new [`Self`] from a percent decoded and UTS46 map normalized value that does not end in a number.
    /// # Safety
    /// Requires that `value` be percent decoded and UTS46 map normalized and does not end in a number.
    /// # Errors
    /// If [`uts46_domain_host_to_ascii`] returns an error, that error is returned.
    ///
    /// If the UTS46 mapped and normalized value contains a byte in [`FORBIDDEN_DOMAIN_HOST_LITERAL`], returns the error [`InvalidDomainHost`].
    ///
    /// If the UTS46 mapped and normalized value is empty, returns the error [`InvalidDomainHost`].
    pub unsafe fn new_not_eian<T: Into<Cow<'a, str>>>(value: T) -> Result<Self, InvalidDomainHost> {
        let (raw, ascii) = uts46_domain_host_to_ascii(value, false)?;

        let value = ascii.map_or(raw, Into::into);

        if value.bytes().any(|b| FORBIDDEN_DOMAIN_HOST_LITERAL.contains(b)) {
            Err(InvalidDomainSegment)?;
        }

        if value.is_empty() {
            Err(InvalidDomainHost)?
        }

        unsafe {
            Ok(Self::new_raw(value))
        }
    }

    /// Make a new [`Self`] from a percent decoded, UTS46 map normalized, and DomainToASCIIi'd value.
    /// # Safety
    /// Requires that `value` be percent decoded, UTS46 map normalized, and DomainToASCII'd.
    pub unsafe fn new_raw<T: Into<Cow<'a, str>>>(value: T) -> Self {
        let value = value.into();

        let details = DomainHostDetails::parse_unchecked(&value);

        unsafe {
            Self::from_parts(value, details)
        }
    }

    /// Make a new [`Self`] with zero validity checks.
    /// # Safety
    /// `value` must be a valid domain literal and `details` must be its [`DomainHostDetails`].
    pub unsafe fn from_parts<T: Into<Cow<'a, str>>>(value: T, details: DomainHostDetails) -> Self {
        Self {
            host: value.into(),
            details
        }
    }

    /// The host as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.host
    }

    /// The [`DomainHostDetails`].
    pub fn details(&self) -> DomainHostDetails {
        self.details
    }

    /// Unwrap into the inner [`Cow`].
    ///
    /// To get the [`DomainHostDetails`] too, see [`Self::into_parts`].
    pub fn into_inner(self) -> Cow<'a, str> {
        self.host
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (Cow<'a, str>, DomainHostDetails) {
        (self.host, self.details)
    }

    /// [`uts46_domain_host_to_unicode`].
    /// # Errors
    /// If [`uts46_domain_host_to_unicode`] returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "It's fine.")]
    pub fn into_unicode(self) -> Result<(Cow<'a, str>, Option<String>, BidiDetail), (Cow<'a, str>, Option<String>)> {
        uts46_domain_host_to_unicode(self.into_inner())
    }

    /// [`Self::into_unicode`] without taking ownership.
    /// # Errors
    /// If [`uts46_domain_host_to_unicode`] returns an error, that error is returned.
    #[expect(clippy::type_complexity, reason = "It's fine.")]
    pub fn to_unicode(&self) -> Result<(Cow<'_, str>, Option<String>, BidiDetail), (Cow<'_, str>, Option<String>)> {
        uts46_domain_host_to_unicode(self.as_str())
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> DomainHost<'static> {
        DomainHost {
            host   : self.host.into_owned().into(),
            details: self.details
        }
    }

    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> DomainHost<'_> {
        DomainHost {
            host   : Cow::Borrowed(&self.host),
            details: self.details,
        }
    }



    /// Shorthand for [`Self::labels_segments`].
    pub fn segments(&self) -> DomainSegmentsIter<'_> {
        self.labels_segments()
    }

    /// Shorthand for [`Self::labels_segment`].
    pub fn segment(&self, index: isize) -> Option<DomainSegment<'_>> {
        self.labels_segment(index)
    }

    /// Shorthand for [`Self::labels_range`].
    pub fn range<B: RangeBounds<isize>>(&self, range: B) -> Option<DomainSegments<'_>> {
        self.labels_range(range)
    }



    /// Shorthand for [`Self::set_labels_segment`].
    /// # Errors
    /// If [`Self::set_labels_segment`] returns an error, that error is returned.
    pub fn set_segment<'b, T: TryInto<DomainSegment<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        self.set_labels_segment(index, value)
    }

    /// Shorthand for [`Self::set_labels_range`].
    /// # Errors
    /// If [`Self::set_labels_range`] returns an error, that error is returned.
    pub fn set_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        self.set_labels_range(range, value)
    }

    /// Shorthand for [`Self::insert_labels_segment`].
    /// # Errors
    /// If [`Self::insert_labels_segment`] returns an error, that error is returned.
    pub fn insert_segment<'b, T: TryInto<DomainSegment<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetDomainError> where SetDomainError: From<T::Error> {
        self.insert_labels_segment(index, value)
    }
}



impl<'a> TryFrom<Cow<'a, [u8]>> for DomainHost<'a> {
    type Error = InvalidDomainHost;

    fn try_from(value: Cow<'a, [u8]>) -> Result<Self, Self::Error> {
        let (_, value, details) = make_domain_host_bytes(value)?;

        unsafe {
            Ok(Self::from_parts(value, details))
        }
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DomainHost<'a> {
    type Error = InvalidDomainHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let (_, value, details) = make_domain_host(value)?;

        unsafe {
            Ok(Self::from_parts(value, details))
        }
    }
}



impl<'a> TryFrom<DomainSegment<'a>> for DomainHost<'a> {
    type Error = DomainSegment<'a>;

    fn try_from(value: DomainSegment<'a>) -> Result<Self, Self::Error> {
        match value.is_a_number() {
            true  => Err(value),
            false => Ok(Self {
                details: DomainHostDetails {
                    data1: 0,
                    data2: 0,
                },
                host: value.into_inner()
            })
        }
    }
}

impl<'a> TryFrom<Host<'a>> for DomainHost<'a> {
    type Error = Host<'a>;

    fn try_from(value: Host<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            Host::Domain(x) => x,
            x               => Err(x)?,
        })
    }
}

impl<'a> TryFrom<FileHost<'a>> for DomainHost<'a> {
    type Error = FileHost<'a>;

    fn try_from(value: FileHost<'a>) -> Result<Self, Self::Error> {
        if let FileHost::Domain(x) = value {
            Ok(x)
        } else {
            Err(value)
        }
    }
}

impl<'a> TryFrom<SpecialNotFileHost<'a>> for DomainHost<'a> {
    type Error = SpecialNotFileHost<'a>;

    fn try_from(value: SpecialNotFileHost<'a>) -> Result<Self, Self::Error> {
        if let SpecialNotFileHost::Domain(x) = value {
            Ok(x)
        } else {
            Err(value)
        }
    }
}

impl<'a> TryFrom<NonSpecialHost<'a>> for DomainHost<'a> {
    type Error = NonSpecialHost<'a>;

    fn try_from(value: NonSpecialHost<'a>) -> Result<Self, Self::Error> {
        Err(value)
    }
}
