//! String getters.

use proc_macro2::TokenStream;
use syn::{Expr, parse::{Parse, ParseStream}, Result, Token};
use quote::quote;

/// A call to the [`get`] macro.
pub(crate) struct GetCall {
    /// If it should keep the [`Option`] layer.
    option: bool,
    /// The [`CallMode`].
    mode: CallMode,
    /// If it should be made owned.
    part: bool,
    /// The expression to get from.
    expr: Expr,
}

/// The `get!` mode.
enum CallMode {
    /// `Cow<'_, str>`.
    Normal,
    /// `&str`.
    Deref,
    /// `String`.
    Owned,
}

impl Parse for CallMode {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(if input.parse::<Token![&]>().is_ok() {
            Self::Deref
        } else if input.parse::<Token![*]>().is_ok() {
            Self::Owned
        } else {
            Self::Normal
        })
    }
}

impl Parse for GetCall {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            option: input.parse::<Token![?]>().is_ok(),
            mode: input.parse()?,
            part: input.parse::<Token![!]>().is_ok(),
            expr: input.parse()?,
        })
    }
}

/// Get strings (and other things) briefly.
pub(crate) fn get(x: GetCall) -> TokenStream {
    match x {
        GetCall {option: false, mode: CallMode::Normal, part: false, expr} => quote!(  #expr.get_some     (task_state, args)??),
        GetCall {option: false, mode: CallMode::Normal, part: true , expr} => quote!(  #expr.get_some_part(task_state, args)??),
        GetCall {option: false, mode: CallMode::Deref , part: false, expr} => quote!(&*#expr.get_some     (task_state, args)??),
        GetCall {option: false, mode: CallMode::Deref , part: true , expr} => quote!(&*#expr.get_some_part(task_state, args)??),
        GetCall {option: false, mode: CallMode::Owned , part: false, expr} => quote!(  #expr.get_some     (task_state, args)??.into_owned()),
        GetCall {option: false, mode: CallMode::Owned , part: true , expr} => quote!(  #expr.get_some_part(task_state, args)??.into_owned()),

        GetCall {option: true , mode: CallMode::Normal, part: false, expr} => quote!(#expr.get     (task_state, args)?),
        GetCall {option: true , mode: CallMode::Normal, part: true , expr} => quote!(#expr.get_part(task_state, args)?),
        GetCall {option: true , mode: CallMode::Deref , part: false, expr} => quote!(#expr.get     (task_state, args)?.as_deref()),
        GetCall {option: true , mode: CallMode::Deref , part: true , expr} => quote!(#expr.get_part(task_state, args)?.as_deref()),
        GetCall {option: true , mode: CallMode::Owned , part: false, expr} => quote!(#expr.get     (task_state, args)?.map(Cow::into_owned)),
        GetCall {option: true , mode: CallMode::Owned , part: true , expr} => quote!(#expr.get_part(task_state, args)?.map(Cow::into_owned)),
    }
}
