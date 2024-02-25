use crate::server::ServerData;
use imlogger::*;
use rconn::conn::{Arc, Mutex, RHandle, THandle};
use rconn::rhandle_impl_new;
use rconn::serde_json::to_string;
use rconn::server::{
    serde::{Deserialize, Serialize},
    serde_json::{from_value, Value},
    Server,
};
use std::net::{Shutdown, TcpStream};

mod crypto;
mod key;
mod parsers;

pub struct MainHandler {
    server_data: ServerData,
}

rhandle_impl_new!(MainHandler);

impl Default for MainHandler {
    fn default() -> Self {
        MainHandler {
            server_data: ServerData::load(),
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
        info!("{} Start handle", peer_addr.to_string());

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
        let par_func: fn(&RequestData, &MainHandler) -> Result<Value, ()> = match req.act.as_str() {
            "ver" => parsers::par_ver,
            "info" => parsers::par_info,
            "url" => parsers::par_url,
            _ => {
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
        info!("{} Finish handle", peer_addr.to_string());
    }
}
