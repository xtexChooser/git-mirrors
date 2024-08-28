use std::fmt::Display;

use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use bytes::Bytes;
use nom::{
    character::complete::{alphanumeric0, char},
    combinator::{fail, success},
    sequence::separated_pair,
    IResult, Parser,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::Keccak512;

use crate::idp::AuthResult;

use super::base64_standard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum HashAlgorithmTag {
    Sha256,
    Sha512,
    Sha384,
    Keccak512,
}

impl HashAlgorithmTag {
    pub fn hash(&self, data: &[u8]) -> Bytes {
        match self {
            HashAlgorithmTag::Sha256 => Sha256::digest(data).to_vec().into(),
            HashAlgorithmTag::Sha512 => Sha512::digest(data).to_vec().into(),
            HashAlgorithmTag::Sha384 => Sha384::digest(data).to_vec().into(),
            HashAlgorithmTag::Keccak512 => {
                Keccak512::digest(data).to_vec().into()
            }
        }
    }
}

pub fn hash_algorithm_tag(input: &str) -> IResult<&str, HashAlgorithmTag> {
    let (input, algo) = alphanumeric0(input)?;
    match algo {
        "sha256" => success(HashAlgorithmTag::Sha256)(input),
        "sha512" => success(HashAlgorithmTag::Sha512)(input),
        "sha384" => success(HashAlgorithmTag::Sha384)(input),
        "keccak512" => success(HashAlgorithmTag::Keccak512)(input),
        _ => fail(input),
    }
}

impl Display for HashAlgorithmTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            HashAlgorithmTag::Sha256 => "sha256",
            HashAlgorithmTag::Sha512 => "sha512",
            HashAlgorithmTag::Sha384 => "sha384",
            HashAlgorithmTag::Keccak512 => "keccak512",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashPasswordSecret {
    pub algorithm: HashAlgorithmTag,
    pub hash: Bytes,
}

impl HashPasswordSecret {
    pub fn new(algorithm: HashAlgorithmTag, password: &str) -> Self {
        Self {
            algorithm,
            hash: algorithm.hash(password.as_bytes()),
        }
    }

    pub fn authenticate(&self, password: &str) -> AuthResult<bool> {
        Ok(self.algorithm.hash(password.as_bytes()) == self.hash)
    }
}

pub fn hash_password_secret(input: &str) -> IResult<&str, HashPasswordSecret> {
    let (input, (algorithm, hash)) =
        (separated_pair(hash_algorithm_tag, char(':'), base64_standard))
            .parse(input)?;
    success(HashPasswordSecret { algorithm, hash })(input)
}

impl Display for HashPasswordSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:{}",
            self.algorithm,
            BASE64_STANDARD_NO_PAD.encode(&self.hash)
        ))
    }
}
