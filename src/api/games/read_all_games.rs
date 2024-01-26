use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};

use crate::api::models::ApiGameItem;
use crate::database::models::{Game, GameError};
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ReadAllGamesError> {
    let mut conn = state.database().acquire().await?;
    let games = Game::read_all(&mut conn).await?;
    let game_items = games.into_iter().map(ApiGameItem::from).collect();
    Ok(GameList { game_items })
}

#[derive(Template)]
#[template(path = "game_list.html")]
struct GameList {
    game_items: Vec<ApiGameItem>,
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
