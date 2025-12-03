//! Utility stuff.

/// An [`Iterator`] to do [`str::lines`] on byte slices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteLines<'a>(pub &'a [u8]);

impl<'a> Iterator for ByteLines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.0.split_inclusive(|x| *x == b'\n').next()?;
        self.0 = self.0.strip_prefix(ret).expect("This to not even generate a panic handler.");
        let Some(ret) = ret.strip_suffix(b"\n") else {return Some(ret);};
        let Some(ret) = ret.strip_suffix(b"\r") else {return Some(ret);};
        Some(ret)
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
