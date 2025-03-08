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

fn split_num(a: &String) -> Result<(u32, u32, u32), ()> {
    let mut p = if let Some(p) = a.find('.') {
        p
    } else {
        return Err(());
    };
    let (an, mut sn) = a.split_at(p);
    sn = sn.split_at(1).1;
    let an: u32 = if let Ok(r) = an.parse() {
        r
    } else {
        return Err(());
    };
    p = if let Some(p) = sn.find('.') {
        p
    } else {
        return Err(());
    };
    let (bn, mut sn) = sn.split_at(p);
    sn = sn.split_at(1).1;
    let bn: u32 = if let Ok(r) = bn.parse() {
        r
    } else {
        return Err(());
    };
    let cn: u32 = if let Ok(r) = sn.parse() {
        r
    } else {
        return Err(());
    };
    Ok((an, bn, cn))
}

/// Return true if a greater than b
pub fn version_greater(a: String, b: String) -> bool {
    let (a1, a2, a3) = if let Ok(r) = split_num(&a) {
        r
    } else {
        return false;
    };
    let (b1, b2, b3) = if let Ok(r) = split_num(&b) {
        r
    } else {
        return false;
    };
    if a1 > b1 {
        return true;
    } else if a1 < b1 {
        return false;
    }
    if a2 > b2 {
        return true;
    } else if a2 < b2 {
        return false;
    }
    if a3 > b3 {
        return true;
    }
    false
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
