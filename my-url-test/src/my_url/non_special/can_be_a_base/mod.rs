use crate::prelude::*;

mod host;
mod no_host;

impl MyUrl {
    pub(super) fn new_can_be_a_base(scheme: Scheme<'_>, rest: &str) -> Result<Self, InvalidUrl> {
        if let Some(rest) = rest.strip_prefix("//") && !rest.is_empty() && !rest.starts_with(['/', '?', '#']) {
            Self::new_can_be_a_base_host(scheme, rest)
        } else {
            Self::new_can_be_a_base_no_host(scheme, rest)
        }
    }
}
