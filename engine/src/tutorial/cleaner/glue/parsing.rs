//! # [Parsing](crate::glue::parse)
//!
//! Some websites, mainly redirect websites that think they're too good to send an HTTP 301 response, need to have their contents parsed for URL cleaner to clean their URLs.
//!
//! ## [Javascript string literals](parse::js::string_literal)
//!
//! The JSSL parser takes a string that begins with a javascript string literal and returns the contents of that string.
//!
//! For example, the input `"\x41BC", "something_else": "whatever"` returns the string `ABC`.
//!
//! If an incomplete or invalid string literal is provided, an error is returned.
//!
//! ## [HTML attributes](parse::html::get_attribute)
//!
//! The get HTML attribute parser takes a string that begins with an HTML opening tag and the name of the attribute to get, then returns the value of the last instance of that attribute.
//!
//! For example, the input `<a href="ignored" href="&#x41;BC">link text` returns the string `ABC`.
//!
//! If an incomplete or invalid HTML opening tag is provided, an error is returned.
//!
//! ## [HTML unescape](parse::html::unescape)
//!
//! Used in [HTML attributes](#html-attributes), the HTML unescape parser replaces escape codes with what they escape.
//!
//! This handles decimal number escape codes (`&#65;`), hexadecimal number escape codes (`&#x41;`), and the over 2000 named escape codes.
//!
//! Yes there are over 2000. Several are duplicates!
//!
//! ## [JSON pointer](StringModification::JsonPointer)
//!
//! While not in the [`glue::parse`] module, [`StringModification::JsonPointer`] is relevant because it "parses" a JSON string and extracts a string value.

pub(crate) use super::*;
