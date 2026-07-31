//! Clean.

use std::collections::HashMap;

use clap::Parser;

use better_url::prelude::*;

mod http;
mod ws;

/// /clean.
#[derive(Debug, Parser)]
pub struct Args {
    /** The instance.               **/ #[arg(default_value = "ws://127.0.0.1:9149/"  )] pub instance       : BetterUrl,
    /** The username.               **/ #[arg(long, short = 'u', requires = "password")] pub username       : Option<String>,
    /** The password.               **/ #[arg(long, short = 'p',                      )] pub password       : Option<String>,
    /** Enable brief unchanged.     **/ #[arg(long, short = 'U',                      )] pub brief_unchanged: bool,
    /** Enable brief error.         **/ #[arg(long, short = 'E',                      )] pub brief_error    : bool,
    /** The JobContext.             **/ #[arg(long,                                   )] pub context        : Option<String>,
    /** The profile.                **/ #[arg(long,                                   )] pub profile        : Option<String>,
    /** The ParamsDiff.             **/ #[arg(long,                                   )] pub params_diff    : Option<String>,
    /** Disable the HTTP client.    **/ #[arg(long, short = 'H'                       )] pub no_http        : bool,
    /** Disable reading from cache. **/ #[arg(long, short = 'R'                       )] pub no_read_cache  : bool,
    /** Disable writing to cache.   **/ #[arg(long, short = 'W'                       )] pub no_write_cache : bool,
    /** Enable cache delay.         **/ #[arg(long, short = 'd'                       )] pub cache_delay    : bool,
    /** Enable unthreading.         **/ #[arg(long                                    )] pub unthread       : bool,
}

impl Args {
    /// Do the command.
    pub async fn r#do(self) {
        let mut instance = self.instance;

        instance.set_path("/clean").unwrap();

        let mut config = HashMap::<_, serde_json::Value>::new();

        if let Some(username   ) = self.username    {config.insert("username"   , username.into());}
        if let Some(password   ) = self.password    {config.insert("password"   , password.into());}
        if let Some(profile    ) = self.profile     {config.insert("profile"    , profile .into());}
        if let Some(params_diff) = self.params_diff {config.insert("params_diff", serde_json::from_str(&params_diff).unwrap());}
        if let Some(context    ) = self.context     {config.insert("context"    , serde_json::from_str(&context    ).unwrap());}

        if self.no_http         {config.insert("http"           , false.into());}
        if self.no_read_cache   {config.insert("read_cache"     , false.into());}
        if self.no_write_cache  {config.insert("write_cache"    , false.into());}
        if self.cache_delay     {config.insert("cache_delay"    , true .into());}
        if self.unthread        {config.insert("unthread"       , true .into());}
        if self.brief_unchanged {config.insert("brief_unchanged", true .into());}
        if self.brief_error     {config.insert("brief_error"    , true .into());}

        instance.set_query_param("config", 0, Some(Some(&serde_json::to_string(&config).unwrap()))).unwrap();

        match instance.scheme_str() {
            "http" | "https" => http::r#do(instance).await,
            "ws"   | "wss"   => ws  ::r#do(instance).await,
            x => panic!("Unknwon protocol {x}")
        }
    }
}
