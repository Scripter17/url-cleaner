//! Error doc generator.

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, parenthesized, Ident, LitInt, Result, Error, Token, custom_keyword};
use syn::parse::{Parse, ParseStream};
use quote::quote;
use proc_macro2::Span;

use crate::util::*;

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
    /// The span.
    span: Span,
    /// The name.
    name: String,
    /// The args.
    args: Option<Args>
}

impl Parse for Call {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        Ok(Self {
            span: name.span(),
            name: name.to_string(),
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
    Path(MemberPath),
    /// An integer.
    Int(u8),
    /// A literal string. Used for shorthand.
    String(String)
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        // I wonder if there's any way to make this not a huge if-let-else chain, like how match works for if-else chains.
        Ok(if let Ok(int) = input.parse::<LitInt>() {
            Self::Int(int.base10_parse()?)
        } else if let Ok(path) = input.parse() {
            Self::Path(path)
        } else {
            Err(input.error("Expected path or integer"))?
        })
    }
}

impl std::fmt::Display for Arg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Path  (path  ) => write!(fmt, "{}", quote!(#path).to_string().replace(' ', "")),
            Self::Int   (int   ) => write!(fmt, "{int}" ),
            Self::String(string) => write!(fmt, "{string}")
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
fn thing2(span: Span, name: &str, args: &[Arg]) -> Result<String> {
    Ok(match (name, args) {
        ("callerr"              , [pa     , epa @ Arg::Path(_), Arg::Int(x)]) => format!("If {} call to [`{pa}`] returns an error, returns the error [`{epa}`].", thing(*x)),
        ("callerr"              , [pa     , epa @ Arg::Path(_)             ]) => thing2(span, name, &[pa.clone(), epa.clone(), Arg::Int(1)])?,

        ("acallerr"             , [pa     , item                           ]) => format!("If [`{item}`]'s call to [`{pa}`] returns an error, that error is returned."),
        ("callerr"              , [pa     , Arg::Int(x)                    ]) => format!("If {} call to [`{pa}`] returns an error, that error is returned.", thing(*x)),
        ("callerr"              , [pa                                      ]) => thing2(span, name, &[pa.clone(), Arg::Int(1)])?,

        ("acallnone"            , [pa, epa, item @ Arg::Path(_)            ]) => format!("If [`{item}`]'s call to [`{pa}`] returns [`None`], returns the error [`{epa}`]."),
        ("callnone"             , [pa, epa, Arg::Int(x)                    ]) => format!("If {} call to [`{pa}`] returns [`None`], returns the error [`{epa}`].", thing(*x)),
        ("callnone"             , [pa, epa                                 ]) => thing2(span, name, &[pa.clone(), epa.clone(), Arg::Int(1)])?,

        ("callerrte"            , [pa, epa                                 ]) => format!("If both calls to [`{pa}`] return errors, both errors are returned in a [`{epa}Error::TryElseError`]."),
        ("callerrfne"           , [pa, epa                                 ]) => format!("If all calls to [`{pa}`] return errors, all errors are returned in a [`{epa}Error::FirstNotErrorErrors`]."),

        ("notfound"             , [pa, epa                                 ]) => format!("If the [`{pa}`] isn't found, returns the error [`{epa}Error::{pa}NotFound`]."),
        ("commonnotfound"       , [pa, epa                                 ]) => format!("If the common [`{pa}`] isn't found, returns the error [`{epa}Error::Common{epa}NotFound`]."),
        ("commoncallargnotfound", [pa, epa                                 ]) => format!("If the common call arg [`{pa}`] isn't found, returns the error [`{epa}Error::CommonCallArg{epa}NotFound`]."),

        ("stringisnone"         , [epa                                     ]) => format!("If the string is [`None`], returns the error [`{epa}Error::StringIsNone`]"),

        ("ageterr"              , [pa     , item                           ]) => thing2(span, "acallerr"  , &[Arg::String(format!("{pa}::get"))         , item.clone()             ])?,
        ("geterr"               , [pa     , x @ Arg::Int(_)                ]) => thing2(span, "callerr"   , &[Arg::String(format!("{pa}::get"))         , x.clone()                ])?,
        ("geterr"               , [pa                                      ]) => thing2(span, name        , &[pa.clone()                                , Arg::Int(1)              ])?,
        ("geterrte"             , [pa, epa                                 ]) => thing2(span, "callerrte" , &[Arg::String(format!("{pa}::get"))         , epa.clone()              ])?,
        ("geterrfne"            , [pa, epa                                 ]) => thing2(span, "callerrfne", &[Arg::String(format!("{pa}::get"))         , epa.clone()              ])?,

        ("agetnone"             , [pa, epa, item                           ]) => thing2(span, "acallnone" , &[Arg::String(format!("{pa}::get"))         , epa.clone(), item.clone()])?,
        ("getnone"              , [pa, epa, x @ Arg::Int(_)                ]) => thing2(span, "callnone"  , &[Arg::String(format!("{pa}::get"))         , epa.clone(), x.clone()   ])?,
        ("getnone"              , [pa, epa                                 ]) => thing2(span, name        , &[pa.clone()                                , epa.clone(), Arg::Int(1) ])?,

        ("seterr"               , [pa     , x @ Arg::Int(_)                ]) => thing2(span, "callerr"   , &[Arg::String(format!("{pa}::set"))         , x.clone()                ])?,
        ("seterr"               , [pa                                      ]) => thing2(span, name        , &[pa.clone()                                , Arg::Int(1)              ])?,

        ("satisfyerr"           , [pa     , x @ Arg::Int(_)                ]) => thing2(span, "callerr"   , &[Arg::String(format!("{pa}::satisfied_by")), x.clone()                ])?,
        ("satisfyerr"           , [pa                                      ]) => thing2(span, name        , &[pa.clone()                                , Arg::Int(1)              ])?,
        ("satisfyerrte"         , [pa, epa                                 ]) => thing2(span, "callerrte" , &[Arg::String(format!("{pa}::satisfied_by")), epa.clone()              ])?,
        ("satisfyerrfne"        , [pa, epa                                 ]) => thing2(span, "callerrfne", &[Arg::String(format!("{pa}::satisfied_by")), epa.clone()              ])?,

        ("applyerr"             , [pa     , x @ Arg::Int(_)                ]) => thing2(span, "callerr"   , &[Arg::String(format!("{pa}::apply"))       , x.clone()                ])?,
        ("applyerr"             , [pa                                      ]) => thing2(span, name        , &[pa.clone()                                , Arg::Int(1)              ])?,
        ("applyerrte"           , [pa, epa                                 ]) => thing2(span, "callerrte" , &[Arg::String(format!("{pa}::apply"))       , epa.clone()              ])?,
        ("applyerrfne"          , [pa, epa                                 ]) => thing2(span, "callerrfne", &[Arg::String(format!("{pa}::apply"))       , epa.clone()              ])?,

        x => Err(Error::new(span, format!("Invalid: {x:?}")))?
    })
}

/// Error doc generator.
#[allow(clippy::useless_format, reason = "Visual consistency.")]
pub(crate) fn edoc(tokens: TokenStream) -> TokenStream {
    let mut ret = String::new();
    let calls = parse_macro_input!(tokens as Calls);
    for call in calls.calls {
        if !ret.is_empty() {ret.push_str("\n\n")}
        if calls.listitem {ret.push_str("- ");}
        match thing2(call.span, &call.name, call.args.map(|args| args.args).as_deref().unwrap_or_default()) {
            Ok(x) => ret.push_str(&x),
            Err(e) => return e.into_compile_error().into()
        }
    }
    quote!{#ret}.into()
}
