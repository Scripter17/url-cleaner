//! The derive macro for URL Cleaner's `Suitability` trait.

use proc_macro2::TokenStream;

use quote::*;
use syn::*;
use syn::parse::*;

/// Keywords.
mod kw {
    syn::custom_keyword!(never);
    syn::custom_keyword!(always);
    syn::custom_keyword!(assert);
}

/// Value for a `#[suitable = "..."]` override.
#[expect(unused, reason = "For ease of parsing.")]
enum Mode {
    /// Normal.
    Normal,
    /// Never suitable.
    Never {
        /// The `never` token.
        never_token: kw::never,
    },
    /// Always suitable.
    Always {
        /// The `always` token.
        always_token: kw::always,
    },
    /// Suitable if and only if the specified function doesn't panic.
    Assert {
        /// The `assert` token.
        assert_token: kw::assert,
        /// The `=` token.
        eq_token: Token![=],
        /// The function expression.
        func: Expr,
    },
}

impl Parse for Mode {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        Ok(if lookahead.peek(kw::never) {
            Mode::Never {
                never_token: input.parse()?,
            }
        } else if lookahead.peek(kw::always) {
            Mode::Always {
                always_token: input.parse()?,
            }
        } else if lookahead.peek(kw::assert) {
            Mode::Assert {
                assert_token: input.parse()?,
                eq_token    : input.parse()?,
                func        : input.parse()?,
            }
        } else {
            Err(input.error("Unknown suitability mode override"))?
        })
    }
}

/// Gets a [`Mode`] from an item's attributes.
fn get_suitability_override(attrs: &[Attribute]) -> Result<Mode> {
    let mut ret = Mode::Normal;

    for attr in attrs {
        if attr.path().is_ident("suitable") {
            ret = attr.parse_args()?;
        }
    }

    Ok(ret)
}

/// The derive macro for URL Cleaner's `Suitability` trait.
pub(crate) fn suitability_derive(mut input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    let logic = match get_suitability_override(&input.attrs) {
        Ok(Mode::Never  {      ..}) => quote!{panic!("{} is never suitable", stringify!(#name));},
        Ok(Mode::Always {      ..}) => quote!{},
        Ok(Mode::Assert {func, ..}) => quote!{#func(self, cleaner);},
        Ok(Mode::Normal           ) => match input.data {
            Data::Struct(data) => {
                let asserts = data.fields.iter().zip(data.fields.members()).map(|(Field {attrs, ..}, ident)| {
                    match get_suitability_override(attrs) {
                        Ok(Mode::Always {      ..}) => quote!{},
                        Ok(Mode::Never  {      ..}) => quote!{panic!("{}::{} is never suitable", stringifiy!(#name), stringify!(#ident));},
                        Ok(Mode::Assert {func, ..}) => quote!{#func(&self.#ident, cleaner);},
                        Ok(Mode::Normal           ) => quote!{Suitability::assert_suitability(&self.#ident, cleaner);},
                        Err(e) => e.into_compile_error(),
                    }
                });

                quote!{#(#asserts)*}
            },
            Data::Enum(data) => {
                let variants = data.variants.iter().map(|Variant {ident: variant, fields, attrs, ..}| {
                    let members = fields.members().map(|member| match member {
                        Member::Named  (ident            ) => ident,
                        Member::Unnamed(Index {index, ..}) => format_ident!("field_{index}"),
                    });

                    let body = match get_suitability_override(attrs) {
                        Ok(Mode::Never  {..}) => quote!{panic!("{}::{} is never suitable", stringify!(#name), stringify!(#variant))},
                        Ok(Mode::Always {..}) => quote!{},
                        Ok(Mode::Assert {..}) => unimplemented!("Suitability cannot use asserts for enum variants."),
                        Ok(Mode::Normal     ) => {
                            let asserts = fields.iter().zip(members.clone()).map(|(Field {attrs, ..}, ident)| {
                                match get_suitability_override(attrs) {
                                    Ok(Mode::Always {      ..}) => quote!{},
                                    Ok(Mode::Never  {      ..}) => quote!{panic!("{}::{}::{} is never suitable", stringify!(#ident), stringify!(#variant), stringify!(#ident));},
                                    Ok(Mode::Assert {func, ..}) => quote!{#func(#ident, cleaner);},
                                    Ok(Mode::Normal)            => quote!{Suitability::assert_suitability(#ident, cleaner);},
                                    Err(e) => e.into_compile_error(),
                                }
                            });

                            quote!{{#(#asserts)*}}
                        },
                        Err(e) => e.into_compile_error(),
                    };

                    match fields {
                        Fields::Named  (_) => quote!{Self::#variant {#(#members),*} => #body,},
                        Fields::Unnamed(_) => quote!{Self::#variant (#(#members),*) => #body,},
                        Fields::Unit       => quote!{Self::#variant                 => #body,},
                    }
                });

                quote!{match self {#(#variants)*}}
            },
            Data::Union(_) => unimplemented!("Suitability cannot be derived for unions."),
        },
        Err(e) => e.into_compile_error(),
    };

    for param in input.generics.params.iter_mut() {
        if let GenericParam::Type(r#type) = param {
            r#type.colon_token.get_or_insert_default();
            r#type.bounds.push(syn::parse_str("Suitability").expect("???"));
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics crate::prelude::Suitability for #name #ty_generics #where_clause {
            fn assert_suitability(&self, cleaner: &crate::prelude::Cleaner) {
                #logic
            }
        }
    }
}
