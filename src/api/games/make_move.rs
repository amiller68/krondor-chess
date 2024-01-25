use askama::Template;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Form,
};
use sqlx::types::Uuid;

use crate::api::models::ApiGameBoard;
use crate::database::models::{Game, GameBoard, GameError};
use crate::AppState;

#[derive(serde::Deserialize)]
pub struct MakeMoveRequest {
    #[serde(rename = "uciMove")]
    uci_move: String,
    resign: Option<bool>,
}

pub async fn handler(
    State(state): State<AppState>,
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
    let game_board = GameBoard::make_move(&mut conn, game_id, &uci_move, resign).await?;

    // If we got here, then either we made a valid move
    //  or no changes were made to the database (invalid move)
    conn.commit().await?;

    let board = game_board.board().clone();
    let status = game_board.status().clone();
    let winner = game_board.winner().clone();
    let outcome = game_board.outcome().clone();
    let game_id = game_id.to_string();
    let api_board = ApiGameBoard {
        game_id,
        board,
        status,
        winner,
        outcome,
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
