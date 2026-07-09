//! Getters.

use crate::prelude::*;

impl MyUrl {
    /// The official href getter.
    pub fn canon_get_href(&self) -> &str {
        &self.serialization
    }

    /// The official protocol getter.
    pub fn canon_get_protocol(&self) -> &str {
        &self.serialization[.. self.scheme_mark as usize + 1]
    }

    /// The official username getter.
    pub fn canon_get_username(&self) -> &str {
        self.username()
    }

    /// The official password getter.
    pub fn canon_get_password(&self) -> &str {
        self.password()
    }

    /// The official hostname getter.
    pub fn canon_get_hostname(&self) -> &str {
        self.host().unwrap_or_default()
    }

    /// The official host getter.
    pub fn canon_get_host(&self) -> &str {
        self.host_port().unwrap_or_default()
    }

    /// The official port getter.
    pub fn canon_get_port(&self) -> &str {
        self.port_str().unwrap_or_default()
    }

    /// The official pathname getter.
    pub fn canon_get_pathname(&self) -> &str {
        self.path()
    }

    /// The official search getter.
    pub fn canon_get_search(&self) -> &str {
        let x = match self.query_mark {
            None => "",
            Some(x) => &self.serialization[x.get() as usize .. self.fragment_mark.map_or(self.len(), |x| x.get() as usize)]
        };

        if x == "?" {
            ""
        } else {
            x
        }
    }

    /// The official hash getter.
    pub fn canon_get_hash(&self) -> &str {
        let x = match self.fragment_mark {
            None => "",
            Some(x) => &self.serialization[x.get() as usize ..]
        };

        if x == "#" {
            ""
        } else {
            x
        }
    }
}
