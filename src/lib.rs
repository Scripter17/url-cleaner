use wasm_bindgen::prelude::*;
use url::Url;
use std::borrow::Cow;
use thiserror::Error;

pub mod rules;
pub mod glue;
pub mod suffix;
pub mod types;

/// Takes a URL string and optionally a [`JsValue`] containing mapper rules.
/// Returns the mapped URL or any errors raised.
#[wasm_bindgen]
pub fn clean_url_str(url: &str, rules: wasm_bindgen::JsValue) -> Result<JsValue, JsError> {
    let mut url=Url::parse(url)?;
    clean_url_with_dcr(&mut url, Some(js_value_to_rules(rules)?.as_ref()), &types::DomainConditionRule::default())?;
    Ok(JsValue::from_str(url.as_str()))
}

/// Takes a URL string and optionally a [`JsValue`] containing mapper rules.
/// Takes a [`types::DomainConditionRule`]
/// Returns the mapped URL or any errors raised.
#[wasm_bindgen]
pub fn clean_url_str_with_dcr(url: &str, rules: wasm_bindgen::JsValue, dcr: wasm_bindgen::JsValue) -> Result<JsValue, JsError> {
    let mut url=Url::parse(url)?;
    clean_url_with_dcr(&mut url, Some(js_value_to_rules(rules)?.as_ref()), &js_value_to_dcr(dcr)?)?;
    Ok(JsValue::from_str(url.as_str()))
}

pub fn clean_url(url: &mut Url, rules: Option<&rules::Rules>) -> Result<(), CleaningError> {
    clean_url_with_dcr(url, rules, &types::DomainConditionRule::default())
}

pub fn clean_url_with_dcr(url: &mut Url, rules: Option<&rules::Rules>, dcr: &types::DomainConditionRule) -> Result<(), CleaningError> {
    suffix::init_tlds();
    match rules {
        Some(rules) => rules.apply_with_dcr(url, dcr)?,
        None => rules::get_default_rules().ok_or(rules::GetRulesError::NoDefaultRules)?.apply_with_dcr(url, dcr)?
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum CleaningError {
    #[error("There was an issue getting the rules.")]
    GetRulesError(#[from] rules::GetRulesError),
    #[error("There was an issue executing a rule.")]
    RuleError(#[from] rules::RuleError)
}

fn js_value_to_rules(rules: wasm_bindgen::JsValue) -> Result<Cow<'static, rules::Rules>, JsError> {
    Ok(if rules.is_null() {
        Cow::Borrowed(rules::get_default_rules().ok_or(JsError::new("URL Cleaner was compiled without default rules."))?)
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
