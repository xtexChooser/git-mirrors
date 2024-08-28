use std::fmt::Display;

use anyhow::{bail, Result};
use argon2::{Argon2, PasswordHash, PasswordHasher};
use nom::{
    combinator::{fail, map_res, rest, success},
    IResult,
};
use password_hash::{PasswordHashString, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;
use rand_core::CryptoRngCore;
use scrypt::Scrypt;
use serde::{Deserialize, Serialize};

use crate::idp::AuthResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum PhcAlgorithmTag {
    Argon2d,
    Argon2i,
    Argon2id,
    Pbkdf2Sha256,
    Pbkdf2Sha512,
    Scrypt,
}

impl PhcAlgorithmTag {
    fn hash(
        &self,
        password: &[u8],
        rng: &mut dyn CryptoRngCore,
    ) -> Result<PasswordHashString> {
        let salt = SaltString::generate(rng);
        let result = match self {
            PhcAlgorithmTag::Argon2d => Argon2::new(
                argon2::Algorithm::Argon2d,
                argon2::Version::default(),
                argon2::Params::default(),
            )
            .hash_password(password, &salt),
            PhcAlgorithmTag::Argon2i => Argon2::new(
                argon2::Algorithm::Argon2i,
                argon2::Version::default(),
                argon2::Params::default(),
            )
            .hash_password(password, &salt),
            PhcAlgorithmTag::Argon2id => Argon2::new(
                argon2::Algorithm::Argon2id,
                argon2::Version::default(),
                argon2::Params::default(),
            )
            .hash_password(password, &salt),
            PhcAlgorithmTag::Pbkdf2Sha256 => Pbkdf2.hash_password_customized(
                password,
                Some(pbkdf2::Algorithm::PBKDF2_SHA256_IDENT),
                None,
                pbkdf2::Params::default(),
                &salt,
            ),
            PhcAlgorithmTag::Pbkdf2Sha512 => Pbkdf2.hash_password_customized(
                password,
                Some(pbkdf2::Algorithm::PBKDF2_SHA512_IDENT),
                None,
                pbkdf2::Params::default(),
                &salt,
            ),
            PhcAlgorithmTag::Scrypt => Scrypt.hash_password(password, &salt),
        };
        Ok(result.map(|h| h.serialize())?)
    }

    fn verify<'a>(
        &self,
        hash: PasswordHash<'a>,
        password: &[u8],
    ) -> Result<()> {
        match self {
            PhcAlgorithmTag::Argon2d => Argon2::new(
                argon2::Algorithm::Argon2d,
                argon2::Version::default(),
                argon2::Params::default(),
            )
            .verify_password(password, &hash),
            PhcAlgorithmTag::Argon2i => Argon2::new(
                argon2::Algorithm::Argon2i,
                argon2::Version::default(),
                argon2::Params::default(),
            )
            .verify_password(password, &hash),
            PhcAlgorithmTag::Argon2id => Argon2::new(
                argon2::Algorithm::Argon2id,
                argon2::Version::default(),
                argon2::Params::default(),
            )
            .verify_password(password, &hash),
            PhcAlgorithmTag::Pbkdf2Sha256 | PhcAlgorithmTag::Pbkdf2Sha512 => {
                Pbkdf2.verify_password(password, &hash)
            }
            PhcAlgorithmTag::Scrypt => Scrypt.verify_password(password, &hash),
        }?;
        Ok(())
    }
}

impl<'a> TryFrom<PasswordHash<'a>> for PhcAlgorithmTag {
    type Error = anyhow::Error;

    fn try_from(
        value: PasswordHash<'a>,
    ) -> std::result::Result<Self, Self::Error> {
        value.algorithm.try_into()
    }
}

impl<'a> TryFrom<password_hash::Ident<'a>> for PhcAlgorithmTag {
    type Error = anyhow::Error;

    fn try_from(
        value: password_hash::Ident<'a>,
    ) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            argon2::ARGON2D_IDENT => Self::Argon2d,
            argon2::ARGON2I_IDENT => Self::Argon2i,
            argon2::ARGON2ID_IDENT => Self::Argon2id,
            pbkdf2::Algorithm::PBKDF2_SHA256_IDENT => Self::Pbkdf2Sha256,
            pbkdf2::Algorithm::PBKDF2_SHA512_IDENT => Self::Pbkdf2Sha512,
            scrypt::ALG_ID => Self::Scrypt,
            _ => bail!("unsupported algo ID found in PHC string"),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhcPasswordSecret {
    pub tag: PhcAlgorithmTag,
    pub hash: PasswordHashString,
}

pub fn phc_password_secret(input: &str) -> IResult<&str, PhcPasswordSecret> {
    let (input, hash) = map_res(rest, PasswordHashString::new)(input)?;
    if let Ok(tag) = hash.algorithm().try_into() {
        success(PhcPasswordSecret { tag, hash })(input)
    } else {
        fail(input)
    }
}

impl PhcPasswordSecret {
    pub fn new(
        algo: PhcAlgorithmTag,
        password: &str,
        rng: &mut dyn CryptoRngCore,
    ) -> Result<Self> {
        Ok(Self {
            tag: algo,
            hash: algo.hash(password.as_bytes(), rng)?,
        })
    }

    pub fn authenticate(&self, password: &str) -> AuthResult<bool> {
        let tag = self.tag;
        let hash = self.hash.password_hash();
        Ok(tag.verify(hash, password.as_bytes()).is_ok())
    }
}

impl Display for PhcPasswordSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.hash.as_str())
    }
}
