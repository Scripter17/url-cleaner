# URL Cleaner CLI

[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner)](https://crates.io/crates/url-cleaner/)

[Documentation for URL Cleaner in general](../README.md)

The CLI interface for URL Cleaner.

Licensed under the Affero General Public License V3 or later.

https://www.gnu.org/licenses/agpl-3.0.html

## Format

URL Cleaner CLI uses [the standard format](../format.md) with the following additional guarantees:

- Each result line is succeeded by a `\n`.

- Each `\n` is precceded by a result line.

- Providing a task line, waiting for its result line, then providing another task line will never deadlock.
