//! Clean.

use std::collections::HashMap;

use clap::Parser;

use better_url::prelude::*;

mod http;
mod ws;

/// /clean.
#[derive(Debug, Parser)]
pub struct Args {
    /** The instance.               **/ #[arg(default_value = "ws://127.0.0.1:9149/")] pub instance       : BetterUrl,
    /** The password.               **/ #[arg(long, short = 'p'                     )] pub password       : Option<String>,
    /** The profile.                **/ #[arg(long,                                 )] pub profile        : Option<String>,
    /** The ParamsDiff.             **/ #[arg(long,                                 )] pub params_diff    : Option<String>,
    /** The JobContext.             **/ #[arg(long,                                 )] pub context        : Option<String>,
    /** Enable brief unchanged.     **/ #[arg(long, short = 'u'                     )] pub brief_unchanged: bool,
    /** Enable brief error.         **/ #[arg(long, short = 'e'                     )] pub brief_error    : bool,
    /** Hide the thread count.      **/ #[arg(long, short = 'T'                     )] pub hide_threads   : bool,
    /** Disable the HTTP client.    **/ #[arg(long, short = 'H'                     )] pub no_http        : bool,
    /** Disable reading from cache. **/ #[arg(long, short = 'R'                     )] pub no_read_cache  : bool,
    /** Disable writing to cache.   **/ #[arg(long, short = 'W'                     )] pub no_write_cache : bool,
    /** Hide cache reads.           **/ #[arg(long, short = 'C'                     )] pub hide_cache     : bool,
}

impl Args {
    /// Do the command.
    pub async fn r#do(self) {
        let mut instance = self.instance;

        instance.set_path("/clean").unwrap();

        let mut config = HashMap::<_, serde_json::Value>::new();

        if let Some(password   ) = self.password    {config.insert("password"   , password   .into()          );}
        if let Some(profile    ) = self.profile     {config.insert("profile"    , profile    .into()          );}
        if let Some(params_diff) = self.params_diff {config.insert("params_diff", params_diff.parse().unwrap());}
        if let Some(context    ) = self.context     {config.insert("context"    , context    .parse().unwrap());}

        if self.brief_unchanged {config.insert("brief_unchanged", true .into());}
        if self.brief_error     {config.insert("brief_error"    , true .into());}
        if self.hide_threads    {config.insert("hide_threads"   , true .into());}
        if self.no_http         {config.insert("http"           , false.into());}
        if self.no_read_cache   {config.insert("read_cache"     , false.into());}
        if self.no_write_cache  {config.insert("write_cache"    , false.into());}
        if self.hide_cache      {config.insert("hide_cache"     , true .into());}

        instance.set_query_param("config", 0, Some(Some(&serde_json::to_string(&config).unwrap()))).unwrap();

        match instance.scheme_str() {
            "http" | "https" => http::r#do(instance).await,
            "ws"   | "wss"   => ws  ::r#do(instance).await,
            x => panic!("Unknwon protocol {x}")
        }
    }
}
