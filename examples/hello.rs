//! Hello world server.
//!
//! A simple client that connects to a mini-redis server, sets key "hello" with value "world",
//! and gets it from the server after
//!
//! You can test this out by running:
//!
//!     cargo run --bin mini-redis-server
//!
//! And then in another terminal run:
//!
//!     cargo run --example hello_world

use minikvdb::{clients::Client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = Client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("got value from the server; success={:?}", result.is_some());

    Ok(())
}
