use rconn::conn::THandle;
use rconn::Lazy;

mod empty_handler;
mod main_handler;

pub static MAIN_HANDLER: Lazy<THandle> = Lazy::new(|| main_handler::MainHandler::new());
pub static EMPTY_HANDLER: Lazy<THandle> = Lazy::new(|| empty_handler::EmptyHandler::new());
