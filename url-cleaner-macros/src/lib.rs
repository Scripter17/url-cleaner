use proc_macro::TokenStream;

mod suitability;
mod error_names;

#[proc_macro_derive(Suitability, attributes(suitable))]
pub fn suitablility_derive(input: TokenStream) -> TokenStream {
    suitability::suitablility_derive(input)
}

#[proc_macro_derive(ErrorFilter)]
pub fn error_filter_derive(input: TokenStream) -> TokenStream {
    error_names::error_filter_derive(input)
}
