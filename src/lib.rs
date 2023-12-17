use wasm_bindgen::prelude::*;
use url::Url;

mod rules;
mod glue;

#[wasm_bindgen]
pub fn main(url: &str, rules: wasm_bindgen::JsValue) -> Result<String, JsValue> {
    let rules = if rules.is_null() {
        rules::get_default_rules().ok_or(JsValue::from_str("No default rules in binary"))?
    } else {
        serde_wasm_bindgen::from_value(rules)?
    };
    let mut url=Url::parse(url).unwrap();
    rules.apply(&mut url).unwrap(); // Temp until I get RuleError -> JsValue working
    Ok(url.as_str().to_string())
}
