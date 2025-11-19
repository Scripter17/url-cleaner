//! The `/` endpoint.

/// The base info to return when getting `/`.
const INFO: &str = concat!("URL Cleaner Site ", env!("CARGO_PKG_VERSION"), r#"
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html
"#, env!("CARGO_PKG_REPOSITORY"), r#"

See /info     to get the ServerInfo.
See /cleaner  to get the Cleaner.
See /profiles to get the ProfilesConfig."#);

/// The `/` endpoint.
#[get("/")]
pub async fn index() -> &'static str {
    &INFO
}

