//! Authentication.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::util::*;

/// Accounts to control who can use a URL Cleaner Site instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Accounts {
    /// A map of usernames to passwords.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub users: HashMap<String, String>,
    /// If [`true`], allow "guest" users.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub allow_guest: bool
}

impl Default for Accounts {
    fn default() -> Self {
        Self {
            users: Default::default(),
            allow_guest: true
        }
    }
}

impl Accounts {
    /// If `auth` is [`Some`], returns true if [`Self::users`] has an entry with the username set to the password.
    ///
    /// If `auth` is [`None`], returns [`Self::allow_guest`].
    pub fn auth(&self, auth: Option<&Auth>) -> bool {
        match auth {
            Some(auth) => self.users.get(&auth.username).is_some_and(|password| &auth.password == password),
            None => self.allow_guest
        }
    }
}

/// A username and password.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Auth {
    /// The username.
    pub username: String,
    /// The password.
    pub password: String
}
