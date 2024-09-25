use std::sync::Arc;

use anyhow::Result;
use mittlife_cycles::verification::Ed25519Verifier;
use poem::{
    handler,
    http::{HeaderMap, StatusCode},
    listener::TcpListener,
    middleware::{AddData, Tracing},
    post,
    web::Data,
    EndpointExt, Request, Route, Server,
};
use serde::Deserialize;
use tokio::sync::Mutex;

use super::backend_state::{new_ed25519_verifier, BackendState};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BackendRequest {
    // TODO: copy from go stuff
}

#[handler]
async fn backend_mock_handler(
    req: &Request,
    body: Vec<u8>,
    headers: &HeaderMap,
    state: Data<&Arc<Mutex<BackendState<Ed25519Verifier>>>>,
) -> poem::Result<()> {
    log::info!("Backend received request {:?}", req);

    state.lock().await.verify_request(&body, headers).await.map_err(|e| {
        log::error!("Failed to verify request: {}", e);
        poem::Error::from_status(StatusCode::BAD_REQUEST)
    })?;

    log::info!("Request verification successful");

    Ok(())
}

pub async fn run_server(addr: &str, mittwald_base_url: &str) -> Result<()> {
    let state = Arc::new(Mutex::new(new_ed25519_verifier(mittwald_base_url)));

    let app = Route::new()
        .at("/v1//backend", post(backend_mock_handler))
        .with(Tracing)
        .with(AddData::new(state));

    log::info!("Starting server at {}", addr);
    Server::new(TcpListener::bind(addr)).name("rusthook").run(app).await?;
    Ok(())
}
