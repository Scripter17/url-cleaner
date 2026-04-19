//! Scheme stuff.

use crate::prelude::*;

impl BetterUrl {
    /// The [`SchemeDetails`].
    pub fn scheme_details(&self) -> SchemeDetails {
        self.scheme_details
    }

    /// The [`SchemeType`].
    pub fn scheme_type(&self) -> SchemeType {
        self.scheme_details().r#type()
    }

    /// The [`Scheme`].
    pub fn scheme(&self) -> Scheme<'_> {
        Scheme {
            scheme: self.url.scheme().into(),
            details: self.scheme_details
        }
    }

    /// The scheme.
    pub fn scheme_str(&self) -> &str {
        self.url.scheme()
    }

    /// Set the scheme.
    /// # Errors
    /// See [`SetSchemeError`].
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_scheme<'a, T: TryInto<Scheme<'a>>>(&mut self, scheme: T) -> Result<(), SetSchemeError> where SetSchemeError: From<T::Error> {
        let scheme = scheme.try_into()?;

        let new_len = self.len() - self.scheme().len() + scheme.len();

        if self.scheme() == scheme {
            return Ok(());
        }

        if new_len > u32::MAX as usize {
            Err(TooLong)?;
        }

        if  self.cannot_be_a_base() &&  scheme.is_special_not_file() {Err(SetSchemeError::CannotBeABaseToSpecialNotFile)?;}
        if  self.is_special()       && !scheme.is_special()          {Err(SetSchemeError::SpecialToNonSpecial          )?;}
        if !self.is_special()       &&  scheme.is_special()          {Err(SetSchemeError::NonSpecialToSpecial          )?;}
        if  self.has_authority()    &&  scheme.is_file()             {Err(SetSchemeError::FileCantHaveAuthority        )?;}
        if !self.has_host()         &&  scheme.is_special()          {Err(SetSchemeError::NoHostToSpecial              )?;}

        self.url.set_scheme(scheme.as_str()).expect("To be valid.");
        self.scheme_details = scheme.details();

        debug_assert_eq!(self.scheme(), scheme);
        debug_assert_eq!(self.len(), new_len);

        Ok(())
    }
}
