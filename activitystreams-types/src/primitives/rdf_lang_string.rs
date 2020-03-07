#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct RDFLangString {
    #[serde(rename = "@value")]
    pub value: String,

    #[serde(rename = "@language")]
    pub language: String,
}

impl std::fmt::Display for RDFLangString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.language, self.value)
    }
}
