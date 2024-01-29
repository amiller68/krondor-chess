use askama::Template;

use crate::api::models::ApiGameBoard;

#[derive(Template)]
#[template(path = "game_index.html")]
pub struct GameIndexTemplate {
    pub api_game_board: ApiGameBoard,
}
