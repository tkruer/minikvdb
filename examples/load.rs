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
use std::time::Instant;

#[tokio::main]
pub async fn main() -> Result<()> {
    // Define how many concurrent clients and how many operations each will perform.
    let num_clients = 100; // Number of concurrent tasks
    let num_requests = 100; // Number of set/get operations per task

    let start_time = Instant::now();
    let mut handles = Vec::new();

    // Spawn multiple tasks concurrently.
    for i in 0..num_clients {
        let handle = tokio::spawn(async move {
            // Each task creates its own client connection.
            let mut client = Client::connect("127.0.0.1:6379").await?;
            for j in 0..num_requests {
                let key = format!("key_{}_{}", i, j);
                let value = format!("value_{}_{}", i, j);
                // Perform a set operation.
                client.set(&key, value.into()).await?;
                // Immediately get the value to verify.
                let res = client.get(&key).await?;
                if res.is_none() {
                    println!("Error: Key {} not found!", key);
                }
            }
            Result::<()>::Ok(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await??;
    }

    let duration = start_time.elapsed();
    println!(
        "Completed {} operations (set+get) in {:?}",
        num_clients * num_requests * 2,
        duration
    );

    Ok(())
}
