#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct XsdAnyUri(String);

#[derive(Clone, Debug, thiserror::Error)]
#[error("Could not parse XsdAnyUri")]
pub struct XsdAnyUriError;

impl std::convert::TryFrom<String> for XsdAnyUri {
    type Error = XsdAnyUriError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(XsdAnyUri(s))
    }
}

impl std::convert::TryFrom<&str> for XsdAnyUri {
    type Error = XsdAnyUriError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(XsdAnyUri(s.to_owned()))
    }
}

impl std::convert::TryFrom<&mut str> for XsdAnyUri {
    type Error = XsdAnyUriError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        Ok(XsdAnyUri(s.to_owned()))
    }
}

impl std::str::FromStr for XsdAnyUri {
    type Err = XsdAnyUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(XsdAnyUri(s.to_owned()))
    }
}

impl std::fmt::Display for XsdAnyUri {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
