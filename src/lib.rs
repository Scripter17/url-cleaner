#![warn(missing_docs)]
#![warn(clippy::expect_used)] // On a separate line so I can comment it out when using Bacon.
// #![deny(clippy::unwrap_used, clippy::missing_panics_doc)]

//! URL Cleaner - A tool to remove tracking garbage from URLs.

use wasm_bindgen::prelude::*;
use url::Url;
use std::borrow::Cow;

/// Contains the logic for conditions and mappers.
pub mod rules;
/// Contains wrappers for [`regex::Regex`], [`glob::Pattern`], and [`std::process::Command`].
/// In the case their respective features are disabled, the wrappers are empty, always fail deseirlaization, and all of their methods panic.
pub mod glue;
/// Contains logic for handling [`rules::conditions::Condition::UnqualifiedAnyTld`] and [`rules::conditions::Condition::QualifiedAnyTld`].
pub mod suffix;
/// Contains types that don't fit in the other modules.
pub mod types;

/// Takes a URL, an optional [`rules::Rules`], an optional [`types::DomainConditionRule`], and returns the result of applying those rules and that DCR to the URL.
#[wasm_bindgen(js_name = clean_url)]
pub fn wasm_clean_url(url: &str, rules: wasm_bindgen::JsValue, dcr: wasm_bindgen::JsValue) -> Result<JsValue, JsError> {
    let mut url=Url::parse(url)?;
    clean_url(&mut url, Some(js_value_to_rules(rules)?.as_ref()), Some(&js_value_to_dcr(dcr)?))?;
    Ok(JsValue::from_str(url.as_str()))
}

/// Takes a URL, an optional [`rules::Rules`], an optional [`types::DomainConditionRule`], and returns the result of applying those rules and that DCR to the URL.
pub fn clean_url(url: &mut Url, rules: Option<&rules::Rules>, dcr: Option<&types::DomainConditionRule>) -> Result<(), types::CleaningError> {
    match rules {
        Some(rules) => rules.apply_with_dcr(url, dcr.unwrap_or(&types::DomainConditionRule::default()))?, // T implementing Default doesn't mean &T implements Default. :/
        None => rules::get_default_rules()?.apply_with_dcr(url, dcr.unwrap_or(&types::DomainConditionRule::default()))?
    }
    Ok(())
}

fn js_value_to_rules(rules: wasm_bindgen::JsValue) -> Result<Cow<'static, rules::Rules>, JsError> {
    Ok(if rules.is_null() {
        Cow::Borrowed(rules::get_default_rules()?)
    } else {
        Cow::Owned(serde_wasm_bindgen::from_value(rules)?)
    })
}

fn js_value_to_dcr(dcr: wasm_bindgen::JsValue) -> Result<types::DomainConditionRule, JsError> {
    Ok(if dcr.is_null() {
        types::DomainConditionRule::default()
    } else {
        serde_wasm_bindgen::from_value(dcr)?
    })
}
