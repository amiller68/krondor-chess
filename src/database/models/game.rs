use sqlx::types::Uuid;
use sqlx::FromRow;
use sqlx::PgPool;
use time::OffsetDateTime;

use super::game_outcome::GameOutcome;
use super::game_status::GameStatus;
use super::game_winner::GameWinner;

pub struct NewGame;

impl NewGame {
    pub async fn create(conn: &PgPool) -> Result<Game, sqlx::Error> {
        let game = sqlx::query_as!(
            Game,
            r#"INSERT INTO games DEFAULT VALUES RETURNING
                id as "id: Uuid",
                current_fen_id as "current_fen_id: Uuid",
                created_at as "created_at: OffsetDateTime",
                updated_at as "updated_at: OffsetDateTime",
                status as "status: GameStatus",
                winner as "winner: GameWinner",
                outcome as "outcome: GameOutcome"
            "#,
        )
        .fetch_one(conn)
        .await?;
        Ok(game)
    }
}

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct Game {
    id: Uuid,
    current_fen_id: Uuid,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
    status: GameStatus,
    winner: Option<GameWinner>,
    outcome: Option<GameOutcome>,
}

impl Game {
    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }
    // pub fn current_fen_id(&self) -> Uuid {
    //     self.current_fen_id
    // }
    // pub fn created_at(&self) -> OffsetDateTime {
    //     self.created_at
    // }
    // pub fn updated_at(&self) -> OffsetDateTime {
    //     self.updated_at
    // }
    pub fn status(&self) -> &GameStatus {
        &self.status
    }
    // pub fn winner(&self) -> &Option<GameWinner> {
    //     &self.winner
    // }
    // pub fn outcome(&self) -> &Option<GameOutcome> {
    //     &self.outcome
    // }

    // TODO: pagination
    pub async fn read_all(conn: &PgPool) -> Result<Vec<Game>, sqlx::Error> {
        let games = sqlx::query_as!(
            Game,
            r#"SELECT
                id as "id: Uuid",
                current_fen_id as "current_fen_id: Uuid",
                created_at as "created_at: OffsetDateTime",
                updated_at as "updated_at: OffsetDateTime",
                status as "status: GameStatus",
                winner as "winner: GameWinner",
                outcome as "outcome: GameOutcome"
            FROM games
            "#,
        )
        .fetch_all(conn)
        .await?;
        Ok(games)
    }
}

#[derive(Debug, Clone)]
pub struct PartialGameWithFen {
    id: Uuid,
    current_fen: String,
}

impl PartialGameWithFen {
    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn current_fen(&self) -> &str {
        &self.current_fen
    }

    pub async fn read(conn: &PgPool, id: Uuid) -> Result<Option<PartialGameWithFen>, sqlx::Error> {
        let game = sqlx::query_as!(
            PartialGameWithFen,
            r#"SELECT
                games.id as "id: Uuid",
                fens.fen as "current_fen: String"
            FROM games
            INNER JOIN fens ON games.current_fen_id = fens.id
            WHERE games.id = $1
            LIMIT 1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;
        Ok(game)
    }
}
