//! Error doc generator.

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, parenthesized, Ident, LitInt, Path, Result, Token, custom_keyword};
use syn::parse::{Parse, ParseStream};
use quote::quote;

/// A list of "calls".
struct Calls {
    /// If [`true`], prepend `- ` to each line.
    listitem: bool,
    /// A "call".
    calls: Vec<Call>
}

impl Parse for Calls {
    fn parse(input: ParseStream) -> Result<Self> {
        custom_keyword!(listitem);
        Ok(Self {
            listitem: if input.peek(listitem) {
                input.parse::<listitem>()?;
                input.parse::<Token![,]>()?;
                true
            } else {
                false
            },
            calls: input.parse_terminated(Call::parse, Token![,])?.into_iter().collect()
        })
    }
}

/// A "call".
struct Call {
    /// The name.
    name: String,
    /// The args.
    args: Option<Args>
}

impl Parse for Call {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            name: input.parse::<Ident>()?.to_string(),
            args: input.parse().ok()
        })
    }
}

/// The args.
struct Args {
    /// The args.
    args: Vec<Arg>
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let args;
        parenthesized!(args in input);
        Ok(Self {
            args: args.parse_terminated(Arg::parse, Token![,])?.into_iter().collect()
        })
    }
}

/// An argument.
#[derive(Debug, Clone)]
enum Arg {
    /// A path
    Path(Path),
    /// An integer.
    Int(u8)
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(if let Ok(path) = input.parse::<Path>() {
            Self::Path(path)
        } else if let Ok(int) = input.parse::<LitInt>() {
            Self::Int(int.base10_parse()?)
        } else {
            Err(input.error("Expected path or integer"))?
        })
    }
}

impl std::fmt::Display for Arg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Path (path)  => write!(fmt, "{}", quote!(#path).to_string().replace(' ', "")),
            Self::Int  (int )  => write!(fmt, "{int}" )
        }
    }
}

/// Thing
fn thing(x: u8) -> &'static str {
    match x {
        0 => panic!("Cannot be 0"),
        1 => "the",
        2 => "either",
        3.. => "any"
    }
}

/// Thing2
fn thing2(name: &str, args: &[Arg]) -> String {
    match (name, args) {
        ("geterr"           , [ty     , Arg::Int(x)]) => format!("If {} call to [`{ty}::get`] returns an error, that error is returned.", thing(*x)),
        ("geterr"           , [ty                  ]) => thing2(name, &[ty.clone(), Arg::Int(1)]),

        ("seterr"           , [ty     , Arg::Int(x)]) => format!("If {} call to [`{ty}::set`] returns an error, that error is returned.", thing(*x)),
        ("seterr"           , [ty                  ]) => thing2(name, &[ty.clone(), Arg::Int(1)]),

        ("getnone"          , [ty, ety, Arg::Int(x)]) => format!("If {} call to [`{ty}::get`] returns [`None`], returns the error [`{ety}Error::{ty}IsNone`].", thing(*x)),
        ("getnone"          , [ty, ety             ]) => thing2(name, &[ty.clone(), ety.clone(), Arg::Int(1)]),
        ("getnone"          , [ty @ Arg::Path(_)   ]) => thing2(name, &[ty.clone(), ty.clone()]),

        ("notfound"         , [ty, ety             ]) => format!("If the [`{ty}`] isn't found, returns the error [`{ety}Error::{ty}NotFound`]."),
        ("notfound"         , [ty @ Arg::Path(_)   ]) => thing2(name, &[ty.clone(), ty.clone()]),

        ("commonnotfound"   , [ty, ety             ]) => format!("If the common [`{ty}`] isn't found, returns the error [`{ety}Error::Common{ety}NotFound`]."),
        ("commonnotfound"   , [ty @ Arg::Path(_)   ]) => thing2(name, &[ty.clone(), ty.clone()]),

        ("callerr"          , [pa     , Arg::Int(x)]) => format!("If {} call to [`{pa}`] returns an error, that error is returned.", thing(*x)),
        ("callerr"          , [pa                  ]) => thing2(name, &[pa.clone(), Arg::Int(1)]),

        ("callnone"         , [pa, epa, Arg::Int(x)]) => format!("If {} call to [`{pa}`] returns [`None`], returns the error [`{epa}`].", thing(*x)),
        ("callnone"         , [pa, epa             ]) => thing2(name, &[pa.clone(), epa.clone(), Arg::Int(1)]),

        ("applyerr"         , [ty     , Arg::Int(x)]) => format!("If {} call to [`{ty}::apply`] returns an error, that error is returned.", thing(*x)),
        ("applyerr"         , [ty                  ]) => thing2(name, &[ty.clone(), Arg::Int(1)]),

        ("applyerrtryelse"  , [ty, ety             ]) => format!("If both calls to [`{ty}::apply`] return errors, both errors are returned in a [`{ety}Error::TryElseError`]."),
        ("applyerrtryelse"  , [ty @ Arg::Path(_)   ]) => thing2(name, &[ty.clone(), ty.clone()]),

        ("applyerrfne"      , [ty, ety             ]) => format!("If all calls to [`{ty}::apply`] return errors, all errors are returned in a [`{ety}Error::FirstNotErrorErrors`]."),
        ("applyerrfne"      , [ty @ Arg::Path(_)   ]) => thing2(name, &[ty.clone(), ty.clone()]),

        ("satisfyerr"       , [ty     , Arg::Int(x)]) => format!("If {} call to [`{ty}::satisfied_by`] returns an error, that error is returned.", thing(*x)),
        ("satisfyerr"       , [ty                  ]) => thing2(name, &[ty.clone(), Arg::Int(1)]),

        ("satisfyerrtryelse", [ty, ety             ]) => format!("If both calls to [`{ty}::satisfied_by`] return errors, both errors are returned in a [`{ety}Error::TryElseError`]."),
        ("satisfyerrtryelse", [ty @ Arg::Path(_)   ]) => thing2(name, &[ty.clone(), ty.clone()]),

        ("satisfyerrfne"    , [ty, ety             ]) => format!("If all calls to [`{ty}::satisfied_by`] return errors, all errors are returned in a [`{ety}Error::FirstNotErrorErrors`]."),
        ("satisfyerrfne"    , [ty @ Arg::Path(_)   ]) => thing2(name, &[ty.clone(), ty.clone()]),
        x => unimplemented!("{x:?}")
    }
}

/// Error doc generator.
#[allow(clippy::useless_format, reason = "Visual consistency.")]
pub(crate) fn edoc(tokens: TokenStream) -> TokenStream {
    let mut ret = String::new();
    let calls = parse_macro_input!(tokens as Calls);
    for call in calls.calls {
        if !ret.is_empty() {ret.push_str("\n\n")}
        if calls.listitem {ret.push_str("- ");}
        ret.push_str(&thing2(&call.name, call.args.map(|args| args.args).as_deref().unwrap_or_default()));
    }
    quote!{#ret}.into()
}
