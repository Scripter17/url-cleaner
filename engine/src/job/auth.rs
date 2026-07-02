//! [`AuthInfo`] and [`AuthMode`].

use crate::prelude::*;

/// Simple system for blocking actually cleaning URLs behind passwords/username and password pairs.
///
/// Used in [`Job::secrets`]'s [`Secrets::auth_info`].
///
/// Should never be used to block getting the [`Cleaner`] and [`ProfilesConfig`].
///
/// Defaults to [`Self::None`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Suitability)]
pub enum AuthInfo {
    /// Requires the user provide no username or password.
    #[default]
    None,
    /// Requires the user provide no username and a password in the set.
    Password(HashSet<String>),
    /// Requires the user provide a username whose value in the map is the password.
    Userinfo(HashMap<String, String>),
}

/// The type of [`AuthInfo`] being used in a format that can be sent to users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
pub enum AuthMode {
    /// [`AuthInfo::None`].
    None,
    /// [`AuthInfo::Password`].
    Password,
    /// [`AuthInfo::Userinfo`].
    Userinfo,
}

impl AuthInfo {
    /// The [`AuthMode`].
    pub fn mode(&self) -> AuthMode {
        match self {
            Self::None        => AuthMode::None    ,
            Self::Password(_) => AuthMode::Password,
            Self::Userinfo(_) => AuthMode::Userinfo,
        }
    }

    /// Check that the username/password pair is valid.
    ///
    /// If [`Self::None`], requires that both `username` and `password` are [`None`].
    ///
    /// If [`Self::Password`], requires that `username` is [`None`] and `password` is [`Some`] and in the set.
    ///
    /// If [`Self::Userinfo`], requires that `username` and `password` are [`Some`] and `username`'s entry in the map is `password`.
    pub fn check(&self, username: Option<&str>, password: Option<&str>) -> bool {
        match (self, username, password) {
            (Self::None       , None          , None          ) => true,
            (Self::Password(x), None          , Some(password)) => x.contains(password),
            (Self::Userinfo(x), Some(username), Some(password)) => x.get(username).is_some_and(|y| y == password),
            _ => false
        }
    }
}

impl AuthMode {
    /// The name of the mode as a [`str`].
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None     => "None"    ,
            Self::Password => "Password",
            Self::Userinfo => "Userinfo",
        }
    }
}
