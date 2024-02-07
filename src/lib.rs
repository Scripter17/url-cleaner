//! URL Cleaner - A tool to remove tracking garbage from URLs.

use wasm_bindgen::prelude::*;
use url::Url;
use std::borrow::Cow;

/// The logic for conditions and mappers.
pub mod rules;
/// Wrappers for [`regex::Regex`], [`glob::Pattern`], and [`std::process::Command`].
pub mod glue;
/// Types that don't fit in the other modules.
pub mod types;
/// Deserializing and handling configuration.
pub mod config;

/// Takes a URL, an optional [`config::Config`], an optional [`config::Params`], and returns the result of applying the config and params to the URL.
/// This function's name is set to `clean_url` in WASM for API simplicity.
/// # Errors
/// If the config or params can't be parsed, returns the parsing error.
/// If applying the rules returns an error, that error is returned.
#[wasm_bindgen(js_name = clean_url)]
pub fn wasm_clean_url(url: &str, config: wasm_bindgen::JsValue, params: wasm_bindgen::JsValue) -> Result<JsValue, JsError> {
    let mut url=Url::parse(url)?;
    clean_url(&mut url, Some(js_value_to_config(config)?.as_ref()), Some(&js_value_to_params(params)?))?;
    Ok(JsValue::from_str(url.as_str()))
}

/// Takes a URL, an optional [`config::Config`], an optional [`config::Params`], and returns the result of applying the config and params to the URL.
/// # Errors
/// If applying the rules returns an error, that error is returned.
pub fn clean_url(url: &mut Url, config: Option<&config::Config>, params: Option<&config::Params>) -> Result<(), types::CleaningError> {
    #[allow(clippy::redundant_closure)] // The closures shrink the lifetime of [`config::Config::get_default`] to the lifetime of `config`.
    match params {
        Some(params) => config.map_or_else(|| config::Config::get_default(), Ok)?.apply_with_params(url, params)?,
        None         => config.map_or_else(|| config::Config::get_default(), Ok)?.apply(url)?
    }
    Ok(())
}

fn js_value_to_config(config: wasm_bindgen::JsValue) -> Result<Cow<'static, config::Config>, JsError> {
    Ok(if config.is_null() {
        Cow::Borrowed(config::Config::get_default()?)
    } else {
        Cow::Owned(serde_wasm_bindgen::from_value(config)?)
    })
}

fn js_value_to_params(params: wasm_bindgen::JsValue) -> Result<config::Params, JsError> {
    Ok(if params.is_null() {
        config::Params::default()
    } else {
        serde_wasm_bindgen::from_value(params)?
    })
}
