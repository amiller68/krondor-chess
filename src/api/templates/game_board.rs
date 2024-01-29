use askama::Template;

use crate::api::models::ApiGameBoard;

#[derive(Template, Clone)]
#[template(path = "game_board.html")]
pub struct GameBoardTemplate {
    pub api_game_board: ApiGameBoard,
}
