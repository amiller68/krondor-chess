use std::convert::TryFrom;

use pleco::board::Board;

use crate::database::models::PartialGameWithFen;

pub struct ApiBoard {
    pub id: String,
    pub pretty_string: String,
}

impl TryFrom<PartialGameWithFen> for ApiBoard {
    type Error = ApiBoardError;

    fn try_from(game: PartialGameWithFen) -> Result<Self, Self::Error> {
        let id = game.id().to_string();
        let board = Board::from_fen(game.current_fen())
            .map_err(|e| ApiBoardError::FenBuilder(format!("{:?}", e)))?;
        // TODO: Board rendering here
        let pretty_string = board.pretty_string();
        Ok(Self { id, pretty_string })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiBoardError {
    // TODO: should use FenBuildError here, but it doesn't implement Error
    #[error("fen builder error: {0}")]
    FenBuilder(String),
}
