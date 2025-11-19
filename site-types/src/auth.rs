//! Authentication.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::util::*;

/// Accounts to control who can use a URL Cleaner Site instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Accounts {
    /// If [`true`], allow "guest" users.
    ///
    /// Defaults to [`true`].
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub allow_guest: bool,
    /// A map of usernames to passwords.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub users: HashMap<String, String>
}

impl Default for Accounts {
    fn default() -> Self {
        Self {
            allow_guest: true,
            users: Default::default()
        }
    }
}

impl Accounts {
    /// If `auth` is [`Some`], returns true if [`Self::users`] has an entry with the username set to the password.
    ///
    /// If `auth` is [`None`], returns [`Self::allow_guest`].
    pub fn check(&self, auth: &Auth) -> bool {
        match auth {
            Auth::Guest => self.allow_guest,
            Auth::User {username, password} => self.users.get(username) == Some(password)
        }
    }
}

/// A username and password.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Auth {
    /// Guest
    Guest,
    /// User
    User {
        /// The username.
        username: String,
        /// The password.
        password: String
    }
}
