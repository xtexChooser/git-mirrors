use anyhow::{bail, Result};
use axum_auth::AuthBasic;

use crate::{config::get_config, role::Role};

pub async fn handle_auth(AuthBasic((id, signature)): AuthBasic) -> Result<&'static Role> {
    match signature {
        None => bail!("no signature provided"),
        Some(signature) => {
            if let Some(role) = Role::get(&id) {
                if signature == role.auth {
                    if role.otp_required.unwrap_or(get_config().otp_required) {
                        bail!("OTP authentication is required")
                    }
                    return Ok(role);
                }
                let totp = role.to_totp()?;
                if totp.check_current(&signature)? {
                    Ok(role)
                } else {
                    bail!("invalid TOTP token")
                }
            } else {
                bail!(format!("role {id} not found"))
            }
        }
    }
}
