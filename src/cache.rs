use std::{fs, path::PathBuf, sync::Arc, time::Duration};

use moka::{
    future::{Cache, FutureExt},
    notification::{ListenerFuture, RemovalCause},
};
use serenity::prelude::TypeMapKey;
use tracing::{error, info, instrument};

use self::error::{Error, LoadError, SaveError};
use crate::config::GuildConfig;

mod error;

const CACHE_DURATION_IN_SECONDS: u64 = 20 * 60;

fn get_config_path(id: u64) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(env!("CARGO_MANIFEST_DIR"));
    path.push("configs");
    path.push(format!("{id}.json"));

    path
}

pub struct GuildConfigCache {
    guild_configs: Cache<u64, GuildConfig>,
}

impl TypeMapKey for GuildConfigCache {
    type Value = Self;
}

impl GuildConfigCache {
    #[instrument]
    pub fn new() -> Self {
        info!(
            "Creating GuildConfigCache with an idle time of {} seconds.",
            CACHE_DURATION_IN_SECONDS
        );

        Self {
            guild_configs: Cache::builder()
                .time_to_idle(Duration::from_secs(CACHE_DURATION_IN_SECONDS))
                .async_eviction_listener(Self::on_eviction)
                .build(),
        }
    }

    #[instrument]
    fn on_eviction(key: Arc<u64>, value: GuildConfig, cause: RemovalCause) -> ListenerFuture {
        info!("Key has evicted.");
        async move {
            if cause == RemovalCause::Expired {
                info!("Key was removed because it expired.");
                Self::save(*key.as_ref(), value).await.unwrap();
            }
        }
        .boxed()
    }

    #[instrument(skip(self))]
    pub async fn insert(&self, id: u64, config: GuildConfig) {
        info!("Inserting key...");
        self.guild_configs.insert(id, config).await;
        info!("Key inserted.");
    }

    #[instrument(skip(self))]
    pub async fn get(&self, id: u64) -> Result<GuildConfig, Error> {
        info!("Trying to get config...");
        let config = self.guild_configs.get(&id).await;

        if config.is_none() {
            info!("Config wasn't loaded.");
            self.load(id).await?;
            return Ok(self
                .guild_configs
                .get(&id)
                .await
                .expect("Config should be in memory."));
        }

        info!("Config was loaded.");
        Ok(config.expect("None case was handled earlier."))
    }

    #[instrument]
    async fn save(id: u64, config: GuildConfig) -> Result<bool, SaveError> {
        info!("Saving GuildConfig...");
        let path = get_config_path(id);

        if !config.has_changed {
            info!("Nothing to do.");
            return Ok(false);
        }

        let json = match serde_json::to_string(&config) {
            Ok(json) => json,
            Err(err) => {
                error!("Failed to serialize config to JSON: {err}");
                return Err(SaveError::from(err));
            }
        };

        match fs::write(path, json) {
            Ok(_) => {
                info!("Saved successfully.");
                Ok(true)
            }
            Err(err) => {
                error!("Failed to write config file: {err}");
                return Err(SaveError::from(err));
            }
        }
    }

    #[instrument(skip(self))]
    async fn load(&self, id: u64) -> Result<GuildConfig, LoadError> {
        info!("Loading config...");
        let path = get_config_path(id);
        if !path.exists() {
            error!("Config file doesn't exist.");
            return Err(LoadError::ConfigNotFound(id));
        }

        let raw_data = match fs::read_to_string(&path) {
            Ok(path) => path,
            Err(err) => {
                error!("Failed to read config file: {err}");
                return Err(LoadError::from(err));
            }
        };

        let config: GuildConfig = match serde_json::from_str(&raw_data) {
            Ok(config) => config,
            Err(err) => {
                error!("Failed to deserialize config from JSON: {err}");
                return Err(LoadError::FailedDeserialization(err));
            }
        };

        self.guild_configs.insert(id, config).await;

        info!("Config loaded.");
        Ok(config)
    }

    #[instrument(skip(self))]
    pub async fn run_pending_tasks(&self) {
        info!("Running pending tasks...");
        self.guild_configs.run_pending_tasks().await;
        info!("Pending tasks completed.");
    }
}
