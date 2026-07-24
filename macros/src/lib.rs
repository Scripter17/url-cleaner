//! Macros for URL Cleaner.

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod suitability;
mod get;

/// Get strings and other things briefly.
#[proc_macro]
pub fn get(args: TokenStream) -> TokenStream {
    get::get(parse_macro_input!(args)).into()
}

/// The derive macro for URL Cleaner's `Suitability` trait.
#[proc_macro_derive(Suitability, attributes(suitable))]
pub fn suitability_derive(input: TokenStream) -> TokenStream {
    suitability::suitability_derive(input)
}
