/// Default port for the server to listen on
pub const DEFAULT_PORT: u16 = 6379;

pub mod db;
pub mod server;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
