use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{sse::Event, IntoResponse, Response, Sse},
    routing::{delete, get, get_service},
    Extension, Form, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::Uuid;
use sqlx::PgPool;
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::broadcast::{channel, Sender};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt as _};
use tower_http::services::{ServeDir, ServeFile};

// mod api;
mod database;
mod stream;

use database::models::{Game, NewGame};
use stream::{handle_game_stream, GamesMutationKind, GamesStream, GamesUpdate};

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:postgres@localhost:5432/postgres"
    )]
    db: PgPool,
) -> shuttle_axum::ShuttleAxum {
    // Run migrations
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Looks like something went wrong with migrations :(");
    // Setup State
    let state = AppState { db };

    let (tx, _rx) = channel::<GamesUpdate>(10);
    let router = Router::new()
        // Home page
        .route("/", get(index))
        // Static assets
        .route(
            "/static",
            get_service(ServeDir::new("static").fallback(ServeFile::new("static/not_found.html"))),
        )
        .route("/stream", get(stream))
        // .route(
        //     "/games",
        //     get(api::games::read_all_games::handler).post(api::games::create_game::handler),
        // )
        .route("/games/stream", get(handle_game_stream))
        .with_state(state)
        .layer(Extension(tx));

    Ok(router.into())
}

async fn index() -> impl IntoResponse {
    IndexTemplate
}

async fn stream() -> impl IntoResponse {
    StreamTemplate
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "stream.html")]
struct StreamTemplate;
