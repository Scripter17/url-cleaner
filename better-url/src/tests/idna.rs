use crate::prelude::*;

const IDNA_TEST_V2: &str = include_str!("data/IdnaTestV2.json");
const TO_ASCII: &str = include_str!("data/toascii.json");

const ADA_TO_UNICODE      : &str = include_str!("data/to_unicode_alternating.txt");
const ADA_TO_ASCII        : &str = include_str!("data/to_ascii_alternating.txt"  );
const ADA_TO_ASCII_INVALID: &str = include_str!("data/to_ascii_invalid.txt"      );

#[derive(Debug, Deserialize)]
struct Test {
    input: String,
    output: Option<String>
}

#[test]
fn wpt_idna_test_v2() {
    do_idna_tests(IDNA_TEST_V2)
}

#[test]
fn wpt_to_ascii() {
    do_idna_tests(TO_ASCII)
}

fn do_idna_tests(data: &str) {
    let mut x = Vec::new();

    for (i, c) in data.char_indices() {
        match c {
            '{' => x.push(i),
            '}' => if let Ok(test) = serde_json::from_str::<Test>(&data[x.pop().unwrap() ..= i]) {
                match encode_domain_host(test.input) {
                    Ok ((_, encoded)) => assert_eq!(encoded, test.output.unwrap()),
                    Err(_           ) => assert!(test.output.is_none_or(|x| x.is_empty())),
                }
            },
            _ => {}
        }
    }
}

#[test]
fn ada_to_unicode() {
    let mut lines = ADA_TO_UNICODE.lines();

    while let Some((input, output)) = lines.next().zip(lines.next()) {
        match domain_host_to_unicode(input) {
            Ok ((raw, new, _)) => assert_eq!(new.map_or(raw, Cow::from), output),
            Err(_            ) => assert_eq!(input                     , output),
        }
    }
}

#[test]
fn ada_to_ascii() {
    let mut lines = ADA_TO_ASCII.lines();

    while let Some((input, output)) = lines.next().zip(lines.next()) {
        match domain_host_to_ascii(input, false) {
            Ok ((raw, value)) => assert_eq!(value.map_or(raw, Into::into), output),
            Err(_           ) => assert_eq!(input                        , output),
        }
    }
}

#[test]
fn ada_to_ascii_invalid() {
    for line in ADA_TO_ASCII_INVALID.lines() {
        domain_host_to_ascii(line, false).unwrap_err();
    }
}
