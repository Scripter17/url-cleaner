use serde::Deserialize;

use crate::prelude::*;

const DATA: &str = include_str!("urltestdata.json");

#[derive(Debug, Parser)]
pub struct Args {
    
}

impl Args {
    pub fn r#do(self) {
        serde_json::from_str::<Tests>(DATA).unwrap().r#do()
    }
}

#[derive(Debug, Deserialize)]
struct Tests(pub Vec<TestOrComment>);

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TestOrComment {
    Comment(String),
    Test(Test)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Test {
    Success(SuccessTest),
    Failure(FailureTest),
}

#[derive(Debug, Deserialize)]
pub struct SuccessTest {
    pub input: String,
    pub base: Option<String>,
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct FailureTest {
    pub input: String,
    pub base: Option<String>
}

impl Tests {
    pub fn r#do(self) {
        for toc in self.0 {
            toc.r#do();
        }
    }
}

impl TestOrComment {
    pub fn r#do(self) {
        println!("{self:?}");

        match self {
            Self::Comment(_) => {},
            Self::Test(test) => test.r#do(),
        }
    }
}

impl Test {
    pub fn r#do(self) {
        match self {
            Self::Success(test) => test.r#do(),
            Self::Failure(test) => test.r#do(),
        }
    }
}

impl SuccessTest {
    pub fn r#do(self) {
        match self.base {
            Some(_) => eprintln!("  Ignoring test with base"),
            None => assert_eq!(MyUrl::new(&self.input).unwrap().as_str(), self.href)
        }
    }
}

impl FailureTest {
    pub fn r#do(self) {
        match self.base {
            Some(_) => eprintln!("  Ignoring test with base"),
            None => {MyUrl::new(&self.input).unwrap_err();}
        }
    }
}
