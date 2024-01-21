use askama::Template;
use axum::{
    response::IntoResponse,
    routing::{get, get_service},
    Extension, Router,
};
use sqlx::PgPool;

use tokio::sync::broadcast::channel;

use tower_http::services::{ServeDir, ServeFile};

mod api;
mod database;
// mod stream;

// use stream::{handle_game_stream, GamesUpdate};

#[derive(Clone)]
pub struct AppState {
    database: PgPool,
}

impl AppState {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }

    pub fn database(&self) -> PgPool {
        self.database.clone()
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:postgres@localhost:5432/postgres"
    )]
    db: PgPool,
) -> shuttle_axum::ShuttleAxum {
    // Run migrations
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Looks like something went wrong with migrations :(");
    // Setup State
    let state = AppState::new(db);

    // Register panics as they happen
    register_panic_logger();

    // Setup Router + Streams
    // let (tx, _rx) = channel::<GamesUpdate>(10);
    let router = Router::new()
        // Home page
        .route("/", get(index))
        // .route("/stream", get(stream))
        .route(
            "/games",
            get(api::games::read_all_games::handler).post(api::games::create_game::handler),
        )
        // .route("/games/stream", get(handle_game_stream))
        .with_state(state)
        // Static assets
        .nest_service("/static", ServeDir::new("static"));
    // .layer(Extension(tx));

    // Run!
    Ok(router.into())
}

async fn index() -> impl IntoResponse {
    IndexTemplate
}

// async fn stream() -> impl IntoResponse {
//     StreamTemplate
// }

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

// #[derive(Template)]
// #[template(path = "stream.html")]
// struct StreamTemplate;

/// Sets up system panics to use the tracing infrastructure to log reported issues. This doesn't
/// prevent the panic from taking out the service but ensures that it and any available information
/// is properly reported using the standard logging mechanism.
fn register_panic_logger() {
    std::panic::set_hook(Box::new(|panic| match panic.location() {
        Some(loc) => {
            tracing::error!(
                message = %panic,
                panic.file = loc.file(),
                panic.line = loc.line(),
                panic.column = loc.column(),
            );
        }
        None => tracing::error!(message = %panic),
    }));
}
