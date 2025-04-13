use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub(crate) fn error_filter_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let original_name = name.to_string().strip_suffix("Error").unwrap().to_string();
    let error_names = format_ident!("{name}Names");
    let filter_name = format_ident!("{name}Filter");

    match input.data {
        Data::Enum(data) => {
            let variants = data.variants.into_iter().map(|x| x.ident).collect::<Vec<_>>();
            quote!{
                #[doc = concat!("The names of errors in [`", stringify!(name), "`]. Used for filtering errors.")]
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Suitability)]
                pub enum #error_names {
                    #(
                        #[doc = concat!("[`", stringify!(#name), "::", stringify!(#variants), "`]")]
                        #variants
                    ),*
                }

                impl #error_names {
                    #[doc = concat!("Gets the name of a [`", stringify!(#name), "`].")]
                    pub fn from_error(error: &#name) -> Self {
                        match error {
                            #(#name::#variants{..} => Self::#variants),*
                        }
                    }

                    #[doc = concat!("Returns [`true`] if the provided [`", stringify!(#name), "`] has the specified name.")]
                    pub fn matches(&self, error: &#name) -> bool {
                        self == &Self::from_error(error)
                    }
                }

                #[doc = concat!("Allows telling error handling [`", #original_name, "`]s which [`", stringify!(#name), "`]s to handle.")]
                #[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Suitability)]
                pub struct #filter_name(pub Option<HashSet<#error_names>>);

                impl #filter_name {
                    /// Returns [`true`] if `error` matches the filter.
                    pub fn matches(&self, error: &#name) -> bool {
                        self.0.as_ref().is_none_or(|names| names.contains(&#error_names::from_error(error)))
                    }
                }
            }
        },
        _ => todo!()
    }.into()
}
