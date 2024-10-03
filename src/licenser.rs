use std::fs;

use anyhow::Result;
use ed25519_dalek::{pkcs8::DecodePrivateKey, SigningKey};
use educe::Educe;
use yjyz_tools::license::{v1, License, LicenseFeatures};

#[derive(Educe)]
#[educe(Default)]
pub struct LicenserWindow {
    #[educe(Default = LicenseFeatures::empty())]
    features: LicenseFeatures,
}

impl LicenserWindow {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Result<()> {
        for (name, flag) in LicenseFeatures::all().iter_names() {
            let mut state = self.features.contains(flag);
            ui.checkbox(&mut state, name);
            self.features.set(flag, state);
        }
        if ui.button("创建").clicked() {
            let mut sign_key = SigningKey::from_pkcs8_pem(&fs::read_to_string("yjyz-tools.pem")?)?;

            let claims = v1::LicenseClaims::from(self.features);
            let license = License::V1(claims.sign(&mut sign_key)?);
            fs::write(".yjyz-tools.lic", license.to_bytes()?)?;
        }
        Ok(())
    }
}
