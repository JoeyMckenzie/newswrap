use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use newswrap::client::HackerNewsClient;

struct AppState {
    client: HackerNewsClient,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState {
        client: HackerNewsClient::new(),
    };

    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let router = Router::new()
        .route("/user/:profile", get(get_user))
        .route("/item/:id", get(get_item))
        .with_state(Arc::new(app_state));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

async fn get_user(Path(profile): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    let user = state.client.users.get_user(&profile).await.unwrap();
    Json(user)
}

async fn get_item(Path(id): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    let item_id = id.parse::<u32>().unwrap();
    let item = state.client.items.get_item(item_id).await.unwrap();
    Json(item)
}
