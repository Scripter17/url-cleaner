//! Allows passing additional details into various types in URL Cleaner.

use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};

use crate::types::*;
use crate::glue::*;
use crate::util::*;

/// Configuration options to choose the behaviour of various URL Cleaner types.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Params {
    /// Booleans variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub flags: HashSet<String>,
    /// String variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub vars: HashMap<String, String>,
    /// Set variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sets: HashMap<String, HashSet<String>>,
    /// List variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub lists: HashMap<String, Vec<String>>,
    /// Map variables used to determine behavior.
    #[serde(default, skip_serializing_if = "is_default")]
    pub maps: HashMap<String, HashMap<String, String>>,
    /// If [`true`], enables reading from caches. Defaults to [`true`]
    #[cfg(feature = "cache")]
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub read_cache: bool,
    /// If [`true`], enables writing to caches. Defaults to [`true`]
    #[cfg(feature = "cache")]
    #[serde(default = "get_true", skip_serializing_if = "is_true")]
    pub write_cache: bool,
    /// The default headers to send in HTTP requests.
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
            #[cfg(feature = "cache")] read_cache: true,
            #[cfg(feature = "cache")] write_cache: true,
            #[cfg(feature = "http")]
            http_client_config: HttpClientConfig::default()
        }
    }
}

impl Params {
    /// Makes sure all the listed things are documented.
    /// # Panics
    /// When it fails, a panic occurs to make debugging easier.
    pub fn is_suitable_for_release(&self, config: &Config) -> bool {
        let x = self.flags.iter().find(|flag| !config.docs.flags.contains_key(&**flag)); assert!(x.is_none(), "Undocumented flag in params: {x:?}");
        let x = self.vars .keys().find(|var | !config.docs.vars .contains_key(&**var )); assert!(x.is_none(), "Undocumented var in params: {x:?}" );
        let x = self.sets .keys().find(|set | !config.docs.sets .contains_key(&**set )); assert!(x.is_none(), "Undocumented set in params: {x:?}" );
        let x = self.lists.keys().find(|list| !config.docs.lists.contains_key(&**list)); assert!(x.is_none(), "Undocumented list in params: {x:?}");
        let x = self.maps .keys().find(|map | !config.docs.maps .contains_key(&**map )); assert!(x.is_none(), "Undocumented map in params: {x:?}" );
        true
    }
}

/// Allows changing [`Config::params`].
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ParamsDiff {
    /// Adds to [`Params::flags`]. Defaults to an empty [`HashSet`].
    #[serde(default, skip_serializing_if = "is_default")] pub flags  : HashSet<String>,
    /// Removes from [`Params::flags`] Defaults to an empty [`HashSet`].
    #[serde(default, skip_serializing_if = "is_default")] pub unflags: HashSet<String>,
    /// Adds to [`Params::vars`]. Defaults to an empty [`HashMap`].
    #[serde(default, skip_serializing_if = "is_default")] pub vars  : HashMap<String, String>,
    /// Removes from [`Params::vars`]. Defaults to an empty [`HashSet`].
    #[serde(default, skip_serializing_if = "is_default")] pub unvars: HashSet<String>,
    /// Initializes new sets in [`Params::sets`].
    #[serde(default, skip_serializing_if = "is_default")] pub init_sets: Vec<String>,
    /// Initializes new sets in [`Params::sets`] if they don't already exist, then inserts values into them.
    #[serde(default, skip_serializing_if = "is_default")] pub insert_into_sets: HashMap<String, Vec<String>>,
    /// If the sets exist in [`Params::sets`], removes values from them.
    #[serde(default, skip_serializing_if = "is_default")] pub remove_from_sets: HashMap<String, Vec<String>>,
    /// If the sets exist in [`Params::sets`], remove them.
    #[serde(default, skip_serializing_if = "is_default")] pub delete_sets: Vec<String>,
    /// Initializes new maps in [`Params::maps`].
    #[serde(default, skip_serializing_if = "is_default")] pub init_maps: Vec<String>,
    /// Initializes new maps in [`Params::maps`] if they don't already exist, then inserts values into them.
    #[serde(default, skip_serializing_if = "is_default")] pub insert_into_maps: HashMap<String, HashMap<String, String>>,
    /// If the maps exist in [`Params::maps`], removes values from them.
    #[serde(default, skip_serializing_if = "is_default")] pub remove_from_maps: HashMap<String, Vec<String>>,
    /// If the maps exist in [`Params::maps`], remove them.
    #[serde(default, skip_serializing_if = "is_default")] pub delete_maps: Vec<String>,
    /// If [`Some`], sets [`Params::read_cache`]. Defaults to [`None`].
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")] pub read_cache : Option<bool>,
    /// If [`Some`], sets [`Params::write_cache`]. Defaults to [`None`].
    #[cfg(feature = "cache")]
    #[serde(default, skip_serializing_if = "is_default")] pub write_cache: Option<bool>,
    /// If [`Some`], calls [`HttpClientConfigDiff::apply`] with `to`'s [`HttpClientConfig`]. Defaults to [`None`].
    #[cfg(feature = "http")]
    #[serde(default, skip_serializing_if = "is_default")] pub http_client_config_diff: Option<HttpClientConfigDiff>
}

impl ParamsDiff {
    /// Applies the differences specified in `self` to `to`.
    /// In order:
    /// 1. Extends `to.flags` with [`Self::flags`].
    /// 2. Removes all flags found in [`Self::unflags`] from `to.flags`.
    /// 3. Extends `to.vars` with [`Self::vars`], overwriting any keys found in both.
    /// 4. Removes all keys found in [`Self::unvars`] from `to.vars`.
    /// 5. Initializes all sets specified by [`Self::init_sets`] to [`HashSet::default`] if they don't exist.
    /// 6. Inserts all values into sets as specified by [`Self::insert_into_sets`].
    /// 7. Removes all values from sets as specified by [`Self::remove_from_sets`].
    /// 8. Deletes all sets specified in [`Self::delete_sets`].
    /// 9. Initializes all maps specified by [`Self::init_maps`] to [`HashSet::default`] if they don't exist.
    /// 10. Inserts all values into maps as specified by [`Self::insert_into_maps`].
    /// 11. Removes all values from maps as specified by [`Self::remove_from_maps`].
    /// 12. Deletes all maps specified in [`Self::delete_maps`].
    /// 13. If [`Self::read_cache`] is [`Some`], sets `to.read_cache` to the contained value.
    /// 14. If [`Self::write_cache`] is [`Some`], sets `to.write_cache` to the contained value.
    /// 15. If [`Self::http_client_config_diff`] is [`Some`], calls [`HttpClientConfigDiff::apply`] with `to.http_client_config`.
    pub fn apply(&self, to: &mut Params) {
        #[cfg(feature = "debug")]
        let old_to = to.clone();

        to.flags.extend(self.flags.clone());
        for flag in &self.unflags {to.flags.remove(flag);}

        to.vars.extend(self.vars.clone());
        for var in &self.unvars {to.vars.remove(var);}

        for k in self.init_sets.iter() {
            if !to.sets.contains_key(k) {to.sets.insert(k.clone(), Default::default());}
        }
        for (k, v) in self.insert_into_sets.iter() {
            to.sets.entry(k.clone()).or_default().extend(v.clone());
        }
        for (k, vs) in self.remove_from_sets.iter() {
            if let Some(x) = to.sets.get_mut(k) {
                for v in vs.iter() {
                    x.remove(v);
                }
            }
        }
        for k in self.delete_sets.iter() {
            to.sets.remove(k);
        }

        for k in self.init_maps.iter() {
            if !to.maps.contains_key(k) {to.maps.insert(k.clone(), Default::default());}
        }
        for (k, v) in self.insert_into_maps.iter() {
            to.maps.entry(k.clone()).or_default().extend(v.clone());
        }
        for (k, vs) in self.remove_from_maps.iter() {
            if let Some(x) = to.maps.get_mut(k) {
                for v in vs {
                    x.remove(v);
                }
            }
        }
        for k in self.delete_maps.iter() {
            to.maps.remove(k);
        }

        #[cfg(feature = "cache")] if let Some(read_cache ) = self.read_cache  {to.read_cache  = read_cache ;}
        #[cfg(feature = "cache")] if let Some(write_cache) = self.write_cache {to.write_cache = write_cache;}

        #[cfg(feature = "http")] if let Some(http_client_config_diff) = &self.http_client_config_diff {http_client_config_diff.apply(&mut to.http_client_config);}
        debug!(ParamsDiff::apply, self, old_to, to);
    }
}

/// Shared argument parser for generating [`ParamsDiff`]s from the CLI.
/// 
/// Used with the [`#[command(flatten)]`](https://docs.rs/clap/latest/clap/_derive/index.html#command-attributes) part of [`clap::Parser`]'s derive macro.
#[derive(Debug, Clone, PartialEq, Eq, clap::Args)]
pub struct ParamsDiffArgParser {
    /// Set flags.
    #[arg(short      , long, value_names = ["NAME"])]
    pub flag  : Vec<String>,
    /// Unset flags set by the config.
    #[arg(short = 'F', long, value_names = ["NAME"])]
    pub unflag: Vec<String>,
    /// For each occurrence of this option, its first argument is the variable name and the second argument is its value.
    #[arg(short      , long, num_args(2), value_names = ["NAME", "VALUE"])]
    pub var: Vec<Vec<String>>,
    /// Unset variables set by the config.
    #[arg(short = 'V', long, value_names = ["NAME"])]
    pub unvar : Vec<String>,
    /// For each occurrence of this option, its first argument is the set name and subsequent arguments are the values to insert.
    #[arg(             long, num_args(1..), value_names = ["NAME", "VALUE1"])]
    pub insert_into_set: Vec<Vec<String>>,
    /// For each occurrence of this option, its first argument is the set name and subsequent arguments are the values to remove.
    #[arg(             long, num_args(1..), value_names = ["NAME", "VALUE1"])]
    pub remove_from_set: Vec<Vec<String>>,
    /// For each occurrence of this option, its first argument is the map name, the second is the map key, and subsequent arguments are the values to insert.
    #[arg(             long, num_args(2..), value_names = ["NAME", "KEY1", "VALUE1"])]
    pub insert_into_map: Vec<Vec<String>>,
    /// For each occurrence of this option, its first argument is the map name, and subsequent arguments are the keys to remove.
    #[arg(             long, num_args(1..), value_names = ["NAME", "KEY1"])]
    pub remove_from_map: Vec<Vec<String>>,
    /// Read stuff from caches. Default value is controlled by the config. Omitting a value means true.
    #[cfg(feature = "cache")]
    #[arg(             long, num_args(0..=1), default_missing_value("true"))]
    pub read_cache : Option<bool>,
    /// Write stuff to caches. Default value is controlled by the config. Omitting a value means true.
    #[cfg(feature = "cache")]
    #[arg(             long, num_args(0..=1), default_missing_value("true"))]
    pub write_cache: Option<bool>,
    /// The proxy to use. Example: socks5://localhost:9150
    #[cfg(feature = "http")]
    #[arg(             long)]
    pub proxy: Option<ProxyConfig>,
    /// Disables all HTTP proxying.
    #[cfg(feature = "http")]
    #[arg(             long, num_args(0..=1), default_missing_value("true"))]
    pub no_proxy: Option<bool>
}

/// The errors that deriving [`clap::Parser`] can't catch.
#[derive(Debug, Error)]
pub enum ParamsDiffArgParserValueWrong {
    /// Returned when a `--var` invocation doesn't have a name (0 arguments).
    #[error("--var didn't have a name specified.")]
    VarNoNameSpecified,
    /// Returned when a `--var` invocation doesn't have a value (1 argument).
    #[error("--var didn't have a value specified.")]
    VarNoValueSpecified,
    /// Returned when a `--var` invocation has too many (3 or more arguments).
    #[error("--var had too many arguments.")]
    VarTooManyArguments,

    /// Returned when an `--insert-into-set` invocation doesn't have a name (0 arguments).
    #[error("--insert-into-set didn't have a name.")]
    InsertIntoSetsNoName,
    /// Returned when a `--remove-from-set` invocation doesn't have a name (0 arguments).
    #[error("--remove-from-set didn't have a name.")]
    RemoveFromSetsNoName,

    /// Returned when an `--insert-into-map` invocation doesn't have a name. (0 arguments).
    #[error("--insert-into-map didn't have a name.")]
    InsertIntoMapNoName,
    /// Returned when an `--insert-into-map` invocation has a key without a value (even number of arguments).
    #[error("--insert-into-map found a key without a value.")]
    InsertIntoMapKeyWithoutValue,
    /// Returned when a `--remove-from-map` invocation doesn't have a map (0 arguments).
    #[error("--remove-from-map didn't have a map specified.")]
    RemoveFromMapNoMapSpecified,
}

impl ParamsDiffArgParserValueWrong {
    /// Gets the error message.
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
        Ok(ParamsDiff {
            flags  : value.flag  .into_iter().collect(),
            unflags: value.unflag.into_iter().collect(),
            vars   : value.var   .into_iter().map(|kv| match <Vec<_> as TryInto<[String; 2]>>::try_into(kv) {
                Ok([k, v]) => Ok((k, v)),
                Err(x) => Err(match x.len() {
                    0 => ParamsDiffArgParserValueWrong::VarNoNameSpecified,
                    1 => ParamsDiffArgParserValueWrong::VarNoValueSpecified,
                    2 => unreachable!(),
                    _ => ParamsDiffArgParserValueWrong::VarTooManyArguments
                })
            }).collect::<Result<_, _>>()?,
            unvars : value.unvar.into_iter().collect(),
            init_sets: Default::default(),
            insert_into_sets: value.insert_into_set.into_iter().map(|mut x| if !x.is_empty() {Ok((x.swap_remove(0), x))} else {Err(ParamsDiffArgParserValueWrong::InsertIntoSetsNoName)}).collect::<Result<_, _>>()?,
            remove_from_sets: value.remove_from_set.into_iter().map(|mut x| if !x.is_empty() {Ok((x.swap_remove(0), x))} else {Err(ParamsDiffArgParserValueWrong::RemoveFromSetsNoName)}).collect::<Result<_, _>>()?,
            delete_sets     : Default::default(),
            init_maps       : Default::default(),
            insert_into_maps: value.insert_into_map.into_iter().map(|x|
                if x.len()%2 == 1 {
                    let mut i = x.into_iter();
                    Ok((i.next().ok_or(ParamsDiffArgParserValueWrong::InsertIntoMapNoName)?, std::iter::from_fn(|| i.next().zip(i.next())).collect()))
                } else {
                    Err(ParamsDiffArgParserValueWrong::InsertIntoMapKeyWithoutValue)?
                }
            ).collect::<Result<_, _>>()?,
            remove_from_maps: value.remove_from_map.into_iter().map(|mut x| if !x.is_empty() {Ok((x.swap_remove(0), x))} else {Err(ParamsDiffArgParserValueWrong::RemoveFromMapNoMapSpecified)}).collect::<Result<HashMap<_, _>, _>>()?,
            delete_maps     : Default::default(),
            #[cfg(feature = "cache")] read_cache : value.read_cache,
            #[cfg(feature = "cache")] write_cache: value.write_cache,
            #[cfg(feature = "http")] http_client_config_diff: Some(HttpClientConfigDiff {
                set_proxies: value.proxy.map(|x| vec![x]),
                no_proxy: value.no_proxy,
                ..HttpClientConfigDiff::default()
            })
        })
    }
}

impl ParamsDiffArgParser {
    /// Returns [`true`] if this would make a [`ParamsDiff`] that actually does anything, or if making a [`ParamsDiff`] would error.
    /// 
    /// It's much faster to check this than make and apply the [`ParamsDiff`].
    pub fn does_anything(&self) -> bool {
        #[allow(unused_mut, reason = "It is used.")]
        let mut feature_flag_make_params_diff = false;
        #[cfg(feature = "cache")] #[allow(clippy::unnecessary_operation, reason = "False positive.")] {feature_flag_make_params_diff = feature_flag_make_params_diff || self.read_cache.is_some()};
        #[cfg(feature = "cache")] #[allow(clippy::unnecessary_operation, reason = "False positive.")] {feature_flag_make_params_diff = feature_flag_make_params_diff || self.write_cache.is_some()};
        #[cfg(feature = "http" )] #[allow(clippy::unnecessary_operation, reason = "False positive.")] {feature_flag_make_params_diff = feature_flag_make_params_diff || self.proxy.is_some()};
        !self.flag.is_empty() || !self.unflag.is_empty() || !self.var.is_empty() || !self.unvar.is_empty() || !self.insert_into_set.is_empty() || !self.remove_from_set.is_empty() || !self.insert_into_map.is_empty() || !self.remove_from_map.is_empty() || feature_flag_make_params_diff
    }
}
