//! Www.

use std::collections::BTreeSet;

use super::prelude::*;

/// Figure out which websites need to be example.com and which need to be www.example.com.
#[derive(Debug, Parser)]
pub struct Args {}

impl Args {
    /// Do the command.
    pub fn r#do(self) {
        let client = reqwest::blocking::Client::new();

        let mut adds = BTreeSet::new();
        let mut dels = BTreeSet::new();

        let mut err_without = BTreeSet::new();
        let mut err_with = BTreeSet::new();

        for domain in std::io::stdin().lines().map(Result::unwrap) {
            match client.get(format!("https://{domain}")).send() {
                Ok(response) => if response.url().host().unwrap().to_string() == format!("www.{domain}") {
                    println!("Add: {domain}");
                    adds.insert(domain.clone());
                },
                Err(_) => {
                    println!("Err without: {domain}");
                    err_without.insert(domain.clone());
                }
            }

            match client.get(format!("https://www.{domain}")).send() {
                Ok(response) => if response.url().host().unwrap().to_string() == domain {
                    println!("Del: {domain}");
                    dels.insert(domain);
                },
                Err(_) => {
                    println!("Err with: {domain}");
                    err_with.insert(domain);
                }
            }
        }

        for item in &err_without {
            if !err_with.contains(item) {
                adds.insert(item.into());
            }
        }

        for item in &err_with {
            if !err_without.contains(item) {
                dels.insert(item.into());
            }
        }

        println!();

        println!("Adds:");
        for domain in adds {
            print!("{domain:?}, ");
        }
        println!();

        println!("Dels:");
        for domain in dels {
            print!("{domain:?}, ");
        }
        println!();
    }
}
