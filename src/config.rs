use std::env;

use serde::{Deserialize, Serialize};
use serde_json::json;

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
        // 读取json配置文件
        let config_json = JSON::read_json(
            config_file_path.to_str().unwrap(),
            &json!({"version": "0.1.0"}).to_string(),
        );
        // 反序列化配置
        let config: Config = serde_json::from_str(&config_json).unwrap();
        config
    }

    pub(crate) fn save_config(&self) {
        let exe_path = env::current_exe().unwrap();
        let mut config_file_path = exe_path.parent().unwrap().to_path_buf();
        config_file_path.push("luster_config.json");
        // 写入配置
        JSON::save_json(
            config_file_path.to_str().unwrap(),
            &serde_json::to_string(self).unwrap(),
        )
        .unwrap();
    }
}
