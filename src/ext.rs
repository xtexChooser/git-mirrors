/*
 * This file is part of ActivityStreams.
 *
 * Copyright © 2020 Riley Trautman
 *
 * ActivityStreams is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Defining extensibility in the ActivityStreams spec
//!
//! In ActivityStreams, there are many times you may want to use an extension. For example, to
//! interact with Mastodon, you need to at least understand the `publicKey` field on their actor
//! type. If not, you won't be able to use HTTP Signatures, and will have your messages rejected.
//!
//! But this library doesn't provide any of the security extensions to ActivityStreams. In order to
//! support it, you could implment your own extensions to this library. Let's cover a basic
//! example.
//!
//! ```rust
//! // For this example, we'll use the Extensible trait, the Extension trait, the Actor trait, and
//! // the Person type
//! use activitystreams::{
//!     actor::{Actor, Person},
//!     ext::{Extensible, Extension},
//! };
//!
//! /// Let's define the PublicKey type. The three fields in this PublicKey struct are how Mastodon
//! /// represents Public Keys on actors. We'll need to derive Serialize and Deserialize for these
//! /// in order for them to be useful.
//! #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct PublicKey {
//!     /// The ID of the key.
//!     ///
//!     /// In mastodon, this is the same as the actor's URL with a #main-key on
//!     /// the end.
//!     pub id: String,
//!
//!     /// The ID of the actor who owns this key.
//!     pub owner: String,
//!
//!     /// This is a PEM file with PKCS#8 encoded data.
//!     pub public_key_pem: String,
//! }
//!
//! /// Now, we'll need more than just a PublicKey struct to make this work. We'll need to define a
//! /// second struct that declares the correct key to house this information inside of
//! ///
//! /// The information is represented as the following json:
//! /// ```json
//! /// {
//! ///     "publicKey": {
//! ///         "id": "key id",
//! ///         "owner": "actor id",
//! ///         "publicKeyPem": "pem string"
//! ///     }
//! /// }
//! /// ```
//! ///
//! /// This means we'll need to define the 'publicKey' key
//! #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct PublicKeyExtension {
//!     /// The public key's information
//!     pub public_key: PublicKey
//! }
//!
//! impl PublicKey {
//!     /// Let's add a convenience method to turn a PublicKey into a PublicKeyExtension
//!     pub fn to_ext(self) -> PublicKeyExtension {
//!         PublicKeyExtension { public_key: self }
//!     }
//! }
//!
//! // And finally, we'll implement the Extension trait for PublicKeyExtension
//! //
//! // We'll bound this extension by the Actor trait, since we don't care about non-actors having
//! // keys. This means that you can put a key on a `Person`, but not on a `Video`.
//! impl<T> Extension<T> for PublicKeyExtension where T: Actor {}
//!
//! // Now that these types are defined, we can put them to use!
//! fn main() {
//!     let person = Person::new();
//!
//!     // let's just create a dummy key for this example
//!     let public_key = PublicKey {
//!         id: "My ID".to_owned(),
//!         owner: "Owner ID".to_owned(),
//!         public_key_pem: "My Public Key".to_owned(),
//!     };
//!
//!     // We're doing it! The person is being extended with a public key
//!     //
//!     // Note that calling `extend` on person here is possible because the Extensible trait is in
//!     // scope
//!     let person_with_key = person.extend(public_key.to_ext());
//! }
//! ```

use crate::{
    activity::{Activity, ActivityBox, IntransitiveActivity, IntransitiveActivityBox},
    actor::{Actor, ActorBox},
    collection::{Collection, CollectionBox, CollectionPage, CollectionPageBox},
    link::{Link, LinkBox},
    object::{Object, ObjectBox},
    Base, BaseBox,
};
use std::{convert::TryFrom, fmt::Debug};

/// Defines an extension to an activitystreams object or link
///
/// This type provides two fields, the first field, `base`, should always the be base
/// ActivityStreams type. As long as `base` implements an ActivityStreams trait, Ext will also.
///
/// For example, the type `Ext<Video, HashMap<String, String>>` will implement the `Object` trait,
/// because `Video` implements that trait.
///
/// Additionally, some level of AsRef and AsMut have been derived for Ext. If type type
/// `Ext<Ext<Follow, SomeTime>, AnotherType>` exists, that will implement
/// `AsRef<ActivityProperties>` just like the innermost `Follow`. This only works for types
/// two levels deep, however.
///
/// Usage:
/// ```rust
/// use activitystreams::object::Video;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut video = Video::full();
///
///     // AsMut works even though this is an Ext<Video, ApObjectProperties>
///     video
///         .as_mut()
///         .set_id("https://example.com")?;
///
///     // set information on the extension
///     video
///         .extension
///         .set_source_xsd_any_uri("https://example.com")?;
///
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "derive", derive(serde::Deserialize, serde::Serialize))]
pub struct Ext<T, U> {
    /// The ActivityStreams base type, or another extension containing one
    #[cfg_attr(feature = "derive", serde(flatten))]
    pub base: T,

    /// The extension being applied to the type
    #[cfg_attr(feature = "derive", serde(flatten))]
    pub extension: U,
}

/// A trait implemented by extensions to the ActivityStreams specification
///
/// This is implemented for a couple types by default, such as
/// ApObjectProperties, and ApActorProperties.
///
/// Extensions are not intended to be used as trait objects
pub trait Extension<T>
where
    T: Base,
{
    /// A default implementation that simply returns the Ext type with Self and the base type
    /// inside of it.
    fn extends(self, base: T) -> Ext<T, Self>
    where
        Self: Sized,
    {
        Ext {
            base,
            extension: self,
        }
    }
}

/// A trait implemented (automatically) by objects and links in the ActivityStreams specification
///
/// This is used to easily extend objects.
///
/// ```rust
/// # use activitystreams::object::{Video, properties::ApObjectProperties};
/// use activitystreams::ext::Extensible;
/// let vid = Video::new();
/// let ap_props = ApObjectProperties::default();
///
/// let extended_vid = vid.extend(ap_props);
/// ```
pub trait Extensible<U> {
    fn extend(self, extension: U) -> Ext<Self, U>
    where
        Self: Sized;
}

impl<T, U> TryFrom<Ext<T, U>> for BaseBox
where
    T: Base + serde::ser::Serialize,
    U: Extension<T> + serde::ser::Serialize + Debug,
{
    type Error = std::io::Error;

    fn try_from(e: Ext<T, U>) -> Result<Self, Self::Error> {
        BaseBox::from_concrete(e)
    }
}

impl<T, U> TryFrom<Ext<T, U>> for ObjectBox
where
    T: Object + serde::ser::Serialize,
    U: Extension<T> + serde::ser::Serialize + Debug,
{
    type Error = std::io::Error;

    fn try_from(e: Ext<T, U>) -> Result<Self, Self::Error> {
        ObjectBox::from_concrete(e)
    }
}

impl<T, U> TryFrom<Ext<T, U>> for LinkBox
where
    T: Link + serde::ser::Serialize,
    U: Extension<T> + serde::ser::Serialize + Debug,
{
    type Error = std::io::Error;

    fn try_from(e: Ext<T, U>) -> Result<Self, Self::Error> {
        LinkBox::from_concrete(e)
    }
}

impl<T, U> TryFrom<Ext<T, U>> for CollectionBox
where
    T: Collection + serde::ser::Serialize,
    U: Extension<T> + serde::ser::Serialize + Debug,
{
    type Error = std::io::Error;

    fn try_from(e: Ext<T, U>) -> Result<Self, Self::Error> {
        CollectionBox::from_concrete(e)
    }
}

impl<T, U> TryFrom<Ext<T, U>> for CollectionPageBox
where
    T: CollectionPage + serde::ser::Serialize,
    U: Extension<T> + serde::ser::Serialize + Debug,
{
    type Error = std::io::Error;

    fn try_from(e: Ext<T, U>) -> Result<Self, Self::Error> {
        CollectionPageBox::from_concrete(e)
    }
}

impl<T, U> TryFrom<Ext<T, U>> for ActivityBox
where
    T: Activity + serde::ser::Serialize,
    U: Extension<T> + serde::ser::Serialize + Debug,
{
    type Error = std::io::Error;

    fn try_from(e: Ext<T, U>) -> Result<Self, Self::Error> {
        ActivityBox::from_concrete(e)
    }
}

impl<T, U> TryFrom<Ext<T, U>> for IntransitiveActivityBox
where
    T: IntransitiveActivity + serde::ser::Serialize,
    U: Extension<T> + serde::ser::Serialize + Debug,
{
    type Error = std::io::Error;

    fn try_from(e: Ext<T, U>) -> Result<Self, Self::Error> {
        IntransitiveActivityBox::from_concrete(e)
    }
}

impl<T, U> TryFrom<Ext<T, U>> for ActorBox
where
    T: Actor + serde::ser::Serialize,
    U: Extension<T> + serde::ser::Serialize + Debug,
{
    type Error = std::io::Error;

    fn try_from(e: Ext<T, U>) -> Result<Self, Self::Error> {
        ActorBox::from_concrete(e)
    }
}

impl<T, U> Extensible<U> for T
where
    T: crate::Base,
    U: Extension<T>,
{
    fn extend(self, item: U) -> Ext<Self, U> {
        item.extends(self)
    }
}

impl<T, U> Base for Ext<T, U>
where
    T: Base,
    U: Debug,
{
}

impl<T, U> Object for Ext<T, U>
where
    T: Object,
    U: Debug,
{
}

impl<T, U> Link for Ext<T, U>
where
    T: Link,
    U: Debug,
{
}

impl<T, U> Actor for Ext<T, U>
where
    T: Actor,
    U: Debug,
{
}

impl<T, U> Collection for Ext<T, U>
where
    T: Collection,
    U: Debug,
{
}

impl<T, U> CollectionPage for Ext<T, U>
where
    T: CollectionPage,
    U: Debug,
{
}

impl<T, U> Activity for Ext<T, U>
where
    T: Activity,
    U: Debug,
{
}

impl<T, U> IntransitiveActivity for Ext<T, U>
where
    T: IntransitiveActivity,
    U: Debug,
{
}
