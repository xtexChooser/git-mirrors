#[derive(Clone, Debug)]
pub struct MimeMediaType(mime::Mime);

#[derive(Clone, Debug, thiserror::Error)]
#[error("Error parsing MIME")]
pub struct MimeMediaTypeError;

impl From<mime::Mime> for MimeMediaType {
    fn from(m: mime::Mime) -> Self {
        MimeMediaType(m)
    }
}

impl From<MimeMediaType> for mime::Mime {
    fn from(m: MimeMediaType) -> Self {
        m.0
    }
}

impl AsRef<mime::Mime> for MimeMediaType {
    fn as_ref(&self) -> &mime::Mime {
        &self.0
    }
}

impl AsMut<mime::Mime> for MimeMediaType {
    fn as_mut(&mut self) -> &mut mime::Mime {
        &mut self.0
    }
}

impl std::convert::TryFrom<String> for MimeMediaType {
    type Error = MimeMediaTypeError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for MimeMediaType {
    type Error = MimeMediaTypeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for MimeMediaType {
    type Error = MimeMediaTypeError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for MimeMediaType {
    type Err = MimeMediaTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MimeMediaType(s.parse().map_err(|_| MimeMediaTypeError)?))
    }
}

impl serde::ser::Serialize for MimeMediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for MimeMediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
