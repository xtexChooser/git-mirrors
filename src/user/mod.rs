use std::fmt::Display;

use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo};

pub mod group;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct User(u64);

impl User {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<&User> for u64 {
    fn from(value: &User) -> Self {
        value.0
    }
}

impl From<u64> for User {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for User {
    fn decode(
        value: sqlx::postgres::PgValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self(i64::decode(value)? as u64))
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for User {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        (self.0 as i64).encode_by_ref(buf)
    }
}

impl sqlx::Type<sqlx::Postgres> for User {
    fn type_info() -> PgTypeInfo {
        i64::type_info()
    }
}
