#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("Unrecognizable HTML: {0} {1:?}")]
    MalformedHTML(&'static str, Option<String>),
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::MalformedHTML("Unparsable integer: ", Some(format!("{}", value)))
    }
}

impl From<time::error::ComponentRange> for Error {
    fn from(value: time::error::ComponentRange) -> Self {
        Self::MalformedHTML(
            "Illegal time component: ",
            Some(format!("{}", value)),
        )
    }
}
