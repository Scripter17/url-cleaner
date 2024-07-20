//! URL Cleaner originally started as a project to remove tracking garbage from URLs but has since grown into a very powerful URL manipulation tool.

use std::borrow::Cow;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;
use url::Url;

pub mod glue;
pub mod types;
pub(crate) mod util;

/// Takes a URL, an optional [`types::Config`], an optional [`types::Params`], and returns the result of applying the config and params to the URL.
/// 
/// This function's name is set to `clean_url` in WASM for API simplicity.
/// # Errors
/// If the config or params can't be parsed, returns the parsing error.
/// If applying the rules returns an error, that error is returned.
#[cfg(target_family = "wasm")]
#[wasm_bindgen(js_name = clean_url)]
pub fn wasm_clean_url(url: &str, config: wasm_bindgen::JsValue, params_diff: wasm_bindgen::JsValue) -> Result<JsValue, JsError> {
    let mut url=Url::parse(url)?;
    clean_url(&mut url, Some(js_value_to_config(config)?.as_ref()), js_value_to_params_diff(params_diff)?)?;
    Ok(JsValue::from_str(url.as_str()))
}

/// Takes a URL, an optional [`types::Config`], an optional [`types::Params`], and returns the result of applying the config and params to the URL.
/// 
/// If an error is returned, the `url` is left unmodified.
/// # Errors
/// If the call to [`types::Config::get_default`] returns an error, that error is returned.
/// 
/// If creating a [`glue::CacheHandler`] returns an error, that error is returned.
/// 
/// If the call to [`clean_url_with_cache_handler`] returns an error, that error is returned.
pub fn clean_url(url: &mut Url, config: Option<&types::Config>, params_diff: Option<&types::ParamsDiff>) -> Result<(), types::CleaningError> {
    let config=match config {
        Some(config) => config,
        None => types::Config::get_default()?
    };
    let cache_handler = config.cache_path.as_path().try_into()?;
    clean_url_with_cache_handler(url, Some(config), params_diff, &cache_handler)
}

/// # Errors
/// If the call to [`types::Config::get_default`] returns an error, that error is returned.
/// 
/// If the call to [`types::Rules::apply`] returns an error, that error is returned.
pub fn clean_url_with_cache_handler(url: &mut Url, config: Option<&types::Config>, params_diff: Option<&types::ParamsDiff>, cache_handler: &glue::CacheHandler) -> Result<(), types::CleaningError> {
    let mut config=Cow::Borrowed(match config {
        Some(config) => config,
        None => types::Config::get_default()?
    });
    if let Some(params_diff) = params_diff {
        params_diff.apply(&mut config.to_mut().params);
    }
    config.rules.apply(&mut types::JobState {
        url,
        params: &config.params,
        vars: Default::default(),
        cache_handler
    })?;

    Ok(())
}

/// # Errors
/// If the call to [`types::Config::get_default`] returns an error, that error is returned.
/// 
/// If creating a [`glue::CacheHandler`] returns an error, that error is returned.
/// 
/// If a call to [`clean_url_with_cache_handler`] returns an error, that error is returned.
pub fn clean_owned_urls<T: IntoIterator<Item = Url>>(urls: T, config: Option<&types::Config>, params_diff: Option<&types::ParamsDiff>) -> Result<Vec<Url>, types::CleaningError> {
    let mut config=Cow::Borrowed(match config {
        Some(config) => config,
        None => types::Config::get_default()?
    });
    if let Some(params_diff) = params_diff {
        params_diff.apply(&mut config.to_mut().params);
    }
    let cache_handler = config.cache_path.as_path().try_into()?;
    urls
        .into_iter()
        .map(|mut url| {clean_url_with_cache_handler(&mut url, Some(&*config), None, &cache_handler)?; Ok(url)})
        .collect()
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
fn js_value_to_params_diff(params_diff: wasm_bindgen::JsValue) -> Result<Option<types::ParamsDiff>, JsError> {
    Ok(serde_wasm_bindgen::from_value(params_diff)?)
}
