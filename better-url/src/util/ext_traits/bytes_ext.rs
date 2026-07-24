//! [`BytesExt`].

/// An extension trait for [`[u8]`].
pub(crate) trait BytesExt {
    /** [`memchr::memchr`]. **/ fn memchr  (&self, b : u8                ) -> Option<usize>;
    /** [`memchr::memchr2`]. **/ fn memchr2 (&self, b1: u8, b2: u8        ) -> Option<usize>;
    /** [`memchr::memchr3`]. **/ fn memchr3 (&self, b1: u8, b2: u8, b3: u8) -> Option<usize>;

    /** [`memchr::memrchr`]. **/ fn memrchr (&self, b : u8                ) -> Option<usize>;
    // /** [`memchr::memrchr2`]. **/ fn memrchr2(&self, b1: u8, b2: u8        ) -> Option<usize>;
    // /** [`memchr::memrchr3`]. **/ fn memrchr3(&self, b1: u8, b2: u8, b3: u8) -> Option<usize>;
}

impl BytesExt for [u8] {
    fn memchr  (&self, b : u8                ) -> Option<usize> {memchr::memchr  (b         , self)}
    fn memchr2 (&self, b1: u8, b2: u8        ) -> Option<usize> {memchr::memchr2 (b1, b2    , self)}
    fn memchr3 (&self, b1: u8, b2: u8, b3: u8) -> Option<usize> {memchr::memchr3 (b1, b2, b3, self)}

    fn memrchr (&self, b : u8                ) -> Option<usize> {memchr::memrchr (b         , self)}
    // fn memrchr2(&self, b1: u8, b2: u8        ) -> Option<usize> {memchr::memrchr2(b1, b2    , self)}
    // fn memrchr3(&self, b1: u8, b2: u8, b3: u8) -> Option<usize> {memchr::memrchr3(b1, b2, b3, self)}
}
