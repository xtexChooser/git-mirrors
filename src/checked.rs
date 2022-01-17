use iri_string::types::IriStr;

#[derive(Clone, Debug)]
pub struct CheckError;

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IRI failed host and port check")
    }
}

impl std::error::Error for CheckError {}

pub(crate) fn check<T: AsRef<IriStr>>(
    iri: T,
    host: &str,
    port: Option<&str>,
) -> Result<T, CheckError> {
    let authority = iri.as_ref().authority_components().ok_or(CheckError)?;

    if authority.host() != host || authority.port() != port {
        return Err(CheckError);
    }

    Ok(iri)
}
