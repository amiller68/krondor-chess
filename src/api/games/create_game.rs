use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};

use crate::api::models::ApiGame;
use crate::database::models::NewGame;
use crate::AppState;

pub async fn handler(State(state): State<AppState>) -> Result<impl IntoResponse, CreateGameError> {
    let game = NewGame::create(&state.database()).await?;

    let api_game = ApiGame::from(game);
    Ok(NewGameTemplate { api_game })
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

impl IntoResponse for CreateGameError {
    fn into_response(self) -> Response {
        let body = format!("{}", self);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
