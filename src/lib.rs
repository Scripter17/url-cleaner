//! URL Cleaner originally started as a project to remove tracking garbage from URLs but has since grown into a very powerful URL manipulation tool.

#[cfg(target_family = "wasm")]
use std::borrow::Cow;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;
use url::Url;

/// The logic for conditions and mappers.
pub mod rules;
/// Wrappers for [`regex::Regex`], [`glob::Pattern`], and [`std::process::Command`].
pub mod glue;
/// Types that don't fit in the other modules.
pub mod types;
pub(crate) mod util;

/// Takes a URL, an optional [`types::Config`], an optional [`types::Params`], and returns the result of applying the config and params to the URL.
/// This function's name is set to `clean_url` in WASM for API simplicity.
/// # Errors
/// If the config or params can't be parsed, returns the parsing error.
/// If applying the rules returns an error, that error is returned.
#[cfg(target_family = "wasm")]
#[wasm_bindgen(js_name = clean_url)]
pub fn wasm_clean_url(url: &str, config: wasm_bindgen::JsValue, params: wasm_bindgen::JsValue) -> Result<JsValue, JsError> {
    let mut url=Url::parse(url)?;
    clean_url(&mut url, Some(js_value_to_config(config)?.as_ref()), Some(&js_value_to_params(params)?))?;
    Ok(JsValue::from_str(url.as_str()))
}

/// Takes a URL, an optional [`types::Config`], an optional [`types::Params`], and returns the result of applying the config and params to the URL.
/// Please note that if an error is returned, the URL is left in a partially modified state.
/// [`rules::Mapper::All`] doesn't apply changes until all the contained mappers work without errors, so at the very least you don't need to worry about that.
/// # Errors
/// If applying the rules returns an error, that error is returned.
/// Please note that if an error is returned, the URL is left in a partially modified state.
/// [`rules::Mapper::All`] doesn't apply changes until all the contained mappers work without errors, so at the very least you don't need to worry about that.
pub fn clean_url(url: &mut Url, config: Option<&types::Config>, params: Option<&types::Params>) -> Result<(), types::CleaningError> {
    let mut config=match config {
        Some(config) => config.clone(),
        None => types::Config::get_default()?.clone()
    };
    if let Some(params) = params {config.params.merge(params.clone());}
    config.apply(url)?;
    Ok(())
}

#[cfg(target_family = "wasm")]
fn js_value_to_config(config: wasm_bindgen::JsValue) -> Result<Cow<'static, types::Config>, JsError> {
    Ok(if config.is_null() {
        Cow::Borrowed(types::Config::get_default()?)
    } else {
        Cow::Owned(serde_wasm_bindgen::from_value(config)?)
    })
}

#[cfg(target_family = "wasm")]
fn js_value_to_params(params: wasm_bindgen::JsValue) -> Result<types::Params, JsError> {
    Ok(if params.is_null() {
        types::Params::default()
    } else {
        serde_wasm_bindgen::from_value(params)?
    })
}
