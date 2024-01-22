use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum GameOutcome {
    Checkmate,
    Stalemate,
    Resignation,
}

impl Display for GameOutcome {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameOutcome::Checkmate => write!(f, "checkmate"),
            GameOutcome::Stalemate => write!(f, "stalemate"),
            GameOutcome::Resignation => write!(f, "resignation"),
        }
    }
}

impl TryFrom<&str> for GameOutcome {
    type Error = GameOutcomeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "checkmate" => Ok(GameOutcome::Checkmate),
            "stalemate" => Ok(GameOutcome::Stalemate),
            "resignation" => Ok(GameOutcome::Resignation),
            _ => Err(GameOutcomeError::InvalidGameOutcome),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GameOutcomeError {
    #[error("Invalid GameOutcome")]
    InvalidGameOutcome,
}
