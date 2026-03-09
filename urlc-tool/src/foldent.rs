//! Foldent.

use std::io::Read;
use std::collections::BTreeSet;

use super::prelude::*;

/// Fold a JSON array, ignoring and preserving indentation.
#[derive(Debug, Parser)]
pub struct Args {
    /// The width to fold to before indenting.
    #[arg(long, default_value_t = 160)]
    pub width: usize
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let mut string = String::new();

        std::io::stdin().read_to_string(&mut string).unwrap();

        let indent  = Regex::new(r#"\t+"#  ).unwrap().find(&string).unwrap().as_str();
        let close   = Regex::new(r#"\t+\]"#).unwrap().find(&string).unwrap().as_str();
        let strings = Regex::new(r#""(?:\\"|.)*?""#).unwrap().find_iter(&string).map(|string| string.as_str()).collect::<BTreeSet<_>>();

        let mut line_len = 0;

        println!("[");

        for string in strings {
            if line_len != 0 && line_len + 1 + string.len() >= self.width {
                println!(",");
                line_len = 0;
            }

            if line_len == 0 {
                print!("{indent}");
            } else {
                print!(", ");
                line_len += 2;
            }

            print!("{string}");
            line_len += string.len();
        }

        println!();
        println!("{close}");
    }
}
