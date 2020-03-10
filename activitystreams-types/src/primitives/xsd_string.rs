#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
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

impl AsRef<str> for XsdString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for XsdString {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsMut<str> for XsdString {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl AsMut<String> for XsdString {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl std::fmt::Display for XsdString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
