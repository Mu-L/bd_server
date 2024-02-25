use bd_server::handlers;
use imlogger::*;
use rconn::conn::{RConnection, THandle};
use rconn::server::Server;
use std::time::Duration;

fn matcher(act: &str) -> THandle {
    match act {
        "any" => handlers::MAIN_HANDLER.clone(),
        _ => handlers::EMPTY_HANDLER.clone(),
    }
}

fn main() -> Result<(), ()> {
    #[cfg(debug_assertions)]
    imloginit(LevelFilter::Debug);
    #[cfg(debug_assertions)]
    let mut server = Server::new("127.0.0.1:11289", 5);
    #[cfg(not(debug_assertions))]
    imloginit(LevelFilter::Info);
    #[cfg(not(debug_assertions))]
    let mut server = Server::new("0.0.0.0:11289", 5);
    server.set_matcher(&matcher);
    info!("Server Started...");
    server.set_timeout(Some(Duration::from_secs(5))).start();

    Ok(())
}
