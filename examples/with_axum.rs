use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use http::StatusCode;
use newswrap::{client::HackerNewsClient, items::HackerNewsItem, users::HackerNewsUser};
use tracing::{error, info};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

struct AppState {
    client: HackerNewsClient,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing application statue and routes");

    let app_state = AppState {
        client: HackerNewsClient::new(),
    };

    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let router = Router::new()
        .route("/user/:profile", get(get_user))
        .route("/item/:id", get(get_item))
        .with_state(Arc::new(app_state));

    info!("now listening on port {}", port);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

async fn get_user(
    Path(profile): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<HackerNewsUser>, (StatusCode, String)> {
    info!("received request to retrieve profile for user {}", profile);

    let user_result = state.client.users.get_user(&profile).await;

    match user_result {
        Ok(user) => {
            info!("user successfully retrieved {:?}", user);
            Ok(Json(user))
        }
        Err(err) => {
            let error = err.to_string();
            error!("error occurred while retrieving user: {}", error);
            Err((StatusCode::INTERNAL_SERVER_ERROR, error))
        }
    }
}

async fn get_item(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<HackerNewsItem>, (StatusCode, String)> {
    info!("received request to retrieve profile for user {}", id);

    let item_id = id
        .parse::<u32>()
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    let item_result = state.client.items.get_item(item_id).await;

    match item_result {
        Ok(item) => {
            info!("item retrieved {:?}", item);
            Ok(Json(item))
        }
        Err(err) => {
            let error = err.to_string();
            error!("error occurred while retrieving item: {}", error);
            Err((StatusCode::INTERNAL_SERVER_ERROR, error))
        }
    }
}
