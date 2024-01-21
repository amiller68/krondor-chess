use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum GameStatus {
    Created,
    Active,
    Completed,
    Abandoned,
}

impl Display for GameStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameStatus::Created => write!(f, "created"),
            GameStatus::Active => write!(f, "active"),
            GameStatus::Completed => write!(f, "completed"),
            GameStatus::Abandoned => write!(f, "abandoned"),
        }
    }
}

impl TryFrom<&str> for GameStatus {
    type Error = GameStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "created" => Ok(GameStatus::Created),
            "active" => Ok(GameStatus::Active),
            "completed" => Ok(GameStatus::Completed),
            "abandoned" => Ok(GameStatus::Abandoned),
            _ => Err(GameStatusError::InvalidGameStatus),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GameStatusError {
    #[error("Invalid GameStatus")]
    InvalidGameStatus,
}