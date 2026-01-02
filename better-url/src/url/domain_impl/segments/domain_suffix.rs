//! Implementing domain suffix segment stuff for [`BetterUrl`].

use super::*;

/// The enum of errors [`BetterUrl::set_domain_suffix_segment`] can return.
#[derive(Debug, Error)]
pub enum SetDomainSuffixSegmentError {
    /// Returned when the URL doesn't have a domain suffix.
    #[error("The URL does not have a domain suffix.")]
    UrlDoesNotHaveDomainSuffix,
    /// Returned when the segment isn't found.
    #[error("The segment wasn't found.")]
    SegmentNotFound,
    /// Returned when a [`SetDomainSuffixError`] is encountered.
    #[error(transparent)]
    SetDomainSuffixError(#[from] SetDomainSuffixError)
}

/// The enum of errors [`BetterUrl::insert_domain_suffix_segment`] can return.
#[derive(Debug, Error)]
pub enum InsertDomainSuffixSegmentError {
    /// Returned when the URL doesn't have a domain suffix.
    #[error("The URL does not have a domain suffix.")]
    UrlDoesNotHaveDomainSuffix,
    /// Returned when the segment isn't found.
    #[error("The segment wasn't found.")]
    SegmentNotFound,
    /// Returned when a [`SetDomainSuffixError`] is encountered.
    #[error(transparent)]
    SetDomainSuffixError(#[from] SetDomainSuffixError)
}

impl BetterUrl {
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let url = BetterUrl::parse("https://abc.def.example.co.uk").unwrap();
    ///
    /// assert_eq!(url.domain_suffix_segment(-3), None             );
    /// assert_eq!(url.domain_suffix_segment(-2), Some("co".into()));
    /// assert_eq!(url.domain_suffix_segment(-1), Some("uk".into()));
    ///
    /// assert_eq!(url.domain_suffix_segment( 0), Some("co".into()));
    /// assert_eq!(url.domain_suffix_segment( 1), Some("uk".into()));
    /// assert_eq!(url.domain_suffix_segment( 2), None             );
    /// ```
    pub fn domain_suffix_segment(&self, index: isize) -> Option<&str> {
        match index {
            0.. => self.domain_suffix()?.split('.').nth(index as usize),
            ..0 => self.domain_suffix()?.split('.').nth_back((-index - 1) as usize)
        }
    }

    /// Sets the specified domain suffix segment.
    /// # Errors
    /// If the URL doesn't have a domain, returns the error [`SetDomainSuffixSegmentError::UrlDoesNotHaveDomainSuffix`].
    ///
    /// If the segment isn't found, returns the error [`SetDomainSuffixSegmentError::SegmentNotFound`].
    ///
    /// If the call to [`Self::set_domain_suffix`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use better_url::prelude::*;
    ///
    /// let mut url = BetterUrl::parse("https://abc.def.example.co.uk").unwrap();
    ///
    /// url.set_domain_suffix_segment(-3, Some("n3")).unwrap_err(); assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    /// url.set_domain_suffix_segment(-2, Some("n2")).unwrap    (); assert_eq!(url.host_str(), Some("abc.def.example.n2.uk"));
    /// url.set_domain_suffix_segment(-1, Some("n1")).unwrap    (); assert_eq!(url.host_str(), Some("abc.def.example.n2.n1"));
    ///
    /// // Setting a domain suffix segment may alter the amount of domain segments the suffix has, leading to weird results.
    /// url.set_domain_suffix_segment( 0, Some("p0")).unwrap    (); assert_eq!(url.host_str(), Some("abc.def.example.n2.p0"));
    /// url.set_domain_suffix_segment( 1, Some("p1")).unwrap_err(); assert_eq!(url.host_str(), Some("abc.def.example.n2.p0"));
    ///
    ///
    ///
    /// url.set_domain_suffix_segment( 0, None).unwrap(); assert_eq!(url.host_str(), Some("abc.def.example.n2"));
    /// url.set_domain_suffix_segment(-1, None).unwrap(); assert_eq!(url.host_str(), Some("abc.def.example"));
    /// ```
    pub fn set_domain_suffix_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainSuffixSegmentError> {
        self.set_domain_suffix(set_segment(
            self.domain_suffix().ok_or(SetDomainSuffixSegmentError::UrlDoesNotHaveDomainSuffix)?,
            ".", index, value, SetDomainSuffixSegmentError::SegmentNotFound
        )?.as_deref())?;
        Ok(())
    }


    /// Inserts a new domain suffix segment at the specified index.
    /// # Errors
    /// If the URL doesn't have a domain, returns the error [`InsertDomainSuffixSegmentError::UrlDoesNotHaveDomainSuffix`].
    ///
    /// If the segment isn't found, returns the error [`InsertDomainSuffixSegmentError::SegmentNotFound`].
    ///
    /// If the call to [`Self::set_domain_suffix`] returns an error, that error is returned.
    pub fn insert_domain_suffix_segment(&mut self, index: isize, value: &str) -> Result<(), InsertDomainSuffixSegmentError> {
        self.set_domain_suffix(Some(&insert_segment(
            self.domain_suffix().ok_or(InsertDomainSuffixSegmentError::UrlDoesNotHaveDomainSuffix)?,
            ".", index, value, InsertDomainSuffixSegmentError::SegmentNotFound
        )?))?;
        Ok(())
    }
}
