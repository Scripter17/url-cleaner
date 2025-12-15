//! [`HttpRequestConfig`].

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Rules for making an HTTP request.
///
/// Currently only capable of making blocking requests.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Suitability)]
#[serde(deny_unknown_fields)]
pub struct HttpRequestConfig {
    /// The URL to send the request to.
    ///
    /// Defaults to [`StringSource::Part`]`(`[`UrlPart::Whole`]`)`.
    #[serde(default = "get_string_source_part_whole", skip_serializing_if = "is_string_source_part_whole")]
    pub url: StringSource,
    /// The method to use.
    ///
    /// Defaults to `"GET"`.
    #[serde(default = "get_string_source_get", skip_serializing_if = "is_string_source_get")]
    pub method: StringSource,
    /// The headers to send that never change.
    ///
    /// If [`None`], does nothing.
    ///
    /// [`Map::if_none`] and [`Map::else`] are ignored.
    ///
    /// Defaults to [`MapSource::None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub const_headers: MapSource,
    /// The headers to send that may change.
    ///
    /// If a call to [`StringSource::get`] returns [`None`], the header it came from isn't sent.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub dynamic_headers: HashMap<String, StringSource>,
    /// The body to send.
    ///
    /// Defaults to [`None`].
    #[serde(default, skip_serializing_if = "is_default")]
    pub body: Option<HttpBodyConfig>
}

impl Default for HttpRequestConfig {
    fn default() -> Self {
        Self {
            url    : UrlPart::Whole.into(),
            method : "GET".into(),
            const_headers: Default::default(),
            dynamic_headers: Default::default(),
            body   : None
        }
    }
}

/// Serde helper function for [`HttpRequestConfig::url`].
fn get_string_source_part_whole() -> StringSource {StringSource::Part(UrlPart::Whole)}
/// Serde helper function for [`HttpRequestConfig::url`].
fn is_string_source_part_whole(value: &StringSource) -> bool {value == &get_string_source_part_whole()}

/// Serde helper function for [`HttpRequestConfig::method`].
fn get_string_source_get() -> StringSource {"GET".into()}
/// Serde helper function for [`HttpRequestConfig::method`].
fn is_string_source_get(value: &StringSource) -> bool {value == &get_string_source_get()}
