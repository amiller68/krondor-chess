
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{sse::Event, IntoResponse, Response, Sse},
    routing::{delete, get, get_service},
    Extension, Form, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use sqlx::types::Uuid;
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::broadcast::{channel, Sender};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt as _};
use tower_http::services::{ServeDir, ServeFile};

// Stream for sending updates to the Games page
pub type GamesStream = Sender<GamesUpdate>;

#[derive(Clone, Serialize, Debug)]
pub enum GamesMutationKind {
    Create,
}

#[derive(Clone, Serialize, Debug)]
pub struct GamesUpdate {
    pub mutation_kind: GamesMutationKind,
    pub id: Uuid,
}

pub async fn handle_game_stream(
    Extension(tx): Extension<GamesStream>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = tx.subscribe();

    let stream = BroadcastStream::new(rx);

    Sse::new(
        stream
            .map(|msg| {
                let msg = msg.unwrap();
                let json = format!("<div>{}</div>", json!(msg));
                Event::default().data(json)
            })
            .map(Ok),
    )
    .keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(600))
            .text("keep-alive-text"),
    )
}
