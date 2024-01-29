use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Form,
};
use sqlx::types::Uuid;

use crate::api::models::ApiGameBoard;
use crate::api::templates::GameBoardTemplate;
use crate::database::models::{Game, GameBoard, GameError};
use crate::AppState;

use super::watch_game_sse::GameUpdateStream;

#[derive(serde::Deserialize, Debug)]
pub struct MakeMoveRequest {
    #[serde(rename = "uciMove")]
    uci_move: String,
    resign: Option<bool>,
}

pub async fn handler(
    State(state): State<AppState>,
    Extension(tx): Extension<GameUpdateStream>,
    Path(game_id): Path<Uuid>,
    Form(request): Form<MakeMoveRequest>,
) -> Result<impl IntoResponse, ReadBoardError> {
    let uci_move = request.uci_move;
    let resign = request.resign.unwrap_or(false);
    let mut conn = state.database().begin().await?;
    if !Game::exists(&mut conn, game_id).await? {
        return Err(ReadBoardError::NotFound);
    }

    // Returns the updated board if the move was valid. Otherwise, returns the latest board.
    GameBoard::make_move(&mut conn, game_id, &uci_move, resign).await?;

    // Wow this really sucks, the client should just read this again
    let api_game_board = ApiGameBoard::from(GameBoard::latest(&mut conn, game_id).await?);
    conn.commit().await?;

    if tx.send(GameBoardTemplate { api_game_board }).is_err() {
        tracing::warn!("failed to send game update: game_id={}", game_id);
    }

    Ok(StatusCode::OK)
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
            ReadBoardError::Game(e) => match e {
                GameError::InvalidMove(_) | GameError::GameComplete => {
                    let body = format!("{}", e);
                    (axum::http::StatusCode::BAD_REQUEST, body).into_response()
                }
                _ => {
                    let body = format!("internal server error: {}", e);
                    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
                }
            },
            _ => {
                let body = format!("{}", self);
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}
