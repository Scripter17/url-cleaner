//! Implementing domain stuff for [`BetterUrl`].

use super::*;

mod segments;
pub use segments::*;

/// The enum of errors [`BetterUrl::set_domain`] can return.
#[derive(Debug, Error)]
pub enum SetDomainError {
    /// Returned when the call to [`BetterUrl::set_host`] returns an error.
    #[error(transparent)]
    SetHostError(#[from] SetHostError),
    /// One of the many other errors that can happen.
    #[error("One of many possible issues I don't want to name.")]
    Other
}

/// The enum of errors [`BetterUrl::set_subdomain`] can return.
#[derive(Debug, Error)]
pub enum SetSubdomainError {
    /// Returned when the [`BetterUrl`]'s host isn't a domain.
    #[error("The host was not a domain.")]
    HostIsNotADomain,
    /// Returned when trying to set the subdomain on a domain without a reg domain.
    #[error("Tried to set the subdomain on a domain without a reg domain.")]
    MissingRegDomain,
    /// Returned when the resulting value isn't parsable as a domain.
    #[error(transparent)]
    SetHostError(#[from] SetHostError)
}

/// The enum of errors [`BetterUrl::set_not_domain_suffix`] can return.
#[derive(Debug, Error)]
pub enum SetNotDomainSuffixError {
    /// Returned when the [`BetterUrl`]'s host isn't a domain.
    #[error("The host was not a domain.")]
    HostIsNotADomain,
    /// Returned when trying to set the not domain suffix on a domain without a domain suffix.
    #[error("Tried to set the not domain suffix on a domain without a domain suffix.")]
    MissingDomainSuffix,
    /// Returned when the resulting value isn't parsable as a domain.
    #[error(transparent)]
    SetHostError(#[from] SetHostError)
}

/// The enum of errors [`BetterUrl::set_domain_middle`] can return.
#[derive(Debug, Error)]
pub enum SetDomainMiddleError {
    /// Returned when the [`BetterUrl`]'s host isn't a domain.
    #[error("The host was not a domain.")]
    HostIsNotADomain,
    /// Returned when trying to set the domain middle on a domain without a domain suffix.
    #[error("Tried to set the domain middle on a domain without a domain suffix.")]
    MissingDomainSuffix,
    /// Returned when the resulting value isn't parsable as a domain.
    #[error(transparent)]
    SetHostError(#[from] SetHostError)
}

/// The enum of errors [`BetterUrl::set_reg_domain`] can return.
#[derive(Debug, Error)]
pub enum SetRegDomainError {
    /// Returned when the [`BetterUrl`]'s host isn't a domain.
    #[error("The host was not a domain.")]
    HostIsNotADomain,
    /// Returned when the resulting value isn't parsable as a domain.
    #[error(transparent)]
    SetHostError(#[from] SetHostError)
}

/// The enum of errors [`BetterUrl::set_domain_suffix`] can return.
#[derive(Debug, Error)]
pub enum SetDomainSuffixError {
    /// Returned when the [`BetterUrl`]'s host isn't a domain.
    #[error("The host was not a domain.")]
    HostIsNotADomain,
    /// Returned when the resulting value isn't parsable as a domain.
    #[error(transparent)]
    SetHostError(#[from] SetHostError)
}

/// The enum of errors [`BetterUrl::set_fqdn`] can return.
#[derive(Debug, Error)]
pub enum SetFqdnError {
    /// Returned when the URL doesn't have a host.
    #[error("The URL didn't have a host.")]
    NoHost,
    /// Returned when the URL's host isn't a domain.
    #[error("The URL's host wasn't a domain.")]
    HostIsNotADomain
}

impl BetterUrl {
    /// The domain, not including any fully qualified domain name period.
    ///
    /// Specifically, if [`Self::host_details`] is [`HostDetails::Domain`], return the substring of [`Url::host_str`] specified by [`DomainDetails::domain_bounds`].
    /// # Examples
    /// ```
    /// use better_url::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().domain(), Some("example.com"      ));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().domain(), Some("example.co.uk"    ));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().domain(), Some("www.example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().domain(), Some("www.example.co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().domain(), Some("www.example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().domain(), Some("www.example.co.uk"));
    /// ```
    pub fn domain(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.domain_bounds())
    }

    /// The subdomain.
    ///
    /// Specifically, if [`Self::host_details`] is [`HostDetails::Domain`], return the substring of [`Url::host_str`] specified by [`DomainDetails::subdomain_bounds`].
    /// # Examples
    /// ```
    /// use better_url::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().subdomain(), None);
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().subdomain(), None);
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().subdomain(), Some("www"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().subdomain(), Some("www"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().subdomain(), Some("www"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().subdomain(), Some("www"));
    /// ```
    pub fn subdomain(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.subdomain_bounds()?)
    }

    /// The "not domain suffix", being [`Self::subdomain`] and [`Self::domain_middle`].
    ///
    /// Specifically, if [`Self::host_details`] is [`HostDetails::Domain`], return the substring of [`Url::host_str`] specified by [`DomainDetails::not_domain_suffix_bounds`].
    /// # Examples
    /// ```
    /// use better_url::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().not_domain_suffix(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().not_domain_suffix(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().not_domain_suffix(), Some("www.example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().not_domain_suffix(), Some("www.example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().not_domain_suffix(), Some("www.example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().not_domain_suffix(), Some("www.example"));
    /// ```
    pub fn not_domain_suffix(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.not_domain_suffix_bounds()?)
    }

    /// The domain middle.
    ///
    /// Specifically, if [`Self::host_details`] is [`HostDetails::Domain`], return the substring of [`Url::host_str`] specified by [`DomainDetails::domain_middle_bounds`].
    /// # Examples
    /// ```
    /// use better_url::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().domain_middle(), Some("example"));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().domain_middle(), Some("example"));
    /// ```
    pub fn domain_middle(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.domain_middle_bounds()?)
    }

    /// The "registerable domain", being the [`Self::domain_middle`] and [`Self::domain_suffix`].
    ///
    /// Specifically, if [`Self::host_details`] is [`HostDetails::Domain`], return the substring of [`Url::host_str`] specified by [`DomainDetails::reg_domain_bounds`].
    /// # Examples
    /// ```
    /// use better_url::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().reg_domain(), Some("example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().reg_domain(), Some("example.co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().reg_domain(), Some("example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().reg_domain(), Some("example.co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().reg_domain(), Some("example.com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().reg_domain(), Some("example.co.uk"));
    /// ```
    pub fn reg_domain(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.reg_domain_bounds()?)
    }

    /// The domain suffix, as defined by the public suffix list, not including any fully qualified domain name period.
    ///
    /// Specifically, if [`Self::host_details`] is [`HostDetails::Domain`], return the substring of [`Url::host_str`] specified by [`DomainDetails::domain_suffix_bounds`].
    /// # Examples
    /// ```
    /// use better_url::*;
    /// assert_eq!(BetterUrl::parse("https://example.com"       ).unwrap().domain_suffix(), Some("com"  ));
    /// assert_eq!(BetterUrl::parse("https://example.co.uk"     ).unwrap().domain_suffix(), Some("co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com"   ).unwrap().domain_suffix(), Some("com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk" ).unwrap().domain_suffix(), Some("co.uk"));
    /// assert_eq!(BetterUrl::parse("https://www.example.com."  ).unwrap().domain_suffix(), Some("com"  ));
    /// assert_eq!(BetterUrl::parse("https://www.example.co.uk.").unwrap().domain_suffix(), Some("co.uk"));
    /// ```
    pub fn domain_suffix(&self) -> Option<&str> {
        self.host_str()?.get(self.domain_details()?.domain_suffix_bounds()?)
    }



    /// Sets the domain.
    ///
    /// If this function returns [`Ok`], then [`Self::domain`] returns the value passed into it.
    /// # Errors
    /// If the URL doesn't have a domain host, `to` isn't a domain, and/or `to` is a fully qualified domain, returns the error [`SetDomainError::Other`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com" ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com" ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com."));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com."));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain(Some("example.com")).unwrap(); assert_eq!(url.host_str(), Some("example.com."));
    /// ```
    pub fn set_domain(&mut self, to: Option<&str>) -> Result<(), SetDomainError> {
        Ok(match (self.host_details(), to.map(|to| (to, HostDetails::parse(to)))) {
            (Some(HostDetails::Domain(DomainDetails {fqdn_period: Some(_), ..})), Some((to, Ok(HostDetails::Domain(mut domain_details @ DomainDetails {fqdn_period: None, ..}))))) => {
                {domain_details.fqdn_period = Some(to.len());}
                self.set_host_with_known_details(Some(&format!("{to}.")), Some(HostDetails::Domain(domain_details)))?
            },
            (Some(HostDetails::Domain(DomainDetails {fqdn_period: None, ..})), Some((to, Ok(host_details @ HostDetails::Domain(DomainDetails {fqdn_period: None, ..}))))) => {
                self.set_host_with_known_details(Some(to), Some(host_details))?
            },
            _ => Err(SetDomainError::Other)?
        })
    }

    /// Sets the subdomain.
    ///
    /// If this function returns [`Ok`], then [`Self::subdomain`] returns the value passed into it.
    /// # Errors
    /// If [`Self`]'s host isn't a domain, returns the error [`SetSubdomainError::HostIsNotADomain`].
    ///
    /// If [`Self`] doesn't have a reg domain, returns the error [`SetSubdomainError::MissingRegDomain`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   )); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(0), suffix_start: Some(8 ), fqdn_period: None})));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   )); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: None})));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   )); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(0), suffix_start: Some(8 ), fqdn_period: None})));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   )); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: None})));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" )); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(0), suffix_start: Some(8 ), fqdn_period: None})));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" )); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: None})));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" )); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(0), suffix_start: Some(8 ), fqdn_period: None})));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" )); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: None})));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  )); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(0), suffix_start: Some(8 ), fqdn_period: Some(11)})));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  )); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: Some(15)})));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  )); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(0), suffix_start: Some(8 ), fqdn_period: Some(11)})));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  )); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: Some(15)})));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk.")); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(0), suffix_start: Some(8 ), fqdn_period: Some(13)})));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk.")); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: Some(17)})));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_subdomain(None       ).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk.")); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(0), suffix_start: Some(8 ), fqdn_period: Some(13)})));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_subdomain(Some("www")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk.")); assert_eq!(url.host_details(), Some(HostDetails::Domain(DomainDetails {middle_start: Some(4), suffix_start: Some(12), fqdn_period: Some(17)})));
    /// ```
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_subdomain(&mut self, to: Option<&str>) -> Result<(), SetSubdomainError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(mut domain_details)) => {
                let new_host = match (to, self.reg_domain(), domain_details.is_fqdn()) {
                    (Some(to), Some(rd), false) => format!("{to}.{rd}"),
                    (Some(to), Some(rd), true ) => format!("{to}.{rd}."),
                    (Some(_ ), None    , false) => Err(SetSubdomainError::MissingRegDomain)?,
                    (Some(_ ), None    , true ) => Err(SetSubdomainError::MissingRegDomain)?,
                    (None    , Some(rd), false) => format!("{rd}"),
                    (None    , Some(rd), true ) => format!("{rd}."),
                    (None    , None    , false) => format!(""),
                    (None    , None    , true ) => format!(".")
                };
                #[expect(clippy::arithmetic_side_effects, reason = "Can't happen.")]
                let new_middle_start = to.map_or(0, |subdomain| subdomain.len() + 1);
                let len_diff = new_middle_start.wrapping_sub(domain_details.middle_start.expect(""));
                domain_details.middle_start = Some(new_middle_start);
                domain_details.suffix_start = domain_details.suffix_start.map(|x| x.wrapping_add(len_diff));
                domain_details.fqdn_period  = domain_details.fqdn_period .map(|x| x.wrapping_add(len_diff));
                self.set_host_with_known_details(Some(&new_host), Some(domain_details.into()))?;
            },
            _ => Err(SetSubdomainError::HostIsNotADomain)?
        })
    }

    /// Sets the "not domain suffix".
    ///
    /// If this function returns [`Ok`], then [`Self::not_domain_suffix`] returns the value passed into it.
    /// # Errors
    /// If [`Self`]'s host isn't a domain, returns the error [`SetSubdomainError::HostIsNotADomain`].
    ///
    /// If [`Self`] doesn't have a domain suffix, returns the error [`SetNotDomainSuffixError::MissingDomainSuffix`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk" ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk."));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_not_domain_suffix(None               ).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_not_domain_suffix(Some(    "example")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_not_domain_suffix(Some("www.example")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    /// ```
    pub fn set_not_domain_suffix(&mut self, to: Option<&str>) -> Result<(), SetNotDomainSuffixError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (to, self.domain_suffix(), domain_details.is_fqdn()) {
                (Some(to), Some(su), false) => self.set_host(Some(&format!("{to}.{su}")))?,
                (Some(to), Some(su), true ) => self.set_host(Some(&format!("{to}.{su}.")))?,
                (Some(_ ), None    , false) => Err(SetNotDomainSuffixError::MissingDomainSuffix)?,
                (Some(_ ), None    , true ) => Err(SetNotDomainSuffixError::MissingDomainSuffix)?,
                (None    , Some(su), false) => self.set_host(Some(&format!("{su}")))?,
                (None    , Some(su), true ) => self.set_host(Some(&format!("{su}.")))?,
                (None    , None    , false) => self.set_host(Some(&format!("")))?,
                (None    , None    , true ) => self.set_host(Some(&format!(".")))?
            },
            _ => Err(SetNotDomainSuffixError::HostIsNotADomain)?
        })
    }

    /// Sets the domain middle.
    ///
    /// If this function returns [`Ok`], then [`Self::domain_middle`] returns the value passed into it.
    /// # Errors
    /// If [`Self`]'s host isn't a domain, returns the error [`SetSubdomainError::HostIsNotADomain`].
    ///
    /// If [`Self`] doesn't have a domain suffix, returns the error [`SetDomainMiddleError::MissingDomainSuffix`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(             "com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some(    "example2.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(         "www.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some("www.example2.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(             "co.uk" ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some(    "example2.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(         "www.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some("www.example2.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(             "com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some(    "example2.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(         "www.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some("www.example2.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(             "co.uk."));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some(    "example2.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain_middle(None            ).unwrap(); assert_eq!(url.host_str(), Some(         "www.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain_middle(Some("example2")).unwrap(); assert_eq!(url.host_str(), Some("www.example2.co.uk."));
    /// ```
    pub fn set_domain_middle(&mut self, to: Option<&str>) -> Result<(), SetDomainMiddleError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (self.subdomain(), to, self.domain_suffix(), domain_details.is_fqdn()) {
                (Some(sd), Some(to), Some(su), false) => self.set_host(Some(&format!("{sd}.{to}.{su}")))?,
                (Some(sd), Some(to), Some(su), true ) => self.set_host(Some(&format!("{sd}.{to}.{su}.")))?,
                (Some(_ ), Some(_ ), None    , false) => Err(SetDomainMiddleError::MissingDomainSuffix)?,
                (Some(_ ), Some(_ ), None    , true ) => Err(SetDomainMiddleError::MissingDomainSuffix)?,
                (Some(sd), None    , Some(su), false) => self.set_host(Some(&format!("{sd}.{su}")))?,
                (Some(sd), None    , Some(su), true ) => self.set_host(Some(&format!("{sd}.{su}.")))?,
                (Some(sd), None    , None    , false) => self.set_host(Some(&format!("{sd}")))?,
                (Some(sd), None    , None    , true ) => self.set_host(Some(&format!("{sd}.")))?,
                (None    , Some(to), Some(su), false) => self.set_host(Some(&format!("{to}.{su}")))?,
                (None    , Some(to), Some(su), true ) => self.set_host(Some(&format!("{to}.{su}.")))?,
                (None    , Some(_ ), None    , false) => Err(SetDomainMiddleError::MissingDomainSuffix)?,
                (None    , Some(_ ), None    , true ) => Err(SetDomainMiddleError::MissingDomainSuffix)?,
                (None    , None    , Some(su), false) => self.set_host(Some(&format!("{su}")))?,
                (None    , None    , Some(su), true ) => self.set_host(Some(&format!("{su}.")))?,
                (None    , None    , None    , false) => self.set_host(Some(&format!("")))?,
                (None    , None    , None    , true ) => self.set_host(Some(&format!(".")))?
            },
            _ => Err(SetDomainMiddleError::HostIsNotADomain)?
        })
    }

    /// Sets the registerable domain.
    ///
    /// If this function returns [`Ok`], then [`Self::reg_domain`] returns the value passed into it.
    /// # Errors
    /// If [`Self`]'s host isn't a domain, returns the error [`SetRegDomainError::HostIsNotADomain`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_reg_domain(None                 ).unwrap_err();
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(            "com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk" ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_reg_domain(None                 ).unwrap(); assert_eq!(url.host_str(), Some(            "www"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(        "www.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(        "www.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_reg_domain(None                 ).unwrap_err();
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(            "com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk" ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_reg_domain(None                 ).unwrap(); assert_eq!(url.host_str(), Some(            "www"   ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(        "www.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(        "www.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_reg_domain(None                 ).unwrap(); assert_eq!(url.host_str(), Some(               "."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(            "com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk."));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_reg_domain(None                 ).unwrap(); assert_eq!(url.host_str(), Some(             "www." ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(        "www.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(        "www.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_reg_domain(None                 ).unwrap(); assert_eq!(url.host_str(), Some(               "."  ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(            "com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(            "co.uk."));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_reg_domain(None                 ).unwrap(); assert_eq!(url.host_str(), Some(            "www."  ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_reg_domain(Some(        "com"  )).unwrap(); assert_eq!(url.host_str(), Some(        "www.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_reg_domain(Some("example.com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_reg_domain(Some(        "co.uk")).unwrap(); assert_eq!(url.host_str(), Some(        "www.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_reg_domain(Some("example.co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    /// ```
    pub fn set_reg_domain(&mut self, to: Option<&str>) -> Result<(), SetRegDomainError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (self.subdomain(), to, domain_details.is_fqdn()) {
                (Some(sd), Some(to), false) => self.set_host(Some(&format!("{sd}.{to}")))?,
                (Some(sd), Some(to), true ) => self.set_host(Some(&format!("{sd}.{to}.")))?,
                (Some(sd), None    , false) => self.set_host(Some(&format!("{sd}")))?,
                (Some(sd), None    , true ) => self.set_host(Some(&format!("{sd}.")))?,
                (None    , Some(to), false) => self.set_host(Some(&format!("{to}")))?,
                (None    , Some(to), true ) => self.set_host(Some(&format!("{to}.")))?,
                (None    , None    , false) => self.set_host(Some(&format!("")))?,
                (None    , None    , true ) => self.set_host(Some(&format!(".")))?
            },
            _ => Err(SetRegDomainError::HostIsNotADomain)?
        })
    }

    /// Sets the domain suffix.
    ///
    /// If this function returns [`Ok`], then [`Self::domain_suffix`] returns the value passed into it.
    /// # Errors
    /// If [`Self`]'s host isn't a domain, returns the error [`SetDomainSuffixError::HostIsNotADomain`].
    /// # Examples
    /// ```
    /// use better_url::*;
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some(    "example"       ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.com"   ).unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some("www.example"       ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.com"   ).unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some(    "example"       ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com"   ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk" ).unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk" ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some("www.example"       ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com"   ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk" ).unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk" ));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some(    "example."      ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.com."  ).unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some("www.example."      ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.com."  ).unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    ///
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some(    "example."      ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some(    "example.com."  ));
    /// let mut url = BetterUrl::parse(    "https://example.co.uk.").unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some(    "example.co.uk."));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain_suffix(None         ).unwrap(); assert_eq!(url.host_str(), Some("www.example."      ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain_suffix(Some("com"  )).unwrap(); assert_eq!(url.host_str(), Some("www.example.com."  ));
    /// let mut url = BetterUrl::parse("https://www.example.co.uk.").unwrap(); url.set_domain_suffix(Some("co.uk")).unwrap(); assert_eq!(url.host_str(), Some("www.example.co.uk."));
    /// ```
    pub fn set_domain_suffix(&mut self, to: Option<&str>) -> Result<(), SetDomainSuffixError> {
        Ok(match self.host_details() {
            #[allow(clippy::useless_format, reason = "Visual consistency.")]
            Some(HostDetails::Domain(domain_details)) => match (self.not_domain_suffix(), to, domain_details.is_fqdn()) {
                (Some(ns), Some(to), false) => self.set_host(Some(&format!("{ns}.{to}")))?,
                (Some(ns), Some(to), true ) => self.set_host(Some(&format!("{ns}.{to}.")))?,
                (Some(ns), None    , false) => self.set_host(Some(&format!("{ns}")))?,
                (Some(ns), None    , true ) => self.set_host(Some(&format!("{ns}.")))?,
                (None    , Some(to), false) => self.set_host(Some(&format!("{to}")))?,
                (None    , Some(to), true ) => self.set_host(Some(&format!("{to}.")))?,
                (None    , None    , false) => self.set_host(Some(&format!("")))?,
                (None    , None    , true ) => self.set_host(Some(&format!(".")))?
            },
            _ => Err(SetDomainSuffixError::HostIsNotADomain)?
        })
    }

    /// Sets the [fully qualified domain name](https://en.wikipedia.org/wiki/Fully_qualified_domain_name) period.
    /// # Errors
    /// If `self` doesn't have a host, returns the error [`SetFqdnError::NoHost`].
    ///
    /// If the host isn't a domain, returns the error [`SetFqdnError::HostIsNotADomain`].
    /// # Examples
    /// ```
    /// use better_url::*;
    /// let mut url = BetterUrl::parse("https://example.com").unwrap();
    ///
    /// url.set_fqdn(false).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com"));
    ///
    /// url.set_fqdn(true).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com."));
    ///
    /// url.set_fqdn(true).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com."));
    ///
    /// url.set_fqdn(false).unwrap();
    /// assert_eq!(url.host_str(), Some("example.com"));
    /// ```
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_fqdn(&mut self, to: bool) -> Result<(), SetFqdnError> {
        match (self.host_details(), to) {
            (Some(HostDetails::Domain(DomainDetails {fqdn_period: None   , ..})), false) => {},
            (Some(HostDetails::Domain(DomainDetails {fqdn_period: None   , ..})), true ) => {self.set_host(Some(&format!("{}.", self.host_str().expect("The URL having a DomainDetails means it has a host.")))).expect("Adding a FQDN period to keep the host valid.")},
            #[expect(clippy::unnecessary_to_owned, reason = "It is necessary.")]
            (Some(HostDetails::Domain(DomainDetails {fqdn_period: Some(_), ..})), false) => {self.set_host(Some(&self.host_str().expect("The URL having a DomainDetails means it has a host.").strip_suffix('.').expect("The URL's DomainDetails::fqdn_period being Some means the host ends with a period.").to_string())).expect("Removing a FQDN period to keep the host valid.")},
            (Some(HostDetails::Domain(DomainDetails {fqdn_period: Some(_), ..})), true ) => {},
            (Some(_), _) => Err(SetFqdnError::HostIsNotADomain)?,
            (None, _) => Err(SetFqdnError::NoHost)?
        }

        Ok(())
    }
}
