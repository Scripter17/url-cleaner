use proc_macro::TokenStream;

mod suitability;

#[proc_macro_derive(Suitability, attributes(suitable))]
pub fn suitablility_derive(input: TokenStream) -> TokenStream {
    suitability::suitablility_derive(input)
}
