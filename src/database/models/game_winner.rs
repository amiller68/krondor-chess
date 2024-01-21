use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum GameWinner {
    White,
    Black,
    Draw,
}

impl Display for GameWinner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameWinner::White => write!(f, "white"),
            GameWinner::Black => write!(f, "black"),
            GameWinner::Draw => write!(f, "draw"),
        }
    }
}

impl TryFrom<&str> for GameWinner {
    type Error = GameWinnerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "white" => Ok(GameWinner::White),
            "black" => Ok(GameWinner::Black),
            "draw" => Ok(GameWinner::Draw),
            _ => Err(GameWinnerError::InvalidGameWinner),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GameWinnerError {
    #[error("Invalid GameWinner")]
    InvalidGameWinner,
}
