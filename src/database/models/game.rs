use pleco::core::Player;
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
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn status(&self) -> &GameStatus {
        &self.status
    }

    pub fn winner(&self) -> &Option<GameWinner> {
        &self.winner
    }

    pub fn outcome(&self) -> &Option<GameOutcome> {
        &self.outcome
    }

    // TODO: make the state machine more robust -- but maybe eventually this will
    //  check if a user has access to a game
    /// Check if a game exists in the database
    pub async fn exists(conn: &mut PgConnection, game_id: Uuid) -> Result<bool, GameError> {
        let maybe_game = sqlx::query!(r#"SELECT id FROM games WHERE id = $1"#, game_id,)
            .fetch_optional(&mut *conn)
            .await?;
        Ok(maybe_game.is_some())
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
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(games)
    }
}

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct GameBoard {
    id: Uuid,
    board: Board,
    status: GameStatus,
    winner: Option<GameWinner>,
    outcome: Option<GameOutcome>,
}

impl GameBoard {
    // Getters
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn status(&self) -> &GameStatus {
        &self.status
    }

    pub fn winner(&self) -> &Option<GameWinner> {
        &self.winner
    }

    pub fn outcome(&self) -> &Option<GameOutcome> {
        &self.outcome
    }

    /* Database Operations */

    /// Return the latest board for a game -- assumes the game exists
    pub async fn latest(conn: &mut PgConnection, game_id: Uuid) -> Result<Self, GameError> {
        let maybe_game = sqlx::query_as!(
            Self,
            r#"SELECT
                g.id as "id: Uuid",
                p.board as "board: Board",
                g.status as "status: GameStatus",
                g.winner as "winner: GameWinner",
                g.outcome as "outcome: GameOutcome"
            FROM positions as p
            JOIN moves as m ON m.position_id = p.id
            JOIN games as g ON g.id = m.game_id
            WHERE g.id = $1
            ORDER BY m.move_number DESC
            LIMIT 1
            "#,
            game_id,
        )
        .fetch_optional(&mut *conn)
        .await?;

        match maybe_game {
            Some(game) => Ok(game),
            None => Ok(Self {
                id: game_id,
                board: Board::new(),
                status: GameStatus::Created,
                winner: None,
                outcome: None,
            }),
        }
    }

    /// Make a move in a game. Return the updated board -- assumes the game exists
    pub async fn make_move(
        conn: &mut PgConnection,
        game_id: Uuid,
        uci_move: &str,
        resign: bool,
    ) -> Result<Self, GameError> {
        let game = Self::latest(conn, game_id).await?;

        // TODO: I don't like that this isn't an explicit error
        // If the game is already over, just return it
        if game.status == GameStatus::Complete {
            return Ok(game);
        }

        let mut board = game.board().clone();
        let player = board.turn();

        // TODO: MAKE MORE CONCISE
        // If the current player is resigning, update the game status and return
        if resign {
            let game_winner = match player {
                Player::White => GameWinner::Black,
                Player::Black => GameWinner::White,
            };
            let game_outcome = GameOutcome::Resignation;
            let game_status = GameStatus::Complete;
            sqlx::query!(
                r#"UPDATE games
                SET status = $1,
                    winner = $2,
                    outcome = $3
                WHERE id = $4
                "#,
                game_status.to_string(),
                game_winner.to_string(),
                game_outcome.to_string(),
                game_id
            )
            .execute(&mut *conn)
            .await?;
            return Ok(Self {
                id: game_id,
                board: board.clone(),
                status: game_status,
                winner: Some(game_winner),
                outcome: Some(game_outcome),
            });
        }

        let move_number = board.moves_played() as i32;

        // TODO: I don't like that this isn't an explicit error
        // Attempt to make the move on the board
        let success = board.apply_uci_move(uci_move);
        if !success {
            return Ok(game);
            // return Err(GameError::InvalidMove(uci_move.to_string(), board.clone()));
        }

        // Insert the FEN into the database if it doesn't already exist
        // Return the position ID
        let board_fen = board.fen();
        // TODO: this is super gross, but I can't figure out how to do this in one query
        let position_id = sqlx::query_scalar!(
            r#"
            WITH attempted_insert AS (
                INSERT INTO positions (board)
                VALUES ($1)
                ON CONFLICT (board)
                DO NOTHING
                RETURNING id
            )
            SELECT id FROM attempted_insert
            UNION ALL
            SELECT id as "id: Uuid" FROM positions WHERE board = $1
            LIMIT 1;
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

        // Check if the game is over
        if board.checkmate() {
            let game_winner = match player {
                Player::White => GameWinner::White,
                Player::Black => GameWinner::Black,
            };
            let game_outcome = GameOutcome::Checkmate;
            let game_status = GameStatus::Complete;
            sqlx::query!(
                r#"UPDATE games
                SET status = $1,
                    winner = $2,
                    outcome = $3
                WHERE id = $4
                "#,
                game_status.to_string(),
                game_winner.to_string(),
                game_outcome.to_string(),
                game_id,
            )
            .execute(&mut *conn)
            .await?;
            return Ok(Self {
                id: game_id,
                board: board.clone(),
                status: game_status,
                winner: Some(game_winner),
                outcome: Some(game_outcome),
            });
        } else if board.stalemate() {
            let game_winner = GameWinner::Draw;
            let game_outcome = GameOutcome::Stalemate;
            let game_status = GameStatus::Complete;
            sqlx::query!(
                r#"UPDATE games
                SET winner = $1,
                    status = $2,
                    outcome = $3
                WHERE id = $4
                "#,
                game_winner.to_string(),
                game_status.to_string(),
                game_outcome.to_string(),
                game_id,
            )
            .execute(&mut *conn)
            .await?;
            return Ok(Self {
                id: game_id,
                board: board.clone(),
                status: game_status,
                winner: Some(game_winner),
                outcome: Some(game_outcome),
            });
        }

        // TODO: find a better way to do this -- maybe there will be an 'accept' game worflow in the future
        // Update the game's status to active if it's not already
        sqlx::query!(
            r#"UPDATE games
            SET status = 'active'
            WHERE id = $1
            AND status = 'created'
            "#,
            game_id,
        )
        .execute(&mut *conn)
        .await?;

        // Return the updated board
        Ok(Self {
            id: game_id,
            board: board.clone(),
            status: GameStatus::Active,
            winner: None,
            outcome: None,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    // #[error("invalid move: {0} on board {1}")]
    // InvalidMove(String, Board),
}
