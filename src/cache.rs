use std::{env::current_dir, fs, path::PathBuf, sync::Arc, time::Duration};

use moka::{
    future::{Cache, FutureExt},
    notification::{ListenerFuture, RemovalCause},
};
use tracing::{debug, error, info, instrument};

use self::error::{Error, LoadError, SaveError};
use crate::config::GuildConfig;

pub mod error;

const CACHE_DURATION_IN_SECONDS: u64 = 20 * 60;

#[instrument]
fn get_config_path(id: u64) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(current_dir().expect("The current directory could not be obtained."));
    path.push("configs");
    path.push(format!("{id}.json"));

    path
}

pub struct GuildConfigCache {
    guild_configs: Cache<u64, GuildConfig>,
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
        debug!("Key has evicted.");
        let id = *key.as_ref();
        async move {
            let _ = Self::save(id, value).await;
        }
        .boxed()
    }

    #[instrument(skip(self))]
    pub fn invalidate_all(&self) {
        self.guild_configs.invalidate_all();
    }

    #[instrument(skip(self))]
    pub async fn insert(&self, id: u64, config: GuildConfig) {
        debug!("Inserting key...");
        self.guild_configs.insert(id, config).await;
        debug!("Key inserted.");
    }

    #[instrument(skip(self))]
    pub async fn get_or_insert(&self, id: u64) -> Result<GuildConfig, Error> {
        debug!("Trying to get config...");
        let config = self.guild_configs.get(&id).await;

        if config.is_none() {
            debug!("Config wasn't loaded.");
            self.load(id, true).await?;
            return Ok(self
                .guild_configs
                .get(&id)
                .await
                .expect("Config should be in memory."));
        }

        debug!("Config was loaded.");
        Ok(config.expect("None case was handled earlier."))
    }

    #[instrument]
    async fn save(id: u64, config: GuildConfig) -> Result<bool, SaveError> {
        debug!("Saving GuildConfig...");

        if !config.has_changed() {
            debug!("Nothing to do.");
            return Ok(false);
        }

        let path = get_config_path(id);
        let directory = path
            .parent()
            .expect("Function should always return a file path.");

        if !directory.exists() {
            if let Err(err) = fs::create_dir(directory) {
                error!("Failed to create directory: {err}");
                return Err(SaveError::from(err));
            };
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
                debug!("Saved successfully.");
                Ok(true)
            }
            Err(err) => {
                error!("Failed to write config file: {err}");
                return Err(SaveError::from(err));
            }
        }
    }

    #[instrument(skip(self))]
    async fn load(&self, id: u64, insert_if_not_found: bool) -> Result<GuildConfig, LoadError> {
        debug!("Loading GuildConfig...");
        let path = get_config_path(id);
        if !path.exists() {
            if insert_if_not_found {
                let config = GuildConfig::default();
                self.guild_configs.insert(id, config.clone()).await;
                debug!("Config loaded by inserting a new one.");
                return Ok(config);
            }

            error!("Config file doesn't exist.");
            return Err(LoadError::from(id));
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
                return Err(LoadError::from(err));
            }
        };

        self.guild_configs.insert(id, config.clone()).await;

        debug!("Config loaded.");
        Ok(config)
    }

    #[instrument(skip(self))]
    pub async fn run_pending_tasks(&self) {
        info!("Running pending tasks...");
        self.guild_configs.run_pending_tasks().await;
        info!("Pending tasks completed.");
    }
}

impl Default for GuildConfigCache {
    fn default() -> Self {
        Self::new()
    }
}
