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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn sizes() {
        assert_eq!(std::mem::size_of::<InvalidHost     >(), 1);
        assert_eq!(std::mem::size_of::<InvalidIpHost   >(), 1);

        assert_eq!(std::mem::size_of::<SetSchemeError  >(), 1);
        assert_eq!(std::mem::size_of::<SetUserinfoError>(), 1);
        assert_eq!(std::mem::size_of::<SetUsernameError>(), 1);
        assert_eq!(std::mem::size_of::<SetPasswordError>(), 1);
        assert_eq!(std::mem::size_of::<SetHostError    >(), 1);
        assert_eq!(std::mem::size_of::<SetDomainError  >(), 1);
        assert_eq!(std::mem::size_of::<SetPathError    >(), 1);
        assert_eq!(std::mem::size_of::<SetQueryError   >(), 1);
    }
}
