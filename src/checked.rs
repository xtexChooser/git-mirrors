use iri_string::types::{IriStr, IriString};

#[derive(Clone, Debug)]
pub struct CheckError(pub(crate) Option<IriString>);

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(iri) = &self.0 {
            write!(f, "IRI failed host and port check: {}", iri)
        } else {
            write!(f, "IRI missing")
        }
    }
}

impl std::error::Error for CheckError {}

pub(crate) fn check<T: AsRef<IriStr>>(
    iri: T,
    host: &str,
    port: Option<&str>,
) -> Result<T, CheckError> {
    let authority = iri
        .as_ref()
        .authority_components()
        .ok_or(CheckError(Some(iri.as_ref().to_owned())))?;

    if authority.host() != host || authority.port() != port {
        return Err(CheckError(Some(iri.as_ref().to_owned())));
    }

    Ok(iri)
}
