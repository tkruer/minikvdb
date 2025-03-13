# minikvdb

minikvdb is a minimal key‑value store written in Rust. Inspired by Redis, it implements a subset of its functionality for educational and experimental purposes. Note: This project is not intended for production use.

## Features

- Basic Key‑Value Operations:
Implements essential commands such as GET, SET, and PING.
- Publish/Subscribe Support:
Includes basic pub/sub functionality for simple messaging.
- Asynchronous Networking:
Built on the Tokio runtime for high-performance, asynchronous IO.
- Multiple Client Implementations:
Provides both blocking and buffered client variants.
- Modular Design:
Organized into modules for commands, connections, database handling, protocol framing, parsing, and graceful shutdown.
- Example Applications:
Includes examples demonstrating basic usage (hello.rs) and load testing (load.rs).

## Installation

Ensure you have Rust installed, then clone the repository and build the project using Cargo:

```bash
git clone https://github.com/tkruer/minikvdb
cd minikvdb
cargo build
```

## Usage

### Running the Server

To start the key‑value server (which listens on port 6379 by default):

```bash
cargo run --bin minikvdb-server
```

You can specify a different port via command‑line options:

```bash
cargo run --bin minikvdb-server -- --port 6380
```

### Running the Client

Interact with the server using the provided command‑line client:

```bash
cargo run --bin minikvdb-cli
```

## Examples

- Hello Example:
Demonstrates connecting to the server, setting a key, and retrieving its value.

```bash
cargo run --example hello
```

- Load Testing Example:
Spawns multiple asynchronous clients to simulate heavy load on the server.

```bash
cargo run --example load
```

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── docs
├── examples
│   ├── hello.rs
│   └── load.rs
├── src
│   ├── bin
│   │   ├── cli.rs         # Command-line client interface
│   │   └── server.rs      # Server entry point
│   ├── clients            # Client implementations (blocking, buffered, async)
│   ├── command            # Command handlers (GET, SET, PING, PUB/SUB, etc.)
│   ├── connection         # TCP connection management
│   ├── db                 # In-memory database implementation
│   ├── frame              # Protocol framing logic
│   ├── lib.rs             # Library entry point
│   ├── parser             # Command parsing utilities
│   ├── server             # Server logic and connection handling
│   └── shutdown           # Graceful shutdown implementation
└── tests                  # Additional test utilities (e.g., flamegraph, load testing)
```
