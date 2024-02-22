use std::cmp::Ord;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StringCmp {
    /// `l<r`.
    Lt,
    /// `l<=r`.
    Le,
    /// `l==r`.
    Eq,
    /// `l>=r`.
    Ge,
    /// `l>r`.
    Gt,
    /// `l!=r`.
    Ne,
    /// [`LengthCmp::satisfied_by`].
    Length(LengthCmp)
}

impl StringCmp {
    pub fn satisfied_by(&self, l: &str, r: &str) -> bool {
        match self {
            Self::Lt => l< r,
            Self::Le => l<=r,
            Self::Eq => l==r,
            Self::Ge => l>=r,
            Self::Gt => l> r,
            Self::Ne => l!=r,
            Self::Length(length_cmp) => length_cmp.satisfied_by(l.len(), r.len())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LengthCmp {
    /// `l<r`.
    Lt,
    /// `l<=r`.
    Le,
    /// `l==r`.
    Eq,
    /// `l>=r`.
    Ge,
    /// `l>r`.
    Gt,
    /// `l!=r`.
    Ne,
    Diff(DiffCmp)
}

impl LengthCmp {
    pub fn satisfied_by(&self, l: usize, r: usize) -> bool {
        match self {
            Self::Lt => l< r,
            Self::Le => l<=r,
            Self::Eq => l==r,
            Self::Ge => l>=r,
            Self::Gt => l> r,
            Self::Ne => l!=r,
            Self::Diff(diff_cmp) => diff_cmp.satisfied_by((l as isize)-(r as isize))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiffCmp {
    cmp: Cmp,
    r: isize
}

impl DiffCmp {
    pub fn satisfied_by(&self, diff: isize) -> bool {
        self.cmp.satisfied_by(diff, self.r)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Cmp {
    /// `l<r`.
    Lt,
    /// `l<=r`.
    Le,
    /// `l==r`.
    Eq,
    /// `l>=r`.
    Ge,
    /// `l>r`.
    Gt,
    /// `l!=r`.
    Ne
}

impl Cmp {
    pub fn satisfied_by<T: Ord>(&self, l: T, r: T) -> bool {
        match self {
            Self::Lt => l< r,
            Self::Le => l<=r,
            Self::Eq => l==r,
            Self::Ge => l>=r,
            Self::Gt => l> r,
            Self::Ne => l!=r
        }
    }
}
