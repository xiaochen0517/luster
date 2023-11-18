use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Read, Write};

pub(crate) struct JSON {
    // ...
}

impl JSON {
    pub(crate) fn read_json(file_path: &str, default_json: &str) -> String {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();
        if json_str.len() == 0 {
            // 将默认配置写入文件
            file.write_all(default_json.as_bytes()).unwrap();
            return default_json.to_string();
        }
        json_str
    }

    pub(crate) fn save_json(file_path: &str, json_str: &str) -> Result<(), Error> {
        // 读取json配置文件
        let mut config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .map_err(|_| Error::new(ErrorKind::Other, "open file error"))?; // 读取json配置文件
                                                                            // 将默认配置写入文件
        config_file
            .write_all(json_str.as_bytes())
            .map_err(|_| Error::new(ErrorKind::Other, "write file error"))?; // 读取json配置文件
        Ok(())
    }
}
