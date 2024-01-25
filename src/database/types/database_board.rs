use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};

use pleco::board::Board;
use sqlx::error::BoxDynError;
use sqlx::postgres::{PgTypeInfo, PgValueRef};
use sqlx::{Decode, Postgres, Type};

#[derive(Clone)]
pub struct DatabaseBoard(Board);

impl DatabaseBoard {
    pub fn new() -> Self {
        Self(Board::start_pos())
    }
}

impl Decode<'_, Postgres> for DatabaseBoard {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let fen_str = <String as Decode<Postgres>>::decode(value)?;

        // TODO: length check
        let board = Board::from_fen(&fen_str).map_err(|_| DatabaseBoardError::InvalidFen)?;

        Ok(Self(board))
    }
}

impl Type<Postgres> for DatabaseBoard {
    fn compatible(ty: &PgTypeInfo) -> bool {
        <String as Type<Postgres>>::compatible(ty)
    }

    fn type_info() -> PgTypeInfo {
        <String as Type<Postgres>>::type_info()
    }
}

impl Deref for DatabaseBoard {
    type Target = Board;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DatabaseBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for DatabaseBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.fen())
    }
}

impl Display for DatabaseBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.fen())
    }
}

impl From<Board> for DatabaseBoard {
    fn from(val: Board) -> Self {
        Self(val)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseBoardError {
    #[error("invalid fen string")]
    InvalidFen,
}
