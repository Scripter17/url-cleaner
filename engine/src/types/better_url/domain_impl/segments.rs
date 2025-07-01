//! Implementing domain, subdomain, and domain suffix segment stuff for [`BetterUrl`].

use super::*;

mod domain;
pub use domain::*;
mod subdomain;
pub use subdomain::*;
mod domain_suffix;
pub use domain_suffix::*;
