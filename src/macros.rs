/// A macro to shorten the `string.parse::<Url>()?` calls inevitably made in downstream code
///
/// ```rust
/// use activitystreams::iri;
///
/// fn fallible() -> Result<(), anyhow::Error> {
///     let my_iri = iri!("https://example.com");
///     Ok(())
/// }
///
/// # fn main() -> Result<(), anyhow::Error> { fallible() }
/// ```
#[macro_export]
macro_rules! iri {
    ( $x:expr ) => {{
        use activitystreams::iri_string::types::IriString;

        $x.parse::<IriString>()?
    }};
}

/// A macro to parse IRI fragments
///
/// ```rust
/// use activitystreams::fragment;
///
/// fn fallible() -> Result<(), anyhow::Error> {
///     let my_fragment = fragment!("main-key");
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! fragment {
    ( $x:expr ) => {{
        use activitystreams::iri_string::types::IriFragmentString;

        $x.parse::<IriFragmentString>()?
    }};
}

/// A macro to parse Rfc3339 datetimes
///
/// ```
/// use activitystreams::datetime;
///
/// fn fallible() -> Result<(), anyhow::Error> {
///     let my_datetime = datetime!("2020-04-20T04:20:00Z");
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! datetime {
    ( $x:expr ) => {{
        use activitystreams::time::{format_description::well_known::Rfc3339, OffsetDateTime};

        OffsetDateTime::parse($x, &Rfc3339)?
    }};
}
