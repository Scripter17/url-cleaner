//! Macros for URL Cleaner.

use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr};

mod suitability;
mod edoc;
mod util;
mod get;

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

/// `&str`.
#[proc_macro]
pub fn get_str(args: TokenStream) -> TokenStream {
    get::get_str(parse_macro_input!(args as Expr)).into()
}

/// `String`.
#[proc_macro]
pub fn get_string(args: TokenStream) -> TokenStream {
    get::get_string(parse_macro_input!(args as Expr)).into()
}

/// `Cow<'_, str>`.
#[proc_macro]
pub fn get_cow(args: TokenStream) -> TokenStream {
    get::get_cow(parse_macro_input!(args as Expr)).into()
}

/// `Option<&str>`.
#[proc_macro]
pub fn get_option_str(args: TokenStream) -> TokenStream {
    get::get_option_str(parse_macro_input!(args as Expr)).into()
}

/// `Option<String>`.
#[proc_macro]
pub fn get_option_string(args: TokenStream) -> TokenStream {
    get::get_option_string(parse_macro_input!(args as Expr)).into()
}

/// `Option<Cow<'_, str>>`.
#[proc_macro]
pub fn get_option_cow(args: TokenStream) -> TokenStream {
    get::get_option_cow(parse_macro_input!(args as Expr)).into()
}

/// `&'j str`.
#[proc_macro]
pub fn get_new_str(args: TokenStream) -> TokenStream {
    get::get_new_str(parse_macro_input!(args as Expr)).into()
}

/// `Option<&'j str>`.
#[proc_macro]
pub fn get_new_option_str(args: TokenStream ) -> TokenStream {
    get::get_new_option_str(parse_macro_input!(args as Expr)).into()
}

/// `Option<Cow<'j, str>>`.
#[proc_macro]
pub fn get_new_option_cow(args: TokenStream ) -> TokenStream {
    get::get_new_option_cow(parse_macro_input!(args as Expr)).into()
}
