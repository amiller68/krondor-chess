use askama::Template;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Form,
};
use sqlx::types::Uuid;

use crate::api::models::ApiBoard;
use crate::database::models::{Game, GameError};
use crate::AppState;

#[derive(serde::Deserialize)]
pub struct MakeMoveRequest {
    #[serde(rename = "uciMove")]
    uci_move: String,
}

pub async fn handler(
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
    Form(request): Form<MakeMoveRequest>,
) -> Result<impl IntoResponse, ReadBoardError> {
    let uci_move = request.uci_move;
    let mut conn = state.database().begin().await?;
    let maybe_board = Game::make_move(&mut conn, game_id, &uci_move).await;
    let board = match maybe_board {
        Ok(board) => board,
        Err(e) => match e {
            GameError::InvalidMove(_, board) => board,
            _ => return Err(e.into()),
        },
    };
    // If we got here, then either we made a valid move
    //  or no changes were made to the database (invalid move)
    conn.commit().await?;

    let api_board = ApiBoard {
        board,
        game_id: game_id.to_string(),
    };

    Ok(TemplateApiBoard { api_board })
}

#[derive(Template)]
#[template(path = "board.html")]
struct TemplateApiBoard {
    api_board: ApiBoard,
}

#[derive(Debug, thiserror::Error)]
pub enum ReadBoardError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("game error: {0}")]
    Game(#[from] GameError),
}

impl IntoResponse for ReadBoardError {
    fn into_response(self) -> Response {
        let body = format!("{}", self);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
