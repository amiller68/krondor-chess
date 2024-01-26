use askama::Template;
use axum::Extension;
use axum::{response::IntoResponse, routing::get, Router};
use sqlx::PgPool;
use tokio::sync::broadcast::channel;
use tower_http::services::ServeDir;

mod api;
mod database;

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
        local_uri = &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set") 
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
    let (tx, _rx) = channel::<api::games::watch_game_sse::GameUpdate>(10);

    // Register panics as they happen
    register_panic_logger();

    // TODO: Move api router into api module
    // Setup Router
    let router = Router::new()
        // Home page
        .route("/", get(index))
        .route(
            "/games",
            get(api::games::read_all_games::handler).post(api::games::create_game::handler),
        )
        .route(
            "/games/:game_id",
            get(api::games::read_game::handler).post(api::games::make_move::handler),
        )
        .route(
            "/games/:game_id/sse",
            get(api::games::watch_game_sse::handler),
        )
        .route(
            "/games/:game_id/board",
            get(api::games::read_game_board::handler),
        )
        .with_state(state)
        .layer(Extension(tx))
        // Static assets
        .nest_service("/static", ServeDir::new("static"));

    // Run!
    Ok(router.into())
}

async fn index() -> impl IntoResponse {
    IndexTemplate
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

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
