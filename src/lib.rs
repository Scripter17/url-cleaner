use wasm_bindgen::prelude::*;
use url::Url;
use std::borrow::Cow;

pub mod rules;
pub mod glue;
pub mod suffix;
pub mod types;

#[wasm_bindgen]
pub fn clean_url_str(url: &str, rules: wasm_bindgen::JsValue) -> Result<String, JsError> {
    _clean_url_str_with_dcr(url, js_value_to_rules(rules)?.as_ref(), &types::DomainConditionRule::default())
}

#[wasm_bindgen]
/// Takes a URL string and optionally a [`JsValue`] containing mapper rules.
/// Returns the mapped URL or any errors raised.
pub fn clean_url_str_with_dcr(url: &str, rules: wasm_bindgen::JsValue, dcr: wasm_bindgen::JsValue) -> Result<String, JsError> {
    _clean_url_str_with_dcr(url, js_value_to_rules(rules)?.as_ref(), &serde_wasm_bindgen::from_value(dcr)?)
}

fn _clean_url_str_with_dcr(url: &str, rules: &rules::Rules, dcr: &types::DomainConditionRule) -> Result<String, JsError> {
    suffix::init_tlds();
    let mut url=Url::parse(url)?;
    rules.apply_with_dcr(&mut url, dcr)?;
    Ok(url.to_string())
}

fn js_value_to_rules(rules: wasm_bindgen::JsValue) -> Result<Cow<'static, rules::Rules>, JsError> {
    Ok(if rules.is_null() {
        Cow::Borrowed(rules::get_default_rules().ok_or(JsError::new("URL Cleaner was compiled without default rules."))?)
    } else {
        Cow::Owned(serde_wasm_bindgen::from_value(rules)?)
    })
}
