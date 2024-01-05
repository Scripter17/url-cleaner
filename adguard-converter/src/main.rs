use url_cleaner::{
    glue::RegexParts,
    types::UrlPartName,
    rules::{
        Rule, Rules,
        conditions::Condition,
        mappers::Mapper
    }
};

use std::fs::File;
use std::io::prelude::*;

use thiserror::Error;

fn split_on_pipe_but_not_in_regex(x: &str) -> Vec<String> {
    let mut escaped=false;
    let mut split=true;
    let mut acc=String::new();
    let mut ret=Vec::new();
    for c in x.chars() {
        if c=='\\' {escaped = !escaped;}
        if c=='/' && !escaped {split = !split;}
        if c=='|' && split {
            ret.push(acc.replace("\\,", ","));
            acc=String::new();
        } else {
            acc.push(c);
        }
    }
    if !acc.is_empty() {ret.push(acc.replace("\\,", ","));}
    ret
}

fn parse_regex(x: &str) -> RegexParts {
    let mut parts=RegexParts::new(x.split_once('/').unwrap().1.rsplit_once('/').unwrap().0);
    parts.add_flags(x.rsplit_once('/').unwrap().1);
    parts
}

#[derive(Debug, Error)]
enum AdGuardError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error)
}

#[tokio::main]
async fn main() -> Result<(), AdGuardError> {
    // https://adguard.com/kb/general/ad-filtering/create-own-filters
    let rule_parser = regex::RegexBuilder::new(r"^
        (?<negation>@@)?
        (?<unqualified>\|\|)?
        (?<host>(?:[\w\-*]+\.?)+)?
        (?<path>/[^?&]*)?
        (?:[?&](?<query>.+?))?
        (?:[^a-zA-Z\d_\-.%])?
        (?:\^?\$(?:(removeparam(?:=(?<removeparam>(\\,|[^,])+)|(?<removequery>))|domain=(?<domains>[^,]+)|[^,]+),?)+)
        $")
        .multi_line(true).ignore_whitespace(true).build().unwrap();
    let list_urls = [
        "https://raw.githubusercontent.com/AdguardTeam/AdguardFilters/master/TrackParamFilter/sections/general_url.txt",
        // "https://raw.githubusercontent.com/AdguardTeam/AdguardFilters/master/TrackParamFilter/sections/specific.txt"
    ];
    let client=reqwest::Client::new();
    let mut rules=Vec::<Rule>::new();

    for list_url in list_urls {
        for adguard_rule in client.get(list_url).send().await?.text().await?.lines().filter(|line| !line.is_empty() && !line.starts_with('!')).inspect(|line| println!("{line}")).map(|line| rule_parser.captures(line).unwrap()) {
            let negation    = adguard_rule.name("negation"   ).is_some();
            let unqualified = adguard_rule.name("unqualified").is_some();
            let host        = adguard_rule.name("host"       ).map(|host   | host   .as_str());
            let path        = adguard_rule.name("path"       ).map(|path   | path   .as_str());
            let query       = adguard_rule.name("query"      ).map(|query  | query  .as_str());
            let removeparam = adguard_rule.name("removeparam").map(|query  | query  .as_str());
            let domains     = adguard_rule.name("domains"    ).map(|domains| domains.as_str());
            println!("{negation:?} {unqualified:?} {host:?} {path:?} {query:?} {removeparam:?} {domains:?}");
        }
    }

    Ok(())
}
