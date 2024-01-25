use sqlx::types::Uuid;
use sqlx::FromRow;
use sqlx::PgConnection;
use sqlx::PgPool;
use time::OffsetDateTime;

use super::game_outcome::GameOutcome;
use super::game_status::GameStatus;
use super::game_winner::GameWinner;

use crate::database::types::DatabaseBoard as Board;

pub struct NewGame;

impl NewGame {
    pub async fn create(conn: &PgPool) -> Result<Game, sqlx::Error> {
        let game = sqlx::query_as!(
            Game,
            r#"INSERT INTO games DEFAULT VALUES RETURNING
                id as "id: Uuid",
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

    pub fn status(&self) -> &GameStatus {
        &self.status
    }

    /* Database Operations */

    /// Return the latest board for a game
    pub async fn latest_board(conn: &mut PgConnection, game_id: Uuid) -> Result<Board, GameError> {
        let maybe_board = sqlx::query_scalar!(
            r#"SELECT board as "board: Board"
            FROM positions
            JOIN moves ON moves.position_id = positions.id
            JOIN games ON games.id = moves.game_id
            WHERE games.id = $1
            ORDER BY moves.move_number DESC
            LIMIT 1
            "#,
            game_id,
        )
        .fetch_optional(&mut *conn)
        .await?;

        match maybe_board {
            Some(board) => Ok(board),
            None => Ok(Board::new()),
        }
    }

    /// Make a move in a game. Return the updated board.
    pub async fn make_move(
        conn: &mut PgConnection,
        game_id: Uuid,
        uci_move: &str,
    ) -> Result<Board, GameError> {
        let mut board = Game::latest_board(conn, game_id).await?;
        let move_number = board.moves_played() as i32;

        // Attempt to make the move on the board
        let success = board.apply_uci_move(uci_move);
        if !success {
            return Err(GameError::InvalidMove(uci_move.to_string(), board.clone()));
        }

        // Insert the FEN into the database if it doesn't already exist
        // Return the position ID
        let board_fen = board.fen();

        let position_id = sqlx::query_scalar!(
            r#"INSERT INTO positions (board)
            VALUES ($1)
            RETURNING id as "id: Uuid"
            "#,
            board_fen,
        )
        .fetch_one(&mut *conn)
        .await?;

        // Insert the move into the database
        sqlx::query!(
            r#"INSERT INTO moves (game_id, position_id, move_number)
            VALUES ($1, $2, $3)
            "#,
            game_id,
            position_id,
            move_number,
        )
        .execute(&mut *conn)
        .await?;

        // Return the updated board
        Ok(board)
    }

    // TODO: pagination
    /// Read all games from the database
    pub async fn read_all(conn: &mut PgConnection) -> Result<Vec<Game>, GameError> {
        let games = sqlx::query_as!(
            Game,
            r#"SELECT
                id as "id: Uuid",
                created_at as "created_at: OffsetDateTime",
                updated_at as "updated_at: OffsetDateTime",
                status as "status: GameStatus",
                winner as "winner: GameWinner",
                outcome as "outcome: GameOutcome"
            FROM games
            "#,
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(games)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("invalid move: {0} on board {1}")]
    InvalidMove(String, Board),
}
