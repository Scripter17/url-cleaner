//! Implementing subdomain segment stuff for [`BetterUrl`].

use super::*;

/// The enum of errors [`BetterUrl::set_subdomain_segment`] can return.
#[derive(Debug, Error)]
pub enum SetSubdomainSegmentError {
    /// Returned when the URL doesn't have a subdomain.
    #[error("The URL does not have a subdomain.")]
    UrlDoesNotHaveSubdomain,
    /// Returned when the segment isn't found.
    #[error("The segment wasn't found.")]
    SegmentNotFound,
    /// Returned when a [`SetSubdomainError`] is encountered.
    #[error(transparent)]
    SetSubdomainError(#[from] SetSubdomainError)
}

/// The enum of errors [`BetterUrl::insert_subdomain_segment`] can return.
#[derive(Debug, Error)]
pub enum InsertSubdomainSegmentError {
    /// Returned when the URL doesn't have a subdomain.
    #[error("The URL does not have a subdomain.")]
    UrlDoesNotHaveSubdomain,
    /// Returned when the segment isn't found.
    #[error("The segment wasn't found.")]
    SegmentNotFound,
    /// Returned when a [`SetSubdomainError`] is encountered.
    #[error(transparent)]
    SetSubdomainError(#[from] SetSubdomainError)
}

impl BetterUrl {
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let url = BetterUrl::parse("https://abc.def.example.co.uk").unwrap();
    ///
    /// assert_eq!(url.subdomain_segment(-3), None              );
    /// assert_eq!(url.subdomain_segment(-2), Some("abc".into()));
    /// assert_eq!(url.subdomain_segment(-1), Some("def".into()));
    ///
    /// assert_eq!(url.subdomain_segment( 0), Some("abc".into()));
    /// assert_eq!(url.subdomain_segment( 1), Some("def".into()));
    /// assert_eq!(url.subdomain_segment( 2), None              );
    /// ```
    pub fn subdomain_segment(&self, index: isize) -> Option<&str> {
        match index {
            0.. => self.subdomain()?.split('.').nth(index as usize),
            ..0 => self.subdomain()?.split('.').nth_back((-index - 1) as usize)
        }
    }

    /// Sets the specified subdomain segment.
    /// # Errors
    /// If the URL doesn't have a domain, returns the error [`SetSubdomainSegmentError::UrlDoesNotHaveSubdomain`].
    ///
    /// If the segment isn't found, returns the error [`SetSubdomainSegmentError::SegmentNotFound`].
    ///
    /// If the call to [`Self::set_subdomain`] returns an error, that error is returned.
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse("https://abc.def.example.co.uk").unwrap();
    ///
    /// url.set_subdomain_segment(-3, Some("n3")).unwrap_err(); assert_eq!(url.host_str(), Some("abc.def.example.co.uk"));
    /// url.set_subdomain_segment(-2, Some("n2")).unwrap    (); assert_eq!(url.host_str(), Some("n2.def.example.co.uk"));
    /// url.set_subdomain_segment(-1, Some("n1")).unwrap    (); assert_eq!(url.host_str(), Some("n2.n1.example.co.uk"));
    ///
    /// url.set_subdomain_segment( 0, Some("p0")).unwrap    (); assert_eq!(url.host_str(), Some("p0.n1.example.co.uk"));
    /// url.set_subdomain_segment( 1, Some("p1")).unwrap    (); assert_eq!(url.host_str(), Some("p0.p1.example.co.uk"));
    /// url.set_subdomain_segment( 2, Some("p2")).unwrap_err(); assert_eq!(url.host_str(), Some("p0.p1.example.co.uk"));
    ///
    ///
    ///
    /// url.set_subdomain_segment( 0, None).unwrap(); assert_eq!(url.host_str(), Some("p1.example.co.uk"));
    /// url.set_subdomain_segment(-1, None).unwrap(); assert_eq!(url.host_str(), Some("example.co.uk"));
    /// ```
    pub fn set_subdomain_segment(&mut self, index: isize, value: Option<&str>) -> Result<(), SetSubdomainSegmentError> {
        self.set_subdomain(set_segment(
            self.subdomain().ok_or(SetSubdomainSegmentError::UrlDoesNotHaveSubdomain)?,
            ".", index, value, SetSubdomainSegmentError::SegmentNotFound
        )?.as_deref())?;
        Ok(())
    }

    /// Inserts a new subdomain segment at the specified index.
    /// # Errors
    /// If the URL doesn't have a domain, returns the error [`InsertSubdomainSegmentError::UrlDoesNotHaveSubdomain`].
    ///
    /// If the segment isn't found, returns the error [`InsertSubdomainSegmentError::SegmentNotFound`].
    ///
    /// If the call to [`Self::set_subdomain`] returns an error, that error is returned.
    pub fn insert_subdomain_segment(&mut self, index: isize, value: &str) -> Result<(), InsertSubdomainSegmentError> {
        self.set_subdomain(Some(&insert_segment(
            self.subdomain().ok_or(InsertSubdomainSegmentError::UrlDoesNotHaveSubdomain)?,
            ".", index, value, InsertSubdomainSegmentError::SegmentNotFound
        )?))?;
        Ok(())
    }
}
