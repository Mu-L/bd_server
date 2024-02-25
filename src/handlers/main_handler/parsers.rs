use super::MainHandler;
use super::RequestData;
use rconn::server::{
    serde::Serialize,
    serde_json::{to_value, Value},
};

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
    let data = Url {
        url: handler.server_data.update_url.clone(),
        hash: handler.server_data.update_hash.clone(),
    };
    match to_value(data) {
        Ok(v) => Ok(v),
        Err(_) => Err(()),
    }
}
