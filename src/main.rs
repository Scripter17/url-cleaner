use url::Url;
use clap::Parser;
// use glob;
use regex::Regex;
use serde_json;

mod rules;

#[derive(Parser)]
struct Args {
    url: Url
}

fn main() {
    match clean_url(Args::parse().url) {
        Ok(cleaned_url) => {println!("{cleaned_url}");}
        Err(e) => {eprintln!("{e:?}");}
    }
}

fn clean_url(mut url: Url) -> Result<Url, rules::RuleError> {
    let rules=vec![
        rules::Rule {
            condition: rules::Condition::All(vec![
                rules::Condition::UnqualifiedHost("deviantart.com".to_string()),
                rules::Condition::Path("/users/outgoing".to_string())
            ]),
            mapping: rules::Mapping::GetUrlFromQueryParam("url".to_string())
        },
        rules::Rule {
            condition: rules::Condition::All(vec![
                rules::Condition::UnqualifiedHost("tumblr.com".to_string()),
                rules::Condition::QueryHasParam("redirect_to".to_string())
            ]),
            mapping: rules::Mapping::Multiple(vec![
                rules::Mapping::PathFromQueryParam("redirect_to".to_string()),
                rules::Mapping::RemoveAllQueryParams
            ])
        },
        rules::Rule {
            condition: rules::Condition::Any(vec![
                rules::Condition::UnqualifiedHost("t.co".to_string()),
                rules::Condition::UnqualifiedHost("bit.ly".to_string()),
                rules::Condition::UnqualifiedHost("pixiv.me".to_string()),
                rules::Condition::All(vec![
                    rules::Condition::UnqualifiedHost("pawoo.net".to_string()),
                    rules::Condition::Path("/oauth_authentications".to_string())
                ]),
                rules::Condition::UnqualifiedHost("tr.ee".to_string())
            ]),
            mapping: rules::Mapping::Expand301
        },
        rules::Rule {
            condition: rules::Condition::AnyTld("google".to_string()),
            mapping: rules::Mapping::AllowSomeQueryParams(vec!["hl".to_string(), "q".to_string(), "tbm".to_string()])
        },
        rules::Rule {
            condition: rules::Condition::UnqualifiedHost("youtube.com".to_string()),
            mapping: rules::Mapping::RemoveSomeQueryParams(vec!["si".to_string()])
        },
        rules::Rule {
            condition: rules::Condition::Any(vec![
                rules::Condition::UnqualifiedHost("twitter.com".to_string()),
                rules::Condition::UnqualifiedHost("vxtwitter.com".to_string()),
                rules::Condition::UnqualifiedHost("fxtwitter.com".to_string()),
                rules::Condition::UnqualifiedHost("x.com".to_string())
            ]),
            mapping: rules::Mapping::SwapHost("twitter.com".to_string())
        },
        rules::Rule {
            condition: rules::Condition::Any(vec![
                rules::Condition::UnqualifiedHost("twitter.com".to_string()),
                rules::Condition::UnqualifiedHost("reddit.com".to_string()),
                rules::Condition::UnqualifiedHost("theonion.com".to_string()),
                rules::Condition::UnqualifiedHost("teespring.com".to_string()),
                rules::Condition::UnqualifiedHost("donmai.com".to_string()),
                rules::Condition::UnqualifiedHost("tumblr.com".to_string()),
                rules::Condition::UnqualifiedHost("instagram.com".to_string()),
            ]),
            mapping: rules::Mapping::RemoveAllQueryParams
        },
        // rules::Rule {
        //     condition: rules::Condition::DomainRegex(Regex::new(r"(?:^|.+\.)amazon(\.\w+(\.\w\w)?)").unwrap()), // Good enough
        //     mapping: rules::Mapping::
        // }
    ];
    println!("{}", serde_json::to_string_pretty(&rules).unwrap());
    println!("{rules:?}");
    for rule in rules {
        let _=rule.apply(&mut url);
    }
    Ok(url)
}
