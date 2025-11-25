
pub struct ByteLines<'a>(&'a [u8]);

impl<'a> ByteLines<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }
}

impl<'a> Iterator for ByteLines<'a> {
    type Item = &'a [u8];

    #[expect(clippy::indexing_slicing, reason = "Always in bounds.")]
    fn next(&mut self) -> Option<Self::Item> {
        use std::ops::Bound;

        if self.0.is_empty() {
            None
        } else {
            let ret = self.0.split(|x| *x == b'\n').next()?;
            if ret.len() < self.0.len() {
                self.0 = &self.0[(Bound::Excluded(ret.len()), Bound::Unbounded)];
                Some(ret.strip_suffix(b"\r").unwrap_or(ret))
            } else {
                self.0 = &[];
                Some(ret)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_lines() {
        macro_rules! test {
            ($x:expr) => {
                println!("Testing {:?}", $x);
                assert_eq!(
                    ByteLines($x.as_bytes()).collect::<Vec<_>>(),
                    $x.lines().map(str::as_bytes).collect::<Vec<_>>()
                );
            }
        }

        test!("");
        test!("a");
        test!("a\r");
        test!("a\r\n");
        test!("a\r\nb");
        test!("a\r\nb\r");
        test!("a\r\nb\r\n");
        test!("a\r\nb\r\n\r");
        test!("a\r\nb\r\n\r\n");
    }
}

