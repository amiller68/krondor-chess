use askama::Template;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use sqlx::types::Uuid;

use crate::api::models::{ApiBoard, ApiBoardError};
use crate::database::models::PartialGameWithFen;
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
) -> Result<impl IntoResponse, ReadBoardError> {
    let conn = state.database();
    let partial_game_with_fen = match PartialGameWithFen::read(&conn, game_id).await? {
        Some(partial_game_with_fen) => partial_game_with_fen,
        None => return Err(ReadBoardError::NotFound),
    };
    let api_board = ApiBoard::try_from(partial_game_with_fen)?;

    Ok(Board { api_board })
}

#[derive(Template)]
#[template(path = "board.html")]
struct Board {
    api_board: ApiBoard,
}

#[derive(Debug, thiserror::Error)]
pub enum ReadBoardError {
    #[error("api board error: {0}")]
    ApiBoard(#[from] ApiBoardError),
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("game not found")]
    NotFound,
}

impl IntoResponse for ReadBoardError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound => {
                let body = format!("{}", self);
                (axum::http::StatusCode::NOT_FOUND, body).into_response()
            }
            _ => {
                let error = format!("{}", self);
                tracing::error!("{}", error);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                )
                    .into_response()
            }
        }
    }
}
