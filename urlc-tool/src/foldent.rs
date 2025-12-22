//! Foldent.

use std::io::Read;
use std::collections::BTreeSet;

use super::prelude::*;

/// Take a JSON array from STDIN and fold it while keeping indentation.
#[derive(Debug, Parser)]
pub struct Args {}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut text = String::new();

        std::io::stdin().read_to_string(&mut text).unwrap();

        let a = Regex::new(r#"(\t*)""# ).unwrap().captures(&text).unwrap().get(1).unwrap().as_str();
        let b = Regex::new(r#"(\t*)\]"#).unwrap().captures(&text).unwrap().get(1).unwrap().as_str();

        let items = serde_json::from_str::<BTreeSet<String>>(&text).unwrap();

        println!("[");

        let mut buf = String::new();
        let mut buf2 = String::new();

        for item in items {
            if !buf.is_empty() {
                buf.push(' ');
            }
            if item.len() + buf.len() + 1 >= 160 {
                buf2.push_str(a);
                buf2.push_str(&buf);
                buf2.push('\n');
                buf.clear();
            }
            buf.push_str(&format!("{item:?},"));
        }

        if !buf.is_empty() {
            buf2.push_str(a);
            buf2.push_str(&buf);
            buf2.push('\n');
        }

        buf2.pop();
        buf2.pop();

        println!("{buf2}");

        println!("{b}]");
    }
}
