use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Extension,
};

use crate::api::models::ApiGame;
use crate::database::models::NewGame;
use crate::AppState;
// use crate::stream::{GamesMutationKind, GamesStream, GamesUpdate};

pub async fn handler(
    State(state): State<AppState>,
    // Extension(tx): Extension<GamesStream>
) -> Result<impl IntoResponse, CreateGameError> {
    let game = NewGame::create(&state.database()).await?;

    // if tx
    //     .send(GamesUpdate {
    //         mutation_kind: GamesMutationKind::Create,
    //         id: game.id(),
    //     })
    //     .is_err()
    // {
    //     eprintln!(
    //         "Record with ID {} was created but nobody's listening to the stream!",
    //         game.id()
    //     );
    // }

    let api_game = ApiGame::from(game);
    Ok(NewGameTemplate { api_game })
}

#[derive(Template)]
#[template(path = "game.html")]
struct NewGameTemplate {
    api_game: ApiGame,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateGameError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl IntoResponse for CreateGameError {
    fn into_response(self) -> Response {
        let body = format!("{}", self);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
