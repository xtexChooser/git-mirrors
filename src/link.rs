//! Types and traits for dealing with Link attributes
//!
//! ```rust
//! # fn main() -> Result<(), anyhow::Error> {
//! use activitystreams::{
//!     link::Mention,
//!     object::Image,
//!     prelude::*,
//!     uri,
//! };
//!
//! let mut mention = Mention::new();
//!
//! mention
//!     .set_href(uri!("https://example.com"))
//!     .set_hreflang("en")
//!     .set_rel("link")
//!     .set_preview(Image::new().into_any_base()?);
//! #
//! # Ok(())
//! # }
//! ```
use crate::{
    base::{AsBase, Base, Extends},
    markers,
    primitives::OneOrMany,
    unparsed::{Unparsed, UnparsedMut, UnparsedMutExt},
};
use std::convert::TryFrom;
use url::Url;

pub mod kind {
    //! Kinds of links defined by the spec
    //!
    //! These types exist only to be statically-typed versions of the associated string. e.g.
    //! `MentionType` -> `"Mention"`

    use crate::kind;

    kind!(MentionType, Mention);
}

use self::kind::MentionType;

/// Implementation trait for deriving Link methods for a type
///
/// Any type implementing AsLink will automatically gain methods provided by LinkExt
pub trait AsLink<Kind>: markers::Link {
    /// Immutable borrow of `Link<Kind>`
    fn link_ref(&self) -> &Link<Kind>;

    /// Mutable borrow of `Link<Kind>`
    fn link_mut(&mut self) -> &mut Link<Kind>;
}

/// Helper methods for interacting with Link types
///
/// This trait represents methods valid for any ActivityStreams Link.
///
/// Documentation for the fields related to these methods can be found on the `Link` struct
pub trait LinkExt<Kind>: AsLink<Kind> {
    /// Fetch the href for the current object
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// let mention_href = mention.href();
    /// ```
    fn href<'a>(&'a self) -> Option<&'a Url>
    where
        Kind: 'a,
    {
        self.link_ref().href.as_ref()
    }

    /// Set the href for the current object
    ///
    /// This overwrites the contents of href
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{link::Mention, uri};
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// mention.set_href(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_href(&mut self, href: Url) -> &mut Self {
        self.link_mut().href = Some(href);
        self
    }

    /// Take the href from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(href) = mention.take_href() {
    ///     println!("{:?}", href);
    /// }
    /// ```
    fn take_href(&mut self) -> Option<Url> {
        self.link_mut().href.take()
    }

    /// Delete the href from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{link::Mention, uri};
    /// # let mut mention = Mention::new();
    /// # mention.set_href(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(mention.href().is_some());
    /// mention.delete_href();
    /// assert!(mention.href().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_href(&mut self) -> &mut Self {
        self.link_mut().href = None;
        self
    }

    /// Fetch the hreflang for the current object
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// let mention_hreflang = mention.hreflang();
    /// ```
    fn hreflang<'a>(&'a self) -> Option<&'a str>
    where
        Kind: 'a,
    {
        self.link_ref().hreflang.as_ref().map(|lr| lr.as_str())
    }

    /// Set the hreflang for the current object
    ///
    /// This overwrites the contents of hreflang
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// mention.set_hreflang("en");
    /// ```
    fn set_hreflang<T>(&mut self, hreflang: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.link_mut().hreflang = Some(hreflang.into());
        self
    }

    /// Take the hreflang from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(hreflang) = mention.take_hreflang() {
    ///     println!("{:?}", hreflang);
    /// }
    /// ```
    fn take_hreflang(&mut self) -> Option<String> {
        self.link_mut().hreflang.take()
    }

    /// Delete the hreflang from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// # mention.set_hreflang("en");
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(mention.hreflang().is_some());
    /// mention.delete_hreflang();
    /// assert!(mention.hreflang().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_hreflang(&mut self) -> &mut Self {
        self.link_mut().hreflang = None;
        self
    }

    /// Fetch the rel for the current object
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// let mention_rel = mention.rel();
    /// ```
    fn rel<'a>(&'a self) -> Option<&'a OneOrMany<String>>
    where
        Kind: 'a,
    {
        self.link_ref().rel.as_ref()
    }

    /// Set the rel for the current object
    ///
    /// This overwrites the contents of rel
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// mention.set_rel("link");
    /// ```
    fn set_rel<T>(&mut self, rel: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.link_mut().rel = Some(rel.into().into());
        self
    }

    /// Set many rels for the current object
    ///
    /// This overwrites the contents of rel
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// mention.set_many_rels(vec!["link".into(), "stylesheet".into()]);
    /// ```
    fn set_many_rels<I>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = String>,
    {
        let v: Vec<_> = items.into_iter().collect();
        self.link_mut().rel = Some(v.into());
        self
    }

    /// Add a rel to the current object
    ///
    /// This does not overwrite the contents of rel, only appends an item
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// mention
    ///     .add_rel("link".into())
    ///     .add_rel("stylesheet".into());
    /// ```
    fn add_rel(&mut self, rel: String) -> &mut Self {
        let v = match self.link_mut().rel.take() {
            Some(mut v) => {
                v.add(rel);
                v
            }
            None => vec![rel].into(),
        };
        self.link_mut().rel = Some(v);
        self
    }

    /// Take the rel from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(rel) = mention.take_rel() {
    ///     println!("{:?}", rel);
    /// }
    /// ```
    fn take_rel(&mut self) -> Option<OneOrMany<String>> {
        self.link_mut().rel.take()
    }

    /// Delete the rel from the current object
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// # mention.set_rel("link");
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(mention.rel().is_some());
    /// mention.delete_rel();
    /// assert!(mention.rel().is_none());
    /// ```
    fn delete_rel(&mut self) -> &mut Self {
        self.link_mut().rel = None;
        self
    }

    /// Fetch the height of the current object
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(height) = mention.height() {
    ///     println!("{:?}", height);
    /// }
    /// ```
    fn height<'a>(&'a self) -> Option<u64>
    where
        Kind: 'a,
    {
        self.link_ref().height
    }

    /// Set the height for the current object
    ///
    /// This overwrites the contents of height
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// mention.set_height(5u64);
    /// ```
    fn set_height<T>(&mut self, height: T) -> &mut Self
    where
        T: Into<u64>,
    {
        self.link_mut().height = Some(height.into());
        self
    }

    /// Take the height of the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(height) = mention.height() {
    ///     println!("{:?}", height);
    /// }
    /// ```
    fn take_height(&mut self) -> Option<u64> {
        self.link_mut().height.take()
    }

    /// Delete the height from the current object
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// # mention.set_height(5u64);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(mention.height().is_some());
    /// mention.delete_height();
    /// assert!(mention.height().is_none());
    /// ```
    fn delete_height(&mut self) -> &mut Self {
        self.link_mut().height = None;
        self
    }

    /// Fetch the width of the current object
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(width) = mention.width() {
    ///     println!("{:?}", width);
    /// }
    /// ```
    fn width<'a>(&'a self) -> Option<u64>
    where
        Kind: 'a,
    {
        self.link_ref().width
    }

    /// Set the width for the current object
    ///
    /// This overwrites the contents of width
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// mention.set_width(5u64);
    /// ```
    fn set_width<T>(&mut self, width: T) -> &mut Self
    where
        T: Into<u64>,
    {
        self.link_mut().width = Some(width.into());
        self
    }

    /// Take the width of the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(width) = mention.take_width() {
    ///     println!("{:?}", width);
    /// }
    /// ```
    fn take_width(&mut self) -> Option<u64> {
        self.link_mut().width.take()
    }

    /// Delete the width from the current object
    ///
    /// ```rust
    /// # use activitystreams::link::Mention;
    /// # let mut mention = Mention::new();
    /// # mention.set_width(5u64);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(mention.width().is_some());
    /// mention.delete_width();
    /// assert!(mention.width().is_none());
    /// ```
    fn delete_width(&mut self) -> &mut Self {
        self.link_mut().width = None;
        self
    }
}

/// A specialized Link that represents an @mention.
///
/// This is just an alias for `Link<MentionType>` because there's no fields inherent to Mention
/// that aren't already present on a Link.
pub type Mention = Link<MentionType>;

/// Define all the properties of the Object base type as described by the Activity Streams
/// vocabulary.
///
/// The properties of the Link object are not the properties of the referenced resource, but are
/// provided as hints for rendering agents to understand how to make use of the resource. For
/// example, height and width might represent the desired rendered size of a referenced image,
/// rather than the actual pixel dimensions of the referenced image.
///
/// The target URI of the Link is expressed using the required href property.
///
/// For example, all Objects can contain an image property whose value describes a graphical
/// representation of the containing object. This property will typically be used to provide the
/// URL to an image (e.g. JPEG, GIF or PNG) resource that can be displayed to the user. Any given
/// object might have multiple such visual representations -- multiple screenshots, for instance,
/// or the same image at different resolutions. In Activity Streams 2.0, there are essentially
/// three ways of describing such references.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link<Kind> {
    /// The target resource pointed to by a Link.
    ///
    /// - Range: xsd:anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<Url>,

    /// Hints as to the language used by the target resource.
    ///
    /// Value MUST be a [BCP47] Language-Tag.
    ///
    /// - Range: [BCP47] Language Tag
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    hreflang: Option<String>,

    /// A link relation associated with a Link.
    ///
    /// The value MUST conform to both the [HTML5] and [RFC5988] "link relation" definitions.
    ///
    /// In the [HTML5], any string not containing the "space" U+0020, "tab" (U+0009), "LF" (U+000A), "FF" (U+000C), "CR" (U+000D) or "," (U+002C) characters can be used as a valid link relation.
    ///
    /// - Range: [RFC5988] or [HTML5] Link Relation
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    rel: Option<OneOrMany<String>>,

    /// On a Link, specifies a hint as to the rendering height in device-independent pixels of the linked resource.
    ///
    /// - Range: xsd:nonNegativeInteger
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u64>,

    /// On a Link, specifies a hint as to the rendering width in device-independent pixels of the linked resource.
    ///
    /// Range: xsd:nonNegativeInteger
    /// Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u64>,

    /// Base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Base<Kind>,
}

impl<Kind> Link<Kind> {
    /// Create a new Link
    ///
    /// ```rust
    /// use activitystreams::link::Link;
    ///
    /// let link = Link::<String>::new();
    /// ```
    pub fn new() -> Self
    where
        Kind: Default,
    {
        Link {
            href: None,
            hreflang: None,
            rel: None,
            height: None,
            width: None,
            inner: Base::new(),
        }
    }

    fn extending(mut inner: Base<Kind>) -> Result<Self, serde_json::Error> {
        Ok(Link {
            href: inner.remove("href")?,
            hreflang: inner.remove("hreflang")?,
            rel: inner.remove("rel")?,
            height: inner.remove("height")?,
            width: inner.remove("width")?,
            inner,
        })
    }

    fn retracting(self) -> Result<Base<Kind>, serde_json::Error> {
        let Link {
            href,
            hreflang,
            rel,
            height,
            width,
            mut inner,
        } = self;

        inner
            .insert("href", href)?
            .insert("hreflang", hreflang)?
            .insert("rel", rel)?
            .insert("height", height)?
            .insert("width", width)?;

        Ok(inner)
    }
}

impl<Kind> markers::Base for Link<Kind> {}
impl<Kind> markers::Link for Link<Kind> {}

impl<Kind> Extends<Kind> for Link<Kind> {
    type Error = serde_json::Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        Self::extending(base)
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        self.retracting()
    }
}

impl<Kind> TryFrom<Base<Kind>> for Link<Kind>
where
    Kind: serde::de::DeserializeOwned,
{
    type Error = serde_json::Error;

    fn try_from(base: Base<Kind>) -> Result<Self, Self::Error> {
        Self::extending(base)
    }
}

impl<Kind> TryFrom<Link<Kind>> for Base<Kind>
where
    Kind: serde::ser::Serialize,
{
    type Error = serde_json::Error;

    fn try_from(link: Link<Kind>) -> Result<Self, Self::Error> {
        link.retracting()
    }
}

impl<Kind> UnparsedMut for Link<Kind> {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Kind> AsBase<Kind> for Link<Kind> {
    fn base_ref(&self) -> &Base<Kind> {
        &self.inner
    }

    fn base_mut(&mut self) -> &mut Base<Kind> {
        &mut self.inner
    }
}

impl<Kind> AsLink<Kind> for Link<Kind> {
    fn link_ref(&self) -> &Link<Kind> {
        self
    }

    fn link_mut(&mut self) -> &mut Link<Kind> {
        self
    }
}

impl<T, Kind> LinkExt<Kind> for T where T: AsLink<Kind> {}
