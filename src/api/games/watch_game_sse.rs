use std::convert::Infallible;
use std::time::Duration;

use askama::Template;
use axum::{
    extract::Path,
    response::{sse::Event, Sse},
    Extension,
};
use sqlx::types::Uuid;
use tokio::sync::broadcast::Sender;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt as _};

use crate::api::templates::GameBoardTemplate;

// TODO: generalize and use the read_game_board handler
pub type GameUpdateStream = Sender<GameBoardTemplate>;

// TODO: proper error handling
pub async fn handler(
    Path(game_id): Path<Uuid>,
    Extension(tx): Extension<GameUpdateStream>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = tx.subscribe();

    let stream = BroadcastStream::new(rx);

    // Catch all updata events for this game
    Sse::new(
        stream
            .map(move |tagb| {
                let tagb = tagb.unwrap();
                Event::default()
                    .event(format!("game-update-{}", game_id))
                    .data(tagb.render().unwrap())
            })
            .map(Ok),
    )
    .keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(60))
            .text("keep-alive-text"),
    )
}
