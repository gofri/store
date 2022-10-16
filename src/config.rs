use std;
use log::info;

use config::{Config, FileFormat, File, ConfigBuilder, builder::DefaultState, ConfigError};

fn get_builder() -> Result<ConfigBuilder<DefaultState>, ConfigError> {
    let config_source = "config.yml";
    if !std::path::Path::new(config_source).exists() {
        info!("missing configuration file at {}, creating an empty one", config_source);
        std::fs::File::create(config_source)
            .map_err(|e| ConfigError::NotFound(e.to_string()))?;
    }
    Config::builder()
        .add_source(File::new(config_source, FileFormat::Yaml))
        .set_default("chunk_size", 100)
}

pub fn get_config() -> Result<Config, ConfigError> {
    match get_builder() {
        Ok(c) => c.build(),
        Err(e) => Err(e)
    }
}