use url_cleaner::{
    glue::RegexParts,
    rules::{
        Rule, Rules,
        conditions::Condition,
        mappers::Mapper
    }
};
use reqwest;
use tokio;
use regex;
use serde_json;

use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    let parser = regex::RegexBuilder::new(r"^
        (?<negation>@@)?
        (?<unqualified>\|\|)?
        (?<host>(?:[\w\-*]+\.?)+)?
        (?<path>/[^?&]*)?
        (?:[?&](?<query>.+?))?
        (?:[^a-zA-Z\d_\-.%])?
        # (?:\^?\$(?:(removeparam(?:=(?<removeparam>[^,]+)|(?<removequery>))|domain=(?<domains>[^,]+)|[^,]+),?)+)
        (?:\^?\$(?:(removeparam(?:=(?<removeparam>(\\,|[^,])+)|(?<removequery>))|[^,]+),?)+)
        $")
        .multi_line(true)
        .ignore_whitespace(true)
        .build()
        .unwrap();
    let urls = [
        "https://raw.githubusercontent.com/AdguardTeam/AdguardFilters/master/TrackParamFilter/sections/general_url.txt",
        "https://raw.githubusercontent.com/AdguardTeam/AdguardFilters/master/TrackParamFilter/sections/specific.txt"
    ];
    let client=reqwest::Client::new();
    let mut rules=Vec::new();
    for url in urls {
        let text=client.get(url).send().await.unwrap().text().await.unwrap();
        for line in text.lines().filter(|line| !line.starts_with("!")) {
            match parser.captures(line) {
                // Some(capture) => println!("{capture:?}"),
                Some(capture) => {
                    let negation    =capture.name("negation"   ).is_some();
                    let unqualified =capture.name("unqualified").is_some();
                    let host        =capture.name("host"       ).map(|host| host.as_str());
                    // let domains     =match (capture.name("host"), capture.name("domains")) {
                    //     (Some(host), Some(domains)) => [host.as_str()].into_iter().chain(domains.as_str().split('|').filter(|domain| !domain.starts_with('~`))).collect::<Vec<_>>(),
                    //     (Some(host), None         ) => vec![host.as_str()],
                    //     (None      , Some(domains)) => domains.as_str().split('|').collect::<Vec<_>>(),
                    //     (None      , None         ) => vec![]
                    // };
                    let path        =capture.name("path"       ).map(|path| path.as_str());
                    let query       =capture.name("query"      ).map(|query| query.as_str().split('&').map(|param| param.split_once('=')).collect::<Vec<_>>());
                    // let removeparams=capture.name("removeparam").map(|list| list.as_str().split('|').collect::<Vec<_>>());
                    let removeparams=match capture.name("removeparam") {
                        Some(params) => {
                            let mut escaped=false;
                            let mut split=true;
                            let mut acc=String::new();
                            let mut ret=Vec::new();
                            for c in params.as_str().chars() {
                                if c=='\\' {escaped=!escaped;}
                                if c=='/' && !escaped {split=!split;}
                                if c=='|' && split {
                                    ret.push(acc.replace("\\,", ","));
                                    acc=String::new();
                                } else {
                                    acc.push(c);
                                }
                            }
                            if !acc.is_empty() {ret.push(acc.replace("\\,", ","));}
                            ret
                        },
                        None => Vec::new()
                    };
                    let removequery =capture.name("removequery").is_some();
                    // println!("{line:?}");
                    // println!("N={negation:?} - U={unqualified:?} - H={host:?} - P={path:?} - Q={query:?} - R={removeparams:?} - A={removequery:?}");
                    
                    let mut conditions=Vec::<Condition>::new();
                    let mut mappers=Vec::<Mapper>::new();
                    if let Some(host)=host {
                        if unqualified {
                            conditions.push(Condition::UnqualifiedDomain(host.to_string()));
                        } else {
                            conditions.push(Condition::QualifiedDomain(host.to_string()));
                        }
                    }
                    if let Some(path)=path {
                        if path.contains('*') {
                            conditions.push(Condition::PathMatchesRegex(RegexParts::new(&path.replace('*', ".*")).try_into().unwrap()));
                        } else {
                            conditions.push(Condition::PathIs(path.to_string()));
                        }
                    }
                    if !removeparams.is_empty() {
                        let (param_patterns, param_names): (Vec<_>, Vec<_>)=removeparams.into_iter().partition(|x| x.starts_with('/'));
                        for param_pattern in param_patterns {
                            // println!("{param_pattern:?}");
                            let mut parts=RegexParts::new(param_pattern.split_once('/').unwrap().1.rsplit_once('/').unwrap().0);
                            parts.add_flags(param_pattern.rsplit_once('/').unwrap().1);
                            mappers.push(Mapper::RemoveQueryParamsMatching(parts.try_into().unwrap()));
                        }
                        if !param_names.is_empty() {
                            mappers.push(Mapper::RemoveSomeQueryParams(param_names));
                        }
                    }
                    
                    let rule=Rule {
                        condition: match conditions.len() {
                            0   => Condition::Always,
                            1   => conditions.pop().unwrap(),
                            2.. => Condition::All(conditions),
                            _   => panic!()
                        },
                        mapper: match mappers.len() {
                            0   => Mapper::None,
                            1   => mappers.pop().unwrap(),
                            2.. => Mapper::Multiple(mappers),
                            _   => panic!()
                        }
                    };
                    // println!("{rule:?}");
                    if rule.mapper!=Mapper::None {
                        rules.push(rule);
                    }
                },
                // Some(_) => {},
                // None => println!("No match: {line:?}")
                None => {}
            }
        }
    }
    let mut rules=Rules::from(rules);
    write!(File::create("adguard-rules.json").unwrap(), "{}", serde_json::to_string(&rules).unwrap());
    write!(File::create("adguard-rules-simplified.json").unwrap(), "{}", serde_json::to_string(&rules.simplify()).unwrap());
}
