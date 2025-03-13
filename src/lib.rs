pub mod clients;
pub mod command;
pub mod connection;
pub mod db;
pub mod frame;
pub mod parser;
pub mod server;
pub mod shutdown;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub const DEFAULT_PORT: u16 = 6379;
