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

/// 1. If `config` is [`None`], get the [`Config`] from [`types::Config::get_default`].
/// 2. If `params_diff` is [`Some`], call [`types::ParamsDiff::apply`] with the config's [`types::Config::params`].
/// 3. Return the potentially modified config.
/// # Errors
/// If the call to [`types::Config::get_default`] returns an error, that error is returned.
fn get_config<'a>(config: Option<&'a types::Config>, params_diff: Option<&types::ParamsDiff>) -> Result<Cow<'a, types::Config>, types::CleaningError> {
    let mut config=Cow::Borrowed(match config {
        Some(config) => config,
        None => types::Config::get_default()?
    });
    if let Some(params_diff) = params_diff {
        params_diff.apply(&mut config.to_mut().params);
    }
    Ok(config)
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
pub fn clean_url(url: &mut Url, context: Option<&types::UrlContext>, config: Option<&types::Config>, params_diff: Option<&types::ParamsDiff>) -> Result<(), types::CleaningError> {
    let config = get_config(config, params_diff)?;
    #[cfg(feature = "cache")]
    let cache_handler = config.cache_path.as_path().try_into()?;
    config.rules.apply(&mut types::JobState {
        url,
        params: &config.params,
        vars: Default::default(),
        context: &*match context {Some(x) => Cow::Borrowed(x), None => Cow::Owned(types::UrlContext::default())},
        #[cfg(feature = "cache")]
        cache_handler: &cache_handler,
        commons: &config.commons,
        common_vars: None
    })?;

    Ok(())
}

/// Like [`clean_url`] but allows a user-provided [`glue::CacheHandler`].
/// # Errors
/// If the call to [`types::Config::get_default`] returns an error, that error is returned.
/// 
/// If the call to [`types::Rules::apply`] returns an error, that error is returned.
#[cfg(feature = "cache")]
pub fn clean_url_with_cache_handler(url: &mut Url, context: Option<&types::UrlContext>, config: Option<&types::Config>, params_diff: Option<&types::ParamsDiff>, cache_handler: &glue::CacheHandler) -> Result<(), types::CleaningError> {
    let config = get_config(config, params_diff)?;
    config.rules.apply(&mut types::JobState {
        url,
        params: &config.params,
        vars: Default::default(),
        context: &*match context {Some(x) => Cow::Borrowed(x), None => Cow::Owned(types::UrlContext::default())},
        cache_handler,
        commons: &config.commons,
        common_vars: None
    })?;

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
fn js_value_to_params_diff(params_diff: wasm_bindgen::JsValue) -> Result<Option<types::ParamsDiff>, JsError> {
    Ok(serde_wasm_bindgen::from_value(params_diff)?)
}
