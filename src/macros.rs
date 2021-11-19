/// A macro to shorten the `string.parse::<Url>()?` calls inevitably made in downstream code
///
/// ```rust
/// use activitystreams::uri;
///
/// fn fallible() -> Result<(), anyhow::Error> {
///     let my_uri = uri!("https://example.com");
///     Ok(())
/// }
///
/// # fn main() -> Result<(), anyhow::Error> { fallible() }
/// ```
#[macro_export]
macro_rules! uri {
    ( $x:expr ) => {{
        use activitystreams::url::Url;

        $x.parse::<Url>()?
    }};
}
