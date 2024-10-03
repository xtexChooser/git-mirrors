use std::fs;

use anyhow::Result;
use ed25519_dalek::{pkcs8::DecodePrivateKey, SigningKey};
use yjyz_tools::license::{v1, License};

fn main() -> Result<()> {
    let mut sign_key = SigningKey::from_pkcs8_pem(&fs::read_to_string("maint/keys/private.pem")?)?;
    let claims = v1::LicenseClaims::from(v1::LicenseFeatures::SUDOER);
    let license = License::V1(claims.sign(&mut sign_key)?);

    fs::write("maint/keys/sudo", license.to_bytes()?)?;
    println!("License written to maint/keys/sudo");

    Ok(())
}
