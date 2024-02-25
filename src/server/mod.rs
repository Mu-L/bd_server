use std::fs::File;
use std::io::Read;

pub struct ServerData {
    pub ver: String,
    pub update_info: String,
    pub update_url: String,
    pub update_hash: String,
}

impl ServerData {
    pub fn load() -> Self {
        // Load ver
        let mut fp_ver = File::open("data/ver").unwrap();
        let mut ver = String::new();
        fp_ver.read_to_string(&mut ver).unwrap();

        // Load Info
        let mut fp_info = File::open("data/updateInfo").unwrap();
        let mut update_info = String::new();
        fp_info.read_to_string(&mut update_info).unwrap();

        // Load url
        let mut fp_url = File::open("data/updateUrl").unwrap();
        let mut update_url = String::new();
        fp_url.read_to_string(&mut update_url).unwrap();

        // Load Hash
        let mut fp_hash = File::open("data/updateHash").unwrap();
        let mut update_hash = String::new();
        fp_hash.read_to_string(&mut update_hash).unwrap();

        ServerData {
            ver,
            update_info,
            update_url,
            update_hash,
        }
    }
}
