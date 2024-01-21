use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum GameWinner {
    White,
    Black,
    Draw
}