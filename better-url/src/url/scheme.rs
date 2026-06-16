//! Scheme stuff.

use crate::prelude::*;

impl BetterUrl {
    /// The [`SchemeDetails`].
    pub fn scheme_details(&self) -> SchemeDetails {
        self.details().scheme
    }

    /// The [`SchemeType`].
    pub fn scheme_type(&self) -> SchemeType {
        self.scheme_details().r#type()
    }

    /// The scheme.
    pub fn scheme_str(&self) -> &str {
        self.url.scheme()
    }

    /// The [`Scheme`].
    pub fn scheme(&self) -> Scheme<'_> {
        Scheme {
            scheme : self.scheme_str().into(),
            details: self.scheme_details()
        }
    }

    /// Set the scheme.
    /// # Errors
    /// See [`SetSchemeError`].
    #[allow(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_scheme<'a, T: TryInto<Scheme<'a>>>(&mut self, value: T) -> Result<bool, SetSchemeError> where SetSchemeError: From<T::Error> {
        let new = value.try_into()?;
        let old = self.scheme();

        if old == new {
            return Ok(false);
        }

        let new_len = self.len() - old.len() + new.len();

        if new_len > u32::MAX as usize {
            Err(TooLong)?;
        }

        if  self.cannot_be_a_base() &&  new.is_special_not_file() {Err(SetSchemeError::CannotBeABaseToSpecialNotFile)?;}
        if  self.is_special      () && !new.is_special         () {Err(SetSchemeError::SpecialToNonSpecial          )?;}
        if !self.is_special      () &&  new.is_special         () {Err(SetSchemeError::NonSpecialToSpecial          )?;}
        if  self.has_authority   () &&  new.is_file            () {Err(SetSchemeError::FileCantHaveAuthority        )?;}
        if !self.has_host        () &&  new.is_special         () {Err(SetSchemeError::NoHostToSpecial              )?;}

        self.url.set_scheme(new.as_str()).expect("To be valid.");
        self.details.scheme = new.details();

        debug_assert_eq!(self.scheme(), new    );
        debug_assert_eq!(self.len   (), new_len);

        Ok(true)
    }
}
