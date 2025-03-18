use clap::Parser;
use minikvdb::{server, DEFAULT_PORT};
use tokio::net::TcpListener;
use tokio::signal;

#[tokio::main]
pub async fn main() -> minikvdb::Result<()> {
    set_up_logging()?;

    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT);

    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;

    server::run(listener, signal::ctrl_c()).await;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(
    name = "minikvdb-server",
    version,
    author,
    about = "a simple Redis server"
)]
struct Cli {
    #[arg(long)]
    port: Option<u16>,
}

fn set_up_logging() -> minikvdb::Result<()> {
    use tracing_subscriber::EnvFilter;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .try_init()?;
    Ok(())
}
