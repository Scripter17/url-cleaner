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
    pub flags: Cow<'a, HashSet<String>>,
    /// The vars.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: Cow<'a, HashMap<String, String>>,
    /// The [`Set`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: Cow<'a, HashMap<String, Set<String>>>,
    /// The [`List`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: Cow<'a, HashMap<String, List<String>>>,
    /// The [`Map`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: Cow<'a, HashMap<String, Map<String>>>,
    /// The [`Partitioning`]s.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub partitionings: Cow<'a, HashMap<String, Partitioning>>
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
        for flag in self.flags.iter() {assert!(cleaner.docs.params.flags.contains_key(flag), "Undocumented flag {flag:?}");}

        for (name, value) in self.vars.iter() {
            match cleaner.docs.params.vars.get(name) {
                Some(doc) => {
                    if let Some(variants) = &doc.variants && !variants.contains_key(value) {
                        panic!("Params Var {name:?} set to undocumented variant {value:?}.");
                    }
                },
                None => panic!("Undocumented Params Var {name:?}.")
            }
        }

        for (name, doc) in cleaner.docs.params.vars.iter() {
            if doc.required && !self.vars.contains_key(name) {
                panic!("Params Var {name:?} required but unset.");
            }
        }

        for set          in self.sets         .keys() {assert!(cleaner.docs.params.sets         .contains_key(set         ), "Undocumented set {set:?}");}
        for list         in self.lists        .keys() {assert!(cleaner.docs.params.lists        .contains_key(list        ), "Undocumented list {list:?}");}
        for map          in self.maps         .keys() {assert!(cleaner.docs.params.maps         .contains_key(map         ), "Undocumented map {map:?}");}
        for partitioning in self.partitionings.keys() {assert!(cleaner.docs.params.partitionings.contains_key(partitioning), "Undocumented partitioning {partitioning:?}");}
    }
}
