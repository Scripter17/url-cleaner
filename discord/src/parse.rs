//! [`parse`].

use std::sync::LazyLock;
use regex::Regex;

/// Regex to split on code blocks.
static CODE_BLOCK: LazyLock<Regex> = LazyLock::new(|| regex::Regex::new(r#"```\w*\n[\s\S]*?```"#).expect("The CODE_BLOCK Regex to be valid."));

/// Regex to split on inline code.
static CODE_LINE: LazyLock<Regex> = LazyLock::new(|| regex::Regex::new(r#"``.+?``|`.+?`?`"#).expect("The CODE_LINE Regex to be valid."));

/// Regex to extract URLS.
static URLS: LazyLock<Regex> = LazyLock::new(|| regex::Regex::new(r#"\[\]\(.|\[[\s\S]+?\]\((https?://\S+?(?:\(\))?)(?:\s.*?)?\)|(https?://\S+)"#).expect("The URLS Regex to be valid."));

/// Parse a discord message for URLs.
pub fn parse(value: &str) -> impl Iterator<Item = &str> {
    CODE_BLOCK.split(value)
        .flat_map(|node| CODE_LINE.split(node))
        .flat_map(|part| URLS.captures_iter(part))
        .filter_map(|x| x.get(1).or(x.get(2)))
        .map(|x| x.as_str())
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#"
https://example.com/1

- a
- b https://example.com/2
- [c](https://example.com/3)

||aa https://example.com/4 aa||

`https://example.com/NO1`

```
https://example.com/NO2
```
https://example.com/5
```
[aaa](https://example.com/6(((()))))
[](https://example.com/NO3(((()))))
https://example.com/7(((())))
[bbb](https://example.com/8 9)
[a[[[b]]]]]]]]]c](https://example.com/10)
[a
b](https://example.com/11)
"#;

    #[test]
    fn parse_test() {
        assert_eq!(
            parse(TEST).collect::<Vec<_>>(),
            [
                "https://example.com/1",
                "https://example.com/2",
                "https://example.com/3",
                "https://example.com/4",
                "https://example.com/5",
                "https://example.com/6(((()",
                "https://example.com/7(((())))",
                "https://example.com/8",
                "https://example.com/10",
                "https://example.com/11",
            ]
        );
    }
}
