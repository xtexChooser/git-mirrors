/// Generate an enum implementing serde's Serialize and Deserialize with a single variant
///
/// This is useful for describing constants
///
/// ```rust
/// # fn main() -> Result<(), anyhow::Error> {
/// use activitystreams::kind;
///
/// kind!(CustomType, Custom);
///
/// #[derive(serde::Deserialize)]
/// struct MyStruct {
///     #[serde(rename = "type")]
///     kind: CustomType,
/// }
///
/// let s: MyStruct = serde_json::from_str(r#"{"type":"Custom"}"#)?;
///
/// assert_eq!(s.kind, CustomType::Custom);
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! kind {
    ($x:ident, $y:ident) => {
        #[derive(
            Clone,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            serde::Deserialize,
            serde::Serialize,
        )]
        /// A type stand-in for the constant $y, deriving serde traits
        pub enum $x {
            $y,
        }

        impl std::fmt::Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, stringify!($y))
            }
        }

        impl Default for $x {
            fn default() -> Self {
                $x::$y
            }
        }
    };
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn to_string_works() {
        kind!(MyType, My);

        assert_eq!(MyType::My.to_string(), "My")
    }
}
