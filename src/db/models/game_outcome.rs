use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
// use sqlx::encode::IsNull;
// use sqlx::error::BoxDynError;
// use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
// use sqlx::{Decode, Encode, Postgres, Type};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum GameOutcome {
    Checkmate,
    Stalemate,
    Resignation
}

impl Display for GameOutcome {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameOutcome::Checkmate => write!(f, "checkmate"),
            GameOutcome::Stalemate => write!(f, "stalemate"),
            GameOutcome::Resignation => write!(f, "resignation")
        }
    }
}

impl TryFrom<&str> for GameOutcome {
    type Error = GameOutcomeError;

    fn try_from(val: &str) -> Result<Self, GameOutcomeError> {
        let variant = match val {
            "checkmate" => GameOutcome::Checkmate,
            "stalemate" => GameOutcome::Stalemate,
            "resignation" => GameOutcome::Resignation,
            _ => return Err(GameOutcomeError::InvalidStateValue),
        };

        Ok(variant)
    }
}

// impl Decode<'_, Postgres> for GameOutcome { 
//     fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
//         let inner_val = <&str as Decode<Postgres>>::decode(value)?;
//         Self::try_from(inner_val).map_err(Into::into)
//     }
// }

// impl Encode<'_, Postgres> for GameOutcome {
//     fn encode_by_ref(&self, args: &mut PgArgumentBuffer) -> IsNull {
//         // args.push(SqliteArgumentValue::Text(self.to_string().into()));
//         // args.push(self.to_string().as_str());
//         args.push(self.to_string().as_slice());
//         IsNull::No
//     }
// }

// impl Type<Postgres> for GameOutcome {
//     fn compatible(ty: &PgTypeInfo) -> bool {
//         <&str as Type<Postgres>>::compatible(ty)
//     }

//     fn type_info() -> PgTypeInfo {
//         <&str as Type<Postgres>>::type_info()
//     }
// }

#[derive(Debug, thiserror::Error)]
pub enum GameOutcomeError {
    #[error("attempted to decode unknown state value")]
    InvalidStateValue,
}

// #[cfg(test)]
// mod property_tests {
//     use proptest::prelude::*;

//     use super::*;

//     proptest! {
//         /// Show that any [`GameOutcome`] may be serialized, and then deserialized.
//         #[test]
//         fn metadata_states_can_be_round_tripped(input in any::<GameOutcome>()) {
//             let round_trip = input.to_string().as_str().try_into().unwrap();
//             prop_assert_eq!(input, round_trip);
//         }
//     }
// }
