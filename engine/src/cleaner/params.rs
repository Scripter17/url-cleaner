//! [`Params`].

use crate::prelude::*;

/// Data for a [`Cleaner`] to use.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Params<'a> {
    /// The flags.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: Cow<'a, FxHashSet<String>>,
    /// The vars.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: Cow<'a, FxHashMap<String, String>>,
    /// The [`Set`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: Cow<'a, FxHashMap<String, Set<String>>>,
    /// The [`List`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: Cow<'a, FxHashMap<String, List<String>>>,
    /// The [`Map`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: Cow<'a, FxHashMap<String, Map<String>>>,
    /// The [`Partitioning`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub partitionings: Cow<'a, FxHashMap<String, Partitioning>>
}

impl<'a> Params<'a> {
    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Params<'_> {
        Params {
            flags        : Cow::Borrowed(&self.flags        ),
            vars         : Cow::Borrowed(&self.vars         ),
            sets         : Cow::Borrowed(&self.sets         ),
            lists        : Cow::Borrowed(&self.lists        ),
            maps         : Cow::Borrowed(&self.maps         ),
            partitionings: Cow::Borrowed(&self.partitionings),
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Params<'static> {
        Params {
            flags        : Cow::Owned(self.flags        .into_owned()),
            vars         : Cow::Owned(self.vars         .into_owned()),
            sets         : Cow::Owned(self.sets         .into_owned()),
            lists        : Cow::Owned(self.lists        .into_owned()),
            maps         : Cow::Owned(self.maps         .into_owned()),
            partitionings: Cow::Owned(self.partitionings.into_owned()),
        }
    }
}

impl Suitability for Params<'_> {
    fn assert_suitability(&self, cleaner: &Cleaner<'_>) {
        for (name, doc) in cleaner.docs.params.vars.iter() {
            assert!(doc.documents(self.vars.get(name).map(|x| &**x)), "Params Var {name:?} required but unset.");
        }

        for flag         in self.flags        .iter() {assert!(cleaner.docs.params.flags        .contains_key(flag        ), "Undocumented flag {flag:?}"                       );}
        for var          in self.vars         .keys() {assert!(cleaner.docs.params.vars         .contains_key(var         ), "Undocumented Params Var {var:?}."                 );}
        for set          in self.sets         .keys() {assert!(cleaner.docs.params.sets         .contains_key(set         ), "Undocumented Params Set {set:?}"                  );}
        for list         in self.lists        .keys() {assert!(cleaner.docs.params.lists        .contains_key(list        ), "Undocumented Params List {list:?}"                );}
        for map          in self.maps         .keys() {assert!(cleaner.docs.params.maps         .contains_key(map         ), "Undocumented Params Map {map:?}"                  );}
        for partitioning in self.partitionings.keys() {assert!(cleaner.docs.params.partitionings.contains_key(partitioning), "Undocumented Params Partitioning {partitioning:?}");}
    }
}
