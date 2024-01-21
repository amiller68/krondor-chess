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
use crate::stream::GamesStream;
use crate::database::models::Game;
use crate::api::models::ApiGame;

pub async fn handler(State(state): State<AppState>) -> Result<Response, ReadAllGamesError> {
    let mut conn = state.db.acquire().await?;
    let games = Game::read_all(&mut *conn).await?;
    let api_games = games.into_iter().map(ApiGame::from).collect();
    Ok(Records { api_games }.into_response())
}

#[derive(Template)]
#[template(path = "games.html")]
struct Records {
    api_games: Vec<ApiGame>,
}

#[derive(Debug, thiserror::Error)]
pub enum ReadAllGamesError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}