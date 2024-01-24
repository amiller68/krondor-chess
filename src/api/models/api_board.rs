use std::convert::TryFrom;

use pleco::board::Board;
use pleco::core::sq::SQ as Sq;
use pleco::core::Piece;

use crate::database::models::PartialGameWithFen;

pub struct ApiBoard {
    game_id: String,
    board_html: String,
}

impl ApiBoard {
    pub fn game_id(&self) -> &str {
        &self.game_id
    }
    pub fn board_html(&self) -> &str {
        &self.board_html
    }
}

impl TryFrom<PartialGameWithFen> for ApiBoard {
    type Error = ApiBoardError;

    fn try_from(game: PartialGameWithFen) -> Result<Self, Self::Error> {
        let game_id = game.id().to_string();
        let board_html = render_html_board(game.current_fen())?;
        Ok(Self {
            game_id,
            board_html,
        })
    }
}

/// Render a FEN formatted str into an HTML chess board
fn render_html_board(fen: &str) -> Result<String, ApiBoardError> {
    // Read the FEN into a board
    let board = Board::from_fen(fen).map_err(|e| ApiBoardError::FenBuilder(format!("{:?}", e)))?;

    // We'll just pass raw HTML to our template
    let mut html_board = String::new();
    html_board.push_str("<table class='chess-board'>");

    // Iterate over ranks to fully construct the board -- we need to populate every cell
    //  with metadata at the moment
    for rank in (0..8).rev() {
        // New rank
        html_board.push_str("<tr class='chess-rank'>");
        for file in 0..8 {
            // Read the piece at this square and populate the cell
            let square = Sq::from(rank * 8 + file);
            let piece = board.piece_at_sq(square);
            let id = square.to_string();
            let color_class = if square.on_light_square() {
                "light"
            } else {
                "dark"
            };

            // Metadata breakdown:
            // - id: the square's readable id (e.g. "a1")
            // - class:
            //  - chess-square-{light|dark}: the square's color
            //  - chess-piece-{piece_char}: the occupying piece, if any. e.g. "chess-piece-P" for a white pawn
            match render_html_piece(piece) {
                Some(piece_html) => {
                    // Note: Since we know `piece` is `Some`, we can call .character_lossy() here
                    html_board.push_str(&format!(
                        "<td id='{}' class='chess-square-{} chess-piece-{}'>{}</td>",
                        id,
                        color_class,
                        piece.character_lossy(),
                        piece_html
                    ));
                }
                None => {
                    html_board.push_str(&format!(
                        "<td id='{}' class='chess-square-{}'></td>",
                        id, color_class
                    ));
                }
            }
        }
        html_board.push_str("</tr>");
    }

    html_board.push_str("</table>");
    Ok(html_board)
}

fn render_html_piece(piece: Piece) -> Option<String> {
    match piece {
        Piece::None => None,
        Piece::WhitePawn => Some("♙".to_string()),
        Piece::WhiteKnight => Some("♘".to_string()),
        Piece::WhiteBishop => Some("♗".to_string()),
        Piece::WhiteRook => Some("♖".to_string()),
        Piece::WhiteQueen => Some("♕".to_string()),
        Piece::WhiteKing => Some("♔".to_string()),
        Piece::BlackPawn => Some("♟︎".to_string()),
        Piece::BlackKnight => Some("♞".to_string()),
        Piece::BlackBishop => Some("♝".to_string()),
        Piece::BlackRook => Some("♜".to_string()),
        Piece::BlackQueen => Some("♛".to_string()),
        Piece::BlackKing => Some("♚".to_string()),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiBoardError {
    // TODO: should use FenBuildError here, but it doesn't implement Error
    #[error("fen builder error: {0}")]
    FenBuilder(String),
}
