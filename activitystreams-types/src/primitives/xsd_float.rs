#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(transparent)]
pub struct XsdFloat(f64);

#[derive(Clone, Debug, thiserror::Error)]
#[error("Error parsing Float")]
pub struct XsdFloatError;

impl AsRef<f64> for XsdFloat {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl AsMut<f64> for XsdFloat {
    fn as_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

impl From<f64> for XsdFloat {
    fn from(f: f64) -> Self {
        XsdFloat(f)
    }
}

impl From<XsdFloat> for f64 {
    fn from(f: XsdFloat) -> Self {
        f.0
    }
}

impl std::convert::TryFrom<String> for XsdFloat {
    type Error = XsdFloatError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for XsdFloat {
    type Error = XsdFloatError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for XsdFloat {
    type Error = XsdFloatError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for XsdFloat {
    type Err = XsdFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(XsdFloat(s.parse().map_err(|_| XsdFloatError)?))
    }
}

impl std::fmt::Display for XsdFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
