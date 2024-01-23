use std::convert::TryFrom;

use pleco::board::Board;
use pleco::core::sq::SQ as Sq;
use pleco::core::Piece;

use crate::database::models::PartialGameWithFen;

pub struct ApiBoard {
    id: String,
    html: String,
}

impl ApiBoard {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn html(&self) -> &str {
        &self.html
    }
}

impl TryFrom<PartialGameWithFen> for ApiBoard {
    type Error = ApiBoardError;

    fn try_from(game: PartialGameWithFen) -> Result<Self, Self::Error> {
        let id = game.id().to_string();
        let html = render_html_board(game.current_fen())?;
        Ok(Self { id, html })
    }
}

fn render_html_board(fen: &str) -> Result<String, ApiBoardError> {
    let board = Board::from_fen(fen).map_err(|e| ApiBoardError::FenBuilder(format!("{:?}", e)))?;

    let mut html_board = String::new();
    html_board.push_str("<table class='chess-board'>");

    for rank in (0..8).rev() {
        html_board.push_str("<tr class='chess-row'>");
        for file in 0..8 {
            let square = Sq::from(rank * 8 + file);
            let piece = board.piece_at_sq(square);
            let class = if square.on_light_square() {
                "light-square"
            } else {
                "dark-square"
            };

            let piece_string = render_html_piece(piece);

            html_board.push_str(&format!(
                "<td class='chess-cell {}'>{}</td>",
                class, piece_string
            ));
        }
        html_board.push_str("</tr>");
    }

    html_board.push_str("</table>");
    Ok(html_board)
}

fn render_html_piece(piece: Piece) -> String {
    match piece {
        Piece::None => " ".to_string(),
        Piece::WhitePawn => "♙".to_string(),
        Piece::WhiteKnight => "♘".to_string(),
        Piece::WhiteBishop => "♗".to_string(),
        Piece::WhiteRook => "♖".to_string(),
        Piece::WhiteQueen => "♕".to_string(),
        Piece::WhiteKing => "♔".to_string(),
        Piece::BlackPawn => "♟︎".to_string(),
        Piece::BlackKnight => "♞".to_string(),
        Piece::BlackBishop => "♝".to_string(),
        Piece::BlackRook => "♜".to_string(),
        Piece::BlackQueen => "♛".to_string(),
        Piece::BlackKing => "♚".to_string(),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiBoardError {
    // TODO: should use FenBuildError here, but it doesn't implement Error
    #[error("fen builder error: {0}")]
    FenBuilder(String),
}
