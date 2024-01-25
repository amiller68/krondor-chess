use askama::Template;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use sqlx::types::Uuid;

use crate::api::models::ApiGameBoard;
use crate::database::models::{Game, GameBoard, GameError};
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
) -> Result<impl IntoResponse, ReadBoardError> {
    let mut conn = state.database().acquire().await?;
    if !Game::exists(&mut conn, game_id).await? {
        return Err(ReadBoardError::NotFound);
    }
    let game_board = GameBoard::latest(&mut conn, game_id).await?;
    let board = game_board.board().clone();
    let api_board = ApiGameBoard {
        board,
        status: game_board.status().clone(),
        winner: game_board.winner().clone(),
        outcome: game_board.outcome().clone(),
        game_id: game_id.to_string(),
    };

    Ok(TemplateApiGameBoard { api_board })
}

#[derive(Template)]
#[template(path = "board.html")]
struct TemplateApiGameBoard {
    api_board: ApiGameBoard,
}

#[derive(Debug, thiserror::Error)]
pub enum ReadBoardError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("game error: {0}")]
    Game(#[from] GameError),
    #[error("game not found")]
    NotFound,
}

impl IntoResponse for ReadBoardError {
    fn into_response(self) -> Response {
        match self {
            ReadBoardError::NotFound => {
                let body = format!("{}", self);
                (axum::http::StatusCode::NOT_FOUND, body).into_response()
            }
            _ => {
                let body = format!("{}", self);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}
