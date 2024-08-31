mod mock_backend;
mod verification;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .try_init()
        .ok();

    let version = env!("CARGO_PKG_VERSION");
    log::info!("rusthook test-example {}", version);

    mock_backend::run_server("127.0.0.1:8090", "http://127.0.0.1:8080").await?;

    Ok(())
}
