use std::sync::Arc;

use actix_web::{cookie::Display, http::uri::PathAndQuery};
use anyhow::Result;
use async_trait::async_trait;
use base64::Engine;
use bytes::Bytes;
use downcast_rs::Downcast;
use hash::{hash_password_secret, HashAlgorithmTag, HashPasswordSecret};
use hkdf::{hkdf_password_secret, HkdfAlgorithmTag, HkdfPasswordSecret};
use nom::{
    bytes::complete::take_while,
    character::complete::{alphanumeric1, char, u32},
    combinator::{eof, fail, map_res},
    sequence::terminated,
    IResult, Parser,
};
use phc::{phc_password_secret, PhcAlgorithmTag, PhcPasswordSecret};
use rand_core::CryptoRngCore;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::server::IdServer;

use super::{
    IdProvider, AuthResult, AuthenticationError, IdAuthenticationProvider,
    IdCred, IdCredExt, IdProviderFactory, IdProviderId, IdSecret,
    IdSecretExt, IdSecretId, IdWebAuthenticationFlow, User,
};

pub mod hash;
pub mod hkdf;
pub mod phc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct PasswordIdPConfig {
    #[serde(alias = "allowed")]
    pub allow_algorithms: Vec<AlgorithmTag>,
}

impl IdProviderFactory for PasswordIdPConfig {
    fn to_idp(&self, id: IdProviderId) -> Result<IdProvider> {
        let idp = Arc::new(PasswordIdP{id, config:self.clone()});
        Ok(IdProvider {
            id,
            authentication: Some(idp.clone()),
            web_authentication: None,
            web_register: None,
        })
    }
}

pub struct PasswordIdP {
    pub id: IdProviderId,
    pub config: PasswordIdPConfig,
}

#[async_trait]
impl IdAuthenticationProvider for PasswordIdP {
    async fn list_secrets(
        &self,
        user: Option<User>,
    ) -> AuthResult<Vec<IdSecretId>> {
        let query = if let Some(user) = user {
            sqlx::query_scalar(
            r#"SELECT "passwd_id" FROM "password" WHERE "passwd_user" = $1"#,
        )
        .bind(user)
        } else {
            sqlx::query_scalar(r#"SELECT "passwd_id" FROM "password""#)
        };
        Ok(query
            .fetch_all(&IdServer::get().database)
            .await
            .map_err(|e| AuthenticationError::Unknown(e.into()))?)
    }

    async fn fetch(
        &self,
        secret: IdSecretId,
    ) -> AuthResult<(User, Box<dyn IdSecret>)> {
        #[derive(FromRow)]
        #[sqlx(no_pg_array)]
        struct FetchResult {
            user: User,
            password: Vec<u8>,
        }
        let result = sqlx::query_as::<_, FetchResult>(
            r#"SELECT "passwd_user" AS "user", "passwd_password" AS "password"
            FROM "password" WHERE "passwd_id" = $1"#,
        )
        .bind(secret)
        .fetch_one(&IdServer::get().database)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AuthenticationError::SecretNotFound,
            _ => AuthenticationError::Unknown(e.into()),
        })?;
        let secret = PasswordSecret::try_from(
            String::from_utf8(result.password)
                .map_err(|_| AuthenticationError::BadSecret)?
                .as_str(),
        )?;
        Ok((result.user, Box::new(secret)))
    }

    async fn authenticate(
        &self,
        _: User,
        _: IdSecretId,
        secret: &Box<dyn IdSecret>,
        cred: &Box<dyn IdCred>,
    ) -> AuthResult<()> {
        let secret = secret.auth_downcast::<PasswordSecret>()?;
        let cred = cred.auth_downcast::<PasswordCred>()?;
        if secret.authenticate(&cred.0)? {
            Ok(())
        } else {
            Err(AuthenticationError::InvalidCredential)
        }
    }

    async fn remove(&self, secret: IdSecretId) -> Result<()> {
        sqlx::query(r#"DELETE FROM "password" WHERE "passwd_id" = $1"#)
            .bind(secret)
            .execute(&IdServer::get().database)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum AlgorithmTag {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "hash")]
    Hash(HashAlgorithmTag),
    #[serde(rename = "hkdf")]
    Hkdf(HkdfAlgorithmTag),
    #[serde(rename = "phc")]
    Phc(PhcAlgorithmTag),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordSecret {
    Plain(String),
    Hash(HashPasswordSecret),
    Hkdf(HkdfPasswordSecret),
    Phc(PhcPasswordSecret),
}

impl IdSecret for PasswordSecret {}

impl PasswordSecret {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, version) = terminated(u32, char(':'))(input)?;
        match version {
            1 => {
                let (input, algo) =
                    terminated(alphanumeric1, char(':'))(input)?;
                match algo {
                    "plain" => Ok(("", Self::Plain(input.to_string()))),
                    "hash" => hash_password_secret.map(Self::Hash).parse(input),
                    "hkdf" => hkdf_password_secret.map(Self::Hkdf).parse(input),
                    "phc" => phc_password_secret.map(Self::Phc).parse(input),
                    _ => fail(input),
                }
            }
            _ => fail(input),
        }
    }

    pub fn new(
        algo: AlgorithmTag,
        password: &str,
        rng: &mut dyn CryptoRngCore,
    ) -> Result<Self> {
        Ok(match algo {
            AlgorithmTag::Plain => Self::Plain(password.to_owned()),
            AlgorithmTag::Hash(algo) => {
                Self::Hash(HashPasswordSecret::new(algo, password))
            }
            AlgorithmTag::Hkdf(algo) => {
                Self::Hkdf(HkdfPasswordSecret::new(algo, password, rng))
            }
            AlgorithmTag::Phc(algo) => {
                Self::Phc(PhcPasswordSecret::new(algo, password, rng)?)
            }
        })
    }

    pub fn authenticate(&self, password: &str) -> AuthResult<bool> {
        match self {
            PasswordSecret::Plain(v) => Ok(v.as_str() == password),
            PasswordSecret::Hash(v) => v.authenticate(password),
            PasswordSecret::Hkdf(v) => v.authenticate(password),
            PasswordSecret::Phc(v) => v.authenticate(password),
        }
    }
}

impl TryFrom<&str> for PasswordSecret {
    type Error = AuthenticationError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let (_, (secret, _)) = Self::parse
            .and(eof)
            .parse(input)
            .map_err(|_| AuthenticationError::BadSecret)?;
        Ok(secret)
    }
}

impl From<PasswordSecret> for Bytes {
    fn from(value: PasswordSecret) -> Self {
        value.to_string().into_bytes().into()
    }
}

impl ToString for PasswordSecret {
    fn to_string(&self) -> String {
        match self {
            PasswordSecret::Plain(v) => format!("1:plain:{v}"),
            PasswordSecret::Hash(v) => format!("1:hash:{v}"),
            PasswordSecret::Hkdf(v) => format!("1:hkdf:{v}"),
            PasswordSecret::Phc(v) => format!("1:phc:{v}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordCred(pub String);
impl IdCred for PasswordCred {}

pub fn base64_standard(input: &str) -> IResult<&str, Bytes> {
    map_res(
        take_while(|c| base64::alphabet::STANDARD.as_str().contains(c)),
        |bytes| {
            base64::prelude::BASE64_STANDARD_NO_PAD
                .decode(bytes)
                .map(Bytes::from)
        },
    )(input)
}
