use serde::Serialize;

use crate::database::models::Game;

pub struct ApiGame {
    pub id: String,
    pub status: String,
}

impl From<Game> for ApiGame {
    fn from(game: Game) -> Self {
        Self {
            id: game.id().to_string(),
            status: game.status().to_string(),
        }
    }
}