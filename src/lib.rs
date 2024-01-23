//! URL Cleaner - A tool to remove tracking garbage from URLs.

use wasm_bindgen::prelude::*;
use url::Url;
use std::borrow::Cow;

/// Contains the logic for conditions and mappers.
pub mod rules;
/// Contains wrappers for [`regex::Regex`], [`glob::Pattern`], and [`std::process::Command`].
/// In the case their respective features are disabled, the wrappers are empty, always fail deserialization, and all of their methods panic.
pub mod glue;
/// Contains types that don't fit in the other modules.
pub mod types;

/// Takes a URL, an optional [`rules::Rules`], an optional [`types::DomainConditionRule`], and returns the result of applying those rules and that Config to the URL.
/// This function's name is set to `clean_url` in WASM for API simplicity.
/// # Errors
/// If the rules or config cannot be converted into a [`rules::Rules`] or [`types::RuleConfig`], returns the parsing error.
/// If the [`rules::Rules`] returns an error, that error is returned.
#[wasm_bindgen(js_name = clean_url)]
pub fn wasm_clean_url(url: &str, rules: wasm_bindgen::JsValue, config: wasm_bindgen::JsValue) -> Result<JsValue, JsError> {
    let mut url=Url::parse(url)?;
    clean_url(&mut url, Some(js_value_to_rules(rules)?.as_ref()), Some(&js_value_to_config(config)?))?;
    Ok(JsValue::from_str(url.as_str()))
}

/// Takes a URL, an optional [`rules::Rules`], an optional [`types::DomainConditionRule`], and returns the result of applying those rules and that Config to the URL.
/// # Errors
/// If the [`rules::Rules`] returns an error, that error is returned.
pub fn clean_url(url: &mut Url, rules: Option<&rules::Rules>, config: Option<&types::RuleConfig>) -> Result<(), types::CleaningError> {
    match rules {
        Some(rules) => rules.apply_with_config(url, config.unwrap_or(&types::RuleConfig::default()))?,
        None => rules::get_default_rules()?.apply_with_config(url, config.unwrap_or(&types::RuleConfig::default()))?
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

fn js_value_to_config(config: wasm_bindgen::JsValue) -> Result<types::RuleConfig, JsError> {
    Ok(if config.is_null() {
        types::RuleConfig::default()
    } else {
        serde_wasm_bindgen::from_value(config)?
    })
}
