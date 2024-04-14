use std::{env::current_dir, fs, path::PathBuf, sync::Arc};

use serenity::prelude::TypeMapKey;
use tracing::{error, info};

use crate::{cache::GuildConfigCache, texts::Texts};

use self::error::Error;

pub mod error;

pub struct Data {
    pub cache: GuildConfigCache,
    pub texts: Texts,
}

impl Data {
    pub fn new() -> Result<Data, Error> {
        let cache = GuildConfigCache::default();

        info!("Loading texts...");

        let mut path = PathBuf::new();
        path.push(current_dir().expect("The current directory could not be obtained."));
        path.push("texts.json");

        if !path.exists() {
            let texts = Texts::default();

            let json = match serde_json::to_string_pretty(&texts) {
                Ok(json) => json,
                Err(err) => {
                    error!("Failed to serialize texts to JSON: {err}");
                    return Err(Error::Serialization(err));
                }
            };

            match fs::write(path, json) {
                Ok(_) => {
                    info!("Texts saved successfully.");
                }
                Err(err) => {
                    error!("Failed to write texts file: {err}");
                    return Err(Error::Writing(err));
                }
            }

            return Ok(Self { cache, texts });
        }

        let raw_data = match fs::read_to_string(&path) {
            Ok(path) => path,
            Err(err) => {
                error!("Failed to read data file: {err}");
                return Err(Error::Reading(err));
            }
        };

        let texts: Texts = match serde_json::from_str(&raw_data) {
            Ok(texts) => texts,
            Err(err) => {
                error!("Failed to deserialize texts from JSON: {err}");
                return Err(Error::Deserialization(err));
            }
        };

        Ok(Self { cache, texts })
    }
}

impl TypeMapKey for Data {
    type Value = Arc<Self>;
}
