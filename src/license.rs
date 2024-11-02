use std::{
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use anyhow::Result;
use log::{info, warn};
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

    pub fn to_claims(&self) -> Result<v1::LicenseClaims> {
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
            | LicenseFeatures::MYTHWARE_PASSWORD
            | LicenseFeatures::MYTHWARE_WINDOWING
            | LicenseFeatures::MYTHWARE_STOPPING
            | LicenseFeatures::MYTHWARE_SUSPENDING
            | LicenseFeatures::POWERSHADOW_PASSWORD
    };

    pub const TRAIL_RIGHTS: &dyn Fn() -> LicenseFeatures = &|| {
        LicenseFeatures::TRAIL
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

pub use v1::LicenseFeatures;

pub fn find_licenses() -> Vec<PathBuf> {
    const LICENSE_NAMES: &[&str] = &[
        ".yjyz-tools.lic",
        ".license.key",
        "yjyz-tools.lic",
        "yjyz-tools.key",
        "yjyz/yjyz-tools.lic",
        "yjyz-tools/yjyz-tools.lic",
        "license.key",
        "license",
        "license.bin",
        "maint/keys/sudo",
    ];

    let mut dirs = Vec::from([PathBuf::from("."), PathBuf::from("C:\\")]);
    for disk in sysinfo::Disks::new_with_refreshed_list().list() {
        dirs.push(disk.mount_point().to_path_buf());
    }
    let mut files = Vec::new();
    for dir in dirs {
        for name in LICENSE_NAMES {
            let path = dir.join(name);
            if path.exists() {
                files.push(path);
            }
        }
    }
    files
}

pub fn load_licenses() -> Vec<v1::LicenseClaims> {
    let mut licenses = Vec::new();
    for path in find_licenses() {
        info!("Loading license from: {}", path.display());
        match License::from_file(&path) {
            Ok(license) => {
                if license.verify() {
                    match license.to_claims() {
                        Ok(claims) => {
                            info!("Loaded license {}", claims.id);
                            licenses.push(claims);
                        }
                        Err(err) => warn!("Failed to upgrade license: {:?}", err),
                    }
                } else {
                    warn!("Unsigned license: {}", path.display())
                }
            }
            Err(err) => {
                warn!("Malformed license at {}: {:?}", path.display(), err)
            }
        }
    }
    licenses
}

pub static LICENSES: LazyLock<Vec<v1::LicenseClaims>> = LazyLock::new(load_licenses);
pub static IS_SUDOER: LazyLock<bool> = LazyLock::new(|| {
    LICENSES
        .iter()
        .any(|claims| claims.features.contains(v1::LicenseFeatures::SUDOER))
});
pub static IS_TRAIL: LazyLock<bool> = LazyLock::new(|| {
    LICENSES
        .iter()
        .all(|claims| claims.features.contains(v1::LicenseFeatures::TRAIL))
});

pub fn is_set(flags: LicenseFeatures) -> bool {
    if *IS_SUDOER {
        return v1::SUDOER_RIGHTS().contains(flags);
    }
    if *IS_TRAIL {
        return v1::TRAIL_RIGHTS().contains(flags);
    }
    LICENSES
        .iter()
        .any(|claims| claims.features.contains(flags))
}
