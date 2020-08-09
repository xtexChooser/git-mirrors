//! Types and traits for dealing with the base attributes common to all ActivityStreams Objects and
//! Links
//!
//! ```rust
//! # fn main() -> Result<(), anyhow::Error> {
//! use activitystreams::{
//!     context,
//!     object::Video,
//!     prelude::*,
//!     security,
//!     uri,
//! };
//! let mut video = Video::new();
//!
//! video
//!     .set_id(uri!("https://example.com"))
//!     .set_context(context())
//!     .add_context(security())
//!     .set_name("Hello");
//!
//! let any_base = video.into_any_base()?;
//!
//! let mut new_video = Video::new();
//!
//! new_video.set_preview(any_base);
//! #
//! # Ok(())
//! # }
//! ```
use crate::{
    either::Either,
    error::DomainError,
    markers,
    primitives::{AnyString, MimeMediaType, OneOrMany},
    unparsed::{Unparsed, UnparsedMut},
};
use mime::Mime;
use url::Url;

/// Implements conversion between `Base<Kind>` and other ActivityStreams objects defined in this
/// crate
pub trait Extends<Kind>: Sized {
    /// The erro produced must be a StdError
    type Error: std::error::Error;

    /// Produce an object from the Base
    fn extends(base: Base<Kind>) -> Result<Self, Self::Error>;

    /// Produce a base from the object
    fn retracts(self) -> Result<Base<Kind>, Self::Error>;
}

/// A helper function implemented for all Extends types to easily produce an AnyBase from a given
/// object.
///
/// This is important because many APIs in this crate deal with AnyBases.
pub trait ExtendsExt<Kind>: Extends<Kind> {
    /// Create an AnyBase from the given object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::{object::Video, prelude::*};
    /// let video = Video::new();
    ///
    /// let any_base = video.into_any_base()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn into_any_base(self) -> Result<AnyBase, Self::Error>
    where
        Kind: serde::ser::Serialize,
        Self::Error: From<serde_json::Error>,
    {
        AnyBase::from_extended(self)
    }

    /// Create an object from an AnyBase
    ///
    /// Before calling this, make sure the `AnyBase::is_base()` and `AnyBase::kind` match your
    /// expectations
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::{object::Video, prelude::*};
    /// # let video = Video::new();
    /// # let any_base = video.into_any_base()?;
    /// let video = Video::from_any_base(any_base)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn from_any_base(any_base: AnyBase) -> Result<Option<Self>, Self::Error>
    where
        Kind: serde::de::DeserializeOwned,
        Self::Error: From<serde_json::Error>,
    {
        if let Some(base) = any_base.take_base() {
            let my_base = base.solidify()?;
            let extended = my_base.extend::<Self>()?;

            return Ok(Some(extended));
        }

        Ok(None)
    }
}

/// Implementation trait for deriving Base methods for a type
///
/// Any type implementating AsBase will automatically gain methods provided by BaseExt
pub trait AsBase<Kind>: markers::Base {
    /// Immutable borrow of `Base<Kind>`
    fn base_ref(&self) -> &Base<Kind>;

    /// Mutable borrow of Base<Kind>
    fn base_mut(&mut self) -> &mut Base<Kind>;
}

/// Helper methods for interacting with Base types
///
/// This trait represents methods valid for Any ActivityStreams type, regardless of whether it's a
/// Link or an Object.
///
/// Documentation for the fields related to these methods can be found on the `Base` struct
pub trait BaseExt<Kind>: AsBase<Kind> {
    /// Fetch the context for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// let video_context = video.context();
    /// ```
    fn context<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Kind: 'a,
    {
        self.base_ref().context.as_ref()
    }

    /// Set the context for the current object
    ///
    /// This overwrites the contents of context
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::{context, prelude::*};
    ///
    /// video.set_context(context());
    /// ```
    fn set_context<T>(&mut self, context: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.base_mut().context = Some(context.into().into());
        self
    }

    /// Set many contexts for the current object
    ///
    /// This overwrites the contents of context
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::{context, prelude::*, security};
    ///
    /// video.set_many_contexts(vec![context(), security()]);
    /// ```
    fn set_many_contexts<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.base_mut().context = Some(v.into());
        self
    }

    /// Add a context to the current object
    ///
    /// This does not overwrite the contents of context, only appends a new item
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::{context, prelude::*, security};
    ///
    /// video
    ///     .add_context(context())
    ///     .add_context(security());
    /// ```
    fn add_context<T>(&mut self, context: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let c = match self.base_mut().context.take() {
            Some(mut c) => {
                c.add(context.into());
                c
            }
            None => vec![context.into()].into(),
        };
        self.base_mut().context = Some(c);
        self
    }

    /// Take the context from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(context) = video.take_context() {
    ///     println!("{:?}", context);
    /// }
    /// ```
    fn take_context(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.base_mut().context.take()
    }

    /// Delete the context from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, object::Video};
    /// # let mut video = Video::new();
    /// # video.set_context(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.context().is_some());
    /// video.delete_context();
    /// assert!(video.context().is_none());
    /// ```
    fn delete_context(&mut self) -> &mut Self {
        self.base_mut().context = None;
        self
    }

    /// Fetch the id for the current object, checking it against the provided domain
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_id(uri!("https://example.com"));
    /// use activitystreams::prelude::*;
    ///
    /// assert_eq!(video.id("example.com")?, Some(&uri!("https://example.com")));
    /// # Ok(())
    /// # }
    /// ```
    fn id<'a>(&'a self, domain: &str) -> Result<Option<&'a Url>, DomainError>
    where
        Kind: 'a,
    {
        if let Some(unchecked) = self.id_unchecked() {
            if unchecked.domain() != Some(domain) {
                return Err(DomainError);
            }

            return Ok(Some(unchecked));
        }

        Ok(None)
    }

    /// Fetch the id for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(id) = video.id_unchecked() {
    ///     println!("{:?}", id);
    /// }
    /// ```
    fn id_unchecked<'a>(&'a self) -> Option<&'a Url>
    where
        Kind: 'a,
    {
        self.base_ref().id.as_ref()
    }

    /// Mutably borrow the ID from the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(id) = video.id_mut() {
    ///     id.set_path("/actor");
    ///     println!("{:?}", id);
    /// }
    /// ```
    fn id_mut<'a>(&'a mut self) -> Option<&'a mut Url>
    where
        Kind: 'a,
    {
        self.base_mut().id.as_mut()
    }

    /// Check if the provided id is equal to the object's id
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::{object::Video, prelude::*, uri};
    ///
    /// let video: Video = serde_json::from_str(r#"{"type":"Video","id":"https://example.com"}"#)?;
    ///
    /// assert!(video.is_id(&uri!("https://example.com")));
    /// # Ok(())
    /// # }
    /// ```
    fn is_id(&self, id: &Url) -> bool {
        self.id_unchecked() == Some(id)
    }

    /// Set the id for the current object
    ///
    /// This overwrites the contents of id
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::{prelude::*, uri};
    ///
    /// video.set_id(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_id(&mut self, id: Url) -> &mut Self {
        self.base_mut().id = Some(id);
        self
    }

    /// Take the id from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(id) = video.take_id() {
    ///     println!("{:?}", id);
    /// }
    /// ```
    fn take_id(&mut self) -> Option<Url> {
        self.base_mut().id.take()
    }

    /// Delete the id from the current object
    ///
    /// ```rust
    /// # use activitystreams::{context, object::Video};
    /// # let mut video = Video::new();
    /// # video.set_id(context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.id_unchecked().is_some());
    /// video.delete_id();
    /// assert!(video.id_unchecked().is_none());
    /// ```
    fn delete_id(&mut self) -> &mut Self {
        self.base_mut().id = None;
        self
    }

    /// Fetch the kind for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(kind) = video.kind() {
    ///     println!("{:?}", kind);
    /// }
    /// ```
    fn kind<'a>(&'a self) -> Option<&'a Kind>
    where
        Kind: 'a,
    {
        self.base_ref().kind.as_ref()
    }

    /// Check if the provided Kind is equal to the object's Kind
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::{base::Base, prelude::*};
    ///
    /// #[derive(PartialEq, serde::Deserialize)]
    /// pub enum ValidKinds {
    ///     Video,
    ///     Image,
    /// }
    ///
    /// let video: Base<ValidKinds> = serde_json::from_str(r#"{"type":"Video"}"#)?;
    ///
    /// assert!(video.is_kind(&ValidKinds::Video));
    /// # Ok(())
    /// # }
    /// ```
    fn is_kind(&self, kind: &Kind) -> bool
    where
        Kind: PartialEq,
    {
        self.kind() == Some(kind)
    }

    /// Set the kind for the current object
    ///
    /// This overwrites the contents of kind
    ///
    /// ```rust
    /// # use activitystreams::object::{Video, kind::VideoType};
    /// # let mut video = Video::new();
    /// use activitystreams::prelude::*;
    ///
    /// video.set_kind(VideoType::Video);
    /// ```
    fn set_kind(&mut self, kind: Kind) -> &mut Self {
        self.base_mut().kind = Some(kind);
        self
    }

    /// Take the kind from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(kind) = video.take_kind() {
    ///     println!("{:?}", kind);
    /// }
    /// ```
    fn take_kind(&mut self) -> Option<Kind> {
        self.base_mut().kind.take()
    }

    /// Delete the kind from the current object
    ///
    /// ```rust
    /// # use activitystreams::{object::{Video, kind::VideoType}};
    /// # let mut video = Video::new();
    /// # video.set_kind(VideoType::Video);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.kind().is_some());
    /// video.delete_kind();
    /// assert!(video.kind().is_none());
    /// ```
    fn delete_kind(&mut self) -> &mut Self {
        self.base_mut().kind = None;
        self
    }

    /// Fetch the name for the current object
    ///
    /// ```
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(name) = video.name() {
    ///     println!("{:?}", name);
    /// }
    /// ```
    fn name<'a>(&'a self) -> Option<OneOrMany<&'a AnyString>>
    where
        Kind: 'a,
    {
        self.base_ref().name.as_ref().map(|o| o.as_ref())
    }

    /// Set the name for the current object
    ///
    /// This overwrites the contents of name
    ///
    /// ```rust
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// video.set_name("hi");
    /// ```
    fn set_name<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<AnyString>,
    {
        self.base_mut().name = Some(name.into().into());
        self
    }

    /// Set many names for the current object
    ///
    /// This overwrites the contents of name
    ///
    /// ```rust
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// video.set_many_names(vec!["hi", "hey"]);
    /// ```
    fn set_many_names<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyString>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.base_mut().name = Some(v.into());
        self
    }

    /// Add a name to the current object
    ///
    /// This does not overwrite the contents of name, only appends a new item
    ///
    /// ```rust
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// video
    ///     .add_name("hi")
    ///     .add_name("hey");
    /// ```
    fn add_name<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<AnyString>,
    {
        let a = match self.base_mut().name.take() {
            Some(mut a) => {
                a.add(name.into());
                a
            }
            None => vec![name.into()].into(),
        };
        self.base_mut().name = Some(a);
        self
    }

    /// Take the name from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(name) = video.take_name() {
    ///     println!("{:?}", name);
    /// }
    /// ```
    fn take_name(&mut self) -> Option<OneOrMany<AnyString>> {
        self.base_mut().name.take()
    }

    /// Delete the name from the current object
    ///
    /// ```rust
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// # video.set_name("hi");
    /// #
    ///
    /// assert!(video.name().is_some());
    /// video.delete_name();
    /// assert!(video.name().is_none());
    /// ```
    fn delete_name(&mut self) -> &mut Self {
        self.base_mut().name = None;
        self
    }

    /// Fetch the media type for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(media_type) = video.media_type() {
    ///     println!("{:?}", media_type);
    /// }
    /// ```
    fn media_type<'a>(&'a self) -> Option<&'a Mime>
    where
        Kind: 'a,
    {
        self.base_ref().media_type.as_ref().map(|m| m.as_ref())
    }

    /// Set the media type for the current object
    ///
    /// This overwrites the contents of media_type
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    ///
    /// video.set_media_type("video/webm".parse()?);
    /// # Ok(())
    /// # }
    /// ```
    fn set_media_type(&mut self, media_type: Mime) -> &mut Self {
        self.base_mut().media_type = Some(media_type.into());
        self
    }

    /// Take the media type from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(media_type) = video.take_media_type() {
    ///     println!("{:?}", media_type);
    /// }
    /// ```
    fn take_media_type(&mut self) -> Option<Mime> {
        self.base_mut().media_type.take().map(|m| m.into_inner())
    }

    /// Delete the media type from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video};
    /// # let mut video = Video::new();
    /// # video.set_media_type("video/webm".parse()?);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.media_type().is_some());
    /// video.delete_media_type();
    /// assert!(video.media_type().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_media_type(&mut self) -> &mut Self {
        self.base_mut().media_type = None;
        self
    }

    /// Fetch the preview for the current object
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(preview) = video.preview() {
    ///     println!("{:?}", preview);
    /// }
    /// ```
    fn preview<'a>(&'a self) -> Option<OneOrMany<&'a AnyBase>>
    where
        Kind: 'a,
    {
        self.base_ref().preview.as_ref().map(|o| o.as_ref())
    }

    /// Set the preview for the current object
    ///
    /// This overwrites the contents of preview
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_preview(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_preview<T>(&mut self, preview: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.base_mut().preview = Some(preview.into().into());
        self
    }

    /// Set many previews for the current object
    ///
    /// This overwrites the contents of preview
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video.set_many_previews(vec![
    ///     uri!("https://example.com/one"),
    ///     uri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_previews<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.base_mut().preview = Some(v.into());
        self
    }

    /// Add a preview to the current object
    ///
    /// This does not overwrite the contents of preview, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    ///
    /// video
    ///     .add_preview(uri!("https://example.com/one"))
    ///     .add_preview(uri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_preview<T>(&mut self, preview: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let a = match self.base_mut().preview.take() {
            Some(mut a) => {
                a.add(preview.into());
                a
            }
            None => vec![preview.into()].into(),
        };
        self.base_mut().preview = Some(a);
        self
    }

    /// Take the preview from the current object, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::object::Video;
    /// # let mut video = Video::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(preview) = video.take_preview() {
    ///     println!("{:?}", preview);
    /// }
    /// ```
    fn take_preview(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.base_mut().preview.take()
    }

    /// Delete the preview from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, uri};
    /// # let mut video = Video::new();
    /// # video.set_preview(uri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(video.preview().is_some());
    /// video.delete_preview();
    /// assert!(video.preview().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_preview(&mut self) -> &mut Self {
        self.base_mut().preview = None;
        self
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
struct IdOrBase(Either<Url, Box<Base<serde_json::Value>>>);

/// A type that can represent Any ActivityStreams type
///
/// A type in activitystreams can be four things
/// - An Object
/// - A Link
/// - The ID of that Link or Object
/// - A string representing that Link or Object
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct AnyBase(Either<IdOrBase, String>);

/// A representation of the common fields between Links and Objects in ActivityStreams
///
/// Although the spec does not define a type more abstract that Object or Link, it does define
/// fields present in both, so for the sake of "Everything derives from something," I've
/// implemented a type.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Base<Kind> {
    /// Identifies the context within which the object exists or an activity was performed.
    ///
    /// The notion of "context"used is intentionally vague. The intended function is to serve as a
    /// means of grouping objects and activities that share a common originating context or
    /// purpose. An example could be all activities relating to a common project or event.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(rename = "@context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<OneOrMany<AnyBase>>,

    /// Provides the globally unique identifier for an Object or Link.
    ///
    /// The id property is expressed as an absolute IRI in the spec, but for now is represented as
    /// a string.
    ///
    /// - Range: xsd:anyUri
    /// - Functional: true
    ///
    /// ### From ActivityStreams's deprecation notes
    ///
    /// When processing Activity Streams 1.0 documents and converting those to 2.0, implementations
    /// ought to treat id as an alias for the JSON-LD @id key word[.]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Url>,

    /// The `type` field
    ///
    /// ### From JSON-LD's `type` documentation
    ///
    /// This section is non-normative.
    ///
    /// In Linked Data, it is common to specify the type of a graph node; in many cases, this can
    /// be inferred based on the properties used within a given node object, or the property for
    /// which a node is a value. For example, in the schema.org vocabulary, the givenName property
    /// is associated with a Person. Therefore, one may reason that if a node object contains the
    /// property givenName, that the type is a Person; making this explicit with @type helps to
    /// clarify the association.
    ///
    /// The type of a particular node can be specified using the @type keyword. In Linked Data,
    /// types are uniquely identified with an IRI.
    ///
    /// ### From ActivityStreams's deprecation notes
    ///
    /// When processing Activity Streams 1.0 documents and converting those to 2.0, implementations
    /// ought to treat [...] the objectType and verb properties as aliases for the JSON-LD @type
    /// keyword.
    #[serde(rename = "type")]
    #[serde(alias = "@type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<Kind>,

    /// A simple, human-readable, plain-text name for the object.
    ///
    /// HTML markup MUST NOT be included. The name MAY be expressed using multiple language-tagged values.
    ///
    /// - Range: xsd:string | rdf:langString
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<OneOrMany<AnyString>>,

    /// When used on an Object, identifies the MIME media type of the value of the content property.
    ///
    /// If not specified, the content property is assumed to contain text/html content.
    ///
    /// - Range: Mime Media Type
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    media_type: Option<MimeMediaType>,

    /// Identifies an entity that provides a preview of this object.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<OneOrMany<AnyBase>>,

    /// Any additional data present on the object if parsed from JSON
    ///
    /// This is used to extend the Base into other kinds of objects and links
    #[serde(flatten)]
    unparsed: Unparsed,
}

impl Base<serde_json::Value> {
    /// Convert this `Base<serde_json::Value>` into a `Base<Kind>`
    ///
    /// This is required before extending Base into the other types found in this crate
    pub fn solidify<Kind>(self) -> Result<Base<Kind>, serde_json::Error>
    where
        Kind: serde::de::DeserializeOwned,
    {
        self.try_map_kind(serde_json::from_value)
    }
}

impl<Kind> Base<Kind> {
    /// Create a new Base
    ///
    /// ```rust
    /// use activitystreams::base::Base;
    ///
    /// let base = Base::<String>::new();
    /// ```
    pub fn new() -> Self
    where
        Kind: Default,
    {
        Base {
            context: None,
            id: None,
            kind: Some(Kind::default()),
            name: None,
            media_type: None,
            preview: None,
            unparsed: Default::default(),
        }
    }

    /// Create a new base with `None` for it's `kind` property
    ///
    /// This means that no `type` field will be present in serialized JSON
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::base::Base;
    ///
    /// let base = Base::<()>::new_none_type();
    ///
    /// let s = serde_json::to_string(&base)?;
    ///
    /// assert_eq!(s, "{}");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_none_type() -> Self {
        Base {
            context: None,
            id: None,
            kind: None,
            name: None,
            media_type: None,
            preview: None,
            unparsed: Default::default(),
        }
    }

    /// Extend the Base into any other ActivityStreams type provided in this crate
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// #   use activitystreams::{base::Base, object::{Video, kind::VideoType}};
    /// #   let base = Base::<VideoType>::new();
    /// #
    /// let video = base.extend::<Video>()?;
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub fn extend<T>(self) -> Result<T, T::Error>
    where
        T: Extends<Kind>,
    {
        T::extends(self)
    }

    /// Retract any other ActivityStreams type into a `Base<Kind>`
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// #   use activitystreams::{base::Base, object::Video};
    /// #   let video = Video::new();
    /// #
    /// let base = Base::retract(video)?;
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub fn retract<T>(t: T) -> Result<Self, T::Error>
    where
        T: Extends<Kind>,
    {
        t.retracts()
    }

    /// Convert this Base into a `Base<serde_json::Value>`
    ///
    /// This is required before putting a Base into an AnyBase type
    pub fn into_generic(self) -> Result<Base<serde_json::Value>, serde_json::Error>
    where
        Kind: serde::ser::Serialize,
    {
        self.try_map_kind(serde_json::to_value)
    }

    /// An inffalible conversion from `Base<T>` to `Base<U>` where there is a known path from T to
    /// U
    ///
    /// ```rust
    /// use activitystreams::{base::Base, prelude::*};
    ///
    /// let mut base = Base::<String>::new();
    /// base.set_kind("Hey".to_owned());
    ///
    /// let new_base = base.map_kind(|kind| match kind.as_str() {
    ///     "Create" => 1,
    ///     "Update" => 5,
    ///     _ => 0,
    /// });
    ///
    /// assert!(*new_base.kind().unwrap() == 0);
    /// ```
    pub fn map_kind<NewKind>(self, f: impl Fn(Kind) -> NewKind) -> Base<NewKind> {
        Base {
            kind: self.kind.map(f),
            context: self.context,
            id: self.id,
            name: self.name,
            media_type: self.media_type,
            preview: self.preview,
            unparsed: self.unparsed,
        }
    }

    /// A fallible conversion from `Base<T> to Base<U>`
    ///
    /// ```rust
    /// use activitystreams::{base::Base, prelude::*};
    ///
    /// let mut base = Base::<String>::new();
    /// base.set_kind("Hey".to_owned());
    ///
    /// let new_base = base.try_map_kind(|kind| match kind.as_str() {
    ///     "Create" => Ok(1),
    ///     "Update" => Ok(5),
    ///     _ => Err(anyhow::Error::msg("invalid kind")),
    /// });
    ///
    /// assert!(new_base.is_err());
    /// ```
    pub fn try_map_kind<NewKind, E>(
        self,
        f: impl Fn(Kind) -> Result<NewKind, E>,
    ) -> Result<Base<NewKind>, E> {
        Ok(Base {
            kind: if let Some(kind) = self.kind {
                Some((f)(kind)?)
            } else {
                None
            },
            context: self.context,
            id: self.id,
            name: self.name,
            media_type: self.media_type,
            preview: self.preview,
            unparsed: self.unparsed,
        })
    }
}

impl AnyBase {
    /// Convert any type that is extended from `Base<Kind>` into an AnyBase for storing
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, base::AnyBase};
    /// # let video = Video::new();
    /// let any_base = AnyBase::from_extended(video)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_extended<T, Kind>(extended: T) -> Result<Self, T::Error>
    where
        T: Extends<Kind>,
        T::Error: From<serde_json::Error>,
        Kind: serde::ser::Serialize,
    {
        Ok(Base::retract(extended)?.into_generic()?.into())
    }

    /// Check if this object is a Url
    ///
    /// ```rust
    /// # use activitystreams::{base::AnyBase, uri};
    /// # fn main() -> Result<(), anyhow::Error> {
    /// let any_base = AnyBase::from_xsd_any_uri(uri!("https://example.com"));
    /// assert!(any_base.is_xsd_any_uri());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_xsd_any_uri(&self) -> bool {
        self.0
            .as_ref()
            .left()
            .and_then(|l| l.as_xsd_any_uri())
            .is_some()
    }

    /// Check if this object is a String
    ///
    /// ```rust
    /// # use activitystreams::base::AnyBase;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// let any_base = AnyBase::from_xsd_string("Profile".into());
    /// assert!(any_base.is_xsd_string());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_xsd_string(&self) -> bool {
        self.0.as_ref().right().is_some()
    }

    /// Check if this object is a `Base<serde_json::Value>`
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, base::AnyBase};
    /// # let video = Video::new();
    /// let any_base = AnyBase::from_extended(video)?;
    /// assert!(any_base.is_base());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_base(&self) -> bool {
        self.0.as_ref().left().and_then(|l| l.as_base()).is_some()
    }

    /// Get the id from the current object
    ///
    /// This method checks if the current object _is_ an ID, and then falls back on the `id` field
    /// within the `Base<serde_json::Value>` if that exists
    ///
    /// #### Get the ID from the nested video
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{object::Video, base::AnyBase, prelude::*, uri};
    /// # let mut video = Video::new();
    /// let id = uri!("https://example.com");
    ///
    /// video.set_id(id.clone());
    ///
    /// let any_base = AnyBase::from_extended(video)?;
    /// assert!(any_base.id().unwrap() == &id);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// #### Get the ID from the AnyBase
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::AnyBase, prelude::*, uri};
    /// let id = uri!("https://example.com");
    ///
    /// let any_base = AnyBase::from_xsd_any_uri(id.clone());
    /// assert!(any_base.id().unwrap() == &id);
    /// # Ok(())
    /// # }
    /// ```
    pub fn id(&self) -> Option<&Url> {
        self.as_xsd_any_uri()
            .or_else(|| self.as_base().and_then(|base| base.id.as_ref()))
    }

    /// Check if the current object's id matches the provided id
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{
    /// #   object::{kind::VideoType, Video}, base::AnyBase, prelude::*, uri
    /// # };
    /// # let mut video = Video::new();
    /// #
    /// video.set_id(uri!("https://example.com"));
    ///
    /// let any_base = AnyBase::from_extended(video)?;
    ///
    /// assert!(any_base.is_id(&uri!("https://example.com")));
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_id(&self, id: &Url) -> bool {
        self.id() == Some(id)
    }

    /// Get the kind from the current object
    ///
    /// This method only produces a value if the current object is a `Base<serde_json::Value>`
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{
    /// #   object::{kind::VideoType, Video}, base::AnyBase, prelude::*,
    /// # };
    /// # let mut video = Video::new();
    /// #
    /// video.set_kind(VideoType::Video);
    ///
    /// let any_base = AnyBase::from_extended(video)?;
    ///
    /// match any_base.kind().and_then(|k| k.as_str()) {
    ///     Some("Video") => println!("yay!"),
    ///     _ => return Err(anyhow::Error::msg("invalid type found")),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn kind(&self) -> Option<&serde_json::Value> {
        self.as_base().and_then(|base| base.kind.as_ref())
    }

    /// Get the kind from the current object as an &str
    ///
    /// This method only produces a value if the current object is a `Base<serde_json::Value>`, and
    /// the kind is present, and a string
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{
    /// #   object::{kind::VideoType, Video}, base::AnyBase, prelude::*,
    /// # };
    /// # let mut video = Video::new();
    /// #
    /// video.set_kind(VideoType::Video);
    ///
    /// let any_base = AnyBase::from_extended(video)?;
    ///
    /// match any_base.kind_str() {
    ///     Some("Video") => println!("yay!"),
    ///     _ => return Err(anyhow::Error::msg("invalid type found")),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn kind_str(&self) -> Option<&str> {
        self.kind().and_then(|k| k.as_str())
    }

    /// Check if the current object's kind matches the provided kind
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{
    /// #   object::{kind::VideoType, Video}, base::AnyBase, prelude::*,
    /// # };
    /// # let mut video = Video::new();
    /// #
    /// video.set_kind(VideoType::Video);
    ///
    /// let any_base = AnyBase::from_extended(video)?;
    ///
    /// assert!(any_base.is_kind("Video"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_kind(&self, kind: &str) -> bool {
        self.kind_str() == Some(kind)
    }

    /// Get the object as a Url
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::AnyBase, uri};
    /// #
    /// let any_base = AnyBase::from_xsd_any_uri(uri!("https://example.com"));
    ///
    /// assert!(any_base.as_xsd_any_uri().is_some());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_xsd_any_uri(&self) -> Option<&Url> {
        self.0.as_ref().left().and_then(|l| l.as_xsd_any_uri())
    }

    /// Get the object as an &str
    ///
    /// ```rust
    /// # use activitystreams::base::AnyBase;
    /// #
    /// let any_base = AnyBase::from_xsd_string("hi".into());
    ///
    /// assert!(any_base.as_xsd_string().is_some());
    /// ```
    pub fn as_xsd_string(&self) -> Option<&str> {
        self.0.as_ref().right().map(|r| r.as_str())
    }

    /// Get the object as a `Base<serde_json::Value>`
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::AnyBase, object::Video};
    /// # let video = Video::new();
    /// #
    /// let any_base = AnyBase::from_extended(video)?;
    ///
    /// assert!(any_base.as_base().is_some());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_base(&self) -> Option<&Base<serde_json::Value>> {
        self.0.as_ref().left().and_then(|l| l.as_base())
    }

    /// Take the Url from the Object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::AnyBase, uri};
    /// #
    /// let any_base = AnyBase::from_xsd_any_uri(uri!("https://example.com"));
    ///
    /// assert!(any_base.take_xsd_any_uri().is_some());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn take_xsd_any_uri(self) -> Option<Url> {
        self.0.left().and_then(|l| l.id())
    }

    /// Take the String from the Object
    ///
    /// ```rust
    /// # use activitystreams::base::AnyBase;
    /// #
    /// let any_base = AnyBase::from_xsd_string("hi".into());
    ///
    /// assert!(any_base.take_xsd_string().is_some());
    /// ```
    pub fn take_xsd_string(self) -> Option<String> {
        self.0.right()
    }

    /// Take the `Base<serde_json::Value>` from the Object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::AnyBase, object::Video};
    /// # let video = Video::new();
    /// #
    /// let any_base = AnyBase::from_extended(video)?;
    ///
    /// assert!(any_base.take_base().is_some());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn take_base(self) -> Option<Base<serde_json::Value>> {
        self.0.left().and_then(|l| l.base())
    }

    /// Replace the object with the provided Url
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::AnyBase, uri};
    /// #
    /// let mut any_base = AnyBase::from_xsd_string("hi".into());
    ///
    /// any_base.set_xsd_any_uri(uri!("https://example.com"));
    ///
    /// assert!(any_base.take_xsd_any_uri().is_some());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_xsd_any_uri(&mut self, id: Url) {
        self.0 = Either::Left(IdOrBase::from_xsd_any_uri(id));
    }

    /// Replace the object with the provided String
    ///
    /// ```
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::AnyBase, uri};
    /// #
    /// let mut any_base = AnyBase::from_xsd_any_uri(uri!("https://example.com"));
    ///
    /// any_base.set_xsd_string("hi");
    ///
    /// assert!(any_base.take_xsd_string().is_some());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_xsd_string<T>(&mut self, xsd_string: T)
    where
        T: Into<String>,
    {
        self.0 = Either::Right(xsd_string.into());
    }

    /// Replace the object with the provided `Base<serde_json::Value>`
    ///
    /// ```
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::{AnyBase, Base}, object::Video};
    /// # let video = Video::new();
    /// let mut any_base = AnyBase::from_xsd_string("hi".into());
    ///
    /// let base = Base::retract(video)?.into_generic()?;
    /// any_base.set_base(base);
    ///
    /// assert!(any_base.take_base().is_some());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_base(&mut self, base: Base<serde_json::Value>) {
        self.0 = Either::Left(IdOrBase::from_base(base));
    }

    /// Create an AnyBase from a Url
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::{base::AnyBase, uri};
    /// let any_base = AnyBase::from_xsd_any_uri(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_xsd_any_uri(id: Url) -> Self {
        AnyBase(Either::Left(IdOrBase::from_xsd_any_uri(id)))
    }

    /// Create an AnyBase from an String
    ///
    /// ```rust
    /// use activitystreams::base::AnyBase;
    /// let any_base = AnyBase::from_xsd_string("hi".into());
    /// ```
    pub fn from_xsd_string(xsd_string: String) -> Self {
        AnyBase(Either::Right(xsd_string))
    }

    /// Create an AnyBase from a `Base<serde_json::Value>`
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::object::Video;
    /// # let video = Video::new();
    /// use activitystreams::base::{AnyBase, Base};
    ///
    /// let base = Base::retract(video)?.into_generic()?;
    /// let any_base = AnyBase::from_base(base);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_base(base: Base<serde_json::Value>) -> Self {
        AnyBase(Either::Left(IdOrBase::from_base(base)))
    }
}

impl IdOrBase {
    fn as_xsd_any_uri(&self) -> Option<&Url> {
        self.0.as_ref().left()
    }

    fn as_base(&self) -> Option<&Base<serde_json::Value>> {
        self.0.as_ref().right().map(|b| b.as_ref())
    }

    fn id(self) -> Option<Url> {
        self.0.left()
    }

    fn base(self) -> Option<Base<serde_json::Value>> {
        self.0.right().map(|b| *b)
    }

    fn from_xsd_any_uri(id: Url) -> Self {
        IdOrBase(Either::Left(id))
    }

    fn from_base(base: Base<serde_json::Value>) -> Self {
        IdOrBase(Either::Right(Box::new(base)))
    }
}

impl OneOrMany<AnyBase> {
    /// Get the ID from a single object if there is only one object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::{Base, BaseExt}, primitives::OneOrMany, uri};
    /// # let mut base = Base::<String>::new();
    /// # let id = uri!("https://example.com");
    /// # base.set_id(id.clone());
    /// # let base = OneOrMany::from_base(base.into_generic()?.into());
    /// #
    /// assert!(base.as_single_id() == Some(&id));
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_single_id(&self) -> Option<&Url> {
        self.as_one().and_then(|one| one.id())
    }

    /// Check if there's only one ID, and if it equals `id`
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::{Base, BaseExt}, primitives::OneOrMany, uri};
    /// # let mut base = Base::<String>::new();
    /// # let id = uri!("https://example.com");
    /// # base.set_id(id.clone());
    /// # let base = OneOrMany::from_base(base.into_generic()?.into());
    /// #
    /// assert!(base.is_single_id(&id));
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_single_id(&self, id: &Url) -> bool {
        self.as_single_id() == Some(id)
    }

    /// Get the kind of a single object if there is only one object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::{Base, BaseExt}, primitives::OneOrMany};
    /// # let mut base = Base::<String>::new();
    /// # base.set_kind(String::from("Person"));
    /// # let base = OneOrMany::from_base(base.into_generic()?.into());
    /// #
    /// assert!(base.as_single_kind_str() == Some("Person"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_single_kind(&self) -> Option<&serde_json::Value> {
        self.as_one().and_then(|one| one.kind())
    }

    /// Get the kind of a single object as an &str
    ///
    /// This returns None if the kind is not present, or not a String
    /// ```
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::{Base, BaseExt}, primitives::OneOrMany};
    /// # let mut base = Base::<String>::new();
    /// # base.set_kind(String::from("Person"));
    /// # let base = OneOrMany::from_base(base.into_generic()?.into());
    /// #
    /// assert!(base.as_single_kind_str() == Some("Person"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_single_kind_str(&self) -> Option<&str> {
        self.as_one().and_then(|one| one.kind_str())
    }

    /// Checks the kind of the inner Base if the current object is a Base
    ///
    /// This returns False if the kind is not present, or not a String
    ///
    /// ```
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{base::{Base, BaseExt}, primitives::OneOrMany};
    /// # let mut base = Base::new();
    /// # base.set_kind(String::from("Person"));
    /// # let base = OneOrMany::from_base(base.into_generic()?.into());
    /// #
    /// assert!(base.is_single_kind("Person"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_single_kind(&self, kind: &str) -> bool {
        self.as_single_kind_str() == Some(kind)
    }

    /// Get a single Url from the object, if that is what is contained
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{primitives::OneOrMany, uri};
    /// #
    /// let one = OneOrMany::from_xsd_any_uri(uri!("https://example.com"));
    ///
    /// assert!(one.as_single_xsd_any_uri().is_some());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_single_xsd_any_uri(&self) -> Option<&Url> {
        self.as_one().and_then(|inner| inner.as_xsd_any_uri())
    }

    /// Get a single &str from the object, if that is what is contained
    ///
    /// ```rust
    /// # use activitystreams::{base::AnyBase, primitives::OneOrMany};
    /// #
    /// let one = OneOrMany::<AnyBase>::from_xsd_string("hi".into());
    ///
    /// assert!(one.as_single_xsd_string().is_some());
    /// ```
    pub fn as_single_xsd_string(&self) -> Option<&str> {
        self.as_one().and_then(|inner| inner.as_xsd_string())
    }

    /// Get a single `Base<serde_json::Value>` from the object, if that is what is contained
    ///
    /// ```rust
    /// # use activitystreams::{base::Base, primitives::OneOrMany};
    /// # let base = Base::new();
    /// #
    /// let one = OneOrMany::from_base(base);
    ///
    /// assert!(one.as_single_base().is_some());
    /// ```
    pub fn as_single_base(&self) -> Option<&Base<serde_json::Value>> {
        self.as_one().and_then(|inner| inner.as_base())
    }

    /// Take a single Url from the object, if that is what is contained
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{primitives::OneOrMany, uri};
    /// #
    /// let one = OneOrMany::from_xsd_any_uri(uri!("https://example.com"));
    ///
    /// assert!(one.single_xsd_any_uri().is_some());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn single_xsd_any_uri(self) -> Option<Url> {
        self.one().and_then(|inner| inner.take_xsd_any_uri())
    }

    /// Take a single String from the object, if that is what is contained
    ///
    /// ```rust
    /// # use activitystreams::{base::AnyBase, primitives::OneOrMany};
    /// #
    /// let one = OneOrMany::<AnyBase>::from_xsd_string("hi".into());
    ///
    /// assert!(one.single_xsd_string().is_some());
    /// ```
    pub fn single_xsd_string(self) -> Option<String> {
        self.one().and_then(|inner| inner.take_xsd_string())
    }

    /// Take a single `Base<serde_json::Value>` from the object, if that is what is contained
    ///
    /// ```rust
    /// # use activitystreams::{base::Base, primitives::OneOrMany};
    /// # let base = Base::new();
    /// #
    /// let one = OneOrMany::from_base(base);
    ///
    /// assert!(one.single_base().is_some());
    /// ```
    pub fn single_base(self) -> Option<Base<serde_json::Value>> {
        self.one().and_then(|inner| inner.take_base())
    }

    /// Create a `OneOrMany<AnyBase>` from a Url
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::{primitives::OneOrMany, uri};
    ///
    /// let one = OneOrMany::from_xsd_any_uri(uri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_xsd_any_uri(id: Url) -> Self {
        OneOrMany(Either::Left(AnyBase::from_xsd_any_uri(id)))
    }

    /// Create a `OneOrMany<AnyBase>` from a String
    ///
    /// ```rust
    /// use activitystreams::{base::AnyBase, primitives::OneOrMany};
    ///
    /// let one = OneOrMany::<AnyBase>::from_xsd_string("hi".into());
    /// ```
    pub fn from_xsd_string(xsd_string: String) -> Self {
        OneOrMany(Either::Left(AnyBase::from_xsd_string(xsd_string)))
    }

    /// Create a `OneOrMany<AnyBase>` from a `Base<serde_json::Value>`
    ///
    /// ```rust
    /// # use activitystreams::base::Base;
    /// # let base = Base::new();
    /// #
    /// use activitystreams::primitives::OneOrMany;
    ///
    /// let one = OneOrMany::from_base(base);
    /// ```
    pub fn from_base(base: Base<serde_json::Value>) -> Self {
        OneOrMany(Either::Left(AnyBase::from_base(base)))
    }

    /// Overwrite the current object with a Url
    ///
    /// ```rust
    /// # use activitystreams::{base::Base, context, primitives::OneOrMany};
    /// # let base = Base::new();
    /// #
    /// let mut one = OneOrMany::from_base(base);
    ///
    /// one.set_single_xsd_any_uri(context());
    ///
    /// assert!(one.as_single_xsd_any_uri().is_some());
    /// ```
    pub fn set_single_xsd_any_uri(&mut self, id: Url) -> &mut Self {
        self.0 = Either::Left(AnyBase::from_xsd_any_uri(id));
        self
    }

    /// Overwrite the current object with a String
    ///
    /// ```rust
    /// # use activitystreams::{base::Base, primitives::OneOrMany};
    /// # let base = Base::new();
    /// #
    /// let mut one = OneOrMany::from_base(base);
    ///
    /// one.set_single_xsd_string("hi".into());
    ///
    /// assert!(one.as_single_xsd_string().is_some());
    /// ```
    pub fn set_single_xsd_string(&mut self, xsd_string: String) -> &mut Self {
        self.0 = Either::Left(AnyBase::from_xsd_string(xsd_string));
        self
    }

    /// Overwrite the current object with a `Base<serde_json::Value>`
    ///
    /// ```
    /// # use activitystreams::{base::Base, context, primitives::OneOrMany};
    /// # let base = Base::new();
    /// #
    /// let mut one = OneOrMany::from_xsd_any_uri(context());
    ///
    /// one.set_single_base(base);
    ///
    /// assert!(one.as_single_base().is_some());
    /// ```
    pub fn set_single_base(&mut self, base: Base<serde_json::Value>) -> &mut Self {
        self.0 = Either::Left(AnyBase::from_base(base));
        self
    }

    /// Append a Url to the current object
    ///
    /// ```rust
    /// use activitystreams::{base::AnyBase, context, primitives::OneOrMany, security};
    ///
    /// let mut many = OneOrMany::<AnyBase>::from_xsd_string("hi".into());
    ///
    /// many.add_xsd_any_uri(security())
    ///     .add_xsd_any_uri(context());
    /// ```
    pub fn add_xsd_any_uri(&mut self, id: Url) -> &mut Self {
        self.add(AnyBase::from_xsd_any_uri(id))
    }

    /// Append a String to the current object
    ///
    /// ```rust
    /// use activitystreams::{context, primitives::OneOrMany};
    ///
    /// let mut many = OneOrMany::from_xsd_any_uri(context());
    ///
    /// many.add_xsd_string("hi".into())
    ///     .add_xsd_string("hello".into());
    /// ```
    pub fn add_xsd_string(&mut self, xsd_string: String) -> &mut Self {
        self.add(AnyBase::from_xsd_string(xsd_string))
    }

    /// Append a `Base<serde_json::Value>` to the current object
    ///
    /// ```rust
    /// # use activitystreams::base::Base;
    /// # let base1 = Base::new();
    /// # let base2 = Base::new();
    /// use activitystreams::{context, primitives::OneOrMany};
    ///
    /// let mut many = OneOrMany::from_xsd_any_uri(context());
    ///
    /// many.add_base(base1).add_base(base2);
    /// ```
    pub fn add_base(&mut self, base: Base<serde_json::Value>) -> &mut Self {
        self.add(AnyBase::from_base(base))
    }
}

impl<Kind> markers::Base for Base<Kind> {}

impl<Kind> UnparsedMut for Base<Kind> {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        &mut self.unparsed
    }
}

impl<Kind> AsBase<Kind> for Base<Kind> {
    fn base_ref(&self) -> &Base<Kind> {
        self
    }

    fn base_mut(&mut self) -> &mut Base<Kind> {
        self
    }
}

impl<T, Kind> ExtendsExt<Kind> for T
where
    T: Extends<Kind>,
    T::Error: From<serde_json::Error>,
{
}
impl<T, Kind> BaseExt<Kind> for T where T: AsBase<Kind> {}

impl From<Base<serde_json::Value>> for AnyBase {
    fn from(o: Base<serde_json::Value>) -> Self {
        Self::from_base(o)
    }
}

impl From<Url> for AnyBase {
    fn from(id: Url) -> Self {
        Self::from_xsd_any_uri(id)
    }
}

impl From<String> for AnyBase {
    fn from(xsd_string: String) -> Self {
        Self::from_xsd_string(xsd_string)
    }
}

impl From<Base<serde_json::Value>> for OneOrMany<AnyBase> {
    fn from(object: Base<serde_json::Value>) -> Self {
        Self::from_base(object)
    }
}

impl From<Url> for OneOrMany<AnyBase> {
    fn from(xsd_any_uri: Url) -> Self {
        Self::from_xsd_any_uri(xsd_any_uri)
    }
}

impl From<String> for OneOrMany<AnyBase> {
    fn from(xsd_string: String) -> Self {
        Self::from_xsd_string(xsd_string)
    }
}

impl From<&str> for OneOrMany<AnyBase> {
    fn from(xsd_string: &str) -> Self {
        Self::from_xsd_string(xsd_string.to_owned())
    }
}
