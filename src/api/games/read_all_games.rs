use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};

use crate::api::models::ApiGame;
use crate::database::models::{Game, GameError};
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ReadAllGamesError> {
    let mut conn = state.database().acquire().await?;
    let games = Game::read_all(&mut conn).await?;
    let api_games = games.into_iter().map(ApiGame::from).collect();
    Ok(Records { api_games })
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
    #[error("game error: {0}")]
    Game(#[from] GameError),
}

impl IntoResponse for ReadAllGamesError {
    fn into_response(self) -> Response {
        let body = format!("{}", self);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
