//! Macros for URL Cleaner.

use proc_macro::TokenStream;

mod suitability;
mod edoc;
mod util;

/// The derive macro for URL Cleaner's `Suitability` trait.
#[proc_macro_derive(Suitability, attributes(suitable))]
pub fn suitability_derive(input: TokenStream) -> TokenStream {
    suitability::suitability_derive(input)
}

/// Error doc generator.
#[proc_macro]
pub fn edoc(args: TokenStream) -> TokenStream {
    edoc::edoc(args)
}
