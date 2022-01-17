use iri_string::types::IriStr;

#[derive(Debug, thiserror::Error)]
#[error("IRI failed host and port check")]
pub struct CheckError;

pub(crate) fn check<'a, T: AsRef<IriStr>>(
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
