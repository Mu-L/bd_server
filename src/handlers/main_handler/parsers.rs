use super::check_version::version_greater;
use super::MainHandler;
use super::RequestData;
use rconn::server::{
    serde::Serialize,
    serde_json::{to_value, Value},
};
use std::path::Path;

#[derive(Serialize)]
struct Ver {
    ver: String,
}

#[derive(Serialize)]
struct Info {
    data: String,
}

#[derive(Serialize)]
struct Url {
    url: String,
    hash: String,
    hash_type: String,
    name: String,
}

pub fn par_ver(_req: &RequestData, handler: &MainHandler) -> Result<Value, ()> {
    let data = Ver {
        ver: handler.server_data.ver.clone(),
    };
    match to_value(data) {
        Ok(v) => Ok(v),
        Err(_) => Err(()),
    }
}

pub fn par_info(req: &RequestData, handler: &MainHandler) -> Result<Value, ()> {
    let info = handler.server_data.update_info.as_str();
    let pos = info.find(req.ver.as_str());
    let info = match pos {
        Some(pos) => info.split_at(pos - 4).0,
        None => info,
    };
    let data = Info {
        data: String::from(info),
    };
    match to_value(data) {
        Ok(v) => Ok(v),
        Err(_) => Err(()),
    }
}

pub fn par_url(_req: &RequestData, handler: &MainHandler) -> Result<Value, ()> {
    let path = handler.server_data.update_url.clone();
    let path = Path::new(&path);
    let name = match path.file_name() {
        Some(s) => String::from(match s.to_str() {
            Some(s) => s,
            None => return Err(()),
        }),
        None => return Err(()),
    };
    let hash;
    let hash_type;
    if version_greater(_req.ver.clone(), "1.3.5".to_string()) {
        hash = handler.server_data.update_hash_sha256.clone();
        hash_type = "sha256".to_string();
    } else {
        hash = handler.server_data.update_hash_md5.clone();
        hash_type = "md5".to_string();
    }
    let data = Url {
        url: handler.server_data.update_url.clone(),
        hash,
        hash_type,
        name,
    };

    match to_value(data) {
        Ok(v) => Ok(v),
        Err(_) => Err(()),
    }
}
