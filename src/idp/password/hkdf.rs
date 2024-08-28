use std::fmt::Display;

use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use bytes::Bytes;
use crypto_common::OutputSizeUser;
use hkdf::{Hkdf, HmacImpl};
use hmac::Hmac;
use nom::{
    character::complete::{alphanumeric0, char},
    combinator::{fail, success},
    sequence::{terminated, Tuple},
    IResult,
};
use rand_core::CryptoRngCore;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Sha384, Sha512};

use crate::idp::AuthResult;

use super::base64_standard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum HkdfAlgorithmTag {
    Sha256,
    Sha512,
    Sha384,
}

pub fn hkdf_algorithm_tag(input: &str) -> IResult<&str, HkdfAlgorithmTag> {
    let (input, algo) = alphanumeric0(input)?;
    match algo {
        "sha256" => success(HkdfAlgorithmTag::Sha256)(input),
        "sha512" => success(HkdfAlgorithmTag::Sha512)(input),
        "sha384" => success(HkdfAlgorithmTag::Sha384)(input),
        _ => fail(input),
    }
}

impl Display for HkdfAlgorithmTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            HkdfAlgorithmTag::Sha256 => "sha256",
            HkdfAlgorithmTag::Sha512 => "sha512",
            HkdfAlgorithmTag::Sha384 => "sha384",
        })
    }
}

fn hkdf_expand<H: OutputSizeUser, I: HmacImpl<H>>(
    ikm: &[u8],
    salt: Option<&[u8]>,
    password: &[u8],
) -> Vec<u8> {
    let hk = Hkdf::<H, I>::new(salt, ikm);
    let mut okm = Vec::new();
    okm.reserve(H::output_size());
    hk.expand(password, &mut okm).unwrap();
    okm
}

impl HkdfAlgorithmTag {
    pub fn expand(
        &self,
        ikm: &[u8],
        salt: Option<&[u8]>,
        password: &[u8],
    ) -> Vec<u8> {
        match self {
            HkdfAlgorithmTag::Sha256 => {
                hkdf_expand::<Sha256, Hmac<_>>(ikm, salt, password)
            }
            HkdfAlgorithmTag::Sha512 => {
                hkdf_expand::<Sha512, Hmac<_>>(ikm, salt, password)
            }
            HkdfAlgorithmTag::Sha384 => {
                hkdf_expand::<Sha384, Hmac<_>>(ikm, salt, password)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HkdfPasswordSecret {
    pub algorithm: HkdfAlgorithmTag,
    pub ikm: Bytes,
    pub salt: Option<Bytes>,
    pub okm: Bytes,
}

impl HkdfPasswordSecret {
    pub fn new(
        algorithm: HkdfAlgorithmTag,
        password: &str,
        rng: &mut dyn CryptoRngCore,
    ) -> Self {
        let mut ikm = vec![0u8; 32];
        let mut salt = vec![0u8; 32];
        rng.fill_bytes(&mut ikm);
        rng.fill_bytes(&mut salt);

        let okm = algorithm.expand(&ikm, Some(&salt), password.as_bytes());
        Self {
            algorithm,
            ikm: Bytes::from(ikm),
            salt: Some(Bytes::from(salt)),
            okm: Bytes::from(okm),
        }
    }

    pub fn authenticate(&self, password: &str) -> AuthResult<bool> {
        let okm = self.algorithm.expand(
            &self.ikm,
            self.salt.as_ref().map(|s| &**s),
            password.as_bytes(),
        );
        Ok(okm.as_slice() == &self.okm)
    }
}

pub fn hkdf_password_secret(input: &str) -> IResult<&str, HkdfPasswordSecret> {
    let (input, (algorithm, ikm, salt, okm)) = (
        terminated(hkdf_algorithm_tag, char(':')),
        terminated(base64_standard, char(':')),
        terminated(base64_standard, char(':')),
        base64_standard,
    )
        .parse(input)?;
    let salt = Some(salt).take_if(|s| !s.is_empty());
    success(HkdfPasswordSecret {
        algorithm,
        ikm,
        salt,
        okm,
    })(input)
}

impl Display for HkdfPasswordSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:{}:{}:{}",
            self.algorithm,
            BASE64_STANDARD_NO_PAD.encode(&self.ikm),
            self.salt
                .as_ref()
                .map(|s| BASE64_STANDARD_NO_PAD.encode(s))
                .unwrap_or_default(),
            BASE64_STANDARD_NO_PAD.encode(&self.okm)
        ))
    }
}
