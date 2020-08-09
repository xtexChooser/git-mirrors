//! Types and traits for dealing with Object attributes
//!
//! ```rust
//! # fn main() -> Result<(), anyhow::Error> {
//! use activitystreams::{
//!     object::Image,
//!     prelude::*,
//!     uri,
//! };
//!
//! let mut image = Image::new();
//!
//! image
//!     .set_url(uri!("https://example.com/image.png"))
//!     .set_attributed_to(uri!("https://example.com/actor"))
//!     .set_generator(uri!("https://example.com/image-generator"))
//!     .set_icon(uri!("https://example.com/icon.png"));
//! #
//! # Ok(())
//! # }
//! ```
use crate::{
    base::{AnyBase, AsBase, Base, Extends},
    markers,
    primitives::{AnyString, OneOrMany, Unit, XsdDateTime, XsdDuration},
    unparsed::{Unparsed, UnparsedMut, UnparsedMutExt},
};
use chrono::{DateTime, Duration, FixedOffset};
use std::convert::TryFrom;
use url::Url;

pub mod kind {
    //! Kinds of objects defined by the spec
    //!
    //! These types exist only to be statically-typed versions of the associated string. e.g.
    //! `PlaceType` -> `"Place"`

    use crate::kind;

    kind!(ArticleType, Article);
    kind!(AudioType, Audio);
    kind!(DocumentType, Document);
    kind!(EventType, Event);
    kind!(ImageType, Image);
    kind!(NoteType, Note);
    kind!(PageType, Page);
    kind!(PlaceType, Place);
    kind!(ProfileType, Profile);
    kind!(RelationshipType, Relationship);
    kind!(TombstoneType, Tombstone);
    kind!(VideoType, Video);
}

use self::kind::*;

/// Implementation trait for deriving Object methods for a type
///
/// Any type implementing AsObject will automatically gain methods provided by ObjectExt
pub trait AsObject<Kind>: markers::Object {
    /// Immutable borrow of `Object<Kind>`
    fn object_ref(&self) -> &Object<Kind>;

    /// Mutable borrow of `Object<Kind>`
    fn object_mut(&mut self) -> &mut Object<Kind>;
}

/// Implementation trait for deriving ActivityPub Object methods for a type
///
/// Any type implementing AsApObject will automatically gain methods provided by ApObjectExt
pub trait AsApObject<Inner>: markers::Object {
    /// Immutable borrow of `ApObject<Inner>`
    fn ap_object_ref(&self) -> &ApObject<Inner>;

    /// Mutable borrow of `ApObject<Inner>`
    fn ap_object_mut(&mut self) -> &mut ApObject<Inner>;
}

/// Implementation trait for deriving Place methods for a type
///
/// Any type implementing AsPlace will automatically gain methods provided by PlaceExt
pub trait AsPlace: markers::Object {
    /// Immutable borrow of `Place`
    fn place_ref(&self) -> &Place;

    /// Mutable borrow of `Place`
    fn place_mut(&mut self) -> &mut Place;
}

/// Implementation trait for deriving Profile methods for a type
///
/// Any type implementing AsProfile will automatically gain methods provided by ProfileExt
pub trait AsProfile: markers::Object {
    /// Immutable borrow of `Profile`
    fn profile_ref(&self) -> &Profile;

    /// Mutable borrow of `Profile`
    fn profile_mut(&mut self) -> &mut Profile;
}

/// Implementation trait for deriving Relationship methods for a type
///
/// Any type implementing AsRelationship will automatically gain methods provided by
/// RelationshipExt
pub trait AsRelationship: markers::Object {
    /// Immutable borrow of `Relationship`
    fn relationship_ref(&self) -> &Relationship;

    /// Mutable borrow of `Relationship`
    fn relationship_mut(&mut self) -> &mut Relationship;
}

/// Implementation trait for deriving Tombstone methods for a type
///
/// Any type implementing AsTombstone will automatically gain methods provided by TombstoneExt
pub trait AsTombstone: markers::Object {
    /// Immutable borrow of `Tombstone`
    fn tombstone_ref(&self) -> &Tombstone;

    /// Mutable borrow of `Tombstone`
    fn tombstone_mut(&mut self) -> &mut Tombstone;
}

/// Helper methods for interacting with Object types
///
/// This trait represents methods valid for any ActivityStreams Object.
///
/// Documentation for the fields related to these methods can be found on the `Object` struct
pub trait ObjectExt<Kind>: AsObject<Kind> {
    /// Fetch the attachment for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(attachment) = video.attachment() {
    ///     println!("{:?}", attachment);
    /// }
    /// ```
    fn attachment<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().attachment.as_ref()
    }

    /// Set the attachment for the current object
    ///
    /// This overwrites the contents of attachment
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_attachment(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_attachment<T>(&mut self, attachment: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().attachment = Some(attachment.into().into());
        self
    }

    /// Set many attachments for the current object
    ///
    /// This overwrites the contents of attachment
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_attachments(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_attachments<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().attachment = Some(v.into());
        self
    }

    /// Add a attachment to the current object
    ///
    /// This does not overwrite the contents of attachment, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_attachment(uri!("https://example.com/one"))
    ///     .add_attachment(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_attachment<T>(&mut self, attachment: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().attachment.take() {
            Some(mut a) => {
                a.add(attachment.into());
                a
            }
            None => vec![attachment.into()].into(),
        };
        self.object_mut().attachment = Some(a);
        self
    }

    /// Take the attachment from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(attachment) = video.take_attachment() {
    ///     println!("{:?}", attachment);
    /// }
    /// ```
    fn take_attachment(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().attachment.take()
    }

    /// Delete the attachment from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_attachment(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.attachment().is_some());
    /// video.delete_attachment();
    /// assert!(video.attachment().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_attachment(&mut self) -> &mut Self {
        self.object_mut().attachment = None;
        self
    }

    /// Fetch the attributed_to for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(attributed_to) = video.attributed_to() {
    ///     println!("{:?}", attributed_to);
    /// }
    /// ```
    fn attributed_to<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().attributed_to.as_ref()
    }

    /// Set the attributed_to for the current object
    ///
    /// This overwrites the contents of attributed_to
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_attributed_to(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_attributed_to<T>(&mut self, attributed_to: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().attributed_to = Some(attributed_to.into().into());
        self
    }

    /// Set many attributed_tos for the current object
    ///
    /// This overwrites the contents of attributed_to
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_attributed_tos(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_attributed_tos<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().attributed_to = Some(v.into());
        self
    }

    /// Add a attributed_to to the current object
    ///
    /// This does not overwrite the contents of attributed_to, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_attributed_to(uri!("https://example.com/one"))
    ///     .add_attributed_to(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_attributed_to<T>(&mut self, attributed_to: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().attributed_to.take() {
            Some(mut a) => {
                a.add(attributed_to.into());
                a
            }
            None => vec![attributed_to.into()].into(),
        };
        self.object_mut().attributed_to = Some(a);
        self
    }

    /// Take the attributed_to from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(attributed_to) = video.take_attributed_to() {
    ///     println!("{:?}", attributed_to);
    /// }
    /// ```
    fn take_attributed_to(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().attributed_to.take()
    }

    /// Delete the attributed_to from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_attributed_to(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.attributed_to().is_some());
    /// video.delete_attributed_to();
    /// assert!(video.attributed_to().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_attributed_to(&mut self) -> &mut Self {
        self.object_mut().attributed_to = None;
        self
    }

    /// Fetch the audience for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(audience) = video.audience() {
    ///     println!("{:?}", audience);
    /// }
    /// ```
    fn audience<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().audience.as_ref()
    }

    /// Set the audience for the current object
    ///
    /// This overwrites the contents of audience
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_audience(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_audience<T>(&mut self, audience: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().audience = Some(audience.into().into());
        self
    }

    /// This overwrites the contents of audience
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_audiences(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_audiences<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().audience = Some(v.into());
        self
    }

    /// Add a audience to the current object
    ///
    /// This does not overwrite the contents of audience, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_audience(uri!("https://example.com/one"))
    ///     .add_audience(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_audience<T>(&mut self, audience: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().audience.take() {
            Some(mut a) => {
                a.add(audience.into());
                a
            }
            None => vec![audience.into()].into(),
        };
        self.object_mut().audience = Some(a);
        self
    }

    /// Take the audience from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(audience) = video.take_audience() {
    ///     println!("{:?}", audience);
    /// }
    /// ```
    fn take_audience(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().audience.take()
    }

    /// Delete the audience from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_audience(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.audience().is_some());
    /// video.delete_audience();
    /// assert!(video.audience().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_audience(&mut self) -> &mut Self {
        self.object_mut().audience = None;
        self
    }

    /// Fetch the content for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(content) = video.content() {
    ///     println!("{:?}", content);
    /// }
    /// ```
    fn content<'a>(&'a self) -> Option<&'a OneOrMany<AnyString>>
    where
        Kind: 'a,
    {
        self.object_ref().content.as_ref()
    }

    /// Set the content for the current object
    ///
    /// This overwrites the contents of content
    ///
    /// ```rust
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    ///
    /// video.set_content("hi");
    /// ```
    fn set_content<T>(&mut self, content: T) -> &mut Self
    where
        T: Into<AnyString>,
    {
        self.object_mut().content = Some(content.into().into());
        self
    }

    /// Set many contents for the current object
    ///
    /// This overwrites the contents of content
    ///
    /// ```rust
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    ///
    /// video.set_many_contents(vec!["hi", "hello"]);
    /// ```
    fn set_many_contents<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyString>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().content = Some(v.into());
        self
    }

    /// Add a content to the current object
    ///
    /// This does not overwrite the contents of content, only appends an item
    ///
    /// ```rust
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_content("hi")
    ///     .add_content("hello");
    /// ```
    fn add_content<T>(&mut self, content: T) -> &mut Self
    where
        T: Into<AnyString>,
    {
        let a = match self.object_mut().content.take() {
            Some(mut a) => {
                a.add(content.into());
                a
            }
            None => vec![content.into()].into(),
        };
        self.object_mut().content = Some(a);
        self
    }

    /// Take the content from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(content) = video.take_content() {
    ///     println!("{:?}", content);
    /// }
    /// ```
    fn take_content(&mut self) -> Option<OneOrMany<AnyString>> {
        self.object_mut().content.take()
    }

    /// Delete the content from the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// # video.set_content("content");
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.content().is_some());
    /// video.delete_content();
    /// assert!(video.content().is_none());
    /// ```
    fn delete_content(&mut self) -> &mut Self {
        self.object_mut().content = None;
        self
    }

    /// Fetch the summary for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(summary) = video.summary() {
    ///     println!("{:?}", summary);
    /// }
    /// ```
    fn summary<'a>(&'a self) -> Option<&'a OneOrMany<AnyString>>
    where
        Kind: 'a,
    {
        self.object_ref().summary.as_ref()
    }

    /// Set the summary for the current object
    ///
    /// This overwrites the contents of summary
    ///
    /// ```rust
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    ///
    /// video.set_summary("hi");
    /// ```
    fn set_summary<T>(&mut self, summary: T) -> &mut Self
    where
        T: Into<AnyString>,
    {
        self.object_mut().summary = Some(summary.into().into());
        self
    }

    /// Set many summaries for the current object
    ///
    /// This overwrites the contents of summary
    ///
    /// ```rust
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    ///
    /// video.set_many_summaries(vec![ "hi", "hello"]);
    /// ```
    fn set_many_summaries<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyString>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().summary = Some(v.into());
        self
    }

    /// Add a summary to the current object
    ///
    /// This does not overwrite the contents of summary, only appends an item
    ///
    /// ```rust
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_summary("hi")
    ///     .add_summary("hello");
    /// ```
    fn add_summary<T>(&mut self, summary: T) -> &mut Self
    where
        T: Into<AnyString>,
    {
        let a = match self.object_mut().summary.take() {
            Some(mut a) => {
                a.add(summary.into());
                a
            }
            None => vec![summary.into()].into(),
        };
        self.object_mut().summary = Some(a);
        self
    }

    /// Take the summary from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(summary) = video.take_summary() {
    ///     println!("{:?}", summary);
    /// }
    /// ```
    fn take_summary(&mut self) -> Option<OneOrMany<AnyString>> {
        self.object_mut().summary.take()
    }

    /// Delete the summary from the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// # video.set_summary("summary");
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.summary().is_some());
    /// video.delete_summary();
    /// assert!(video.summary().is_none());
    /// ```
    fn delete_summary(&mut self) -> &mut Self {
        self.object_mut().summary = None;
        self
    }

    /// Fetch the url for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(url) = video.url() {
    ///     println!("{:?}", url);
    /// }
    /// ```
    fn url<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().url.as_ref()
    }

    /// Set the url for the current object
    ///
    /// This overwrites the contents of url
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_url(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_url<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().url = Some(url.into().into());
        self
    }

    /// Set many urls for the current object
    ///
    /// This overwrites the contents of url
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_urls(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_urls<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().url = Some(v.into());
        self
    }

    /// Add a url to the current object
    ///
    /// This does not overwrite the contents of url, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_url(uri!("https://example.com/one"))
    ///     .add_url(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_url<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().url.take() {
            Some(mut a) => {
                a.add(url.into());
                a
            }
            None => vec![url.into()].into(),
        };
        self.object_mut().url = Some(a);
        self
    }

    /// Take the url from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(url) = video.take_url() {
    ///     println!("{:?}", url);
    /// }
    /// ```
    fn take_url(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().url.take()
    }

    /// Delete the url from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_url(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.url().is_some());
    /// video.delete_url();
    /// assert!(video.url().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_url(&mut self) -> &mut Self {
        self.object_mut().url = None;
        self
    }

    /// Fetch the generator for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(generator) = video.generator() {
    ///     println!("{:?}", generator);
    /// }
    /// ```
    fn generator<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().generator.as_ref()
    }

    /// Set the generator for the current object
    ///
    /// This overwrites the contents of generator
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_generator(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_generator<T>(&mut self, generator: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().generator = Some(generator.into().into());
        self
    }

    /// Set many generators for the current object
    ///
    /// This overwrites the contents of generator
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_generators(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_generators<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().generator = Some(v.into());
        self
    }

    /// Add a generator to the current object
    ///
    /// This does not overwrite the contents of generator, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_generator(uri!("https://example.com/one"))
    ///     .add_generator(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_generator<T>(&mut self, generator: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().generator.take() {
            Some(mut a) => {
                a.add(generator.into());
                a
            }
            None => vec![generator.into()].into(),
        };
        self.object_mut().generator = Some(a);
        self
    }

    /// Take the generator from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(generator) = video.take_generator() {
    ///     println!("{:?}", generator);
    /// }
    /// ```
    fn take_generator(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().generator.take()
    }

    /// Delete the generator from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_generator(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.generator().is_some());
    /// video.delete_generator();
    /// assert!(video.generator().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_generator(&mut self) -> &mut Self {
        self.object_mut().generator = None;
        self
    }

    /// Fetch the icon for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(icon) = video.icon() {
    ///     println!("{:?}", icon);
    /// }
    /// ```
    fn icon<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().icon.as_ref()
    }

    /// Set the icon for the current object
    ///
    /// This overwrites the contents of icon
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_icon(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_icon<T>(&mut self, icon: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().icon = Some(icon.into().into());
        self
    }

    /// Set many icons for the current object
    ///
    /// This overwrites the contents of icon
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_icons(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_icons<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().icon = Some(v.into());
        self
    }

    /// Add a icon to the current object
    ///
    /// This does not overwrite the contents of icon, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_icon(uri!("https://example.com/one"))
    ///     .add_icon(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_icon<T>(&mut self, icon: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().icon.take() {
            Some(mut a) => {
                a.add(icon.into());
                a
            }
            None => vec![icon.into()].into(),
        };
        self.object_mut().icon = Some(a);
        self
    }

    /// Take the icon from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(icon) = video.take_icon() {
    ///     println!("{:?}", icon);
    /// }
    /// ```
    fn take_icon(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().icon.take()
    }

    /// Delete the icon from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_icon(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.icon().is_some());
    /// video.delete_icon();
    /// assert!(video.icon().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_icon(&mut self) -> &mut Self {
        self.object_mut().icon = None;
        self
    }

    /// Fetch the image for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(image) = video.image() {
    ///     println!("{:?}", image);
    /// }
    /// ```
    fn image<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().image.as_ref()
    }

    /// Set the image for the current object
    ///
    /// This overwrites the contents of image
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_image(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_image<T>(&mut self, image: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().image = Some(image.into().into());
        self
    }

    /// Set many images for the current object
    ///
    /// This overwrites the contents of image
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_images(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_images<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().image = Some(v.into());
        self
    }

    /// Add a image to the current object
    ///
    /// This does not overwrite the contents of image, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_image(uri!("https://example.com/one"))
    ///     .add_image(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_image<T>(&mut self, image: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().image.take() {
            Some(mut a) => {
                a.add(image.into());
                a
            }
            None => vec![image.into()].into(),
        };
        self.object_mut().image = Some(a);
        self
    }

    /// Take the image from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(image) = video.take_image() {
    ///     println!("{:?}", image);
    /// }
    /// ```
    fn take_image(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().image.take()
    }

    /// Delete the image from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_image(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.image().is_some());
    /// video.delete_image();
    /// assert!(video.image().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_image(&mut self) -> &mut Self {
        self.object_mut().image = None;
        self
    }

    /// Fetch the location for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(location) = video.location() {
    ///     println!("{:?}", location);
    /// }
    /// ```
    fn location<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().location.as_ref()
    }

    /// Set the location for the current object
    ///
    /// This overwrites the contents of location
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_location(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_location<T>(&mut self, location: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().location = Some(location.into().into());
        self
    }

    /// Set many locations for the current object
    ///
    /// This overwrites the contents of location
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_locations(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_locations<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().location = Some(v.into());
        self
    }

    /// Add a location to the current object
    ///
    /// This does not overwrite the contents of location, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_location(uri!("https://example.com/one"))
    ///     .add_location(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_location<T>(&mut self, location: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().location.take() {
            Some(mut a) => {
                a.add(location.into());
                a
            }
            None => vec![location.into()].into(),
        };
        self.object_mut().location = Some(a);
        self
    }

    /// Take the location from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(location) = video.take_location() {
    ///     println!("{:?}", location);
    /// }
    /// ```
    fn take_location(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().location.take()
    }

    /// Delete the location from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_location(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.location().is_some());
    /// video.delete_location();
    /// assert!(video.location().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_location(&mut self) -> &mut Self {
        self.object_mut().location = None;
        self
    }

    /// Fetch the tag for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(tag) = video.tag() {
    ///     println!("{:?}", tag);
    /// }
    /// ```
    fn tag<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().tag.as_ref()
    }

    /// Set the tag for the current object
    ///
    /// This overwrites the contents of tag
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_tag(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_tag<T>(&mut self, tag: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().tag = Some(tag.into().into());
        self
    }

    /// Set many tags for the current object
    ///
    /// This overwrites the contents of tag
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_tags(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_tags<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().tag = Some(v.into());
        self
    }

    /// Add a tag to the current object
    ///
    /// This does not overwrite the contents of tag, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_tag(uri!("https://example.com/one"))
    ///     .add_tag(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_tag<T>(&mut self, tag: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().tag.take() {
            Some(mut a) => {
                a.add(tag.into());
                a
            }
            None => vec![tag.into()].into(),
        };
        self.object_mut().tag = Some(a);
        self
    }

    /// Take the tag from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(tag) = video.take_tag() {
    ///     println!("{:?}", tag);
    /// }
    /// ```
    fn take_tag(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().tag.take()
    }

    /// Delete the tag from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_tag(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.tag().is_some());
    /// video.delete_tag();
    /// assert!(video.tag().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_tag(&mut self) -> &mut Self {
        self.object_mut().tag = None;
        self
    }

    /// Fetch the start_time for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(start_time) = video.start_time() {
    ///     println!("{:?}", start_time);
    /// }
    /// ```
    fn start_time<'a>(&'a self) -> Option<DateTime<FixedOffset>>
    where
        Kind: 'a,
    {
        self.object_ref()
            .start_time
            .as_ref()
            .map(|d| d.clone().into_inner())
    }

    /// Set the start_time for the current object
    ///
    /// This overwrites the contents of start_time
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video};
    /// # let mut video = Video::new();
    ///
    /// video.set_start_time("2020-04-20T04:20:00Z".parse()?);
    /// # Ok(())
    /// # }
    /// ```
    fn set_start_time(&mut self, start_time: DateTime<FixedOffset>) -> &mut Self {
        self.object_mut().start_time = Some(start_time.into());
        self
    }

    /// Take the start_time from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(start_time) = video.take_start_time() {
    ///     println!("{:?}", start_time);
    /// }
    /// ```
    fn take_start_time(&mut self) -> Option<DateTime<FixedOffset>> {
        self.object_mut().start_time.take().map(|d| d.into_inner())
    }

    /// Delete the start_time from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// # video.set_start_time("2020-04-20T04:20:00Z".parse()?);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.start_time().is_some());
    /// video.delete_start_time();
    /// assert!(video.start_time().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_start_time(&mut self) -> &mut Self {
        self.object_mut().start_time = None;
        self
    }

    /// Fetch the end_time for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(end_time) = video.end_time() {
    ///     println!("{:?}", end_time);
    /// }
    /// ```
    fn end_time<'a>(&'a self) -> Option<DateTime<FixedOffset>>
    where
        Kind: 'a,
    {
        self.object_ref()
            .end_time
            .as_ref()
            .map(|d| d.clone().into_inner())
    }

    /// Set the end_time for the current object
    ///
    /// This overwrites the contents of end_time
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    ///
    /// video.set_end_time("2020-04-20T04:20:00-05:00".parse()?);
    /// # Ok(())
    /// # }
    /// ```
    fn set_end_time(&mut self, end_time: DateTime<FixedOffset>) -> &mut Self {
        self.object_mut().end_time = Some(end_time.into());
        self
    }

    /// Take the end_time from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(end_time) = video.take_end_time() {
    ///     println!("{:?}", end_time);
    /// }
    /// ```
    fn take_end_time(&mut self) -> Option<DateTime<FixedOffset>> {
        self.object_mut().end_time.take().map(|d| d.into_inner())
    }

    /// Delete the end_time from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// # video.set_end_time("2020-04-20T04:20:00Z".parse()?);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.end_time().is_some());
    /// video.delete_end_time();
    /// assert!(video.end_time().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_end_time(&mut self) -> &mut Self {
        self.object_mut().end_time = None;
        self
    }

    /// Fetch the duration for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(duration) = video.duration() {
    ///     println!("{:?}", duration);
    /// }
    /// ```
    fn duration<'a>(&'a self) -> Option<Duration>
    where
        Kind: 'a,
    {
        self.object_ref()
            .duration
            .as_ref()
            .map(|d| d.clone().into_inner())
    }

    /// Set the duration for the current object
    ///
    /// This overwrites the contents of duration
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// use chrono::Duration;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    ///
    /// video.set_duration(Duration::minutes(4) + Duration::seconds(20));
    /// # Ok(())
    /// # }
    /// ```
    fn set_duration(&mut self, duration: Duration) -> &mut Self {
        self.object_mut().duration = Some(duration.into());
        self
    }

    /// Take the duration from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(duration) = video.take_duration() {
    ///     println!("{:?}", duration);
    /// }
    /// ```
    fn take_duration(&mut self) -> Option<Duration> {
        self.object_mut().duration.take().map(|d| d.into_inner())
    }

    /// Delete the duration from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::Video;
    /// # use chrono::Duration;
    /// # let mut video = Video::new();
    /// # video.set_duration(Duration::hours(1));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.duration().is_some());
    /// video.delete_duration();
    /// assert!(video.duration().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_duration(&mut self) -> &mut Self {
        self.object_mut().duration = None;
        self
    }

    /// Fetch the published for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(published) = video.published() {
    ///     println!("{:?}", published);
    /// }
    /// ```
    fn published<'a>(&'a self) -> Option<DateTime<FixedOffset>>
    where
        Kind: 'a,
    {
        self.object_ref()
            .published
            .as_ref()
            .map(|d| d.clone().into_inner())
    }

    /// Set the published for the current object
    ///
    /// This overwrites the contents of published
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    ///
    /// video.set_published("2020-04-20T04:20:00Z".parse()?);
    /// # Ok(())
    /// # }
    /// ```
    fn set_published(&mut self, published: DateTime<FixedOffset>) -> &mut Self {
        self.object_mut().published = Some(published.into());
        self
    }

    /// Take the published from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(published) = video.take_published() {
    ///     println!("{:?}", published);
    /// }
    /// ```
    fn take_published(&mut self) -> Option<DateTime<FixedOffset>> {
        self.object_mut().published.take().map(|d| d.into_inner())
    }

    /// Delete the published from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// # video.set_published("2020-04-20T04:20:00Z".parse()?);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.published().is_some());
    /// video.delete_published();
    /// assert!(video.published().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_published(&mut self) -> &mut Self {
        self.object_mut().published = None;
        self
    }

    /// Fetch the updated for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(updated) = video.updated() {
    ///     println!("{:?}", updated);
    /// }
    /// ```
    fn updated<'a>(&'a self) -> Option<DateTime<FixedOffset>>
    where
        Kind: 'a,
    {
        self.object_ref()
            .updated
            .as_ref()
            .map(|d| d.clone().into_inner())
    }

    /// Set the updated for the current object
    ///
    /// This overwrites the contents of updated
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    ///
    /// video.set_updated("2020-04-20T04:20:00Z".parse()?);
    /// # Ok(())
    /// # }
    /// ```
    fn set_updated(&mut self, updated: DateTime<FixedOffset>) -> &mut Self {
        self.object_mut().updated = Some(updated.into());
        self
    }

    /// Take the updated from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(updated) = video.take_updated() {
    ///     println!("{:?}", updated);
    /// }
    /// ```
    fn take_updated(&mut self) -> Option<DateTime<FixedOffset>> {
        self.object_mut().updated.take().map(|d| d.into_inner())
    }

    /// Delete the updated from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// # video.set_updated("2020-04-20T04:20:00Z".parse()?);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.updated().is_some());
    /// video.delete_updated();
    /// assert!(video.updated().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_updated(&mut self) -> &mut Self {
        self.object_mut().updated = None;
        self
    }

    /// Fetch the in_reply_to for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(in_reply_to) = video.in_reply_to() {
    ///     println!("{:?}", in_reply_to);
    /// }
    /// ```
    fn in_reply_to<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().in_reply_to.as_ref()
    }

    /// Set the in_reply_to for the current object
    ///
    /// This overwrites the contents of in_reply_to
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_in_reply_to(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_in_reply_to<T>(&mut self, in_reply_to: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().in_reply_to = Some(in_reply_to.into().into());
        self
    }

    /// Set many in_reply_tos for the current object
    ///
    /// This overwrites the contents of in_reply_to
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_in_reply_tos(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_in_reply_tos<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().in_reply_to = Some(v.into());
        self
    }

    /// Add a in_reply_to to the current object
    ///
    /// This does not overwrite the contents of in_reply_to, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_in_reply_to(uri!("https://example.com/one"))
    ///     .add_in_reply_to(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_in_reply_to<T>(&mut self, in_reply_to: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().in_reply_to.take() {
            Some(mut a) => {
                a.add(in_reply_to.into());
                a
            }
            None => vec![in_reply_to.into()].into(),
        };
        self.object_mut().in_reply_to = Some(a);
        self
    }

    /// Take the in_reply_to from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(in_reply_to) = video.take_in_reply_to() {
    ///     println!("{:?}", in_reply_to);
    /// }
    /// ```
    fn take_in_reply_to(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().in_reply_to.take()
    }

    /// Delete the in_reply_to from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_in_reply_to(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.in_reply_to().is_some());
    /// video.delete_in_reply_to();
    /// assert!(video.in_reply_to().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_in_reply_to(&mut self) -> &mut Self {
        self.object_mut().in_reply_to = None;
        self
    }

    /// Fetch the replies for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(replies) = video.replies() {
    ///     println!("{:?}", replies);
    /// }
    /// ```
    fn replies<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().replies.as_ref()
    }

    /// Set the replies for the current object
    ///
    /// This overwrites the contents of replies
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_reply(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_reply<T>(&mut self, replies: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().replies = Some(replies.into().into());
        self
    }

    /// Set many replies for the current object
    ///
    /// This overwrites the contents of replies
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_replies(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_replies<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().replies = Some(v.into());
        self
    }

    /// Add a replies to the current object
    ///
    /// This does not overwrite the contents of replies, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_reply(uri!("https://example.com/one"))
    ///     .add_reply(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_reply<T>(&mut self, replies: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().replies.take() {
            Some(mut a) => {
                a.add(replies.into());
                a
            }
            None => vec![replies.into()].into(),
        };
        self.object_mut().replies = Some(a);
        self
    }

    /// Take the replies from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(replies) = video.take_replies() {
    ///     println!("{:?}", replies);
    /// }
    /// ```
    fn take_replies(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().replies.take()
    }

    /// Delete the replies from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_reply(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.replies().is_some());
    /// video.delete_replies();
    /// assert!(video.replies().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_replies(&mut self) -> &mut Self {
        self.object_mut().replies = None;
        self
    }

    /// Fetch the to for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(to) = video.to() {
    ///     println!("{:?}", to);
    /// }
    /// ```
    fn to<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().to.as_ref()
    }

    /// Set the to for the current object
    ///
    /// This overwrites the contents of to
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_to(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_to<T>(&mut self, to: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().to = Some(to.into().into());
        self
    }

    /// Set many tos for the current object
    ///
    /// This overwrites the contents of to
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_tos(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_tos<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().to = Some(v.into());
        self
    }

    /// Add a to to the current object
    ///
    /// This does not overwrite the contents of to, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_to(uri!("https://example.com/one"))
    ///     .add_to(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_to<T>(&mut self, to: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().to.take() {
            Some(mut a) => {
                a.add(to.into());
                a
            }
            None => vec![to.into()].into(),
        };
        self.object_mut().to = Some(a);
        self
    }

    /// Take the to from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(to) = video.take_to() {
    ///     println!("{:?}", to);
    /// }
    /// ```
    fn take_to(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().to.take()
    }

    /// Delete the to from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_to(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.to().is_some());
    /// video.delete_to();
    /// assert!(video.to().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_to(&mut self) -> &mut Self {
        self.object_mut().to = None;
        self
    }

    /// Fetch the bto for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(bto) = video.bto() {
    ///     println!("{:?}", bto);
    /// }
    /// ```
    fn bto<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().bto.as_ref()
    }

    /// Set the bto for the current object
    ///
    /// This overwrites the contents of bto
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_bto(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_bto<T>(&mut self, bto: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().bto = Some(bto.into().into());
        self
    }

    /// Set many btos for the current object
    ///
    /// This overwrites the contents of bto
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_btos(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_btos<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().bto = Some(v.into());
        self
    }

    /// Add a bto to the current object
    ///
    /// This does not overwrite the contents of bto, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_bto(uri!("https://example.com/one"))
    ///     .add_bto(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_bto<T>(&mut self, bto: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().bto.take() {
            Some(mut a) => {
                a.add(bto.into());
                a
            }
            None => vec![bto.into()].into(),
        };
        self.object_mut().bto = Some(a);
        self
    }

    /// Take the bto from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(bto) = video.take_bto() {
    ///     println!("{:?}", bto);
    /// }
    /// ```
    fn take_bto(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().bto.take()
    }

    /// Delete the bto from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_bto(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.bto().is_some());
    /// video.delete_bto();
    /// assert!(video.bto().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_bto(&mut self) -> &mut Self {
        self.object_mut().bto = None;
        self
    }

    /// Fetch the cc for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(cc) = video.cc() {
    ///     println!("{:?}", cc);
    /// }
    /// ```
    fn cc<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().cc.as_ref()
    }

    /// Set the cc for the current object
    ///
    /// This overwrites the contents of cc
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_cc(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_cc<T>(&mut self, cc: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().cc = Some(cc.into().into());
        self
    }

    /// Set many ccs for the current object
    ///
    /// This overwrites the contents of cc
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_ccs(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_ccs<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().cc = Some(v.into());
        self
    }

    /// Add a cc to the current object
    ///
    /// This does not overwrite the contents of cc, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_cc(uri!("https://example.com/one"))
    ///     .add_cc(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_cc<T>(&mut self, cc: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().cc.take() {
            Some(mut a) => {
                a.add(cc.into());
                a
            }
            None => vec![cc.into()].into(),
        };
        self.object_mut().cc = Some(a);
        self
    }

    /// Take the cc from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(cc) = video.take_cc() {
    ///     println!("{:?}", cc);
    /// }
    /// ```
    fn take_cc(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().cc.take()
    }

    /// Delete the cc from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_cc(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.cc().is_some());
    /// video.delete_cc();
    /// assert!(video.cc().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_cc(&mut self) -> &mut Self {
        self.object_mut().cc = None;
        self
    }

    /// Fetch the bcc for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(bcc) = video.bcc() {
    ///     println!("{:?}", bcc);
    /// }
    /// ```
    fn bcc<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.object_ref().bcc.as_ref()
    }

    /// Set the bcc for the current object
    ///
    /// This overwrites the contents of bcc
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_bcc(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_bcc<T>(&mut self, bcc: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.object_mut().bcc = Some(bcc.into().into());
        self
    }

    /// Set many bcc for the current object
    ///
    /// This overwrites the contents of bcc
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_bcc(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_bcc<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.object_mut().bcc = Some(v.into());
        self
    }

    /// Add a bcc to the current object
    ///
    /// This does not overwrite the contents of bcc, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_bcc(uri!("https://example.com/one"))
    ///     .add_bcc(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_bcc<T>(&mut self, bcc: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.object_mut().bcc.take() {
            Some(mut a) => {
                a.add(bcc.into());
                a
            }
            None => vec![bcc.into()].into(),
        };
        self.object_mut().bcc = Some(a);
        self
    }

    /// Take the bcc from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(bcc) = video.take_bcc() {
    ///     println!("{:?}", bcc);
    /// }
    /// ```
    fn take_bcc(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.object_mut().bcc.take()
    }

    /// Delete the bcc from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_bcc(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.bcc().is_some());
    /// video.delete_bcc();
    /// assert!(video.bcc().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_bcc(&mut self) -> &mut Self {
        self.object_mut().bcc = None;
        self
    }
}

/// Helper methods for interacting with ActivityPub Object types
///
/// This trait represents methods valid for any ActivityPub Object.
///
/// Documentation for the fields related to these methods can be found on the `ApObject` struct
pub trait ApObjectExt<Inner>: AsApObject<Inner> {
    /// Fetch the shares for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(shares) = video.shares() {
    ///     println!("{:?}", shares);
    /// }
    /// ```
    fn shares<'a>(&'a self) -> Option<&'a Url>
    where
        Inner: 'a,
    {
        self.ap_object_ref().shares.as_ref()
    }

    /// Set the shares for the current object
    ///
    /// This overwrites the contents of shares
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::{prelude::*, uri};
    ///
    /// video.set_shares(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_shares(&mut self, shares: Url) -> &mut Self {
        self.ap_object_mut().shares = Some(shares.into());
        self
    }

    /// Take the shares from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(shares) = video.take_shares() {
    ///     println!("{:?}", shares);
    /// }
    /// ```
    fn take_shares(&mut self) -> Option<Url> {
        self.ap_object_mut().shares.take()
    }

    /// Delete the shares from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, object::{ApObject, Video}};
    /// # let mut video = ApObject::new(Video::new());
    /// # video.set_shares(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.shares().is_some());
    /// video.delete_shares();
    /// assert!(video.shares().is_none());
    /// ```
    fn delete_shares(&mut self) -> &mut Self {
        self.ap_object_mut().shares = None;
        self
    }

    /// Fetch the likes for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(likes) = video.likes() {
    ///     println!("{:?}", likes);
    /// }
    /// ```
    fn likes<'a>(&'a self) -> Option<&'a Url>
    where
        Inner: 'a,
    {
        self.ap_object_ref().likes.as_ref()
    }

    /// Set the likes for the current object
    ///
    /// This overwrites the contents of likes
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::{prelude::*, uri};
    ///
    /// video.set_likes(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_likes(&mut self, likes: Url) -> &mut Self {
        self.ap_object_mut().likes = Some(likes.into());
        self
    }

    /// Take the likes from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(likes) = video.take_likes() {
    ///     println!("{:?}", likes);
    /// }
    /// ```
    fn take_likes(&mut self) -> Option<Url> {
        self.ap_object_mut().likes.take()
    }

    /// Delete the likes from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, object::{ApObject, Video}};
    /// # let mut video = ApObject::new(Video::new());
    /// # video.set_likes(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.likes().is_some());
    /// video.delete_likes();
    /// assert!(video.likes().is_none());
    /// ```
    fn delete_likes(&mut self) -> &mut Self {
        self.ap_object_mut().likes = None;
        self
    }

    /// Fetch the source for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(source) = video.source() {
    ///     println!("{:?}", source);
    /// }
    /// ```
    fn source<'a>(&'a self) -> Option<&'a AnyBase>
    where
        Inner: 'a,
    {
        self.ap_object_ref().source.as_ref()
    }

    /// Set the source for the current object
    ///
    /// This overwrites the contents of source
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::{ApObject, Video}, uri};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// video.set_source(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_source<T>(&mut self, source: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.ap_object_mut().source = Some(source.into());
        self
    }

    /// Take the source from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(source) = video.take_source() {
    ///     println!("{:?}", source);
    /// }
    /// ```
    fn take_source(&mut self) -> Option<AnyBase> {
        self.ap_object_mut().source.take()
    }

    /// Delete the source from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, object::{ApObject, Video}};
    /// # let mut video = ApObject::new(Video::new());
    /// # video.set_source(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.source().is_some());
    /// video.delete_source();
    /// assert!(video.source().is_none());
    /// ```
    fn delete_source(&mut self) -> &mut Self {
        self.ap_object_mut().source = None;
        self
    }

    /// Fetch the upload_media for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(upload_media) = video.upload_media() {
    ///     println!("{:?}", upload_media);
    /// }
    /// ```
    fn upload_media<'a>(&'a self) -> Option<OneOrMany<&'a Url>>
    where
        Inner: 'a,
    {
        self.ap_object_ref()
            .upload_media
            .as_ref()
            .map(|o| o.as_ref())
    }

    /// Set the upload_media for the current object
    ///
    /// This overwrites the contents of upload_media
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::{prelude::*, uri};
    ///
    /// video.set_upload_media(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_upload_media(&mut self, upload_media: Url) -> &mut Self {
        self.ap_object_mut().upload_media = Some(upload_media.into());
        self
    }

    /// Set many upload_medias for the current object
    ///
    /// This overwrites the contents of upload_media
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::{prelude::*, uri};
    ///
    /// video.set_many_upload_medias(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_upload_medias<I, U>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = U>,
        U: Into<Url>,
    {
        let v: Vec<Url> = items.into_iter().map(|u| u.into()).collect();
        self.ap_object_mut().upload_media = Some(v.into());
        self
    }

    /// Add a upload_media to the current object
    ///
    /// This does not overwrite the contents of upload_media, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::{prelude::*, uri};
    ///
    /// video
    ///     .add_upload_media(uri!("https://example.com/one"))
    ///     .add_upload_media(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_upload_media(&mut self, upload_media: Url) -> &mut Self {
        let v = match self.ap_object_mut().upload_media.take() {
            Some(mut v) => {
                v.add(upload_media);
                v
            }
            None => vec![upload_media.into()].into(),
        };
        self.ap_object_mut().upload_media = Some(v);
        self
    }

    /// Take the upload_media from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::{ApObject, Video};
    /// # let mut video = ApObject::new(Video::new());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(upload_media) = video.take_upload_media() {
    ///     println!("{:?}", upload_media);
    /// }
    /// ```
    fn take_upload_media(&mut self) -> Option<OneOrMany<Url>> {
        self.ap_object_mut().upload_media.take()
    }

    /// Delete the upload_media from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, object::{ApObject, Video}};
    /// # let mut video = ApObject::new(Video::new());
    /// # video.set_upload_media(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.upload_media().is_some());
    /// video.delete_upload_media();
    /// assert!(video.upload_media().is_none());
    /// ```
    fn delete_upload_media(&mut self) -> &mut Self {
        self.ap_object_mut().upload_media = None;
        self
    }
}

/// Helper methods for interacting with Place types
///
/// This trait represents methods valid for any Place.
///
/// Documentation for the fields related to these methods can be found on the `Place` struct
pub trait PlaceExt: AsPlace {
    /// Fetch the accuracy of the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(accuracy) = place.accuracy() {
    ///     println!("{:?}", accuracy);
    /// }
    /// ```
    fn accuracy(&self) -> Option<f64> {
        self.place_ref().accuracy
    }

    /// Set the accuracy for the current object
    ///
    /// This overwrites the contents of accuracy
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// place.set_accuracy(5f64);
    /// ```
    fn set_accuracy<T>(&mut self, float: T) -> &mut Self
    where
        T: Into<f64>,
    {
        self.place_mut().accuracy = Some(float.into());
        self
    }

    /// Take the accuracy of the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(accuracy) = place.accuracy() {
    ///     println!("{:?}", accuracy);
    /// }
    /// ```
    fn take_accuracy(&mut self) -> Option<f64> {
        self.place_mut().accuracy.take()
    }

    /// Delete the accuracy from the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// # place.set_accuracy(5f64);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(place.accuracy().is_some());
    /// place.delete_accuracy();
    /// assert!(place.accuracy().is_none());
    /// ```
    fn delete_accuracy(&mut self) -> &mut Self {
        self.place_mut().accuracy = None;
        self
    }

    /// Fetch the altitude of the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(altitude) = place.altitude() {
    ///     println!("{:?}", altitude);
    /// }
    /// ```
    fn altitude(&self) -> Option<f64> {
        self.place_ref().altitude
    }

    /// Set the altitude for the current object
    ///
    /// This overwrites the contents of altitude
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// place.set_altitude(5f64);
    /// ```
    fn set_altitude<T>(&mut self, float: T) -> &mut Self
    where
        T: Into<f64>,
    {
        self.place_mut().altitude = Some(float.into());
        self
    }

    /// Take the altitude of the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(altitude) = place.altitude() {
    ///     println!("{:?}", altitude);
    /// }
    /// ```
    fn take_altitude(&mut self) -> Option<f64> {
        self.place_mut().altitude.take()
    }

    /// Delete the altitude from the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// # place.set_altitude(5f64);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(place.altitude().is_some());
    /// place.delete_altitude();
    /// assert!(place.altitude().is_none());
    /// ```
    fn delete_altitude(&mut self) -> &mut Self {
        self.place_mut().altitude = None;
        self
    }

    /// Fetch the latitude of the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(latitude) = place.latitude() {
    ///     println!("{:?}", latitude);
    /// }
    /// ```
    fn latitude(&self) -> Option<f64> {
        self.place_ref().latitude
    }

    /// Set the latitude for the current object
    ///
    /// This overwrites the contents of latitude
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// place.set_latitude(5f64);
    /// ```
    fn set_latitude<T>(&mut self, float: T) -> &mut Self
    where
        T: Into<f64>,
    {
        self.place_mut().latitude = Some(float.into());
        self
    }

    /// Take the latitude of the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(latitude) = place.latitude() {
    ///     println!("{:?}", latitude);
    /// }
    /// ```
    fn take_latitude(&mut self) -> Option<f64> {
        self.place_mut().latitude.take()
    }

    /// Delete the latitude from the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// # place.set_latitude(5f64);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(place.latitude().is_some());
    /// place.delete_latitude();
    /// assert!(place.latitude().is_none());
    /// ```
    fn delete_latitude(&mut self) -> &mut Self {
        self.place_mut().latitude = None;
        self
    }

    /// Fetch the longitude of the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(longitude) = place.longitude() {
    ///     println!("{:?}", longitude);
    /// }
    /// ```
    fn longitude(&self) -> Option<f64> {
        self.place_ref().longitude
    }

    /// Set the longitude for the current object
    ///
    /// This overwrites the contents of longitude
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// place.set_longitude(5f64);
    /// ```
    fn set_longitude<T>(&mut self, float: T) -> &mut Self
    where
        T: Into<f64>,
    {
        self.place_mut().longitude = Some(float.into());
        self
    }

    /// Take the longitude of the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(longitude) = place.longitude() {
    ///     println!("{:?}", longitude);
    /// }
    /// ```
    fn take_longitude(&mut self) -> Option<f64> {
        self.place_mut().longitude.take()
    }

    /// Delete the longitude from the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// # place.set_longitude(5f64);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(place.longitude().is_some());
    /// place.delete_longitude();
    /// assert!(place.longitude().is_none());
    /// ```
    fn delete_longitude(&mut self) -> &mut Self {
        self.place_mut().longitude = None;
        self
    }

    /// Fetch the radius of the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(radius) = place.radius() {
    ///     println!("{:?}", radius);
    /// }
    /// ```
    fn radius(&self) -> Option<f64> {
        self.place_ref().radius
    }

    /// Set the radius for the current object
    ///
    /// This overwrites the contents of radius
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// place.set_radius(5f64);
    /// ```
    fn set_radius<T>(&mut self, float: T) -> &mut Self
    where
        T: Into<f64>,
    {
        self.place_mut().radius = Some(float.into());
        self
    }

    /// Take the radius of the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(radius) = place.radius() {
    ///     println!("{:?}", radius);
    /// }
    /// ```
    fn take_radius(&mut self) -> Option<f64> {
        self.place_mut().radius.take()
    }

    /// Delete the radius from the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// # place.set_radius(5f64);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(place.radius().is_some());
    /// place.delete_radius();
    /// assert!(place.radius().is_none());
    /// ```
    fn delete_radius(&mut self) -> &mut Self {
        self.place_mut().radius = None;
        self
    }

    /// Fetch the units of the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(units) = place.units() {
    ///     println!("{:?}", units);
    /// }
    /// ```
    fn units(&self) -> Option<&Unit> {
        self.place_ref().units.as_ref()
    }

    /// Set the units for the current object
    ///
    /// This overwrites the contents of units
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::{prelude::*, primitives::Unit};
    ///
    /// place.set_units(Unit::centimeters());
    /// ```
    fn set_units<T>(&mut self, units: T) -> &mut Self
    where
        T: Into<Unit>,
    {
        self.place_mut().units = Some(units.into());
        self
    }

    /// Take the units of the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Place;
    /// # let mut place = Place::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(units) = place.units() {
    ///     println!("{:?}", units);
    /// }
    /// ```
    fn take_units(&mut self) -> Option<Unit> {
        self.place_mut().units.take()
    }

    /// Delete the units from the current object
    ///
    /// ```rust
    /// # use activitystreams::{object::Place, primitives::Unit};
    /// # let mut place = Place::new();
    /// # place.set_units(Unit::centimeters());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(place.units().is_some());
    /// place.delete_units();
    /// assert!(place.units().is_none());
    /// ```
    fn delete_units(&mut self) -> &mut Self {
        self.place_mut().units = None;
        self
    }
}

/// Helper methods for interacting with Profile types
///
/// This trait represents methods valid for any Profile.
///
/// Documentation for the fields related to these methods can be found on the `Profile` struct
pub trait ProfileExt: AsProfile {
    /// Fetch the described object for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Profile;
    /// # let mut profile = Profile::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(describes) = profile.describes() {
    ///     println!("{:?}", describes);
    /// }
    /// ```
    fn describes(&self) -> Option<&AnyBase> {
        self.profile_ref().describes.as_ref()
    }

    /// Set the described object for the current object
    ///
    /// This overwrites the contents of describes
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Profile, uri};
    /// # let mut profile = Profile::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// profile.set_describes(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_describes<T>(&mut self, describes: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.profile_mut().describes = Some(describes.into());
        self
    }

    /// Take the described object from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Profile;
    /// # let mut profile = Profile::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(describes) = profile.take_describes() {
    ///     println!("{:?}", describes);
    /// }
    /// ```
    fn take_describes(&mut self) -> Option<AnyBase> {
        self.profile_mut().describes.take()
    }

    /// Delete the described object from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, object::Profile};
    /// # let mut profile = Profile::new();
    /// # profile.set_describes(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(profile.describes().is_some());
    /// profile.delete_describes();
    /// assert!(profile.describes().is_none());
    /// ```
    fn delete_describes(&mut self) -> &mut Self {
        self.profile_mut().describes = None;
        self
    }
}

/// Helper methods for interacting with Relationship types
///
/// This trait represents methods valid for any Relationship.
///
/// Documentation for the fields related to these methods can be found on the `Relationship` struct
pub trait RelationshipExt: AsRelationship {
    /// Fetch the subject for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Relationship;
    /// # let mut relationship = Relationship::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(subject) = relationship.subject() {
    ///     println!("{:?}", subject);
    /// }
    /// ```
    fn subject(&self) -> Option<&AnyBase> {
        self.relationship_ref().subject.as_ref()
    }

    /// Set the subject for the current object
    ///
    /// This overwrites the contents of subject
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Relationship, uri};
    /// # let mut relationship = Relationship::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// relationship.set_subject(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_subject<T>(&mut self, subject: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.relationship_mut().subject = Some(subject.into());
        self
    }

    /// Take the subject from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Relationship;
    /// # let mut relationship = Relationship::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(subject) = relationship.take_subject() {
    ///     println!("{:?}", subject);
    /// }
    /// ```
    fn take_subject(&mut self) -> Option<AnyBase> {
        self.relationship_mut().subject.take()
    }

    /// Delete the subject from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, object::Relationship};
    /// # let mut relationship = Relationship::new();
    /// # relationship.set_subject(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(relationship.subject().is_some());
    /// relationship.delete_subject();
    /// assert!(relationship.subject().is_none());
    /// ```
    fn delete_subject(&mut self) -> &mut Self {
        self.relationship_mut().subject = None;
        self
    }

    /// Fetch the object for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Relationship;
    /// # let mut relationship = Relationship::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(object) = relationship.object() {
    ///     println!("{:?}", object);
    /// }
    /// ```
    fn object(&self) -> Option<&OneOrMany<AnyBase>> {
        self.relationship_ref().object.as_ref()
    }

    /// Set the object for the current object
    ///
    /// This overwrites the contents of object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Relationship, uri};
    /// # let mut relationship = Relationship::new();
    ///
    /// relationship.set_object(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_object<T>(&mut self, object: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.relationship_mut().object = Some(object.into().into());
        self
    }

    /// Set many objects for the current object
    ///
    /// This overwrites the contents of object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Relationship, uri};
    /// # let mut relationship = Relationship::new();
    ///
    /// relationship.set_many_objects(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_objects<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.relationship_mut().object = Some(v.into());
        self
    }

    /// Add a object to the current object
    ///
    /// This does not overwrite the contents of object, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Relationship, uri};
    /// # let mut relationship = Relationship::new();
    ///
    /// relationship
    ///     .add_object(uri!("https://example.com/one"))
    ///     .add_object(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_object<T>(&mut self, object: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let v = match self.relationship_mut().object.take() {
            Some(mut v) => {
                v.add(object.into());
                v
            }
            None => vec![object.into()].into(),
        };
        self.relationship_mut().object = Some(v);
        self
    }

    /// Take the object from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Relationship;
    /// # let mut relationship = Relationship::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(object) = relationship.take_object() {
    ///     println!("{:?}", object);
    /// }
    /// ```
    fn take_object(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.relationship_mut().object.take()
    }

    /// Delete the object from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Relationship, uri};
    /// # let mut relationship = Relationship::new();
    /// # relationship.set_object(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(relationship.object().is_some());
    /// relationship.delete_object();
    /// assert!(relationship.object().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_object(&mut self) -> &mut Self {
        self.relationship_mut().object = None;
        self
    }

    /// Fetch the relationship for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Relationship;
    /// # let mut relationship = Relationship::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(relationship) = relationship.relationship() {
    ///     println!("{:?}", relationship);
    /// }
    /// ```
    fn relationship(&self) -> Option<&OneOrMany<AnyBase>> {
        self.relationship_ref().relationship.as_ref()
    }

    /// Set the relationship for the current object
    ///
    /// This overwrites the contents of relationship
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Relationship, uri};
    /// # let mut relationship = Relationship::new();
    ///
    /// relationship.set_relationship(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_relationship<T>(&mut self, relationship: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.relationship_mut().relationship = Some(relationship.into().into());
        self
    }

    /// Set many relationships for the current object
    ///
    /// This overwrites the contents of relationship
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Relationship, uri};
    /// # let mut relationship = Relationship::new();
    ///
    /// relationship.set_many_relationships(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_relationships<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.relationship_mut().relationship = Some(v.into());
        self
    }

    /// Add a relationship to the current object
    ///
    /// This does not overwrite the contents of relationship, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Relationship, uri};
    /// # let mut relationship = Relationship::new();
    ///
    /// relationship
    ///     .add_relationship(uri!("https://example.com/one"))
    ///     .add_relationship(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_relationship<T>(&mut self, relationship: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let v = match self.relationship_mut().relationship.take() {
            Some(mut v) => {
                v.add(relationship.into());
                v
            }
            None => vec![relationship.into()].into(),
        };
        self.relationship_mut().relationship = Some(v);
        self
    }

    /// Take the relationship from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Relationship;
    /// # let mut relationship = Relationship::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(relationship) = relationship.take_relationship() {
    ///     println!("{:?}", relationship);
    /// }
    /// ```
    fn take_relationship(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.relationship_mut().relationship.take()
    }

    /// Delete the relationship from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Relationship, uri};
    /// # let mut relationship = Relationship::new();
    /// # relationship.set_relationship(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(relationship.relationship().is_some());
    /// relationship.delete_relationship();
    /// assert!(relationship.relationship().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_relationship(&mut self) -> &mut Self {
        self.relationship_mut().relationship = None;
        self
    }
}

/// Helper methods for interacting with Tombstone types
///
/// This trait represents methods valid for any Tombstone.
///
/// Documentation for the fields related to these methods can be found on the `Tombstone` struct
pub trait TombstoneExt: AsTombstone {
    /// Fetch the former_type for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Tombstone;
    /// # let mut tombstone = Tombstone::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(former_type) = tombstone.former_type() {
    ///     println!("{:?}", former_type);
    /// }
    /// ```
    fn former_type(&self) -> Option<&OneOrMany<AnyBase>> {
        self.tombstone_ref().former_type.as_ref()
    }

    /// Set the former_type for the current object
    ///
    /// This overwrites the contents of former_type
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Tombstone, uri};
    /// # let mut tombstone = Tombstone::new();
    ///
    /// tombstone.set_former_type(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_former_type<T>(&mut self, former_type: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.tombstone_mut().former_type = Some(former_type.into().into());
        self
    }

    /// Set many former_types for the current object
    ///
    /// This overwrites the contents of former_type
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Tombstone, uri};
    /// # let mut tombstone = Tombstone::new();
    ///
    /// tombstone.set_many_former_types(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_former_types<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.tombstone_mut().former_type = Some(v.into());
        self
    }

    /// Add a former_type to the current object
    ///
    /// This does not overwrite the contents of former_type, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Tombstone, uri};
    /// # let mut tombstone = Tombstone::new();
    ///
    /// tombstone
    ///     .add_former_type(uri!("https://example.com/one"))
    ///     .add_former_type(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_former_type<T>(&mut self, former_type: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let v = match self.tombstone_mut().former_type.take() {
            Some(mut v) => {
                v.add(former_type.into());
                v
            }
            None => vec![former_type.into()].into(),
        };
        self.tombstone_mut().former_type = Some(v);
        self
    }

    /// Take the former_type from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Tombstone;
    /// # let mut tombstone = Tombstone::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(former_type) = tombstone.take_former_type() {
    ///     println!("{:?}", former_type);
    /// }
    /// ```
    fn take_former_type(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.tombstone_mut().former_type.take()
    }

    /// Delete the former_type from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Tombstone, uri};
    /// # let mut tombstone = Tombstone::new();
    /// # tombstone.set_former_type(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(tombstone.former_type().is_some());
    /// tombstone.delete_former_type();
    /// assert!(tombstone.former_type().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_former_type(&mut self) -> &mut Self {
        self.tombstone_mut().former_type = None;
        self
    }

    /// Fetch the deleted for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Tombstone;
    /// # let mut tombstone = Tombstone::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(deleted) = tombstone.deleted() {
    ///     println!("{:?}", deleted);
    /// }
    /// ```
    fn deleted(&self) -> Option<DateTime<FixedOffset>> {
        self.tombstone_ref()
            .deleted
            .as_ref()
            .map(|d| d.clone().into_inner())
    }

    /// Set the deleted for the current object
    ///
    /// This overwrites the contents of deleted
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Tombstone;
    /// # let mut tombstone = Tombstone::new();
    ///
    /// tombstone.set_deleted("2020-04-20T04:20:00Z".parse()?);
    /// # Ok(())
    /// # }
    /// ```
    fn set_deleted(&mut self, deleted: DateTime<FixedOffset>) -> &mut Self {
        self.tombstone_mut().deleted = Some(deleted.into());
        self
    }

    /// Take the deleted from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Tombstone;
    /// # let mut tombstone = Tombstone::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(deleted) = tombstone.take_deleted() {
    ///     println!("{:?}", deleted);
    /// }
    /// ```
    fn take_deleted(&mut self) -> Option<DateTime<FixedOffset>> {
        self.tombstone_mut().deleted.take().map(|d| d.into_inner())
    }

    /// Delete the deleted from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::Tombstone;
    /// # let mut tombstone = Tombstone::new();
    /// # tombstone.set_deleted("2020-04-20T04:20:00Z".parse()?);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(tombstone.deleted().is_some());
    /// tombstone.delete_deleted();
    /// assert!(tombstone.deleted().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_deleted(&mut self) -> &mut Self {
        self.tombstone_mut().deleted = None;
        self
    }
}

/// Represents any kind of multi-paragraph written work.
///
/// This is just an alias for `Object<ArticleType>` because there's no fields inherent to Article
/// that aren't already present on an Object.
pub type Article = Object<ArticleType>;

/// Represents an audio document of any kind.
///
/// This is just an alias for `Object<AudioType>` because there's no fields inherent to Audio
/// that aren't already present on an Object.
pub type Audio = Object<AudioType>;

/// Represents a document of any kind.
///
/// This is just an alias for `Object<DocumentType>` because there's no fields inherent to Document
/// that aren't already present on an Object.
pub type Document = Object<DocumentType>;

/// Represents any kind of event.
///
/// This is just an alias for `Object<EventType>` because there's no fields inherent to Event
/// that aren't already present on an Object.
pub type Event = Object<EventType>;

/// An image document of any kind.
///
/// This is just an alias for `Object<ImageType>` because there's no fields inherent to Image
/// that aren't already present on an Object.
pub type Image = Object<ImageType>;

/// Represents a short written work typically less than a single paragraph in length.
///
/// This is just an alias for `Object<NoteType>` because there's no fields inherent to Note
/// that aren't already present on an Object.
pub type Note = Object<NoteType>;

/// Represents a Web Page.
///
/// This is just an alias for `Object<PageType>` because there's no fields inherent to Page
/// that aren't already present on an Object.
pub type Page = Object<PageType>;

/// Represents a video document of any kind.
///
/// This is just an alias for `Object<VideoType>` because there's no fields inherent to Video
/// that aren't already present on an Object.
pub type Video = Object<VideoType>;

/// Describes an object of any kind.
///
/// The Object type serves as the base type for most of the other kinds of objects defined in the
/// Activity Vocabulary, including other Core types such as Activity, IntransitiveActivity,
/// Collection and OrderedCollection.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Object<Kind> {
    /// Identifies a resource attached or related to an object that potentially requires special
    /// handling.
    ///
    /// The intent is to provide a model that is at least semantically similar to attachments in
    /// email.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment: Option<OneOrMany<AnyBase>>,

    /// Identifies one or more entities to which this object is attributed.
    ///
    /// The attributed entities might not be Actors. For instance, an object might be attributed to
    /// the completion of another activity.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    attributed_to: Option<OneOrMany<AnyBase>>,

    /// Identifies one or more entities that represent the total population of entities for which
    /// the object can considered to be relevant.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    audience: Option<OneOrMany<AnyBase>>,

    /// The content or textual representation of the Object encoded as a JSON string.
    ///
    /// By default, the value of content is HTML. The mediaType property can be used in the object
    /// to indicate a different content type.
    ///
    /// The content MAY be expressed using multiple language-tagged values.
    ///
    /// - Range: xsd:string | rdf:langString
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<OneOrMany<AnyString>>,

    /// A natural language summarization of the object encoded as HTML.
    ///
    /// Multiple language tagged summaries MAY be provided.
    ///
    /// - Range: xsd:string | rdf:langString
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<OneOrMany<AnyString>>,

    /// Identifies one or more links to representations of the object.
    ///
    /// - Range: xsd:anyUri | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<OneOrMany<AnyBase>>,

    /// Identifies the entity (e.g. an application) that generated the object.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    generator: Option<OneOrMany<AnyBase>>,

    /// Indicates an entity that describes an icon for this object.
    ///
    /// The image should have an aspect ratio of one (horizontal) to one (vertical) and should be
    /// suitable for presentation at a small size.
    ///
    /// - Range: Image | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<OneOrMany<AnyBase>>,

    /// Indicates an entity that describes an image for this object.
    ///
    /// Unlike the icon property, there are no aspect ratio or display size limitations assumed.
    ///
    /// - Range: Image | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<OneOrMany<AnyBase>>,

    /// Indicates one or more physical or logical locations associated with the object.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<OneOrMany<AnyBase>>,

    /// One or more "tags" that have been associated with an objects. A tag can be any kind of Object.
    ///
    /// The key difference between attachment and tag is that the former implies association by
    /// inclusion, while the latter implies associated by reference.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<OneOrMany<AnyBase>>,

    /// The date and time describing the actual or expected starting time of the object.
    ///
    /// When used with an Activity object, for instance, the start_time property specifies the
    /// moment the activity began or is scheduled to begin.
    ///
    /// - Range: xsd:DateTime
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<XsdDateTime>,

    /// The date and time describing the actual or expected ending time of the object.
    ///
    /// When used with an Activity object, for instance, the endTime property specifies the moment
    /// the activity concluded or is expected to conclude.
    ///
    /// - Range: xsd:dateTime
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<XsdDateTime>,

    /// When the object describes a time-bound resource, such as an audio or video, a meeting, etc,
    /// the duration property indicates the object's approximate duration.
    ///
    /// The value MUST be expressed as an xsd:duration as defined by
    /// [xmlschema11-2](https://www.w3.org/TR/xmlschema11-2/), section 3.3.6 (e.g. a period of 5
    /// seconds is represented as "PT5S").
    ///
    /// - Range: xsd:duration
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<XsdDuration>,

    /// The date and time at which the object was published.
    ///
    /// - Range: xsd:dateTime
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    published: Option<XsdDateTime>,

    /// The date and time at which the object was updated,
    ///
    /// - Range: xsd:dateTime
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<XsdDateTime>,

    /// Indicates one or more entities for which this object is considered a response.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<OneOrMany<AnyBase>>,

    /// Identifies a Collection containing objects considered to be responses to this object.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    replies: Option<OneOrMany<AnyBase>>,

    /// Identifies an entity considered to be part of the public primary audience of an Object.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<OneOrMany<AnyBase>>,

    /// Identifies an Object that is part of the private primary audience of this Object.
    ///
    /// Range: Object | Link
    /// Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    bto: Option<OneOrMany<AnyBase>>,

    /// Identifies an Object that is part of the public secondary audience of this Object.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    cc: Option<OneOrMany<AnyBase>>,

    /// Identifies one or more Objects that are part of the private secondary audience of this Object.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    bcc: Option<OneOrMany<AnyBase>>,

    /// Base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Base<Kind>,
}

/// Define activitypub properties for the Object type as described by the Activity Pub vocabulary.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApObject<Inner> {
    /// This is a list of all Announce activities with this object as the object property, added as
    /// a side effect.
    ///
    /// The shares collection MUST be either an OrderedCollection or a Collection and MAY be
    /// filtered on privileges of an authenticated user or as appropriate when no authentication is
    /// given.
    ///
    /// - Range: anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    shares: Option<Url>,

    /// This is a list of all Like activities with this object as the object property, added as a
    /// side effect.
    ///
    /// The likes collection MUST be either an OrderedCollection or a Collection and MAY be
    /// filtered on privileges of an authenticated user or as appropriate when no authentication is
    /// given.
    ///
    /// - Range: anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    likes: Option<Url>,

    /// The source property is intended to convey some sort of source from which the content markup
    /// was derived, as a form of provenance, or to support future editing by clients.
    ///
    /// In general, clients do the conversion from source to content, not the other way around.
    ///
    /// The value of source is itself an object which uses its own content and mediaType fields to
    /// supply source information.
    ///
    /// - Range: Object
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<AnyBase>,

    /// Servers MAY support uploading document types to be referenced in activites, such as images,
    /// video or other binary data, but the precise mechanism is out of scope for this version of
    /// ActivityPub.
    ///
    /// The Social Web Community Group is refining the protocol in the
    /// [ActivityPub Media Upload report](https://www.w3.org/wiki/SocialCG/ActivityPub/MediaUpload).
    ///
    /// - Range: anyUri
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_media: Option<OneOrMany<Url>>,

    /// The ActivityStreams object being extended
    #[serde(flatten)]
    inner: Inner,
}

/// Represents a logical or physical location.
///
/// The Place object is used to represent both physical and logical locations. While numerous
/// existing vocabularies exist for describing locations in a variety of ways, inconsistencies and
/// incompatibilities between those vocabularies make it difficult to achieve appropriate
/// interoperability between implementations. The Place object is included within the Activity
/// vocabulary to provide a minimal, interoperable starting point for describing locations
/// consistently across Activity Streams 2.0 implementations.
///
/// The Place object is intentionally flexible. It can, for instance, be used to identify a
/// location simply by name, or by longitude and latitude.
///
/// The Place object can also describe an area around a given point using the radius property, the
/// altitude of the location, and a degree of accuracy.
///
/// While publishers are not required to use these specific properties and MAY make use of other
/// mechanisms for describing locations, consuming implementations that support the Place object
/// MUST support the use of these properties.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Place {
    /// Indicates the accuracy of position coordinates on a Place objects.
    ///
    /// Expressed in properties of percentage. e.g. "94.0"means "94.0% accurate".
    ///
    /// - Range: xsd:float [>= 0.0f, <= 100.0f]
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    accuracy: Option<f64>,

    /// Indicates the altitude of a place. The measurement units is indicated using the units
    /// property.
    ///
    /// If units is not specified, the default is assumed to be "m" indicating meters.
    ///
    /// - Range: xsd:float
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    altitude: Option<f64>,

    ///The latitude of a place.
    ///
    /// - Range: xsd:float
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    latitude: Option<f64>,

    /// The longitude of a place.
    ///
    /// - Range: xsd:float
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    longitude: Option<f64>,

    /// The radius from the given latitude and longitude for a Place.
    ///
    /// The units is expressed by the units property. If units is not specified, the default is
    /// assumed to be "m" indicating meters.
    ///
    /// - Range: xsd:float
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<f64>,

    /// Specifies the measurement units for the radius and altitude properties on a Place object.
    ///
    /// If not specified, the default is assumed to be "m" for meters.
    ///
    /// - Range: "cm" | "feet" | "inches" | "km" | "m" | xsd:anyUri | xsd:anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    units: Option<Unit>,

    /// The object being extended
    #[serde(flatten)]
    inner: Object<PlaceType>,
}

/// A Profile is a content object that describes another Object, typically used to describe Actor
/// Type objects.
///
/// The describes property is used to reference the object being described by the profile.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Profile {
    /// On a Profile object, the describes property identifies the object described by the Profile.
    ///
    /// - Range: Object
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    describes: Option<AnyBase>,

    /// The object being extended
    #[serde(flatten)]
    inner: Object<ProfileType>,
}

/// Describes a relationship between two individuals.
///
/// The subject and object properties are used to identify the connected individuals.
///
/// The Relationship object is used to represent relationships between individuals. It can be used,
/// for instance, to describe that one person is a friend of another, or that one person is a
/// member of a particular organization. The intent of modeling Relationship in this way is to
/// allow descriptions of activities that operate on the relationships in general, and to allow
/// representation of Collections of relationships.
///
/// For instance, many social systems have a notion of a "friends list". These are the collection
/// of individuals that are directly connected within a person's social graph. Suppose we have a
/// user, Sally, with direct relationships to users Joe and Jane. Sally follows Joe's updates while
/// Sally and Jane have a mutual relationship.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Relationship {
    /// On a Relationship object, the subject property identifies one of the connected individuals.
    ///
    /// For instance, for a Relationship object describing "John is related to Sally", subject
    /// would refer to John.
    ///
    /// - Range: Object | Link
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<AnyBase>,

    /// When used within a Relationship describes the entity to which the subject is related.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    object: Option<OneOrMany<AnyBase>>,

    /// On a Relationship object, the relationship property identifies the kind of relationship
    /// that exists between subject and object.
    ///
    /// - Range: Object
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    relationship: Option<OneOrMany<AnyBase>>,

    /// The object being extended
    #[serde(flatten)]
    inner: Object<RelationshipType>,
}

/// A Tombstone represents a content object that has been deleted.
///
/// It can be used in Collections to signify that there used to be an object at this position, but
/// it has been deleted.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Tombstone {
    /// On a Tombstone object, the formerType property identifies the type of the object that was
    /// deleted.
    ///
    /// - Range: Object
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    former_type: Option<OneOrMany<AnyBase>>,

    /// On a Tombstone object, the deleted property is a timestamp for when the object was deleted.
    ///
    /// - Range: xsd:dateTime
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    deleted: Option<XsdDateTime>,

    /// The object being extended
    #[serde(flatten)]
    inner: Object<TombstoneType>,
}

impl<Kind> Object<Kind> {
    /// Create a new Object
    ///
    /// ```rust
    /// use activitystreams::object::Object;
    ///
    /// let object = Object::<String>::new();
    /// ```
    pub fn new() -> Self
    where
        Kind: Default,
    {
        Object {
            attachment: None,
            attributed_to: None,
            audience: None,
            content: None,
            summary: None,
            url: None,
            generator: None,
            icon: None,
            image: None,
            location: None,
            tag: None,
            start_time: None,
            end_time: None,
            duration: None,
            published: None,
            updated: None,
            in_reply_to: None,
            replies: None,
            to: None,
            bto: None,
            cc: None,
            bcc: None,
            inner: Base::new(),
        }
    }

    fn extending(mut base: Base<Kind>) -> Result<Self, serde_json::Error> {
        Ok(Object {
            attachment: base.remove("attachment")?,
            attributed_to: base.remove("attributedTo")?,
            audience: base.remove("audience")?,
            content: base.remove("content")?,
            summary: base.remove("summary")?,
            url: base.remove("url")?,
            generator: base.remove("generator")?,
            icon: base.remove("image")?,
            image: base.remove("image")?,
            location: base.remove("location")?,
            tag: base.remove("tag")?,
            start_time: base.remove("startTime")?,
            end_time: base.remove("endTime")?,
            duration: base.remove("duration")?,
            published: base.remove("published")?,
            updated: base.remove("updated")?,
            in_reply_to: base.remove("inReplyTo")?,
            replies: base.remove("replies")?,
            to: base.remove("to")?,
            bto: base.remove("bto")?,
            cc: base.remove("cc")?,
            bcc: base.remove("bcc")?,
            inner: base,
        })
    }

    fn retracting(self) -> Result<Base<Kind>, serde_json::Error> {
        let Object {
            attachment,
            attributed_to,
            audience,
            content,
            summary,
            url,
            generator,
            icon,
            image,
            location,
            tag,
            start_time,
            end_time,
            duration,
            published,
            updated,
            in_reply_to,
            replies,
            to,
            bto,
            cc,
            bcc,
            mut inner,
        } = self;

        inner
            .insert("attachment", attachment)?
            .insert("attributedTo", attributed_to)?
            .insert("audience", audience)?
            .insert("content", content)?
            .insert("summary", summary)?
            .insert("url", url)?
            .insert("generator", generator)?
            .insert("icon", icon)?
            .insert("image", image)?
            .insert("location", location)?
            .insert("tag", tag)?
            .insert("startTime", start_time)?
            .insert("endTime", end_time)?
            .insert("duration", duration)?
            .insert("published", published)?
            .insert("updated", updated)?
            .insert("inReplyTo", in_reply_to)?
            .insert("replies", replies)?
            .insert("to", to)?
            .insert("bto", bto)?
            .insert("cc", cc)?
            .insert("bcc", bcc)?;

        Ok(inner)
    }
}

impl<Inner> ApObject<Inner> {
    /// Create a new ActivityPub Object
    ///
    /// ```rust
    /// use activitystreams::object::{ApObject, Place};
    ///
    /// let object = ApObject::new(Place::new());
    /// ```
    pub fn new(inner: Inner) -> Self
    where
        Inner: markers::Object,
    {
        ApObject {
            shares: None,
            likes: None,
            source: None,
            upload_media: None,
            inner,
        }
    }

    /// Deconstruct the ApObject into its parts
    ///
    /// ```rust
    /// use activitystreams::object::{ApObject, Image};
    ///
    /// let object = ApObject::new(Image::new());
    ///
    /// let (shares, likes, source, upload_media, image) = object.into_parts();
    /// ```
    pub fn into_parts(
        self,
    ) -> (
        Option<Url>,
        Option<Url>,
        Option<AnyBase>,
        Option<OneOrMany<Url>>,
        Inner,
    ) {
        (
            self.shares,
            self.likes,
            self.source,
            self.upload_media,
            self.inner,
        )
    }

    fn extending(mut inner: Inner) -> Result<Self, serde_json::Error>
    where
        Inner: UnparsedMut + markers::Object,
    {
        let shares = inner.remove("shares")?;
        let likes = inner.remove("likes")?;
        let source = inner.remove("source")?;
        let upload_media = inner.remove("uploadMedia")?;

        Ok(ApObject {
            shares,
            likes,
            source,
            upload_media,
            inner,
        })
    }

    fn retracting(self) -> Result<Inner, serde_json::Error>
    where
        Inner: UnparsedMut + markers::Object,
    {
        let ApObject {
            shares,
            likes,
            source,
            upload_media,
            mut inner,
        } = self;

        inner
            .insert("uploadMedia", upload_media)?
            .insert("source", source)?
            .insert("likes", likes)?
            .insert("shares", shares)?;

        Ok(inner)
    }

    /// Borrow inner
    pub fn inner(&self) -> &Inner {
        &self.inner
    }

    /// Mutably borrow Inner
    pub fn inner_mut(&mut self) -> &mut Inner {
        &mut self.inner
    }
}

impl Place {
    /// Create a new ActivityPub Object
    ///
    /// ```rust
    /// use activitystreams::object::Place;
    ///
    /// let object = Place::new();
    /// ```
    pub fn new() -> Self {
        Place {
            accuracy: None,
            altitude: None,
            latitude: None,
            longitude: None,
            radius: None,
            units: None,
            inner: Object::new(),
        }
    }

    fn extending(mut inner: Object<PlaceType>) -> Result<Self, serde_json::Error> {
        let accuracy = inner.remove("accuracy")?;
        let altitude = inner.remove("altitude")?;
        let latitude = inner.remove("latitude")?;
        let longitude = inner.remove("longitude")?;
        let radius = inner.remove("radius")?;
        let units = inner.remove("units")?;

        Ok(Place {
            accuracy,
            altitude,
            latitude,
            longitude,
            radius,
            units,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<PlaceType>, serde_json::Error> {
        let Place {
            accuracy,
            altitude,
            latitude,
            longitude,
            radius,
            units,
            mut inner,
        } = self;

        inner
            .insert("units", units)?
            .insert("radius", radius)?
            .insert("longitude", longitude)?
            .insert("latitude", latitude)?
            .insert("altitude", altitude)?
            .insert("accuracy", accuracy)?;

        Ok(inner)
    }
}

impl Profile {
    /// Create a new ActivityPub Object
    ///
    /// ```rust
    /// use activitystreams::object::Profile;
    ///
    /// let object = Profile::new();
    /// ```
    pub fn new() -> Self {
        Profile {
            describes: None,
            inner: Object::new(),
        }
    }

    fn extending(mut inner: Object<ProfileType>) -> Result<Self, serde_json::Error> {
        let describes = inner.remove("describes")?;

        Ok(Profile { describes, inner })
    }

    fn retracting(self) -> Result<Object<ProfileType>, serde_json::Error> {
        let Profile {
            describes,
            mut inner,
        } = self;

        inner.insert("describes", describes)?;
        Ok(inner)
    }
}

impl Relationship {
    /// Create a new ActivityPub Object
    ///
    /// ```rust
    /// use activitystreams::object::Relationship;
    ///
    /// let object = Relationship::new();
    /// ```
    pub fn new() -> Self {
        Relationship {
            subject: None,
            object: None,
            relationship: None,
            inner: Object::new(),
        }
    }

    fn extending(mut inner: Object<RelationshipType>) -> Result<Self, serde_json::Error> {
        let subject = inner.remove("subject")?;
        let object = inner.remove("object")?;
        let relationship = inner.remove("relationship")?;

        Ok(Relationship {
            subject,
            object,
            relationship,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<RelationshipType>, serde_json::Error> {
        let Relationship {
            subject,
            object,
            relationship,
            mut inner,
        } = self;

        inner
            .insert("subject", subject)?
            .insert("object", object)?
            .insert("relationship", relationship)?;

        Ok(inner)
    }
}

impl Tombstone {
    /// Create a new ActivityPub Object
    ///
    /// ```rust
    /// use activitystreams::object::Tombstone;
    ///
    /// let object = Tombstone::new();
    /// ```
    pub fn new() -> Self {
        Tombstone {
            former_type: None,
            deleted: None,
            inner: Object::new(),
        }
    }

    fn extending(mut inner: Object<TombstoneType>) -> Result<Self, serde_json::Error> {
        let former_type = inner.remove("formerType")?;
        let deleted = inner.remove("deleted")?;

        Ok(Tombstone {
            former_type,
            deleted,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<TombstoneType>, serde_json::Error> {
        let Tombstone {
            former_type,
            deleted,
            mut inner,
        } = self;

        inner
            .insert("formerType", former_type)?
            .insert("deleted", deleted)?;

        Ok(inner)
    }
}

impl<Kind> Extends<Kind> for Object<Kind> {
    type Error = serde_json::Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        Self::extending(base)
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        self.retracting()
    }
}

impl<Kind> TryFrom<Base<Kind>> for Object<Kind> {
    type Error = serde_json::Error;

    fn try_from(base: Base<Kind>) -> Result<Self, Self::Error> {
        Self::extending(base)
    }
}

impl<Kind> TryFrom<Object<Kind>> for Base<Kind> {
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        object.retracting()
    }
}

impl<Inner, Kind> Extends<Kind> for ApObject<Inner>
where
    Inner: Extends<Kind, Error = serde_json::Error> + UnparsedMut + markers::Object,
{
    type Error = serde_json::Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl Extends<PlaceType> for Place {
    type Error = serde_json::Error;

    fn extends(base: Base<PlaceType>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<PlaceType>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl TryFrom<Object<PlaceType>> for Place {
    type Error = serde_json::Error;

    fn try_from(object: Object<PlaceType>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl TryFrom<Place> for Object<PlaceType> {
    type Error = serde_json::Error;

    fn try_from(place: Place) -> Result<Self, Self::Error> {
        place.retracting()
    }
}

impl Extends<ProfileType> for Profile {
    type Error = serde_json::Error;

    fn extends(base: Base<ProfileType>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<ProfileType>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl TryFrom<Object<ProfileType>> for Profile {
    type Error = serde_json::Error;

    fn try_from(object: Object<ProfileType>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl TryFrom<Profile> for Object<ProfileType> {
    type Error = serde_json::Error;

    fn try_from(profile: Profile) -> Result<Self, Self::Error> {
        profile.retracting()
    }
}

impl Extends<RelationshipType> for Relationship {
    type Error = serde_json::Error;

    fn extends(base: Base<RelationshipType>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<RelationshipType>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl TryFrom<Object<RelationshipType>> for Relationship {
    type Error = serde_json::Error;

    fn try_from(object: Object<RelationshipType>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl TryFrom<Relationship> for Object<RelationshipType> {
    type Error = serde_json::Error;

    fn try_from(relationship: Relationship) -> Result<Self, Self::Error> {
        relationship.retracting()
    }
}

impl Extends<TombstoneType> for Tombstone {
    type Error = serde_json::Error;

    fn extends(base: Base<TombstoneType>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<TombstoneType>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl TryFrom<Object<TombstoneType>> for Tombstone {
    type Error = serde_json::Error;

    fn try_from(object: Object<TombstoneType>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl TryFrom<Tombstone> for Object<TombstoneType> {
    type Error = serde_json::Error;

    fn try_from(tombstone: Tombstone) -> Result<Self, Self::Error> {
        tombstone.retracting()
    }
}

impl<Kind> UnparsedMut for Object<Kind> {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Inner> UnparsedMut for ApObject<Inner>
where
    Inner: UnparsedMut,
{
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl UnparsedMut for Place {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl UnparsedMut for Profile {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl UnparsedMut for Relationship {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl UnparsedMut for Tombstone {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Kind> markers::Base for Object<Kind> {}
impl<Kind> markers::Object for Object<Kind> {}

impl<Inner> markers::Base for ApObject<Inner> where Inner: markers::Base {}
impl<Inner> markers::Object for ApObject<Inner> where Inner: markers::Object {}

impl markers::Base for Place {}
impl markers::Object for Place {}

impl markers::Base for Profile {}
impl markers::Object for Profile {}

impl markers::Base for Relationship {}
impl markers::Object for Relationship {}

impl markers::Base for Tombstone {}
impl markers::Object for Tombstone {}

impl<T, Kind> ObjectExt<Kind> for T where T: AsObject<Kind> {}
impl<T, Inner> ApObjectExt<Inner> for T where T: AsApObject<Inner> {}
impl<T> PlaceExt for T where T: AsPlace {}
impl<T> ProfileExt for T where T: AsProfile {}
impl<T> RelationshipExt for T where T: AsRelationship {}
impl<T> TombstoneExt for T where T: AsTombstone {}

impl<Kind> AsBase<Kind> for Object<Kind> {
    fn base_ref(&self) -> &Base<Kind> {
        &self.inner
    }

    fn base_mut(&mut self) -> &mut Base<Kind> {
        &mut self.inner
    }
}

impl<Kind> AsObject<Kind> for Object<Kind> {
    fn object_ref(&self) -> &Object<Kind> {
        self
    }

    fn object_mut(&mut self) -> &mut Object<Kind> {
        self
    }
}

impl<Inner, Kind> AsBase<Kind> for ApObject<Inner>
where
    Inner: AsBase<Kind>,
{
    fn base_ref(&self) -> &Base<Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Kind> {
        self.inner.base_mut()
    }
}

impl<Inner, Kind> AsObject<Kind> for ApObject<Inner>
where
    Inner: AsObject<Kind>,
{
    fn object_ref(&self) -> &Object<Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Kind> {
        self.inner.object_mut()
    }
}

impl<Inner> AsApObject<Inner> for ApObject<Inner>
where
    Inner: markers::Object,
{
    fn ap_object_ref(&self) -> &ApObject<Inner> {
        self
    }

    fn ap_object_mut(&mut self) -> &mut ApObject<Inner> {
        self
    }
}

impl<Inner> AsPlace for ApObject<Inner>
where
    Inner: AsPlace,
{
    fn place_ref(&self) -> &Place {
        self.inner.place_ref()
    }

    fn place_mut(&mut self) -> &mut Place {
        self.inner.place_mut()
    }
}

impl<Inner> AsProfile for ApObject<Inner>
where
    Inner: AsProfile,
{
    fn profile_ref(&self) -> &Profile {
        self.inner.profile_ref()
    }

    fn profile_mut(&mut self) -> &mut Profile {
        self.inner.profile_mut()
    }
}

impl<Inner> AsRelationship for ApObject<Inner>
where
    Inner: AsRelationship,
{
    fn relationship_ref(&self) -> &Relationship {
        self.inner.relationship_ref()
    }

    fn relationship_mut(&mut self) -> &mut Relationship {
        self.inner.relationship_mut()
    }
}

impl<Inner> AsTombstone for ApObject<Inner>
where
    Inner: AsTombstone,
{
    fn tombstone_ref(&self) -> &Tombstone {
        self.inner.tombstone_ref()
    }

    fn tombstone_mut(&mut self) -> &mut Tombstone {
        self.inner.tombstone_mut()
    }
}

impl AsBase<PlaceType> for Place {
    fn base_ref(&self) -> &Base<PlaceType> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<PlaceType> {
        self.inner.base_mut()
    }
}

impl AsObject<PlaceType> for Place {
    fn object_ref(&self) -> &Object<PlaceType> {
        &self.inner
    }

    fn object_mut(&mut self) -> &mut Object<PlaceType> {
        &mut self.inner
    }
}

impl AsPlace for Place {
    fn place_ref(&self) -> &Place {
        self
    }

    fn place_mut(&mut self) -> &mut Place {
        self
    }
}

impl AsBase<ProfileType> for Profile {
    fn base_ref(&self) -> &Base<ProfileType> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<ProfileType> {
        self.inner.base_mut()
    }
}

impl AsObject<ProfileType> for Profile {
    fn object_ref(&self) -> &Object<ProfileType> {
        &self.inner
    }

    fn object_mut(&mut self) -> &mut Object<ProfileType> {
        &mut self.inner
    }
}

impl AsProfile for Profile {
    fn profile_ref(&self) -> &Profile {
        self
    }

    fn profile_mut(&mut self) -> &mut Profile {
        self
    }
}

impl AsBase<RelationshipType> for Relationship {
    fn base_ref(&self) -> &Base<RelationshipType> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<RelationshipType> {
        self.inner.base_mut()
    }
}

impl AsObject<RelationshipType> for Relationship {
    fn object_ref(&self) -> &Object<RelationshipType> {
        &self.inner
    }

    fn object_mut(&mut self) -> &mut Object<RelationshipType> {
        &mut self.inner
    }
}

impl AsRelationship for Relationship {
    fn relationship_ref(&self) -> &Relationship {
        self
    }

    fn relationship_mut(&mut self) -> &mut Relationship {
        self
    }
}

impl AsBase<TombstoneType> for Tombstone {
    fn base_ref(&self) -> &Base<TombstoneType> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<TombstoneType> {
        self.inner.base_mut()
    }
}

impl AsObject<TombstoneType> for Tombstone {
    fn object_ref(&self) -> &Object<TombstoneType> {
        &self.inner
    }

    fn object_mut(&mut self) -> &mut Object<TombstoneType> {
        &mut self.inner
    }
}

impl AsTombstone for Tombstone {
    fn tombstone_ref(&self) -> &Tombstone {
        self
    }

    fn tombstone_mut(&mut self) -> &mut Tombstone {
        self
    }
}
