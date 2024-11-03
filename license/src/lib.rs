use std::{fs, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum License {
    V1(v1::License),
}

impl License {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize(bytes)?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(self)?)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::from_bytes(&fs::read(path)?)
    }

    pub fn verify(&self) -> bool {
        match self {
            License::V1(license) => license.verify(),
        }
    }

    pub fn to_latest_claims(&self) -> Result<LatestLicenseClaims> {
        match self {
            License::V1(license) => license.to_claims(),
        }
    }
}

pub mod v1 {
    use anyhow::{bail, Result};
    use bitflags::bitflags;
    use ed25519_dalek::{
        ed25519::signature::SignerMut, Signature, SigningKey, Verifier, VerifyingKey,
    };
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct License {
        #[serde(with = "serde_bytes")]
        pub claims: Vec<u8>,
        #[serde(with = "serde_bytes")]
        pub signature: [u8; 64],
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LicenseClaims {
        pub magic: u32,
        #[serde(with = "uuid::serde::compact")]
        pub id: Uuid,
        pub features: LicenseFeatures,
    }

    pub const CLAIMS_MAGIC: u32 = 0xe2f84969;
    pub const PUBLIC_KEY: [u8; 32] =
        hex_literal::hex!("539fde6154abb6b525786ba527fc051844005064ef07aae74e6ff2a1860ae8e1");

    bitflags! {
        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct LicenseFeatures: u128 {
            const SUDOER = 1 << 0;
            const TRAIL = 1 << 1;

            const NO_TELEMETRY = 1 << 2; //
            const NO_UPDATE = 1 << 3; //
            const MUST_UPDATE = 1 << 4; //
            const MUST_ONLINE = 1 << 5; //
            const NO_SECURITY_CHECK = 1 << 6;
            const ALLOW_INSECURE = 1 << 7;
            const SHOW_SOURCE_LINK = 1 << 8;
            const SHOW_CONSOLE = 1 << 9;

            const MYTHWARE_ALLOW_TEACHER = 1 << 16;
            const MYTHWARE_PASSWORD = 1 << 17;
            const MYTHWARE_WINDOWING = 1 << 18;
            const MYTHWARE_STOPPING = 1 << 19;
            const MYTHWARE_SUSPENDING = 1 << 20;

            const POWERSHADOW_PASSWORD = 1 << 32;
        }
    }

    pub const SUDOER_RIGHTS: &dyn Fn() -> LicenseFeatures = &|| {
        LicenseFeatures::SUDOER
            | LicenseFeatures::SHOW_SOURCE_LINK
            | LicenseFeatures::SHOW_CONSOLE
            | LicenseFeatures::MYTHWARE_PASSWORD
            | LicenseFeatures::MYTHWARE_WINDOWING
            | LicenseFeatures::MYTHWARE_STOPPING
            | LicenseFeatures::MYTHWARE_SUSPENDING
            | LicenseFeatures::POWERSHADOW_PASSWORD
    };

    pub const TRAIL_RIGHTS: &dyn Fn() -> LicenseFeatures = &|| {
        LicenseFeatures::TRAIL
            | LicenseFeatures::MUST_UPDATE
            | LicenseFeatures::MYTHWARE_WINDOWING
            | LicenseFeatures::MYTHWARE_STOPPING
    };

    impl LicenseClaims {
        pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
            let claims = bincode::deserialize::<LicenseClaims>(bytes)?;
            if claims.magic != CLAIMS_MAGIC {
                bail!("Invalid v1 claims magic");
            }
            Ok(claims)
        }

        pub fn to_bytes(&self) -> Result<Vec<u8>> {
            Ok(bincode::serialize(self)?)
        }

        pub fn sign(&self, key: &mut SigningKey) -> Result<License> {
            let claims = self.to_bytes()?;
            Ok(License {
                signature: key.try_sign(&claims)?.to_bytes(),
                claims,
            })
        }
    }

    impl From<LicenseFeatures> for LicenseClaims {
        fn from(value: LicenseFeatures) -> Self {
            Self {
                magic: CLAIMS_MAGIC,
                id: Uuid::now_v7(),
                features: value,
            }
        }
    }

    impl License {
        pub fn verify(&self) -> bool {
            self.verify_with(VerifyingKey::from_bytes(&PUBLIC_KEY).unwrap())
        }

        pub fn verify_with(&self, key: VerifyingKey) -> bool {
            key.verify(&self.claims, &Signature::from_bytes(&self.signature))
                .is_ok()
        }

        pub fn to_claims(&self) -> Result<LicenseClaims> {
            LicenseClaims::from_bytes(&self.claims)
        }
    }
}

pub use v1::License as LatestLicense;
pub use v1::LicenseClaims as LatestLicenseClaims;
pub use v1::LicenseFeatures as FeatureFlags;
