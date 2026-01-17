//! Clean.

use std::collections::HashMap;

use url::Url;
use clap::Parser;

mod http;
mod ws;

/// /clean.
#[derive(Debug, Parser)]
pub struct Args {
    /// The instance (HTTP, HTTPS, WS, or WSS)
    #[arg(default_value = "ws://127.0.0.1:9149")]
    pub instance: String,
    /// The password.
    #[arg(long, help_heading = "JobConfig args")]
    pub password: Option<String>,
    /// The JobContext.
    #[arg(long, help_heading = "JobConfig args")]
    pub context: Option<String>,
    /// The profile.
    #[arg(long, help_heading = "JobConfig args")]
    pub profile: Option<String>,
    /// The ParamsDiff.
    #[arg(long, help_heading = "JobConfig args")]
    pub params_diff: Option<String>,
    /// Disable reading from cache.
    #[arg(long, help_heading = "JobConfig args")]
    pub no_read_cache: bool,
    /// Disable writing to cache.
    #[arg(long, help_heading = "JobConfig args")]
    pub no_write_cache: bool,
    /// Enable cache delay.
    #[arg(long, help_heading = "JobConfig args")]
    pub cache_delay: bool,
    /// Enable unthreading
    #[arg(long, help_heading = "JobConfig args")]
    pub unthread: bool
}

impl Args {
    /// Do the command.
    pub async fn r#do(self) {
        let mut instance = Url::parse(&self.instance).unwrap();

        instance.set_path("/clean");

        let mut config = HashMap::<_, serde_json::Value>::new();

        if let Some(password   ) = self.password    {config.insert("password"   , password.into());}
        if let Some(profile    ) = self.profile     {config.insert("profile"    , profile .into());}
        if let Some(params_diff) = self.params_diff {config.insert("params_diff", serde_json::from_str(&params_diff).unwrap());}
        if let Some(context    ) = self.context     {config.insert("context"    , serde_json::from_str(&context    ).unwrap());}

        if self.no_read_cache  {config.insert("read_cache" , false.into());}
        if self.no_write_cache {config.insert("write_cache", false.into());}
        if self.cache_delay    {config.insert("cache_delay", true .into());}
        if self.unthread       {config.insert("unthread"   , true .into());}

        instance.query_pairs_mut().append_pair("config", &serde_json::to_string(&config).unwrap());

        match instance.scheme() {
            "http" | "https" => http::r#do(instance).await,
            "ws"   | "wss"   => ws  ::r#do(instance).await,
            x => panic!("Unknwon protocol {x}")
        }
    }
}

