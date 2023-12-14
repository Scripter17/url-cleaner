mod rules;
mod cleaner;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn clean_url_str(url: &str) -> String {
    cleaner::clean_url_str(url, None).unwrap().to_string()
}
