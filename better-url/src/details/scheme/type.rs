//! [`SchemeType`].

use crate::prelude::*;

/// The type of a scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SchemeType {
    /** File.                 **/ File          ,
    /** Special but not file. **/ SpecialNotFile,
    /** Non-special.          **/ NonSpecial    ,
}

impl SchemeType {
    /** If it's [`Self::File`] or [`Self::SpecialNotFile`]. **/ pub fn is_special         (self) -> bool {matches!(self, Self::File | Self::SpecialNotFile)}
    /** If it's [`Self::File`].                             **/ pub fn is_file            (self) -> bool {matches!(self, Self::File                       )}
    /** If it's [`Self::SpecialNotFile`].                   **/ pub fn is_special_not_file(self) -> bool {matches!(self, Self::SpecialNotFile             )}
    /** If it's [`Self::NonSpecial`].                       **/ pub fn is_non_special     (self) -> bool {matches!(self, Self::NonSpecial                 )}
}
