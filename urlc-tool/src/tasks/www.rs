//! Www.

use std::collections::BTreeSet;

use super::prelude::*;

/// Figure out which websites need to be example.com and which need to be www.example.com.
#[derive(Debug, Parser)]
pub struct Args {}

/// Thing.
enum Thing {
    /// Stay.
    Stay,
    /// Swap.
    Swap,
    /// Error.
    Error,
    /// Other.
    Other,
}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let client = reqwest::blocking::Client::builder().redirect(reqwest::redirect::Policy::none()).build().unwrap();

        let mut removes  = BTreeSet::new();
        let mut adds     = BTreeSet::new();
        let mut invalids = BTreeSet::new();
        let mut others   = BTreeSet::new();

        for domain in std::io::stdin().lines().map(Result::unwrap) {
            println!("{domain}");

            let without = format!("https://{domain}/");
            let with    = format!("https://www.{domain}/");

            let a = match client.get(&without).send() {
                Ok(res) => if let Some(location) = res.headers().get("location") {
                    if location.as_bytes() == with.as_bytes() {
                        Thing::Swap
                    } else {
                        Thing::Other
                    }
                } else if res.status().is_success() {
                    Thing::Stay
                } else {
                    Thing::Error
                },
                Err(_) => Thing::Error
            };

            let b = match client.get(&with).send() {
                Ok(res) => if let Some(location) = res.headers().get("location") {
                    if location.as_bytes() == without.as_bytes() {
                        Thing::Swap
                    } else {
                        Thing::Other
                    }
                } else if res.status().is_success() {
                    Thing::Stay
                } else {
                    Thing::Error
                },
                Err(_) => Thing::Error
            };

            match (a, b) {
                (Thing::Stay               , Thing::Stay               ) => {},
                (Thing::Stay               , Thing::Swap | Thing::Error) => {removes.insert(domain);},
                (Thing::Swap | Thing::Error, Thing::Stay               ) => {adds.insert(domain);},
                (Thing::Swap | Thing::Error, Thing::Swap | Thing::Error) => {invalids.insert(domain);},
                (Thing::Other, _) | (_, Thing::Other) => {others.insert(domain);}
            }
        }
        println!();

        println!("removes : {}", serde_json::to_string(&removes ).unwrap());
        println!("adds    : {}", serde_json::to_string(&adds    ).unwrap());
        println!("invalids: {}", serde_json::to_string(&invalids).unwrap());
        println!("others  : {}", serde_json::to_string(&others  ).unwrap());
        println!();
    }
}
