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
use sqlx::PgPool;
use sqlx::types::Uuid;
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::broadcast::{channel, Sender};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt as _};
use tower_http::services::{ServeDir, ServeFile};

use crate::AppState;
use crate::database::models::NewGame;
use crate::api::models::ApiGame;
use crate::stream::{GamesMutationKind, GamesStream, GamesUpdate};

pub async fn handler(
    State(state): State<AppState>,
    Extension(tx): Extension<GamesStream>
) -> Result<Response, CreateGameError> {
    let mut conn = state.db.acquire().await?;
    let game = NewGame::create(&mut *conn).await?;

    if tx
        .send(GamesUpdate {
            mutation_kind: GamesMutationKind::Create,
            id: game.id(),
        })
        .is_err()
    {
        eprintln!(
            "Record with ID {} was created but nobody's listening to the stream!",
            game.id()
        );
    }

    let api_game = ApiGame::from(game);
    Ok(NewGameTemplate { api_game }.into_response())
}

#[derive(Template)]
#[template(path = "game.html")]
struct NewGameTemplate {
    api_game: ApiGame,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateGameError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}