use std::env::args;

use anyhow::Result;
use yjyz_tools::license::License;

fn main() -> Result<()> {
    let path = args()
        .collect::<Vec<_>>()
        .get(1)
        .cloned()
        .unwrap_or_else(|| "maint/keys/sudo".to_string());

    println!("License @ {}", path);
    let license = License::from_file(path)?;
    if license.verify() {
        println!("- Signature verified")
    } else {
        println!("- SIGNATURE INVALID")
    }
    let claims = license.to_claims()?;
    println!("{:#?}", claims);

    Ok(())
}
