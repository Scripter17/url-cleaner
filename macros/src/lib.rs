//! Macros for URL Cleaner.
//!
//! Licensed under the Affero General Public License V3 or later
//!
//! <https://www.gnu.org/licenses/agpl-3.0.html>
//!
//! <https://github.com/Scripter17/url-cleaner>

use proc_macro::TokenStream;

mod suitability;

/// The derive macro for URL Cleaner's `Suitability` trait.
#[proc_macro_derive(Suitability, attributes(suitable))]
pub fn suitability_derive(input: TokenStream) -> TokenStream {
    suitability::suitability_derive(syn::parse_macro_input!(input)).into()
}
