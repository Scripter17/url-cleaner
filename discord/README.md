# URL Cleaner Discord

[Documentation for URL Cleaner in general](../README.md)

[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner-discord)](https://crates.io/crates/url-cleaner-discord/)

A basic discord app to use URL Cleaner.

Licensed under the Affero General Public License V3 or later.

https://www.gnu.org/licenses/agpl-3.0.html



# Usage

Uses the `URLCD_TOKEN` environment variable to login.

Creates one context menu command for the default profile and one additional context menu command for each named profile.

Entire messages can be cleaned by right clicking/long pressing on them, clicking/tapping on "apps", then on the "Clean URLs" command corresponding to your desired profile.

The app will then reply to the user with an ephemeral message only they can see containing every URL in the message and its embeds after cleaning.

Additionally, the `clean_url` slash command can be used to clean URLs without sending them.

The app will then reply to the user with an ephemeral message only they can see containing the cleaned URL.

## Format

The [standard format](../format.md) is intentionally disregarded due to not being able to send response messages by chunks and the output being meant for users rather than computers.
