use sqlx::postgres::PgConnection;
use sqlx::types::Uuid;

use crate::database::types::DatabaseBoard as Board;

pub struct Position; 

impl Position {
    pub async fn record_move(conn: &mut PgConnection, board: &mut Board, uci_move: &str) -> Result<Uuid, PositionError> {
        // Attempt to make the move on the board
        let success = board.apply_uci_move(uci_move);
        if !success {
            return Err(PositionError::InvalidUciMove(uci_move.to_string(), board.clone()));
        }
        // Insert the FEN into the database if it doesn't already exist
        // Return the FEN ID
        let board_fen = board.fen();
        let position_id = sqlx::query_scalar!(
            r#"INSERT INTO positions (board)
            VALUES ($1)
            RETURNING id as "id: Uuid"
            "#,
            board_fen,
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(position_id)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PositionError {
    #[error("invalid uci move on board: {0} | {1}")]
    InvalidUciMove(String, Board),
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error)
}