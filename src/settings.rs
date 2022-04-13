use config::{Config, ConfigError, File};
use sea_orm::ConnectOptions;
//use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[allow(unused)]
pub struct ServerSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(unused)]
pub struct DatabaseSettings {
    pub db_type: String,
    pub db_name: String,
    pub username: String,
    password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn database_connect(&self) ->  ConnectOptions{
        if self.db_type == "sqlite" {
            return ConnectOptions::new(format!("{}", self.db_name)) 
        } else {
            return ConnectOptions::new(format!("postgres://{}:{}@{}:{}/", self.username, self.password, self.host, self.port)) 
        };
    }
}
impl Settings {
    pub fn new() -> Result<Settings, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("configuration/default"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("configuration/{}", run_mode)).required(false))
            .add_source(config::Environment::with_prefix("app").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}

/// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}