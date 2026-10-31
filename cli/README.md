# URL Cleaner CLI

[Documentation for URL Cleaner in general](../README.md)

[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner)](https://crates.io/crates/url-cleaner/)

A CLI interface for URL Cleaner.

Licensed under the Affero General Public License V3 or later.

https://www.gnu.org/licenses/agpl-3.0.html



# Usage

URLs can be provided as unnmaed CLI arguments and/or as lines of STDIN.

## Format

URL Cleaner CLI uses [the standard format](../format.md) with the following additional guarantees:

- Every result line is succeeded by a `\n`.

- Every `\n` is preceded by a result line.
