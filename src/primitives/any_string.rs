use crate::{
    either::Either,
    primitives::{OneOrMany, RdfLangString},
};

/// A type representing any kind of string
///
/// In the ActivityStreams specification, string types are often defined as either an xsd:String or
/// and rdf:langString. The AnyString type represents this union.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct AnyString(Either<String, RdfLangString>);

impl AnyString {
    /// Borrow the AnyString as an &str
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::primitives::AnyString;
    /// # let any_string = AnyString::from_xsd_string("hi");
    /// #
    /// let s_borrow = any_string
    ///     .as_xsd_string()
    ///     .ok_or(anyhow::Error::msg("Wrong string type"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_xsd_string(&self) -> Option<&str> {
        self.0.as_ref().left().map(|l| l.as_str())
    }

    /// Borrow the AnyString as an RdfLangString
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::primitives::{AnyString, RdfLangString};
    /// # let any_string = AnyString::from_rdf_lang_string(RdfLangString {
    /// #     value: "hi".into(),
    /// #     language: "en".into(),
    /// # });
    /// #
    /// let s_borrow = any_string
    ///     .as_rdf_lang_string()
    ///     .ok_or(anyhow::Error::msg("Wrong string type"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_rdf_lang_string(&self) -> Option<&RdfLangString> {
        self.0.as_ref().right()
    }

    /// Take the AnyString as a String
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::primitives::AnyString;
    /// # let any_string = AnyString::from_xsd_string("hi");
    /// #
    /// let xsd_string = any_string
    ///     .xsd_string()
    ///     .ok_or(anyhow::Error::msg("Wrong string type"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn xsd_string(self) -> Option<String> {
        self.0.left()
    }

    /// Take the AnyString as an RdfLangString
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::primitives::{AnyString, RdfLangString};
    /// # let any_string = AnyString::from_rdf_lang_string(RdfLangString {
    /// #     value: "hi".into(),
    /// #     language: "en".into(),
    /// # });
    /// #
    /// let rdf_lang_string = any_string
    ///     .rdf_lang_string()
    ///     .ok_or(anyhow::Error::msg("Wrong string type"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn rdf_lang_string(self) -> Option<RdfLangString> {
        self.0.right()
    }

    /// Create a new AnyString from an `Into<String>`
    ///
    /// ```rust
    /// use activitystreams::primitives::AnyString;
    ///
    /// let any_string = AnyString::from_xsd_string("hi");
    /// ```
    pub fn from_xsd_string<T>(string: T) -> Self
    where
        T: Into<String>,
    {
        AnyString(Either::Left(string.into()))
    }

    /// Create a new AnyString from an RdfLangString
    ///
    /// ```rust
    /// use activitystreams::primitives::{AnyString, RdfLangString};
    ///
    /// let any_string = AnyString::from_rdf_lang_string(RdfLangString {
    ///     value: "hi".into(),
    ///     language: "en".into(),
    /// });
    /// ```
    pub fn from_rdf_lang_string<T>(string: T) -> Self
    where
        T: Into<RdfLangString>,
    {
        AnyString(Either::Right(string.into()))
    }

    /// Replace the contents of self with a String
    ///
    /// ```rust
    /// use activitystreams::primitives::{AnyString, RdfLangString};
    ///
    /// let mut any_string = AnyString::from_rdf_lang_string(RdfLangString {
    ///     value: "hi".into(),
    ///     language: "en".into(),
    /// });
    ///
    /// any_string.set_xsd_string("hi");
    ///
    /// assert!(any_string.as_xsd_string().is_some());
    /// ```
    pub fn set_xsd_string<T>(&mut self, string: T)
    where
        T: Into<String>,
    {
        self.0 = Either::Left(string.into());
    }

    /// Replace the contents of self with an RdfLangString
    ///
    /// ```rust
    /// use activitystreams::primitives::{AnyString, RdfLangString};
    ///
    /// let mut any_string = AnyString::from_xsd_string("hi");
    ///
    /// any_string.set_rdf_lang_string(RdfLangString {
    ///     value: "hi".into(),
    ///     language: "en".into(),
    /// });
    ///
    /// assert!(any_string.as_rdf_lang_string().is_some());
    /// ```
    pub fn set_rdf_lang_string<T>(&mut self, string: T)
    where
        T: Into<RdfLangString>,
    {
        self.0 = Either::Right(string.into());
    }
}

impl OneOrMany<AnyString> {
    /// Try to borrow a single String from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::primitives::{OneOrMany, AnyString};
    /// # let string = OneOrMany::<AnyString>::from_xsd_string("Hey");
    /// string
    ///     .as_single_xsd_string()
    ///     .ok_or(anyhow::Error::msg("Wrong string type"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_single_xsd_string(&self) -> Option<&str> {
        self.as_one()
            .and_then(|any_string| any_string.as_xsd_string())
    }

    /// Try to borrow a single RdfLangString from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::primitives::{OneOrMany, RdfLangString};
    /// # let string = OneOrMany::from_rdf_lang_string(RdfLangString {
    /// #   value: "hi".into(),
    /// #   language: "en".into(),
    /// # });
    /// string
    ///     .as_single_rdf_lang_string()
    ///     .ok_or(anyhow::Error::msg("Wrong string type"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_single_rdf_lang_string(&self) -> Option<&RdfLangString> {
        self.as_one()
            .and_then(|any_string| any_string.as_rdf_lang_string())
    }

    /// Try to take a single String from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::primitives::{OneOrMany, AnyString};
    /// # let string = OneOrMany::<AnyString>::from_xsd_string("Hey");
    /// string
    ///     .single_xsd_string()
    ///     .ok_or(anyhow::Error::msg("Wrong string type"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn single_xsd_string(self) -> Option<String> {
        self.one().and_then(|any_string| any_string.xsd_string())
    }

    /// Try to take a single RdfLangString from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::primitives::{OneOrMany, RdfLangString};
    /// # let string = OneOrMany::from_rdf_lang_string(RdfLangString {
    /// #   value: "hi".into(),
    /// #   language: "en".into(),
    /// # });
    /// string
    ///     .single_rdf_lang_string()
    ///     .ok_or(anyhow::Error::msg("Wrong string type"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn single_rdf_lang_string(self) -> Option<RdfLangString> {
        self.one()
            .and_then(|any_string| any_string.rdf_lang_string())
    }

    /// Create the object from a single String
    ///
    /// ```rust
    /// use activitystreams::primitives::{OneOrMany, AnyString};
    ///
    /// let string = OneOrMany::<AnyString>::from_xsd_string("hi");
    /// ```
    pub fn from_xsd_string<T>(string: T) -> Self
    where
        T: Into<String>,
    {
        Self::from_one(AnyString::from_xsd_string(string))
    }

    /// Create the object from a single RdfLangString
    ///
    /// ```rust
    /// use activitystreams::primitives::{OneOrMany, RdfLangString};
    ///
    /// let string = OneOrMany::from_rdf_lang_string(RdfLangString {
    ///     value: "hi".into(),
    ///     language: "en".into(),
    /// });
    /// ```
    pub fn from_rdf_lang_string<T>(string: T) -> Self
    where
        T: Into<RdfLangString>,
    {
        Self::from_one(AnyString::from_rdf_lang_string(string))
    }

    /// Add a String to the object, appending to whatever is currently included
    ///
    /// ```rust
    /// use activitystreams::primitives::{OneOrMany, AnyString};
    ///
    /// let mut string = OneOrMany::<AnyString>::from_xsd_string("Hello");
    ///
    /// string
    ///     .add_xsd_string("Hey")
    ///     .add_xsd_string("hi");
    /// ```
    pub fn add_xsd_string<T>(&mut self, string: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.add(string.into())
    }

    /// Add an RdfLangString to the object, appending to whatever is currently included
    ///
    /// ```rust
    /// use activitystreams::primitives::{AnyString, OneOrMany, RdfLangString};
    ///
    /// let mut string = OneOrMany::<AnyString>::from_xsd_string("Hello");
    ///
    /// string
    ///     .add_rdf_lang_string(RdfLangString {
    ///         value: "Hey".into(),
    ///         language: "en".into(),
    ///     })
    ///     .add_rdf_lang_string(RdfLangString {
    ///         value: "hi".into(),
    ///         language: "en".into(),
    ///     });
    /// ```
    pub fn add_rdf_lang_string<T>(&mut self, string: T) -> &mut Self
    where
        T: Into<RdfLangString>,
    {
        self.add(string.into())
    }
}

impl From<&str> for AnyString {
    fn from(s: &str) -> Self {
        AnyString::from_xsd_string(s.to_owned())
    }
}

impl From<String> for AnyString {
    fn from(s: String) -> Self {
        AnyString::from_xsd_string(s)
    }
}

impl From<RdfLangString> for AnyString {
    fn from(s: RdfLangString) -> Self {
        AnyString::from_rdf_lang_string(s)
    }
}

impl From<&str> for OneOrMany<AnyString> {
    fn from(s: &str) -> Self {
        OneOrMany::<AnyString>::from_xsd_string(s.to_owned())
    }
}

impl From<String> for OneOrMany<AnyString> {
    fn from(s: String) -> Self {
        OneOrMany::<AnyString>::from_xsd_string(s)
    }
}

impl From<RdfLangString> for OneOrMany<AnyString> {
    fn from(s: RdfLangString) -> Self {
        OneOrMany::<AnyString>::from_rdf_lang_string(s)
    }
}
