use std::env;

use serde::{Deserialize, Serialize};

use crate::utils::json_util::JSON;

pub(crate) trait JsonTrait {}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    version: String,
    name: Option<String>,
}

impl JsonTrait for Config {}

impl Config {
    pub(crate) fn new(version: String) -> Self {
        Config {
            version,
            name: None,
        }
    }

    pub(crate) fn read_config() -> Self {
        let exe_path = env::current_exe().unwrap();
        let mut config_file_path = exe_path.parent().unwrap().to_path_buf();
        config_file_path.push("luster_config.json");
        // read config
        let config_json = JSON::read_json(
            config_file_path.to_str().unwrap(),
            &serde_json::to_string(&Self::new("0.1.0".to_string())).unwrap(),
        );
        // parse config
        let config: Config = serde_json::from_str(&config_json).unwrap();
        config
    }

    #[allow(dead_code)]
    pub(crate) fn save_config(&self) {
        let exe_path = env::current_exe().unwrap();
        let mut config_file_path = exe_path.parent().unwrap().to_path_buf();
        config_file_path.push("luster_config.json");
        // save config
        JSON::save_json(
            config_file_path.to_str().unwrap(),
            &serde_json::to_string(self).unwrap(),
        )
        .unwrap();
    }
}
