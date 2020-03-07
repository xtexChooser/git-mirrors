#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(transparent)]
pub struct XsdAnyURI(String);

#[derive(Clone, Debug, thiserror::Error)]
#[error("Could not parse XsdAnyURI")]
pub struct XsdAnyURIError;

impl std::convert::TryFrom<String> for XsdAnyURI {
    type Error = XsdAnyURIError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(XsdAnyURI(s))
    }
}

impl std::convert::TryFrom<&str> for XsdAnyURI {
    type Error = XsdAnyURIError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(XsdAnyURI(s.to_owned()))
    }
}

impl std::convert::TryFrom<&mut str> for XsdAnyURI {
    type Error = XsdAnyURIError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        Ok(XsdAnyURI(s.to_owned()))
    }
}

impl std::str::FromStr for XsdAnyURI {
    type Err = XsdAnyURIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(XsdAnyURI(s.to_owned()))
    }
}

impl std::fmt::Display for XsdAnyURI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
