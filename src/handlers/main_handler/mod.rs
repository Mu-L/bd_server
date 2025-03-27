use crate::server::ServerData;
use imlogger::*;
use rconn::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_string, Value};
use std::collections::HashMap;
use std::net::{Shutdown, TcpStream};

mod check_version;
mod crypto;
mod key;
mod parsers;

pub struct MainHandler {
    server_data: ServerData,
    par_map: HashMap<String, ParFunc>,
}

rhandle_impl_new!(MainHandler);

type ParFunc = fn(&RequestData, &MainHandler) -> Result<Value, ()>;

impl Default for MainHandler {
    fn default() -> Self {
        let mut par_map: HashMap<String, ParFunc> = HashMap::new();
        par_map.insert(String::from("ver"), parsers::par_ver);
        par_map.insert(String::from("info"), parsers::par_info);
        par_map.insert(String::from("url"), parsers::par_url);

        MainHandler {
            server_data: ServerData::load(),
            par_map,
        }
    }
}

#[derive(Deserialize)]
struct CryptoData {
    key: String,
}

#[derive(Deserialize)]
struct RequestData {
    act: String,
    ver: String,
}

#[derive(Serialize)]
struct ResponseData {
    data: String,
    key: String,
}

impl RHandle for MainHandler {
    fn handle(&mut self, tcp: &mut TcpStream, json_data: &Value, custom_data: &Vec<u8>) {
        let peer_addr = tcp.peer_addr().unwrap();
        debug!("{} Start handle", peer_addr.to_string());

        let cry: CryptoData = if let Ok(r) = from_value(json_data.clone()) {
            r
        } else {
            error!("{} Get key Failed", peer_addr.to_string());
            tcp.shutdown(Shutdown::Both).ok();
            return;
        };

        debug!("{} Get Key {}", peer_addr.to_string(), (&cry.key).clone());

        let req = if let Ok(r) = crypto::decrypt(custom_data, &cry.key) {
            r
        } else {
            error!("{} Decrypt Request Failed", peer_addr.to_string());
            tcp.shutdown(Shutdown::Both).ok();
            return;
        };
        info!(
            "{} Act: {}, Ver: {}",
            peer_addr.to_string(),
            req.act,
            req.ver
        );
        let par_func = match self.par_map.get(&req.act) {
            Some(par_func) => par_func,
            None => {
                tcp.shutdown(Shutdown::Both).ok();
                return;
            }
        };
        let resp = if let Ok(r) = par_func(&req, self) {
            r
        } else {
            tcp.shutdown(Shutdown::Both).ok();
            return;
        };
        let resp = match to_string(&resp) {
            Ok(resp) => resp,
            Err(_) => {
                error!("{} Error Serialize Json", peer_addr.to_string());
                tcp.shutdown(Shutdown::Both).ok();
                return;
            }
        };
        let (resp, custom) = match crypto::encrypt(Vec::from(resp.as_bytes()).as_ref()) {
            Ok((resp, key)) => {
                let rdata = ResponseData {
                    data: String::from("ok"),
                    key,
                };
                (rdata, resp)
            }
            Err(_) => {
                error!("{} Error encrypt data", peer_addr.to_string());
                tcp.shutdown(Shutdown::Both).ok();
                return;
            }
        };
        Server::send_data(tcp, resp, &custom);
        debug!("{} Finish handle", peer_addr.to_string());
    }
}
