#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(transparent)]
pub struct XsdNonNegativeInteger(u64);

#[derive(Clone, Debug, thiserror::Error)]
#[error("Error parsing NonNegativeInteger")]
pub struct XsdNonNegativeIntegerError;

impl AsRef<u64> for XsdNonNegativeInteger {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl From<XsdNonNegativeInteger> for u64 {
    fn from(i: XsdNonNegativeInteger) -> Self {
        i.0
    }
}

impl std::convert::TryFrom<u64> for XsdNonNegativeInteger {
    type Error = XsdNonNegativeIntegerError;

    fn try_from(f: u64) -> Result<Self, Self::Error> {
        Ok(XsdNonNegativeInteger(f))
    }
}

impl std::convert::TryFrom<String> for XsdNonNegativeInteger {
    type Error = XsdNonNegativeIntegerError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for XsdNonNegativeInteger {
    type Error = XsdNonNegativeIntegerError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for XsdNonNegativeInteger {
    type Error = XsdNonNegativeIntegerError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for XsdNonNegativeInteger {
    type Err = XsdNonNegativeIntegerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = s.parse().map_err(|_| XsdNonNegativeIntegerError)?;
        Ok(XsdNonNegativeInteger(f))
    }
}

impl std::fmt::Display for XsdNonNegativeInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
