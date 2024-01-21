use sqlx::FromRow;
use sqlx::PgConnection;
use time::OffsetDateTime;
use sqlx::types::Uuid;

use super::game_outcome::GameOutcome;
use super::game_status::GameStatus;
use super::game_winner::GameWinner;

pub struct NewGame;

impl NewGame {
    pub async fn create(conn: &mut PgConnection) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO games DEFAULT VALUES RETURNING id
            "#,
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok(id)
    }
}

#[derive(FromRow)]
pub struct Game {
    id: Uuid,
    current_fen_id: Option<Uuid>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
    status: GameStatus,
    winner: Option<GameWinner>,
    outcome: Option<GameOutcome>,
}

// impl Game {
//     pub async fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Game, sqlx::Error> {
//         let game = sqlx::query_as!(
//             Game,
//             r#"
//             SELECT * FROM games WHERE id = $1
//             "#,
//             id
//         )
//         .fetch_one(&mut *conn)
//         .await?;
//         Ok(game)
//     }
// }