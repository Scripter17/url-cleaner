//! [`TaskContext`].

use crate::prelude::*;

/// The context of a [`Task`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskContext {
    /// The flags.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// The vars.
    ///
    /// Defaulted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>
}

impl TaskContext {
    /// If [`Self::flags`] and [`Self::vars`] are empty.
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty() && self.vars.is_empty()
    }
}

impl Suitability for TaskContext {
    fn assert_suitability(&self, cleaner: &Cleaner<'_>) {
        for name in self.flags.iter() {assert!(cleaner.docs.task_context.flags.contains_key(name), "Undocumented TaskContext Flag {name:?}");}

        for (name, value) in self.vars.iter() {
            match cleaner.docs.task_context.vars.get(name) {
                Some(doc) => {
                    if let Some(variants) = &doc.variants && !variants.contains_key(value) {
                        panic!("TaskContext Var {name:?} set to undocumented value {value:?}.");
                    }
                },
                None => panic!("Undocumented TaskContext Var {name:?}.")
            }
        }
    }
}
