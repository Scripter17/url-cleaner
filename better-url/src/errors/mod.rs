//! Error types.

mod generic;
mod scheme;
mod userinfo;
mod host;
mod port;
mod path;
mod query;
mod fragment;

pub use generic::*;
pub use scheme::*;
pub use userinfo::*;
pub use host::*;
pub use port::*;
pub use path::*;
pub use query::*;
pub use fragment::*;

/// Implement [`From`] of [`std::convert::Infallible`].
macro_rules! from_infallible {
    ($($t:ty),*) => {
        $(
            impl From<std::convert::Infallible> for $t {
                fn from(value: std::convert::Infallible) -> Self {
                    match value {}
                }
            }
        )*
    }
}

from_infallible!(
    InvalidScheme, SetSchemeError,
    InvalidEmptyPath,
    InvalidHost, SetHostError,
    InvalidDomainSegment, InvalidDomainSegments, SetDomainError
);

#[cfg(test)]
mod tests {
    use crate::prelude::*;


    macro_rules! assert_size_1 {
        ($($t:ty),*) => {
            $(assert_eq!(std::mem::size_of::<$t>(), 1, "{}", stringify!($t));)*
        }
    }

    #[test]
    fn sizes() {
        // TODO: Make SetHostError 1 byte.

        assert_size_1!(
            InvalidHost, InvalidIpHost, SetDomainError,
            SetSchemeError,
            SetUserinfoError, SetUsernameError, SetPasswordError,
            SetPathError,
            SetQueryError
        );
    }
}
