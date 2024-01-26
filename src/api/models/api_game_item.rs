use crate::database::models::Game;
use crate::database::models::GameOutcome;
use crate::database::models::GameStatus;
use crate::database::models::GameWinner;

pub struct ApiGameItem {
    id: String,
    status: GameStatus,
    winner: Option<GameWinner>,
    outcome: Option<GameOutcome>,
}

impl ApiGameItem {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn status(&self) -> String {
        self.status.to_string()
    }

    pub fn winner(&self) -> String {
        match &self.winner {
            Some(winner) => winner.to_string(),
            None => "None".to_string(),
        }
    }

    pub fn outcome(&self) -> String {
        match &self.outcome {
            Some(outcome) => outcome.to_string(),
            None => "None".to_string(),
        }
    }
}

impl From<Game> for ApiGameItem {
    fn from(game: Game) -> Self {
        Self {
            id: game.id().to_string(),
            status: game.status().clone(),
            winner: game.winner().clone(),
            outcome: game.outcome().clone(),
        }
    }
}
