//! A system to stop me from accidentally committing debug stuff to the bundled cleaner.

use std::fmt::Debug;
use std::collections::{HashSet, HashMap};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use indexmap::{IndexMap, IndexSet};

use crate::prelude::*;

/// A trait for things that may or may not be suitable for use in the bundled cleaner.
pub(crate) trait Suitability: Debug {
    /// If `self` is deemed unsuitable to be in the bundled cleaner, panics.
    /// # Panics
    /// If `self` is deemed unsuitable to be in the bundled cleaner, panics.
    fn assert_suitability(&self, cleaner: &Cleaner);
}

/// Generate [`Suitability`] impls for types that are always suitable for use in the bundled cleaner.
macro_rules! always_suitable {
    ($($t:ty),*$(,)?) => {
        $(
            impl Suitability for $t {
                fn assert_suitability(&self, _: &Cleaner) {}
            }
        )*
    }
}

always_suitable!(
    char, str, bool, (),
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
    NonZero<u8>, NonZero<u16>, NonZero<u32>, NonZero<u64>, NonZero<u128>, NonZero<usize>,
    NonZero<i8>, NonZero<i16>, NonZero<i32>, NonZero<i64>, NonZero<i128>, NonZero<isize>,

    String, Path, PathBuf,
    std::time::Duration,

    serde_json::Value, serde_json::Number,

    BetterUrl,
);

#[cfg(feature = "http")] always_suitable!(reqwest::header::HeaderMap, reqwest::header::HeaderValue, reqwest::Method);

/// Macro for generating Suitability impls for tuples.
macro_rules! suitable_tuple {
    ($($t:ident,)*) => {
        impl<$($t: Suitability,)*> Suitability for ($($t,)*) {
            fn assert_suitability(&self, cleaner: &Cleaner) {
                #[allow(non_snake_case, reason = "Declarative macros can't fix it.")]
                let ($($t, )*) = self;
                $($t.assert_suitability(cleaner);)*
            }
        }
    };
}

suitable_tuple!(T1, );
suitable_tuple!(T1, T2, );
suitable_tuple!(T1, T2, T3, );
suitable_tuple!(T1, T2, T3, T4, );
suitable_tuple!(T1, T2, T3, T4, T5, );
suitable_tuple!(T1, T2, T3, T4, T5, T6, );
suitable_tuple!(T1, T2, T3, T4, T5, T6, T7, );
suitable_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, );
suitable_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, );
suitable_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, );
suitable_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, );
suitable_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, );

/// Suitability for [`FlagSource::Params`].
pub(crate) fn flag_source_params(name: &StringSource, cleaner: &Cleaner) {
    match name {
        StringSource::String(name) => assert!(cleaner.docs.params.flags.contains_key(name), "Undocumented FlagSource::Params: {name}"),
        name => name.assert_suitability(cleaner),
    }
}

/// Suitability for [`FlagSource::TaskContext`].
pub(crate) fn flag_source_task_context(name: &StringSource, cleaner: &Cleaner) {
    match name {
        StringSource::String(name) => assert!(cleaner.docs.task_context.flags.contains_key(name), "Undocumented FlagSource::TaskContext: {name}"),
        name => name.assert_suitability(cleaner),
    }
}

/// Suitability for [`FlagSource::JobContext`].
pub(crate) fn flag_source_job_context(name: &StringSource, cleaner: &Cleaner) {
    match name {
        StringSource::String(name) => assert!(cleaner.docs.job_context.flags.contains_key(name), "Undocumented FlagSource::JobContext: {name}"),
        name => name.assert_suitability(cleaner),
    }
}



/// Suitability for [`VarSource::Params`].
pub(crate) fn var_source_params(name: &StringSource, cleaner: &Cleaner) {
    match name {
        StringSource::String(name) => assert!(cleaner.docs.params.vars.contains_key(name), "Undocumented VarSource::Params: {name}"),
        name => name.assert_suitability(cleaner),
    }
}

/// Suitability for [`VarSource::TaskContext`].
pub(crate) fn var_source_task_context(name: &StringSource, cleaner: &Cleaner) {
    match name {
        StringSource::String(name) => assert!(cleaner.docs.task_context.vars.contains_key(name), "Undocumented VarSource::TaskContext: {name}"),
        name => name.assert_suitability(cleaner),
    }
}

/// Suitability for [`VarSource::JobContext`].
pub(crate) fn var_source_job_context(name: &StringSource, cleaner: &Cleaner) {
    match name {
        StringSource::String(name) => assert!(cleaner.docs.job_context.vars.contains_key(name), "Undocumented VarSource::JobContext: {name}"),
        name => name.assert_suitability(cleaner),
    }
}

/// Suitability for [`VarSource::Secret`].
pub(crate) fn var_source_secret(name: &StringSource, cleaner: &Cleaner) {
    match name {
        StringSource::String(name) => assert!(cleaner.docs.secrets.vars.contains_key(name), "Undocumented VarSource::Secret: {name}"),
        name => name.assert_suitability(cleaner),
    }
}



/// Suitability for [`ListSource::Params`].
pub(crate) fn list_source_params(name: &StringSource, cleaner: &Cleaner) {
    match name {
        StringSource::String(name) => {
            assert!(cleaner.     params.lists.contains_key(name), "Unset ListSource::Params: {name}");
            assert!(cleaner.docs.params.lists.contains_key(name), "Undocumented ListSource::Params: {name}");
        },
        name => name.assert_suitability(cleaner),
    }
}

/// Suitability for [`SetSource::Params`].
pub(crate) fn set_source_params(name: &StringSource, cleaner: &Cleaner) {
    match name {
        StringSource::String(name) => {
            assert!(cleaner.     params.sets.contains_key(name), "Unset SetSource::Params: {name}");
            assert!(cleaner.docs.params.sets.contains_key(name), "Undocumented SetSource::Params: {name}");
        },
        name => name.assert_suitability(cleaner),
    }
}

/// Suitability for [`MapSource::Params`].
pub(crate) fn map_source_params(name: &StringSource, cleaner: &Cleaner) {
    match name {
        StringSource::String(name) => {
            assert!(cleaner.     params.maps.contains_key(name), "Unset MapSource::Params: {name}");
            assert!(cleaner.docs.params.maps.contains_key(name), "Undocumented MapSource::Params: {name}");
        },
        name => name.assert_suitability(cleaner),
    }
}

/// Suitability for [`PartitioningSource::Params`].
pub(crate) fn partitioning_source_params(name: &StringSource, cleaner: &Cleaner) {
    match name {
        StringSource::String(name) => {
            assert!(cleaner.     params.partitionings.contains_key(name), "Unset PartitioningSource::Params: {name}");
            assert!(cleaner.docs.params.partitionings.contains_key(name), "Undocumented PartitioningSource::Params: {name}");
        },
        name => name.assert_suitability(cleaner),
    }
}



impl<T: Suitability + ?Sized> Suitability for &T     {fn assert_suitability(&self, cleaner: &Cleaner) {T::assert_suitability(self, cleaner);}}
impl<T: Suitability + ?Sized> Suitability for Box<T> {fn assert_suitability(&self, cleaner: &Cleaner) {T::assert_suitability(self, cleaner);}}
impl<T: Suitability + ?Sized> Suitability for Rc <T> {fn assert_suitability(&self, cleaner: &Cleaner) {T::assert_suitability(self, cleaner);}}
impl<T: Suitability + ?Sized> Suitability for Arc<T> {fn assert_suitability(&self, cleaner: &Cleaner) {T::assert_suitability(self, cleaner);}}

impl<K: Suitability, V: Suitability, S> Suitability for HashMap <K, V, S> {fn assert_suitability(&self, cleaner: &Cleaner) {for x in self.iter() {x.assert_suitability(cleaner);}}}
impl<T: Suitability, S                > Suitability for HashSet <T,    S> {fn assert_suitability(&self, cleaner: &Cleaner) {for x in self.iter() {x.assert_suitability(cleaner);}}}
impl<K: Suitability, V: Suitability, S> Suitability for IndexMap<K, V, S> {fn assert_suitability(&self, cleaner: &Cleaner) {for x in self.iter() {x.assert_suitability(cleaner);}}}
impl<T: Suitability, S                > Suitability for IndexSet<T,    S> {fn assert_suitability(&self, cleaner: &Cleaner) {for x in self.iter() {x.assert_suitability(cleaner);}}}
impl<T: Suitability                   > Suitability for Vec<T>            {fn assert_suitability(&self, cleaner: &Cleaner) {for x in self.iter() {x.assert_suitability(cleaner);}}}
impl<T: Suitability                   > Suitability for [T]               {fn assert_suitability(&self, cleaner: &Cleaner) {for x in self.iter() {x.assert_suitability(cleaner);}}}



impl<T: Suitability> Suitability for Option<T> {
    fn assert_suitability(&self, cleaner: &Cleaner) {
        if let Some(x) = self {
            x.assert_suitability(cleaner);
        }
    }
}

impl<'a, T: Suitability + ToOwned + ?Sized + 'a> Suitability for Cow<'a, T> where <T as ToOwned>::Owned: Suitability {
    fn assert_suitability(&self, cleaner: &Cleaner) {
        match self {
            Self::Owned   (x) => x.assert_suitability(cleaner),
            Self::Borrowed(x) => x.assert_suitability(cleaner),
        }
    }
}

impl<T: Suitability> Suitability for std::ops::Bound<T> {
    fn assert_suitability(&self, cleaner: &Cleaner) {
        match self {
            std::ops::Bound::Unbounded => {},
            std::ops::Bound::Included(x) => x.assert_suitability(cleaner),
            std::ops::Bound::Excluded(x) => x.assert_suitability(cleaner),
        }
    }
}
