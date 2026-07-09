//! Setters.

use crate::prelude::*;

impl MyUrl {
    /// The official protocol setter.
    /// # Errors
    /// If the call to [`Self::set_scheme`] returns an error, that error is returned.
    pub fn canon_set_protocol<'a, T: Into<Cow<'a, str>>>(&mut self, value: T) -> Result<(), SetSchemeError> {
        let (_, value) = canonize_scheme_setter(value);

        self.set_scheme(value)
    }

    /// The official username setter.
    /// # Errors
    /// If the call to [`Self::set_username`] returns an error, that error is returned.
    pub fn canon_set_username<'a, T: Into<Cow<'a, str>>>(&mut self, value: T) -> Result<(), SetUsernameError> {
        let (_, value) = canonize_set_username(value);

        self.set_username(value)
    }

    /// The official password setter.
    /// # Errors
    /// If the call to [`Self::set_password`] returns an error, that error is returned.
    pub fn canon_set_password<'a, T: Into<Cow<'a, str>>>(&mut self, value: T) -> Result<(), SetUsernameError> {
        let (_, value) = canonize_set_password(value);

        self.set_password(value)
    }

    /// The official hostname setter.
    /// # Errors
    /// If the call to [`Self::set_host`] returns an error, that error is returned.
    pub fn canon_set_hostname<'a, T: Into<Cow<'a, str>>>(&mut self, value: T) -> Result<(), SetHostError> {
        let (_, value) = canonize_hostname_setter(value, self.is_special());

        self.set_host(value)
    }

    /// The official host setter.
    /// # Errors
    /// If the call to [`Self::set_host_port`] returns an error, that error is returned.
    pub fn canon_set_host<'a, T: Into<Cow<'a, str>>>(&mut self, value: T) -> Result<(), SetHostPortError> {
        let (host, port) = canonize_host_setter(value, self.scheme_type())?;

        self.set_host_port(host, port)
    }

    /// The official port setter.
    /// # Errors
    /// If the call to [`Self::set_port`] returns an error, that error is returned.
    pub fn canon_set_port<'a, T: Into<Cow<'a, str>>>(&mut self, value: T) -> Result<(), SetPortError> {
        let (_, value) = canonize_port_setter(value);

        self.set_port(value)
    }

    /// The official pathname setter.
    /// # Errors
    /// If the call to [`Self::set_path`] returns an error, that error is returned.
    pub fn canon_set_pathname<'a, T: Into<Cow<'a, str>>>(&mut self, value: T) -> Result<(), SetPathError> {
        let (_, value) = canonize_part_setter(value);

        self.set_path(value)
    }

    /// The official search setter.
    /// # Errors
    /// If the call to [`Self::set_query`] returns an error, that error is returned.
    pub fn canon_set_search<'a, T: Into<Cow<'a, str>>>(&mut self, value: T) -> Result<(), SetQueryError> {
        let (_, value) = canonize_maybe_query_setter(value);

        self.set_query(value)
    }

    /// The official hash setter.
    /// # Errors
    /// If the call to [`Self::set_fragment`] returns an error, that error is returned.
    pub fn canon_set_hash<'a, T: Into<Cow<'a, str>>>(&mut self, value: T) -> Result<(), SetFragmentError> {
        let (_, value) = canonize_maybe_fragment_setter(value);

        self.set_fragment(value)
    }
}
