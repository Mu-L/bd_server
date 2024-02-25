use rconn::conn::{Arc, Mutex, RHandle, THandle};
use rconn::rhandle_impl_new;
use rconn::server::serde_json::Value;
use std::net::{Shutdown, TcpStream};

pub struct EmptyHandler;

rhandle_impl_new!(EmptyHandler);

impl Default for EmptyHandler {
    fn default() -> Self {
        EmptyHandler {}
    }
}

impl RHandle for EmptyHandler {
    fn handle(&mut self, tcp: &mut TcpStream, _json_data: &Value, _custom_data: &Vec<u8>) {
        tcp.shutdown(Shutdown::Both).ok();
    }
}
