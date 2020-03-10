#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct RdfLangString {
    #[serde(rename = "@value")]
    pub value: String,

    #[serde(rename = "@language")]
    pub language: String,
}

impl std::fmt::Display for RdfLangString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.language, self.value)
    }
}
