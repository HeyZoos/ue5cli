use config::builder::DefaultState;
use config::{Config, ConfigBuilder};
use extend::ext;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;

pub fn clear() {
    fs::write(config_path(), "").unwrap();
}

pub fn get(key: &str) -> String {
    config().get_string(key).unwrap()
}

pub fn set(key: &str, value: &str) {
    let config = config()
        .to_builder()
        .set_override(key, value)
        .unwrap()
        .build()
        .unwrap();
    fs::write(config_path(), config.to_toml()).unwrap();
}

fn config() -> Config {
    config_path().ensure().to_config()
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .map(|p| p.join("ue5cli").join("config.toml"))
        .unwrap()
}

#[ext]
impl Config {
    fn to_builder(self) -> ConfigBuilder<DefaultState> {
        Config::builder().add_source(config())
    }

    fn to_map(self) -> HashMap<String, String> {
        self.try_deserialize().unwrap()
    }

    fn to_toml(self) -> String {
        self.to_map().to_toml()
    }
}

#[ext]
impl HashMap<String, String> {
    fn to_toml(self) -> String {
        toml::to_string(&self).unwrap()
    }
}

#[ext]
impl PathBuf {
    fn ensure(self) -> PathBuf {
        if let Some(parent) = &self.parent() {
            std::fs::create_dir_all(&parent).unwrap();
        }

        OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self)
            .map(|_| self)
            .unwrap()
    }

    fn to_config(self) -> Config {
        Config::builder()
            .add_source(config::File::from(self))
            .build()
            .unwrap()
    }
}
