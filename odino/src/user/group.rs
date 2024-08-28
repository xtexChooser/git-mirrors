use std::{borrow::Cow, fmt::Display};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgTypeInfo;

use crate::server::IdServer;

use super::User;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    sqlx::Encode,
    sqlx::Decode,
)]
pub struct UserGroup(Cow<'static, str>);

impl UserGroup {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the "everyone" user group.
    #[inline]
    pub fn everyone() -> Self {
        "*".into()
    }

    /// Checks if the group is the "everyone" user group ("*" group).
    /// Such groups include all accessors, including guests.
    #[inline]
    pub fn is_everyone(&self) -> bool {
        self == "*"
    }

    /// Returns the "all users" user group.
    #[inline]
    pub fn user() -> Self {
        "user".into()
    }

    /// Checks if the group is the "all users" user group ("user" group).
    /// Such groups include all users, excluding guests.
    #[inline]
    pub fn is_user(&self) -> bool {
        self == "user"
    }

    /// Checks if the group is an implicit group.
    /// Implicit groups should never appear in the "user_group" database table,
    /// and cannot be added/removed by anyone.
    #[inline]
    pub fn is_implicit(&self) -> bool {
        self.is_everyone() || self.is_user()
    }

    pub fn contains_guest(&self) -> bool {
        self.is_everyone()
    }

    pub fn contains_all_users(&self) -> bool {
        self.is_everyone() || self.is_user()
    }
}

impl Default for UserGroup {
    fn default() -> Self {
        Self::everyone()
    }
}

impl AsRef<str> for UserGroup {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for UserGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for UserGroup {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<&'static str> for UserGroup {
    fn from(value: &'static str) -> Self {
        Self(value.into())
    }
}

impl PartialEq<str> for UserGroup {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<String> for UserGroup {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == *other
    }
}

impl sqlx::Type<sqlx::Postgres> for UserGroup {
    fn type_info() -> PgTypeInfo {
        Cow::<'static, str>::type_info()
    }
}

// @TODO: clear expired memberships
impl UserGroup {
    pub async fn contains(&self, user: Option<User>) -> Result<bool> {
        if let Some(user) = user {
            self.contains_user(user).await
        } else {
            Ok(self.contains_guest())
        }
    }

    pub async fn contains_user(&self, user: User) -> Result<bool> {
        if self.contains_all_users() {
            Ok(true)
        } else {
            Ok(sqlx::query_scalar::<_,bool>(r#"SELECT TRUE FROM "user_group" WHERE "ug_user" = $1 AND "ug_group" = $2"#)
            .bind(&user)
            .bind(&self)
            .fetch_optional(& IdServer::get().database).await?.is_some())
        }
    }

    pub async fn grant(
        &self,
        user: User,
        expiry: Option<UserGroupExpiryUpdate>,
    ) -> Result<()> {
        let mut query = r#"INSERT INTO "user_group" ("ug_user", "ug_group", "ug_expiry") VALUES ($1, $2, "#.to_string();
        match expiry {
            Some(UserGroupExpiryUpdate::Clear) | None => query.push_str("NULL"),
            _ => query.push_str("$3"),
        }
        query.push_str(r#") "#);
        if let Some(expiry) = &expiry {
            query.push_str(r#"ON CONFLICT DO UPDATE SET "ug_expiry" = "#);
            match expiry {
                UserGroupExpiryUpdate::Set(_) => query.push_str(r#"$3"#),
                UserGroupExpiryUpdate::Extend(_) => query.push_str(
                    r#"CASE WHEN $3 > "ug_expiry" THEN $3 ELSE "ug_expiry""#,
                ),
                UserGroupExpiryUpdate::CutDown(_) => query.push_str(
                    r#"CASE WHEN $3 < "ug_expiry" THEN $3 ELSE "ug_expiry""#,
                ),
                UserGroupExpiryUpdate::Clear => query.push_str(r#"NULL"#),
            }
            query.push_str(r#" WHERE "ug_user" = $1 AND "ug_group" = $2"#);
        } else {
            query.push_str(r#"ON CONFLICT DO NOTHING"#);
        }
        let mut query = sqlx::query(&query).bind(user).bind(self);
        match expiry {
            Some(UserGroupExpiryUpdate::Set(expiry))
            | Some(UserGroupExpiryUpdate::Extend(expiry))
            | Some(UserGroupExpiryUpdate::CutDown(expiry)) => {
                query = query.bind(expiry)
            }
            Some(UserGroupExpiryUpdate::Clear) | None => {}
        }
        query.execute(&IdServer::get().database).await?;
        Ok(())
    }

    pub async fn revoke(&self, user: User) -> Result<()> {
        sqlx::query(r#"DELETE FROM "user_group" WHERE "ug_user" = $1 AND "ug_group" = $2"#)
            .bind(user).bind(self)
            .execute(&IdServer::get().database).await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum UserGroupExpiryUpdate {
    Set(DateTime<Utc>),
    Extend(DateTime<Utc>),
    CutDown(DateTime<Utc>),
    Clear,
}
