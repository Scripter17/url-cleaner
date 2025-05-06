//! Flags, variables, etc. that adjust the exact behavior of a config.

use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use thiserror::Error;
use serde_with::serde_as;

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Flags, variables, etc. that adjust the exact behavior of a config.
///
/// Bundles all the state that determines how the [`Cleaner`] works in one convenient area.
#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Suitability)]
pub struct Params {
    /// Flags allow enabling and disabling certain behavior.
    ///
    /// Defaults to an empty [`HashSet`].
    #[serde_with = "SetPreventDuplicates<_>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// Vars allow setting strings used for certain behaviors.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// Sets allow quickly checking if a string is in a certain genre of possible values.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: HashMap<String, Set<String>>,
    /// Lists are a niche thing that lets you iterate over a set of values in a known order.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: HashMap<String, Vec<String>>,
    /// Maps allow mapping input values to output values.
    ///
    /// Please note that [`Map`]s make this more powerful than a normal [`HashMap`], notably including a default value.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: HashMap<String, Map<String>>,
    /// Named partitionings effectively let you check which if several sets a value is in.
    ///
    /// See [this Wikipedia article](https://en.wikipedia.org/wiki/Partition_of_a_set) for the math end of this idea.
    ///
    /// Defaults to an empty [`HashMap`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")]
    pub named_partitionings: HashMap<String, NamedPartitioning>,
    /// If [`true`], things that interact with the cache will read from the cache.
    ///
    /// Defaults to true.
    #[cfg(feature = "cache")]
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub read_cache: bool,
    /// If [`true`], things that interact with the cache will write to the cache.
    ///
    /// Defaults to [`true`].
    #[cfg(feature = "cache")]
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub write_cache: bool,
    /// The default [`HttpClientConfig`], prior to relevant [`HttpClientConfigDiff`]s.
    ///
    /// Defaults to [`HttpClientConfig::default`].
    #[cfg(feature = "http")]
    #[serde(default, skip_serializing_if = "is_default")]
    pub http_client_config: HttpClientConfig
}

#[allow(clippy::derivable_impls, reason = "When the `cache` feature is enabled, this can't be derived.")]
impl Default for Params {
    fn default() -> Self {
        Self {
            flags: HashSet::default(),
            vars : HashMap::default(),
            sets : HashMap::default(),
            lists: HashMap::default(),
            maps : HashMap::default(),
            named_partitionings: HashMap::default(),
            #[cfg(feature = "cache")] read_cache: true,
            #[cfg(feature = "cache")] write_cache: true,
            #[cfg(feature = "http")]
            http_client_config: HttpClientConfig::default()
        }
    }
}

/// Rules for updating a [`Params`].
///
/// Often you'll have a default [`ParamsDiff`] you use for all your URLs and only sometimes you want to change that behavior.
///
/// The diff pattern handles this use case very well without requiring you change the actual config file.
#[serde_as]
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ParamsDiff {
    /// [`Params::flags`] to set.
    #[serde_with = "SetPreventDuplicates<_>"]
    #[serde(default, skip_serializing_if = "is_default")] pub flags  : HashSet<String>,
    /// [`Params::flags`] to unset.
    #[serde_with = "SetPreventDuplicates<_>"]
    #[serde(default, skip_serializing_if = "is_default")] pub unflags: HashSet<String>,
    /// [`Params::vars`] to set.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub vars  : HashMap<String, String>,
    /// [`Params::vars`] to unset.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub unvars: HashSet<String>,
    /// [`Params::sets`] to init.
    ///
    /// Shouldn't ever actually change anything, but if you're really fussy or something.
    #[serde(default, skip_serializing_if = "is_default")] pub init_sets: Vec<String>,
    /// [`Params::sets`] and values to insert into them.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub insert_into_sets: HashMap<String, Vec<Option<String>>>,
    /// [`Params::sets`] and values to remove from them.
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub remove_from_sets: HashMap<String, Vec<Option<String>>>,
    /// [`Params::sets`] to delete.
    #[serde(default, skip_serializing_if = "is_default")] pub delete_sets: Vec<String>,
    /// [`Params::maps`] to init.
    ///
    /// Shouldn't ever actually change anything, but if you're really fussy or something.
    #[serde(default, skip_serializing_if = "is_default")] pub init_maps: Vec<String>,
    /// [`MapDiff`]s to apply to [`Params::maps`].
    #[serde_with = "MapPreventDuplicates<_, _>"]
    #[serde(default, skip_serializing_if = "is_default")] pub map_diffs: HashMap<String, MapDiff<String>>,
    /// [`Params::maps`] to delete.
    #[serde(default, skip_serializing_if = "is_default")] pub delete_maps: Vec<String>,
    /// If [`Some`], sets, [`Params::read_cache`].
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")] pub read_cache : Option<bool>,
    /// If [`Some`], sets [`Params::write_cache`].
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")] pub write_cache: Option<bool>,
    /// If [`Some`], applies the [`HttpClientConfigDiff`] to [`Params::http_client_config`].
    #[cfg(feature = "http")]
    #[serde(default, skip_serializing_if = "is_default")] pub http_client_config_diff: Option<HttpClientConfigDiff>
}

impl ParamsDiff {
    /// Applies the diff.
    ///
    /// Exact order is not guaranteed to be stable, but currently removals/deletions happen after inittings/insertions/settings.
    pub fn apply(self, to: &mut Params) {
        to.flags.extend(self.flags);
        for flag in self.unflags {to.flags.remove(&flag);}

        to.vars.extend(self.vars);
        for var in self.unvars {to.vars.remove(&var);}

        for k in self.init_sets {
            to.sets.entry(k).or_default();
        }
        for (k, v) in self.insert_into_sets {
            to.sets.entry(k).or_default().extend(v);
        }
        for (k, vs) in self.remove_from_sets {
            if let Some(x) = to.sets.get_mut(&k) {
                for v in vs {
                    x.remove(v.as_ref());
                }
            }
        }
        for k in self.delete_sets {
            to.sets.remove(&k);
        }

        for k in self.init_maps {
            to.maps.entry(k).or_default();
        }
        for (k, v) in self.map_diffs {
            v.apply(to.maps.entry(k).or_default());
        }
        for k in self.delete_maps {
            to.maps.remove(&k);
        }

        #[cfg(feature = "cache")] if let Some(read_cache ) = self.read_cache  {to.read_cache  = read_cache ;}
        #[cfg(feature = "cache")] if let Some(write_cache) = self.write_cache {to.write_cache = write_cache;}

        #[cfg(feature = "http")] if let Some(http_client_config_diff) = &self.http_client_config_diff {http_client_config_diff.apply(&mut to.http_client_config);}
    }
}

/// Allows generating [`ParamsDiff`]s from [`clap::Args`].
/// # Examples
/// ```
/// use url_cleaner::types::*;
///
/// #[derive(clap::Args)]
/// pub struct Args {
///     // Your stuff goes here
///     #[command(flatten)]
///     pub params_diff_args: ParamsDiffArgParser,
///     // Your stuff also goes here
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, clap::Args)]
pub struct ParamsDiffArgParser {
    /// Set a params flag.
    #[arg(short, long, value_names = ["NAME"])]
    pub flag  : Vec<String>,
    /// Unset a params flag.
    #[arg(long, value_names = ["NAME"])]
    pub unflag: Vec<String>,
    /// Set a params var.
    #[arg(short, long, num_args(2), value_names = ["NAME", "VALUE"])]
    pub var: Vec<Vec<String>>,
    /// Unset a params var.
    #[arg(long, value_names = ["NAME"])]
    pub unvar : Vec<String>,
    /// Insert a value into a params set.
    #[arg(long, num_args(1..), value_names = ["NAME", "VALUE1"])]
    pub insert_into_set: Vec<Vec<String>>,
    /// Remove a value from a params set.
    #[arg(long, num_args(1..), value_names = ["NAME", "VALUE1"])]
    pub remove_from_set: Vec<Vec<String>>,
    /// Insert a value into a params map.
    #[arg(long, num_args(2..), value_names = ["NAME", "KEY1", "VALUE1"])]
    pub insert_into_map: Vec<Vec<String>>,
    /// Remove a value from a params map.
    #[arg(long, num_args(1..), value_names = ["NAME", "KEY1"])]
    pub remove_from_map: Vec<Vec<String>>,
    /// Overrides if the cleaner reads from the cache. If no value is provided, assumes `true`.
    #[cfg(feature = "cache")]
    #[arg(long, num_args(0..=1), default_missing_value("true"))]
    pub read_cache : Option<bool>,
    /// Overrides if the cleaner writes to the cache. If no value is provided, assumes `true`.
    #[cfg(feature = "cache")]
    #[arg(long, num_args(0..=1), default_missing_value("true"))]
    pub write_cache: Option<bool>,
    /// Overrides the proxy to use.
    #[cfg(feature = "http")]
    #[arg(long)]
    pub proxy: Option<ProxyConfig>,
    /// Overrides if all proxies should be ignored. If no value is provided, assumes `true`.
    #[cfg(feature = "http")]
    #[arg(long, num_args(0..=1), default_missing_value("true"))]
    pub no_proxy: Option<bool>
}

/// The enum of errors [`ParamsDiffArgParser::build`] can return
///
/// [`clap`] is currently missing ways to express certain requirements at parse time.
#[derive(Debug, Error)]
pub enum ParamsDiffArgParserValueWrong {
    /// Returned when an invocation of [`ParamsDiffArgParser::var`] doesn't have a name specified.
    #[error("--var didn't have a name specified.")]
    VarNoNameSpecified,
    /// Returned when an invocation of [`ParamsDiffArgParser::var`] doesn't have a value specified.
    #[error("--var didn't have a value specified.")]
    VarNoValueSpecified,
    /// Returned when an invocation of [`ParamsDiffArgParser::var`] has too many arguments.
    #[error("--var had too many arguments.")]
    VarTooManyArguments,
    /// Returned when an invocation of [`ParamsDiffArgParser::insert_into_set`] doesn't have a name.
    #[error("--insert-into-set didn't have a name.")]
    InsertIntoSetsNoName,
    /// Returned when an invocation of [`ParamsDiffArgParser::remove_from_set`] doesn't have a name.
    #[error("--remove-from-set didn't have a name.")]
    RemoveFromSetsNoName,
    /// Returned when an invocation of [`ParamsDiffArgParser::insert_into_map`] doesn't have a name.
    #[error("--insert-into-map didn't have a name.")]
    InsertIntoMapNoName,
    /// Returned when an invocation of [`ParamsDiffArgParser::insert_into_map`] has a key without a value.
    #[error("--insert-into-map found a key without a value.")]
    InsertIntoMapKeyWithoutValue,
    /// Returned when an invocation of [`ParamsDiffArgParser::remove_from_map`] doesn't have a map specified.
    #[error("--remove-from-map didn't have a map specified.")]
    RemoveFromMapNoMapSpecified,
}

impl ParamsDiffArgParserValueWrong {
    /// Gets a [`str`] of the error message.
    pub fn as_str(&self) -> &str {
        match self {
            Self::VarNoNameSpecified           => "--var didn't have a name specified.",
            Self::VarNoValueSpecified          => "--var didn't have a value specified.",
            Self::VarTooManyArguments          => "--var had too many arguments.",

            Self::InsertIntoSetsNoName         => "--insert-into-set didn't have a name.",
            Self::RemoveFromSetsNoName         => "--remove-from-set didn't have a name.",

            Self::InsertIntoMapNoName          => "--insert-into-map didn't have a name.",
            Self::InsertIntoMapKeyWithoutValue => "--insert-into-map found a key without a value.",
            Self::RemoveFromMapNoMapSpecified  => "--remove-from-map didn't have a map specified.",
        }
    }
}

impl TryFrom<ParamsDiffArgParser> for ParamsDiff {
    type Error = ParamsDiffArgParserValueWrong;

    fn try_from(value: ParamsDiffArgParser) -> Result<Self, ParamsDiffArgParserValueWrong> {
        value.build()
    }
}

impl ParamsDiffArgParser {
    /// Builds the [`ParamsDiff`].
    /// # Errors
    /// If an invocation of [`Self::var`] doesn't have a name specified, returns the error [`ParamsDiffArgParserValueWrong::VarNoNameSpecified`].
    /// 
    /// If an invocation of [`Self::var`] doesn't have a value specified, returns the error [`ParamsDiffArgParserValueWrong::VarNoValueSpecified`].
    ///
    /// If an invocation of [`Self::var`] has more than 2 values, returns the error [`ParamsDiffArgParserValueWrong::VarTooManyArguments`].
    ///
    /// If an invocation of [`Self::insert_into_set`] doesn't have a name pacified, returns the error [`ParamsDiffArgParserValueWrong::InsertIntoSetsNoName`].
    ///
    /// If an invocation of [`Self::remove_from_set`] doesn't have a name specified, returns the error [`ParamsDiffArgParserValueWrong::RemoveFromSetsNoName`].
    ///
    /// If an invocation of [`Self::insert_into_map`] doesn't have a name specified, returns the error [`ParamsDiffArgParserValueWrong::InsertIntoMapNoName`].
    ///
    /// If an invocation of [`Self::insert_into_map`] has a key with no value specified, returns the error [`ParamsDiffArgParserValueWrong::InsertIntoMapKeyWithoutValue`].
    ///
    /// If an invocation of [`Self::remove_from_map`] doesn't have a name specified, returns the error [`ParamsDiffArgParserValueWrong::RemoveFromMapNoMapSpecified`].
    pub fn build(self) -> Result<ParamsDiff, ParamsDiffArgParserValueWrong> {
        Ok(ParamsDiff {
            flags  : self.flag  .into_iter().collect(),
            unflags: self.unflag.into_iter().collect(),
            vars   : self.var   .into_iter().map(|kv| match <Vec<_> as TryInto<[String; 2]>>::try_into(kv) {
                Ok([k, v]) => Ok((k, v)),
                Err(x) => Err(match x.len() {
                    0 => ParamsDiffArgParserValueWrong::VarNoNameSpecified,
                    1 => ParamsDiffArgParserValueWrong::VarNoValueSpecified,
                    2 => unreachable!(),
                    _ => ParamsDiffArgParserValueWrong::VarTooManyArguments
                })
            }).collect::<Result<_, _>>()?,
            unvars : self.unvar.into_iter().collect(),
            init_sets: Default::default(),
            insert_into_sets: self.insert_into_set.into_iter().map(|mut x| if !x.is_empty() {Ok((x.swap_remove(0), x.into_iter().map(Some).collect()))} else {Err(ParamsDiffArgParserValueWrong::InsertIntoSetsNoName)}).collect::<Result<_, _>>()?,
            remove_from_sets: self.remove_from_set.into_iter().map(|mut x| if !x.is_empty() {Ok((x.swap_remove(0), x.into_iter().map(Some).collect()))} else {Err(ParamsDiffArgParserValueWrong::RemoveFromSetsNoName)}).collect::<Result<_, _>>()?,
            delete_sets     : Default::default(),
            init_maps       : Default::default(),
            map_diffs       : {
                let mut ret = HashMap::<String, MapDiff<String>>::new();

                for invocation in self.insert_into_map {
                    if invocation.len() % 2 == 1 {
                        let mut x = invocation.into_iter();
                        ret.entry(x.next().ok_or(ParamsDiffArgParserValueWrong::InsertIntoMapNoName)?).or_default().insert= std::iter::from_fn(|| x.next().zip(x.next())).collect();
                    } else {
                        Err(ParamsDiffArgParserValueWrong::InsertIntoMapKeyWithoutValue)?
                    }
                }

                for invocation in self.remove_from_map {
                    if !invocation.is_empty() {
                        let mut x = invocation.into_iter();
                        let name = x.next().ok_or(ParamsDiffArgParserValueWrong::RemoveFromMapNoMapSpecified)?;
                        ret.entry(name).or_default().remove = x.collect();
                    } else {
                        Err(ParamsDiffArgParserValueWrong::RemoveFromMapNoMapSpecified)?
                    }
                }

                ret
            },
            delete_maps     : Default::default(),
            #[cfg(feature = "cache")] read_cache : self.read_cache,
            #[cfg(feature = "cache")] write_cache: self.write_cache,
            #[cfg(feature = "http")] http_client_config_diff: Some(HttpClientConfigDiff {
                set_proxies: self.proxy.map(|x| vec![x]),
                no_proxy: self.no_proxy,
                ..HttpClientConfigDiff::default()
            })
        })
    }

    /// Checks if calling [`Self::build`] and applying the resulting [`ParamsDiff`] would do anything.
    ///
    /// MUCH faster than just unconditionally trying it.
    pub fn does_anything(&self) -> bool {
        #[allow(unused_mut, reason = "It is used.")]
        let mut feature_flag_make_params_diff = false;
        #[cfg(feature = "cache")] #[allow(clippy::unnecessary_operation, reason = "False positive.")] {feature_flag_make_params_diff = feature_flag_make_params_diff || self.read_cache.is_some()};
        #[cfg(feature = "cache")] #[allow(clippy::unnecessary_operation, reason = "False positive.")] {feature_flag_make_params_diff = feature_flag_make_params_diff || self.write_cache.is_some()};
        #[cfg(feature = "http" )] #[allow(clippy::unnecessary_operation, reason = "False positive.")] {feature_flag_make_params_diff = feature_flag_make_params_diff || self.proxy.is_some()};
        !self.flag.is_empty() || !self.unflag.is_empty() || !self.var.is_empty() || !self.unvar.is_empty() || !self.insert_into_set.is_empty() || !self.remove_from_set.is_empty() || !self.insert_into_map.is_empty() || !self.remove_from_map.is_empty() || feature_flag_make_params_diff
    }
}
