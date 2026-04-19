//! String getters.

use proc_macro2::TokenStream;
use syn::Expr;
use quote::quote;

/// `&str`.
pub(crate) fn get_str(x: Expr) -> TokenStream {
    quote!(&*get_cow!(#x))
}

/// `String`.
pub(crate) fn get_string(x: Expr) -> TokenStream {
    quote!(get_cow!(#x).into_owned())
}

/// `Cow<'_, str>`.
pub(crate) fn get_cow(x: Expr) -> TokenStream {
    quote!(match #x.get_self() {
        StringSource::String(value) => std::borrow::Cow::Borrowed(value.as_str()),
        value => value.get(task_state)?.ok_or(StringSourceIsNone)?
    })
}



/// `Option<&str>`.
pub(crate) fn get_option_str(x: Expr) -> TokenStream {
    quote!(get_option_cow!(#x).as_deref())
}

/// `Option<String>`.
pub(crate) fn get_option_string(x: Expr) -> TokenStream {
    quote!(get_option_cow!(#x).map(|x| x.into_owned()))
}

/// `Option<Cow<'_, str>>`.
pub(crate) fn get_option_cow(x: Expr) -> TokenStream {
    quote!(match #x.get_self() {
        StringSource::None => None,
        StringSource::String(value) => Some(std::borrow::Cow::Borrowed(value.as_str())),
        value => value.get(task_state)?
    })
}



/// `&'j str`.
pub(crate) fn get_new_str(x: Expr) -> TokenStream {
    quote!(&*match #x {
        StringSource::String(value) => std::borrow::Cow::Borrowed(value.as_str()),
        value => Cow::Owned(get_string!(value))
    })
}



/// `Option<&'j str>`.
pub(crate) fn get_new_option_str(x: Expr) -> TokenStream {
    quote!(get_new_option_cow!(#x).as_deref())
}

/// `Option<Cow<'j, str>>`.
pub(crate) fn get_new_option_cow(x: Expr) -> TokenStream {
    quote!(match #x {
        StringSource::String(value) => Some(std::borrow::Cow::Borrowed(value.as_str())),
        StringSource::None => None,
        value => get_option_string!(value).map(Cow::Owned)
    })
}
