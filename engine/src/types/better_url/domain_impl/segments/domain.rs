//! Implementing domain segment stuff for [`BetterUrl`].

use super::*;

/// The enum of errors [`BetterUrl::set_domain_segment`] can return.
#[derive(Debug, Error)]
pub enum SetDomainSegmentError {
    /// Returned when the URL doesn't have a domain.
    #[error("The URL does not have a domain.")]
    UrlDoesNotHaveDomain,
    /// Returned when the segment isn't found.
    #[error("The segment wasn't found.")]
    SegmentNotFound,
    /// Returned when a [`SetDomainError`] is encountered.
    #[error(transparent)]
    SetDomainError(#[from] SetDomainError)
}

/// The enum of errors [`BetterUrl::insert_domain_segment_at`] and [`BetterUrl::insert_domain_segment_after`] can return.
#[derive(Debug, Error)]
pub enum InsertDomainSegmentError {
    /// Returned when the URL doesn't have a domain.
    #[error("The URL does not have a domain.")]
    UrlDoesNotHaveDomain,
    /// Returned when the segment isn't found.
    #[error("The segment wasn't found.")]
    SegmentNotFound,
    /// Returned when a [`SetDomainError`] is encountered.
    #[error(transparent)]
    SetDomainError(#[from] SetDomainError)
}

impl BetterUrl {
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let url = BetterUrl::parse("https://abc.def.example.co.uk").unwrap();
    ///
    /// assert_eq!(url.domain_segment(-6), None                  );
    /// assert_eq!(url.domain_segment(-5), Some("abc"    .into()));
    /// assert_eq!(url.domain_segment(-4), Some("def"    .into()));
    /// assert_eq!(url.domain_segment(-3), Some("example".into()));
    /// assert_eq!(url.domain_segment(-2), Some("co"     .into()));
    /// assert_eq!(url.domain_segment(-1), Some("uk"     .into()));
    ///
    /// assert_eq!(url.domain_segment( 0), Some("abc"    .into()));
    /// assert_eq!(url.domain_segment( 1), Some("def"    .into()));
    /// assert_eq!(url.domain_segment( 2), Some("example".into()));
    /// assert_eq!(url.domain_segment( 3), Some("co"     .into()));
    /// assert_eq!(url.domain_segment( 4), Some("uk"     .into()));
    /// assert_eq!(url.domain_segment( 5), None                  );
    /// ```
    pub fn domain_segment(&self, index: isize) -> Option<&str> {
        match index {
            0.. => self.domain()?.split('.').nth(index as usize),
            #[allow(clippy::arithmetic_side_effects, reason = "Can't happen.")]
            ..0 => self.domain()?.split('.').nth_back((-index - 1) as usize)
        }
    }

    /// Sets the specified [`UrlPart::DomainSegment`].
    /// # Errors
    /// If the URL doesn't have a domain, returns the error [`SetDomainSegmentError::UrlDoesNotHaveDomain`].
    ///
    /// If the segment isn't found, returns the error [`SetDomainSegmentError::SegmentNotFound`].
    /// # Examples
    /// ```
    /// use url_cleaner_engine::types::*;
    ///
    /// let mut url = BetterUrl::parse("https://abc.def.example.co.uk").unwrap();
    ///
    /// url.set_domain_segment(-6, Some("n6")).unwrap_err(); assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    /// url.set_domain_segment(-5, Some("n5")).unwrap    (); assert_eq!(url.host_str(), Some("n5.def.example.co.uk" ));
    /// url.set_domain_segment(-4, Some("n4")).unwrap    (); assert_eq!(url.host_str(), Some("n5.n4.example.co.uk"  ));
    /// url.set_domain_segment(-3, Some("n3")).unwrap    (); assert_eq!(url.host_str(), Some("n5.n4.n3.co.uk"       ));
    /// url.set_domain_segment(-2, Some("n2")).unwrap    (); assert_eq!(url.host_str(), Some("n5.n4.n3.n2.uk"       ));
    /// url.set_domain_segment(-1, Some("n1")).unwrap    (); assert_eq!(url.host_str(), Some("n5.n4.n3.n2.n1"       ));
    ///
    /// url.set_domain_segment( 0, Some("p0")).unwrap    (); assert_eq!(url.host_str(), Some("p0.n4.n3.n2.n1"       ));
    /// url.set_domain_segment( 1, Some("p1")).unwrap    (); assert_eq!(url.host_str(), Some("p0.p1.n3.n2.n1"       ));
    /// url.set_domain_segment( 2, Some("p2")).unwrap    (); assert_eq!(url.host_str(), Some("p0.p1.p2.n2.n1"       ));
    /// url.set_domain_segment( 3, Some("p3")).unwrap    (); assert_eq!(url.host_str(), Some("p0.p1.p2.p3.n1"       ));
    /// url.set_domain_segment( 4, Some("p4")).unwrap    (); assert_eq!(url.host_str(), Some("p0.p1.p2.p3.p4"       ));
    /// url.set_domain_segment( 5, Some("p5")).unwrap_err(); assert_eq!(url.host_str(), Some("p0.p1.p2.p3.p4"       ));
    ///
    ///
    ///
    /// url.set_domain_segment( 0, None).unwrap(); assert_eq!(url.host_str(), Some("p1.p2.p3.p4"));
    /// url.set_domain_segment(-1, None).unwrap(); assert_eq!(url.host_str(), Some("p1.p2.p3"));
    /// ```
    #[doc = edoc!(callerr(Self::set_domain))]
    pub fn set_domain_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetDomainSegmentError> {
        let segments = set_segment(
            self.domain().ok_or(SetDomainSegmentError::UrlDoesNotHaveDomain)?,
            index, value, SetDomainSegmentError::SegmentNotFound, '.'
        )?;
        let new = match &*segments {
            [] => None,
            _ => Some(segments.join("."))
        };
        self.set_domain(new.as_deref())?;
        Ok(())
    }

    /// Inserts a new [`UrlPart::DomainSegment`] at the specified index.
    /// # Errors
    /// If the URL doesn't have a domain, returns the error [`InsertDomainSegmentError::UrlDoesNotHaveDomain`].
    ///
    /// If the segment isn't found, returns the error [`InsertDomainSegmentError::SegmentNotFound`].
    ///
    #[doc = edoc!(callerr(Self::set_domain))]
    pub fn insert_domain_segment_at(&mut self, index: isize, value: &str) -> Result<(), InsertDomainSegmentError> {
        self.set_domain(Some(&insert_segment_at(
            self.domain().ok_or(InsertDomainSegmentError::UrlDoesNotHaveDomain)?,
            index, value, InsertDomainSegmentError::SegmentNotFound, '.', "."
        )?))?;
        Ok(())
    }

    /// Inserts a new [`UrlPart::DomainSegment`] after the specified index.
    /// # Errors
    /// If the URL doesn't have a domain, returns the error [`InsertDomainSegmentError::UrlDoesNotHaveDomain`].
    ///
    /// If the segment isn't found, returns the error [`InsertDomainSegmentError::SegmentNotFound`].
    ///
    #[doc = edoc!(callerr(Self::set_domain))]
    pub fn insert_domain_segment_after(&mut self, index: isize, value: &str) -> Result<(), InsertDomainSegmentError> {
        self.set_domain(Some(&insert_segment_after(
            self.domain().ok_or(InsertDomainSegmentError::UrlDoesNotHaveDomain)?,
            index, value, InsertDomainSegmentError::SegmentNotFound, '.', "."
        )?))?;
        Ok(())
    }
}
