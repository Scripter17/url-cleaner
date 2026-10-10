//! Punycode.

const ADA_UTF8_PUNYCODE_ALTERNATING: &str = include_str!("data/utf8_punycode_alternating.txt");

#[test]
fn ada_utf8_punycode_alternating() {
    let mut lines = ADA_UTF8_PUNYCODE_ALTERNATING.lines();

    while let Some((decoded, encoded)) = lines.next().zip(lines.next()) {
        assert_eq!(crate::util::encode_punycode(decoded)         , encoded, "{decoded}");
        assert_eq!(crate::util::decode_punycode(encoded).unwrap(), decoded, "{encoded}");
    }
}
