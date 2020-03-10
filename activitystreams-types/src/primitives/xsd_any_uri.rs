#[derive(Clone, Debug)]
pub struct XsdAnyUri(url::Url);

#[derive(Clone, Debug, thiserror::Error)]
#[error("Could not parse XsdAnyUri")]
pub struct XsdAnyUriError;

impl Default for XsdAnyUri {
    fn default() -> Self {
        "data:text/plain,uwu".parse().unwrap()
    }
}

impl std::convert::TryFrom<String> for XsdAnyUri {
    type Error = XsdAnyUriError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for XsdAnyUri {
    type Error = XsdAnyUriError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for XsdAnyUri {
    type Error = XsdAnyUriError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for XsdAnyUri {
    type Err = XsdAnyUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(XsdAnyUri(s.parse().map_err(|_| XsdAnyUriError)?))
    }
}

impl std::fmt::Display for XsdAnyUri {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl serde::ser::Serialize for XsdAnyUri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for XsdAnyUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
