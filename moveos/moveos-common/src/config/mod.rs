// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;

// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    pub rpc_server: ServerConfig,
    pub http_server: ServerConfig,
}

impl Config {
    pub fn load(filename: impl AsRef<Path>) -> Result<Self, Error> {
        let config = fs::read_to_string(filename).map_err(|_| Error::ConfigReadError)?;
        serde_yaml::from_str(&config).map_err(|_| Error::ConfigParseError)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn url(&self, https: bool) -> String {
        let schema = if https { "https" } else { "http" };

        format!("{}://{}:{}", schema, self.host, self.port)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 50051,
        }
    }
}

// Load config file from env or default path or default value
pub fn load_config() -> Result<Config> {
    let filename = std::env::var("ROOCH_CONFIG")
        .unwrap_or_else(|_| Path::new("./rooch.yml").to_str().unwrap().to_string());
    Config::load(filename).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_should_work() {
        let config = Config::load("../../fixtures/config.yml").unwrap();

        assert_eq!(
            config,
            Config {
                rpc_server: ServerConfig {
                    host: "0.0.0.0".to_string(),
                    port: 50051
                },
                http_server: ServerConfig {
                    host: "0.0.0.0".to_string(),
                    port: 50051
                }
            }
        )
    }
}
