use std::{fs, path::PathBuf};

use anyhow::Result;
use ed25519_dalek::{
    pkcs8::{spki::der::pem::LineEnding, EncodePrivateKey, EncodePublicKey},
    SigningKey, VerifyingKey,
};
use rand::rngs::OsRng;

fn main() -> Result<()> {
    let sign_key = SigningKey::generate(&mut OsRng);
    let verify_key = VerifyingKey::from(&sign_key);

    fs::create_dir_all("maint/keys")?;
    fs::write("maint/keys/private.key", sign_key.as_bytes())?;
    fs::write("maint/keys/public.key", verify_key.as_bytes())?;
    fs::write(
        "maint/keys/private.pem",
        sign_key.to_pkcs8_pem(LineEnding::LF)?,
    )?;
    fs::write(
        "maint/keys/public.pem",
        verify_key.to_public_key_pem(LineEnding::LF)?,
    )?;

    println!("Keys written to {}", std::path::absolute(PathBuf::from("maint/keys"))?.display());
    println!("Public key hex: {}", hex::encode(verify_key.to_bytes()));

    Ok(())
}
