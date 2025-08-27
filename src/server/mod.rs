use rconn::server::serde::Deserialize;
use rconn::server::serde_json::from_str;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct UpdateHash {
    pub md5: String,
    pub sha256: String,
}

#[derive(Deserialize)]
pub struct JsonConfig {
    pub ver: String,
    pub url: String,
    pub info_path: String,
    pub hash: UpdateHash,
}

pub struct ServerData {
    pub update_info: String,
    pub json_data: JsonConfig,
}

impl ServerData {
    pub fn load() -> Self {
        let mut fp_json = File::open("data/config.json").unwrap();
        let mut json_data = String::new();
        fp_json.read_to_string(&mut json_data).unwrap();
        let json_data: JsonConfig = from_str(&json_data).unwrap();

        // Load Info
        let mut fp_info = File::open(json_data.info_path.clone()).unwrap();
        let mut update_info = String::new();
        fp_info.read_to_string(&mut update_info).unwrap();

        ServerData {
            json_data,
            update_info,
        }
    }
}
