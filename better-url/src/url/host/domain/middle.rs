//! Middle stuff.

use crate::prelude::*;

impl BetterUrl {
    /// [`DomainPartsDetails::has_middle`].
    pub fn has_domain_middle(&self) -> bool {
        self.domain_parts_details().is_some_and(DomainPartsDetails::has_middle)
    }



    /// The domain middle as a [`str`].
    pub fn domain_middle_str(&self) -> Option<&str> {
        Some(&self.host_str()?[self.domain_parts_details()?.middle_range()?])
    }

    /// The [`BidiDetail`] for the domain middle.
    pub fn domain_middle_bidi_detail(&self) -> Option<BidiDetail> {
        self.domain_details()?.middle_bidi_detail()
    }

    /// The domain middle as a [`DomainSegment`].
    pub fn domain_middle(&self) -> Option<DomainSegment<'_>> {
        Some(DomainSegment {
            segment    : self.domain_middle_str()?.into(),
            bidi_detail: self.domain_middle_bidi_detail()?,
        })
    }



    /// [`DomainHost::set_middle`].
    /// # Errors
    /// If the call to [`Self::domain`] returns [`None`], returns the error [`NoDomain`].
    ///
    /// If the call to [`DomainHost::set_middle`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_host`] reutrns an error, that error is returned.
    pub fn set_domain_middle<'b, T: TryInto<DomainSegments<'b>>>(&mut self, value: Option<T>) -> Result<bool, SetHostError> where SetDomainError: From<T::Error> {
        let mut domain = self.domain().ok_or(NoDomain)?;

        if domain.set_middle(value)? {
            self.set_host(Some(domain.into_owned()))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
