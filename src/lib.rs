use wasm_bindgen::prelude::*;
use url::Url;

pub mod rules;
pub mod glue;
pub mod types;

#[wasm_bindgen]
/// Takes a URL string and optionally a [`JsValue`] containing mapper rules.
/// Returns the mapped URL or any errors raised.
pub fn main(url: &str, rules: wasm_bindgen::JsValue) -> Result<String, JsValue> {
    let rules = if rules.is_null() {
        rules::get_rules(None).or(Err(JsValue::from_str("No default rules in binary")))?
    } else {
        serde_wasm_bindgen::from_value(rules)?
    };
    let mut url=Url::parse(url).unwrap();
    rules.apply(&mut url).unwrap(); // Temp until I get RuleError -> JsValue working
    Ok(url.as_str().to_string())
}
