//! [`PathType`] and [`SegmentedPathType`].

use crate::prelude::*;

/// The type of a [`Path`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PathType {
    /** [`Path::Segmented`]. **/ Segmented(SegmentedPathType),
    /** [`Path::Opaque`].    **/ Opaque                      ,
}

impl From<SegmentedPathType> for PathType {
    fn from(value: SegmentedPathType) -> Self {
        Self::Segmented(value)
    }
}

/// The type of a [`SegmentedPath`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SegmentedPathType {
    /** [`SegmentedPath::File`].           **/ File          ,
    /** [`SegmentedPath::SpecialNotFile`]. **/ SpecialNotFile,
    /** [`SegmentedPath::NonSpecial`].     **/ NonSpecial    ,
}

impl From<SchemeType> for SegmentedPathType {
    fn from(value: SchemeType) -> Self {
        match value {
            SchemeType::File           => Self::File          ,
            SchemeType::SpecialNotFile => Self::SpecialNotFile,
            SchemeType::NonSpecial     => Self::NonSpecial    ,
        }
    }
}
