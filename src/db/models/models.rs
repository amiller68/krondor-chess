use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GameStatus {
    Created,
    Active,
    Completed,
    Abandoned
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GameWinner {
    White,
    Black,
    Draw
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GameOutcome {
    Checkmate,
    Stalemate,
    Resignation
}

#[derive(FromRow)]
pub struct Game {
    id: Uuid,
    current_fen_id: Uuid,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
    status: GameStatus,
    winner: GameWinner,
    Outcome: GameOutcome
}

#[derive(FromRow)]
pub struct Fen {
    id: Uuid,
    // TODO: Database type for FEN
    fen: String,
    created_at: OffsetDateTime,
}

#[derive(FromRow)]
pub struct Move {
    id: Uuid,
    game_id: Uuid,
    fen_id: Uuid,
    move_number: i32,
    // TODO: Database tupe for move
    r#move: String,
    created_at: OffsetDateTime
}


