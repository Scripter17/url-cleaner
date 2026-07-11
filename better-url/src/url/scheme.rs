//! [`Scheme`].

use crate::prelude::*;

impl BetterUrl {
    /// The scheme as a [`str`].
    pub fn scheme_str(&self) -> &str {
        &self.serialization[.. self.scheme_mark as usize]
    }

    /// The [`SchemeDetails`].
    pub fn scheme_details(&self) -> SchemeDetails {
        self.details.scheme
    }

    /// The [`Scheme`].
    pub fn scheme(&self) -> Scheme<'_> {
        Scheme {
            scheme: self.scheme_str().into(),
            details: self.details.scheme
        }
    }

    /** [`SchemeDetails::type`].                **/ pub fn scheme_type        (&self) -> SchemeType {self.details.scheme.r#type             ()}
    /** [`SchemeDetails::is_special`].          **/ pub fn is_special         (&self) -> bool       {self.details.scheme.is_special         ()}
    /** [`SchemeDetails::is_file`].             **/ pub fn is_file            (&self) -> bool       {self.details.scheme.is_file            ()}
    /** [`SchemeDetails::is_special_not_file`]. **/ pub fn is_special_not_file(&self) -> bool       {self.details.scheme.is_special_not_file()}
    /** [`SchemeDetails::is_non_special`].      **/ pub fn is_non_special     (&self) -> bool       {self.details.scheme.is_non_special     ()}

    /// Set the scheme.
    /// # Errors
    /// If the call to [`Scheme::new`] returns an error, that error is returned.
    ///
    /// If the URL would become too long, returns the error [`TooLong`].
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't be possible.")]
    pub fn set_scheme<'a, T: TryInto<Scheme<'a>>>(&mut self, value: T) -> Result<(), SetSchemeError> where SetSchemeError: From<T::Error> {
        let new = value.try_into()?;

        if  self.is_special  () && !new .is_special   () {return Ok(());}
        if !self.is_special  () &&  new .is_special   () {return Ok(());}
        if  self.has_username() &&  new .is_file      () {return Ok(());}
        if  self.has_port    () &&  new .is_file      () {return Ok(());}
        if  self.is_file     () &&  self.host_is_empty() {return Ok(());}

        let start_len = self.len();
        let after_len = self.len() - self.scheme_mark as usize + new.len();

        if after_len > u32::MAX as usize {
            Err(TooLong)?;
        }

        let diff = (after_len as u32).wrapping_sub(start_len as u32);

        self.serialization.replace_range(..self.scheme_mark as usize, new.as_str());
        self.details.scheme = new.details();

        self.scheme_mark = self.scheme_mark.wrapping_add(diff);
        self.path_start  = self.path_start .wrapping_add(diff);

        if let Some(x) = self.username_after {self.username_after = NonZero::new(x.get().wrapping_add(diff));}
        if let Some(x) = self.host_start     {self.host_start     = NonZero::new(x.get().wrapping_add(diff));}
        if let Some(x) = self.port_mark      {self.port_mark      = NonZero::new(x.get().wrapping_add(diff));}
        if let Some(x) = self.query_mark     {self.query_mark     = NonZero::new(x.get().wrapping_add(diff));}
        if let Some(x) = self.fragment_mark  {self.fragment_mark  = NonZero::new(x.get().wrapping_add(diff));}

        if let Some((x, y)) = self.port_num().zip(self.details.scheme.default_port_num()) && x == y {
            self.set_port(None::<&str>).expect("???");
        }

        Ok(())
    }
}
