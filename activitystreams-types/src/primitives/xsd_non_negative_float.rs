#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(transparent)]
pub struct XsdNonNegativeFloat(f64);

#[derive(Clone, Debug, thiserror::Error)]
#[error("Error parsing NonNegativeFloat")]
pub struct XsdNonNegativeFloatError;

impl AsRef<f64> for XsdNonNegativeFloat {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl From<XsdNonNegativeFloat> for f64 {
    fn from(f: XsdNonNegativeFloat) -> Self {
        f.0
    }
}

impl std::convert::TryFrom<f64> for XsdNonNegativeFloat {
    type Error = XsdNonNegativeFloatError;

    fn try_from(f: f64) -> Result<Self, Self::Error> {
        if f < 0.0 {
            return Err(XsdNonNegativeFloatError);
        }

        Ok(XsdNonNegativeFloat(f))
    }
}

impl std::convert::TryFrom<String> for XsdNonNegativeFloat {
    type Error = XsdNonNegativeFloatError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for XsdNonNegativeFloat {
    type Error = XsdNonNegativeFloatError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for XsdNonNegativeFloat {
    type Error = XsdNonNegativeFloatError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for XsdNonNegativeFloat {
    type Err = XsdNonNegativeFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = s.parse().map_err(|_| XsdNonNegativeFloatError)?;
        if f < 0.0 {
            return Err(XsdNonNegativeFloatError);
        }
        Ok(XsdNonNegativeFloat(f))
    }
}

impl std::fmt::Display for XsdNonNegativeFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
