#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(transparent)]
pub struct XsdString(String);

impl From<String> for XsdString {
    fn from(s: String) -> Self {
        XsdString(s)
    }
}

impl From<&str> for XsdString {
    fn from(s: &str) -> Self {
        XsdString(s.to_owned())
    }
}

impl From<&mut str> for XsdString {
    fn from(s: &mut str) -> Self {
        XsdString(s.to_owned())
    }
}

impl From<XsdString> for String {
    fn from(s: XsdString) -> Self {
        s.0
    }
}

impl std::fmt::Display for XsdString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
