//! Middle stuff.

use crate::prelude::*;

impl BetterUrl {
    /// If it has a domain middle.
    pub fn has_domain_middle(&self) -> bool {
        self.domain_details().is_some_and(|x| x.ss != 0)
    }



    /// The [`Range`] of the domain middle.
    fn domain_middle_thing(&self) -> Option<Range<usize>> {
        let hs = self.host_start    ()?;
        let dd = self.domain_details()?;

        match dd.ss {
            0 => None,
            _ => Some(hs + dd.ms as usize .. hs + dd.ss as usize - 1),
        }
    }

    /// The domain middle as a [`str`].
    pub fn domain_middle_str(&self) -> Option<&str> {
        Some(unsafe {self.as_str().get_unchecked(self.domain_middle_thing()?)})
    }

    /// The domain middle as a [`DomainSegment`].
    pub fn domain_middle(&self) -> Option<DomainSegment<'_>> {
        Some(unsafe {DomainSegment::new_unchecked(self.domain_middle_str()?)})
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
            self.set_host(domain.into_owned())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
