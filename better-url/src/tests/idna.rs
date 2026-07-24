use crate::prelude::*;

const DATA: &str = include_str!("data/IdnaTestV2.json");

#[test]
fn test_idna() {
    let mut x = Vec::new();

    for (i, c) in DATA.char_indices() {
        match c {
            '{' => x.push(i),
            '}' => {
                let test = &DATA[x.pop().unwrap() ..= i];
                if let Ok(test) = serde_json::from_str::<Test>(test) {
                    test.r#do()
                } else {
                    println!("Err: {test:?}");
                }
            },
            _ => {}
        }
    }
}

#[derive(Debug, Deserialize)]
struct Test {
    input: String,
    output: Option<String>
}

impl Test {
    fn r#do(self) {
        println!("{self:?}");

        match encode_domain_host(self.input) {
            Ok((_, encoded)) => assert_eq!(encoded, self.output.unwrap()),
            Err(_) => assert!(self.output.is_none_or(|x| x.is_empty()))
        }
    }
}
