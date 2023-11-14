use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    version: String,
    name: Option<String>,
}

impl Config {
    pub(crate) fn read_config() -> Self {
        let exe_path = env::current_exe().unwrap();
        let mut config_file_path = exe_path.parent().unwrap().to_path_buf();
        config_file_path.push("luster_config.json");
        // 读取json配置文件
        let mut config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(config_file_path)
            .unwrap();
        let mut config_json = String::new();
        config_file.read_to_string(&mut config_json).unwrap();
        println!("config_json: {}", config_json);
        if config_json.len() == 0 {
            // 写入默认配置
            config_json = json!({"version": "0.1.0"}).to_string();
            // 将默认配置写入文件
            config_file.write_all(config_json.as_bytes()).unwrap();
        }
        // 反序列化配置
        let config: Config = serde_json::from_str(&config_json).unwrap();
        println!("config: {:?}", config);
        config
    }
    
    pub(crate) fn save_config(&self) {
        let exe_path = env::current_exe().unwrap();
        let mut config_file_path = exe_path.parent().unwrap().to_path_buf();
        config_file_path.push("luster_config.json");
        // 读取json配置文件
        let mut config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(config_file_path)
            .unwrap();
        // 将默认配置写入文件
        let config_json = json!(self).to_string();
        config_file.write_all(config_json.as_bytes()).unwrap();
    }
}
