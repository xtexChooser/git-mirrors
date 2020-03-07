#[derive(Clone, Debug)]
pub struct XsdDateTime(chrono::DateTime<chrono::Utc>);

#[derive(Clone, Debug, thiserror::Error)]
#[error("Error parsing DateTime")]
pub struct XsdDateTimeError;

impl From<chrono::DateTime<chrono::Utc>> for XsdDateTime {
    fn from(d: chrono::DateTime<chrono::Utc>) -> Self {
        XsdDateTime(d)
    }
}

impl From<XsdDateTime> for chrono::DateTime<chrono::Utc> {
    fn from(d: XsdDateTime) -> Self {
        d.0
    }
}

impl AsRef<chrono::DateTime<chrono::Utc>> for XsdDateTime {
    fn as_ref(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.0
    }
}

impl AsMut<chrono::DateTime<chrono::Utc>> for XsdDateTime {
    fn as_mut(&mut self) -> &mut chrono::DateTime<chrono::Utc> {
        &mut self.0
    }
}

impl std::convert::TryFrom<String> for XsdDateTime {
    type Error = XsdDateTimeError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for XsdDateTime {
    type Error = XsdDateTimeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for XsdDateTime {
    type Error = XsdDateTimeError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for XsdDateTime {
    type Err = XsdDateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(XsdDateTime(
            chrono::DateTime::parse_from_rfc3339(s)
                .map_err(|_| XsdDateTimeError)?
                .into(),
        ))
    }
}

impl std::fmt::Display for XsdDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self.0.to_rfc3339();
        std::fmt::Display::fmt(&s, f)
    }
}

impl serde::ser::Serialize for XsdDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for XsdDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
