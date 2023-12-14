use url::{Url, ParseError};
use crate::rules::Rule;
use serde_json;
use std::include_str;
use std::sync::OnceLock;
use const_str;

const RULES_STR: &str=const_str::replace!(const_str::replace!(include_str!("../config.json"), '\t', ""), '\n', "");
static RULES: OnceLock<Vec<Rule>>=OnceLock::new();

fn get_default_rules() -> &'static [Rule] {
    RULES.get_or_init(|| {
        serde_json::from_str(RULES_STR).unwrap()
    })
}

#[derive(Debug)]
pub enum CleaningError {
    UrlParseError(ParseError)
}

impl From<ParseError> for CleaningError {
    fn from(value: ParseError) -> Self {
        Self::UrlParseError(value)
    }
}

pub fn clean_url_str(url: &str, rules: Option<&[Rule]>) -> Result<Url, CleaningError> {
    clean_url(Url::parse(url)?, rules)
}

pub fn clean_url(mut url: Url, rules: Option<&[Rule]>) -> Result<Url, CleaningError> {
    for rule in rules.unwrap_or_else(|| get_default_rules()) {
        let _=rule.apply(&mut url);
    }
    Ok(url)
}
