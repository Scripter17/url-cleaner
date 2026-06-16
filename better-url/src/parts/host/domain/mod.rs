//! [`DomainHost`].

use crate::prelude::*;

mod segment;
mod segments;

pub use segment::*;
pub use segments::*;

mod prefix;
mod middle;
mod suffix;
mod fqddot;
mod labels;
mod origin;
mod normal;

/// A domain host.
///
/// # Segments
///
/// A domain host segments is a `.` delimited list of substrings of [`Self::as_str`] with any single trailing `.` removed.
///
/// For example, `example.com` and `example.com.` both have the segments `["example", "com"]`, but `example.com..` has segments `["example", "com", ""]`.
///
/// There is always at least one segment. That is, the domain host `.` has segments `[""]`. The empty domain host, were it valid, would probably also have segments `[""]`.
///
/// When a setter returns [`CantBeEmpty`], such as by setting `example.com.`'s labels to [`None`], it refers to making this list contain zero elements.
///
/// # Parts
///
/// There are 6 parts:
///
/// ```
/// use better_url::prelude::*;
///
/// // The prefix, the segments below the middle.
/// assert_eq!(DomainHost::new("abc.def.example.co.uk" ).unwrap().prefix_str(), Some("abc.def"                ));
/// assert_eq!(DomainHost::new("abc.def.example.co.uk.").unwrap().prefix_str(), Some("abc.def"                ));
///
/// // The middle, the segment just below the suffix.
/// assert_eq!(DomainHost::new("abc.def.example.co.uk" ).unwrap().middle_str(), Some(        "example"        ));
/// assert_eq!(DomainHost::new("abc.def.example.co.uk.").unwrap().middle_str(), Some(        "example"        ));
///
/// // The suffix, as defined by the PSL.
/// assert_eq!(DomainHost::new("abc.def.example.co.uk" ).unwrap().suffix_str(),                      "co.uk"   );
/// assert_eq!(DomainHost::new("abc.def.example.co.uk.").unwrap().suffix_str(),                      "co.uk"   );
///
/// // The labels (better name pending), everything but the FQDDot.
/// assert_eq!(DomainHost::new(                     ".").unwrap().labels_str(),                      "");
/// assert_eq!(DomainHost::new(                "co.uk.").unwrap().labels_str(),                 "co.uk");
/// assert_eq!(DomainHost::new(        "example.co.uk.").unwrap().labels_str(),         "example.co.uk");
/// assert_eq!(DomainHost::new(    "www.example.co.uk.").unwrap().labels_str(),     "www.example.co.uk");
/// assert_eq!(DomainHost::new("abc.def.example.co.uk.").unwrap().labels_str(), "abc.def.example.co.uk");
///
/// // The origin, the suffix and the middle.
/// // Also called the registerable domain but that doesn't have a 6 letter equivalent.
/// assert_eq!(DomainHost::new(                "co.uk.").unwrap().origin_str(), None                 );
/// assert_eq!(DomainHost::new(        "example.co.uk.").unwrap().origin_str(), Some("example.co.uk"));
/// assert_eq!(DomainHost::new("abc.def.example.co.uk.").unwrap().origin_str(), Some("example.co.uk"));
///
/// // The normal, the labels (better name still pending), excluding the prefix if it's `www`.
/// assert_eq!(DomainHost::new(                     ".").unwrap().normal_str(),                      "");
/// assert_eq!(DomainHost::new(                "co.uk.").unwrap().normal_str(),                 "co.uk");
/// assert_eq!(DomainHost::new(        "example.co.uk.").unwrap().normal_str(),         "example.co.uk");
/// assert_eq!(DomainHost::new(    "www.example.co.uk.").unwrap().normal_str(),         "example.co.uk");
/// assert_eq!(DomainHost::new("abc.def.example.co.uk.").unwrap().normal_str(), "abc.def.example.co.uk");
/// ```
///
/// # Setters
///
/// For performance reasons, setters don't check that the part and value match.
///
/// ```
/// use better_url::prelude::*;
///
/// let mut domain = DomainHost::new("example.com").unwrap();
///
/// domain.set_suffix(Some("example.com")).unwrap();
///
/// assert_eq!(domain         , "example.example.com");
/// assert_eq!(domain.suffix(),                 "com");
/// ```
///
/// Or that you're setting/inserting only one segment.
///
/// ```
/// use better_url::prelude::*;
///
/// let mut domain = DomainHost::new("example.com").unwrap();
///
/// domain.set_middle(Some("www.example")).unwrap();
///
/// assert_eq!(domain             ,      "www.example.com" );
/// assert_eq!(domain.prefix_str(), Some("www"            ));
/// assert_eq!(domain.middle_str(), Some(    "example"    ));
/// assert_eq!(domain.suffix_str(),                  "com" );
///
/// domain.insert_prefix_segment(-1, "123.456").unwrap();
///
/// assert_eq!(domain             ,      "www.123.456.example.com" );
/// assert_eq!(domain.prefix_str(), Some("www.123.456"            ));
/// assert_eq!(domain.middle_str(), Some(            "example"    ));
/// assert_eq!(domain.suffix_str(),                          "com" );
/// ```
///
/// However, they do ensure that the result is a valid domain host.
///
/// ```
/// use better_url::prelude::*;
///
/// // Domains can't "end in a number".
/// DomainHost::new("example.com" ).unwrap().set_suffix(Some("123")).unwrap_err();
/// DomainHost::new("example.com.").unwrap().set_suffix(Some("123")).unwrap_err();
///
/// // Unicode segments are IDNA'd.
/// let mut domain = DomainHost::new("example.com").unwrap();
/// domain.set_suffix(Some("δοκιμή")).unwrap();
/// assert_eq!(domain, "example.xn--jxalpdlp");
///
/// // IDNA segment literals are accepted.
/// let mut domain = DomainHost::new("example.com").unwrap();
/// domain.set_suffix(Some("xn--jxalpdlp")).unwrap();
/// assert_eq!(domain, "example.xn--jxalpdlp");
///
/// // But invalid ones are not.
/// DomainHost::new("example.com").unwrap()
///     .set_suffix(Some("xn--a-2"))
///     .unwrap_err();
/// ```
///
/// Additionally, attempting to set the suffix of a non-FQDN domain to end in an empty segment returns an error.
///
/// ```
/// use better_url::prelude::*;
///
/// DomainHost::new("example.com").unwrap().set_suffix(Some(    "")).unwrap_err();
/// DomainHost::new("example.com").unwrap().set_suffix(Some(   ".")).unwrap_err();
/// DomainHost::new("example.com").unwrap().set_suffix(Some("com.")).unwrap_err();
///
/// DomainHost::new("example..com").unwrap().set_suffix(None::<&str>).unwrap_err();
///
/// let mut domain = DomainHost::new("example.com.").unwrap();
/// domain.set_suffix(Some("com.")).unwrap();
/// assert_eq!(domain, "example.com..");
/// ```
///
/// This is to ensure that if you have a domain with X segments and replace a range of Y segments with Z segments (and the setter returns [`Ok`]), you always end up with a domain with X-Y+Z segments.
#[derive(Debug, Clone)]
pub struct DomainHost<'a> {
    /// The host string.
    pub(crate) host: Cow<'a, str>,
    /// The [`DomainDetails`].
    pub(crate) details: DomainDetails,
}

impl<'a> DomainHost<'a> {
    /// Make a new [`Self`] from an already percent decoded input.
    /// # Errors
    /// If the call to [`encode_domain`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::new_raw`] returns an error, that error is returned.
    pub fn new_percent_decoded<T: Into<Cow<'a, str>>>(value: T) -> Result<Self, InvalidDomainHost> {
        let (_, value, bidi_details) = encode_percent_decoded_domain(value)?;

        Ok(Self {
            details: DomainDetails {
                parts: DomainPartsDetails::from_raw_unchecked(&value),
                bidi: bidi_details
            },
            host: value
        })
    }

    /// The host as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.host
    }

    /// The [`DomainDetails`].
    pub fn details(&self) -> &DomainDetails {
        &self.details
    }

    /// The [`DomainPartsDetails`].
    pub fn parts_details(&self) -> DomainPartsDetails {
        self.details.parts
    }

    /// The [`BidiDetails`].
    pub fn bidi_details(&self) -> &BidiDetails {
        &self.details.bidi
    }

    /// Unwrap into the host and details.
    pub fn into_parts(self) -> (Cow<'a, str>, DomainDetails) {
        (self.host, self.details)
    }

    /// [`decode_normalized_domain_unchecked`].
    pub fn decode(self) -> (Cow<'a, str>, DomainDetails) {
        let (_, value) = decode_normalized_domain_unchecked(self.host);
        (value, self.details)
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
            details: self.details.clone()
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
    /// If the call to [`Self::set_labels_segment`] returns an error, that error is returned.
    pub fn set_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        self.set_labels_segment(index, value)
    }

    /// Shorthand for [`Self::set_labels_range`].
    /// # Errors
    /// If the call to [`Self::set_labels_range`] returns an error, that error is returned.
    pub fn set_range<'b, T: TryInto<DomainSegments<'b>>, B: RangeBounds<isize>>(&mut self, range: B, value: Option<T>) -> Result<bool, SetDomainError> where SetDomainError: From<T::Error> {
        self.set_labels_range(range, value)
    }

    /// Shorthand for [`Self::insert_labels_segment`].
    /// # Errors
    /// If the call to [`Self::insert_labels_segment`] returns an error, that error is returned.
    pub fn insert_segment<'b, T: TryInto<DomainSegments<'b>>>(&mut self, index: isize, value: T) -> Result<(), SetDomainError> where SetDomainError: From<T::Error> {
        self.insert_labels_segment(index, value)
    }
}

impl<'a> TryFrom<Cow<'a, str>> for DomainHost<'a> {
    type Error = InvalidDomainHost;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let (_, value, bidi_details) = encode_domain(value)?;

        Ok(Self {
            details: DomainDetails {
                parts: DomainPartsDetails::from_raw_unchecked(&value),
                bidi: bidi_details,
            },
            host: value,
        })
    }
}

impl<'a> TryFrom<DomainSegment<'a>> for DomainHost<'a> {
    type Error = DomainSegment<'a>;

    fn try_from(value: DomainSegment<'a>) -> Result<Self, Self::Error> {
        match value.is_a_number() {
            true  => Err(value),
            false => Ok(Self {
                details: DomainDetails {
                    parts: DomainPartsDetails {
                        ms: 0,
                        ss: 0,
                        sa: value.len() as u32,
                        fq: false,
                        wp: false,
                        mi: None,
                    },
                    bidi: value.bidi_detail.into(),
                },
                host: value.segment
            })
        }
    }
}

impl<'a> TryFrom<Host<'a>> for DomainHost<'a> {
    type Error = Host<'a>;

    fn try_from(value: Host<'a>) -> Result<Self, Self::Error> {
        match value {
            Host::Domain(x) => Ok(x),
            _ => Err(value)
        }
    }
}
