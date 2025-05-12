//! The derive macro for URL Cleaner's `Suitability` trait.

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::*;
use syn::*;
use syn::parse::*;
use syn::ext::IdentExt;

/// Value for a `#[suitable = "..."]` override.
enum SuitabilityOverride {
    /// Never suitable.
    Never,
    /// Always suitable.
    Always,
    /// Suitable if and only if the specified function doesn't panic.
    Assert(Path)
}

impl Parse for SuitabilityOverride {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.call(Ident::parse_any)?;
        Ok(if name == "never" {
            Self::Never
        } else if name == "always" {
            Self::Always
        } else if name == "assert" {
            <Token![=]>::parse(input)?;
            Self::Assert(<LitStr as Parse>::parse(input)?.parse()?)
        } else {
            Err(Error::new(input.span(), "Unknwon suitability override"))?
        })
    }
}

/// Getsa [`SuitabilityOverride`] from an item's attributes.
fn get_suitability_override(attrs: &[Attribute]) -> Result<Option<SuitabilityOverride>> {
    attrs.iter().find(|attr| attr.path().is_ident("suitable")).map(|attr| attr.parse_args()).transpose()
}

/// The derive macro for URL Cleaner's `Suitability` trait.
pub(crate) fn suitablility_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let logic = match get_suitability_override(&input.attrs) {
        Ok(Some(SuitabilityOverride::Never))        => quote!{panic!("{} is never suitable", stringify!(#name));},
        Ok(Some(SuitabilityOverride::Always))       => quote!{},
        Ok(Some(SuitabilityOverride::Assert(func))) => quote!{#func(self, config);},
        Ok(None) => match input.data {
            Data::Struct(data) => {
                let x = data.fields.into_iter().enumerate().map(|(i, Field {attrs, ident, ..})| {
                    let member = match ident {
                        Some(ident) => Member::Named(ident),
                        None => Member::Unnamed(i.into())
                    };

                    match get_suitability_override(&attrs) {
                        Ok(Some(SuitabilityOverride::Never))        => quote!{panic!("{} is never suitable", stringify!(#name.#member));},
                        Ok(Some(SuitabilityOverride::Always))       => quote!{},
                        Ok(Some(SuitabilityOverride::Assert(func))) => quote!{#func(self.#member, config);},
                        Ok(None)                                    => quote!{self.#member.assert_suitability(config);},
                        Err(e)                                      => e.into_compile_error()
                    }
                });

                quote!{#(#x)*}
            },
            Data::Enum(data) => {
                let x = data.variants.into_iter().map(|Variant {attrs, ident: variant, fields, ..}| {
                    let names = fields.members().map(|member| match member {
                        Member::Named(ident) => ident.clone(),
                        Member::Unnamed(index) => format_ident!("field_{}", index)
                    });

                    let captures = match fields {
                        Fields::Named(_) => quote!{{#(#names),*}},
                        Fields::Unnamed(_) => quote!{(#(#names),*)},
                        Fields::Unit => quote!{}
                    };

                    match get_suitability_override(&attrs) {
                        Ok(Some(SuitabilityOverride::Never))        => quote!{Self::#variant #captures => panic!("{} is never suitable", stringify!(#name::#variant))},
                        Ok(Some(SuitabilityOverride::Always))       => quote!{Self::#variant #captures => {}},
                        Ok(Some(SuitabilityOverride::Assert(func))) => quote!{Self::#variant #captures => #func(self, config)},
                        Ok(None) => {
                            let x = fields.iter().enumerate().map(|(i, Field {attrs, ident, ..})| {
                                let ident = match ident {
                                    Some(ident) => ident.clone(),
                                    None => format_ident!("field_{}", i)
                                };

                                match get_suitability_override(attrs) {
                                    Ok(Some(SuitabilityOverride::Never))        => quote!{panic!("{} is never suitable", stringify!(#name::#variant.#ident));},
                                    Ok(Some(SuitabilityOverride::Always))       => quote!{},
                                    Ok(Some(SuitabilityOverride::Assert(func))) => quote!{#func(#ident, config);},
                                    Ok(None)                                    => quote!{#ident.assert_suitability(config);},
                                    Err(e)                                      => e.into_compile_error()
                                }
                            });
                            quote!{Self::#variant #captures => {#(#x)*}}
                        },
                        Err(e) => e.into_compile_error()
                    }
                });

                quote!{match self {#(#x),*}}
            },
            Data::Union(_) => unimplemented!()
        },
        Err(e) => return e.into_compile_error().into()
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let types = input.generics.type_params().map(|TypeParam {ident, ..}| ident);

    let expanded = quote! {
        #[allow(unused_variables)]
        impl #impl_generics Suitability for #name #ty_generics #where_clause where #(#types: ::std::fmt::Debug + Suitability),* {
            fn assert_suitability(&self, config: &crate::types::Cleaner) {
                #logic
            }
        }
    };

    expanded.into()
}

