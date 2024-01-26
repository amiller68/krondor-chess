use pleco::core::sq::SQ as Sq;
use pleco::core::Piece;
use pleco::core::Player;

use crate::database::models::GameBoard;
use crate::database::models::GameOutcome;
use crate::database::models::GameStatus;
use crate::database::models::GameWinner;
use crate::database::types::DatabaseBoard as Board;

pub struct ApiGameBoard {
    pub game_id: String,
    pub board: Board,
    pub status: GameStatus,
    pub winner: Option<GameWinner>,
    pub outcome: Option<GameOutcome>,
}

impl From<GameBoard> for ApiGameBoard {
    fn from(game_board: GameBoard) -> Self {
        Self {
            game_id: game_board.id().to_string(),
            board: game_board.board().clone(),
            status: game_board.status().clone(),
            winner: game_board.winner().clone(),
            outcome: game_board.outcome().clone(),
        }
    }
}

impl ApiGameBoard {
    pub fn game_id(&self) -> &str {
        &self.game_id
    }

    // TODO: I should consolidate these types
    pub fn turn(&self) -> String {
        let player = match self.board.turn() {
            Player::White => GameWinner::White,
            Player::Black => GameWinner::Black,
        };
        player.to_string()
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

    pub fn board_html(&self) -> String {
        // We'll just pass raw HTML to our template
        let mut html_board = String::new();
        html_board.push_str("<table class='chess-board'>");

        // Iterate over ranks to fully construct the board -- we need to populate every cell
        //  with metadata at the moment
        // Depending on the player, we'll either iterate from 0..8 or 8..0
        match self.board.turn() {
            Player::White => {
                for rank in (0..8).rev() {
                    // New rank
                    html_board.push_str("<tr class='chess-rank'>");
                    for file in 0..8 {
                        // Read the piece at this square and populate the cell
                        let square = Sq::from(rank * 8 + file);
                        let piece = self.board.piece_at_sq(square);
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
            }
            Player::Black => {
                for rank in 0..8 {
                    // New rank
                    html_board.push_str("<tr class='chess-rank'>");
                    for file in (0..8).rev() {
                        // Read the piece at this square and populate the cell
                        let square = Sq::from(rank * 8 + file);
                        let piece = self.board.piece_at_sq(square);
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
            }
        };

        html_board.push_str("</table>");
        html_board
    }
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
