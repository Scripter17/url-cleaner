//! Utility stuff.

use syn::{Token, punctuated::Punctuated, Member, parse::{Parse, ParseStream}, Result};
use quote::ToTokens;
use proc_macro2::TokenStream;

/// Like [`syn::Path`] but using [`BetterMember`]s.
#[derive(Debug, Clone)]
pub struct MemberPath {
    /// The leading colon, if any.
    pub leading_colon: Option<Token![::]>,
    /// The segments.
    pub segments: Punctuated<BetterMember, Token![::]>
}

impl Parse for MemberPath {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            leading_colon: input.parse()?,
            segments: Punctuated::parse_separated_nonempty(input)?
        })
    }
}

impl ToTokens for MemberPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.leading_colon.to_tokens(tokens);
        self.segments.to_tokens(tokens);
    }
}

/// Like [`syn::Member`] but better.
#[derive(Debug, Clone)]
pub enum BetterMember {
    /// A [`Member`].
    Member(Member),
    /// A [`struct@syn::token::SelfType`]
    SelfType(Token![Self])
}

impl Parse for BetterMember {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(if let Ok(x) = input.parse() {
            Self::Member(x)
        } else if let Ok(x) = input.parse() {
            Self::SelfType(x)
        } else {
            Err(input.error("hm"))?
        })
    }
}

impl ToTokens for BetterMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Member(x) => x.to_tokens(tokens),
            Self::SelfType(x) => x.to_tokens(tokens)
        }
    }
}
