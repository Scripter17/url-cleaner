//! Macros for URL Cleaner.

use proc_macro::TokenStream;

mod suitability;

/// The derive macro for URL Cleaner's `Suitability` trait.
#[proc_macro_derive(Suitability, attributes(suitable))]
pub fn suitability_derive(input: TokenStream) -> TokenStream {
    suitability::suitability_derive(syn::parse_macro_input!(input)).into()
}
