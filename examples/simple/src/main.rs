#[allow(clippy::default_constructed_unit_structs)]
mod mock_backend;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .try_init()
        .ok();

    let version = env!("CARGO_PKG_VERSION");
    log::info!("rusthook test-example {}", version);

    mock_backend::run_server("0.0.0.0:8090", "http://local-dev:8080").await?;

    Ok(())
}
