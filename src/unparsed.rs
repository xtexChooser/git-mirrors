//! Types and Traits for dealing with Unparsed data
//!
//! Since ActivityStreams is extensible, and structured in a heirarchy of types, Data that was not
//! required to construct a given type might still come in handy when extending a into a subobject.
//!
//! For example, a `Create` activity is an extension of an `Object`, but if we need to store that
//! type on another object, we need to make the type system happy. Since any number of types both
//! provided and extended might need to be stored in the same key, a type called `AnyBase` exists.
//! AnyBase, when containing a `Base`, can be extended into a `Create` activity so long as the
//! required keys are present in it's `Unparsed` struct.
//!
//! For the most part, users of this library won't care about this module, but if you need to
//! create an Extension, it will come in handy.
//!
//! Let's implement a part of the Security extension to be compatible with Mastodon.
//!
//! ```rust
//! use activitystreams::{
//!     actor::{AsApActor, ApActor},
//!     base::{AsBase, Base, Extends},
//!     markers,
//!     object::{AsObject, Object},
//!     prelude::*,
//!     primitives::*,
//!     unparsed::*,
//!     url::Url,
//! };
//!
//! /// First, we'll define our public key types
//!
//! #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct PublicKeyValues {
//!     pub id: Url,
//!     pub owner: Url,
//!     pub public_key_pem: String,
//! }
//!
//! #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct PublicKey<Inner> {
//!     pub public_key: PublicKeyValues,
//!     pub inner: Inner,
//! }
//!
//!
//! /// Then, we'll implement Extends so we can produce a PublicKey<Object> from an AnyBase.
//!
//! impl<Inner, Kind> Extends<Kind> for PublicKey<Inner>
//! where
//!     Inner: Extends<Kind, Error=serde_json::Error> + UnparsedMut,
//! {
//!     type Error = serde_json::Error;
//!
//!     fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
//!         let mut inner = Inner::extends(base)?;
//!
//!         Ok(PublicKey {
//!             public_key: inner.unparsed_mut().remove("publicKey")?,
//!             inner,
//!         })
//!     }
//!
//!     fn retracts(self) -> Result<Base<Kind>, Self::Error> {
//!         let PublicKey {
//!             public_key,
//!             mut inner,
//!         } = self;
//!
//!         inner.unparsed_mut().insert("publicKey", public_key)?;
//!
//!         inner.retracts()
//!     }
//! }
//!
//!
//! /// Auto-implement Base, Object, and Actor when Inner supports it
//! impl<Inner> markers::Base for PublicKey<Inner> where Inner: markers::Base {}
//! impl<Inner> markers::Object for PublicKey<Inner> where Inner: markers::Object {}
//! impl<Inner> markers::Actor for PublicKey<Inner> where Inner: markers::Actor {}
//!
//!
//! /// If we want to easily access getters and setters for internal types, we'll need to forward
//! /// those, too.
//!
//! /// Forward for base methods
//! ///
//! /// This allows us to access methods related to `context`, `id`, `kind`, `name`,
//! /// `media_type`, and `preview` directly from the PublicKey struct
//! impl<Inner, Kind> AsBase<Kind> for PublicKey<Inner>
//! where
//!     Inner: AsBase<Kind>,
//! {
//!     fn base_ref(&self) -> &Base<Kind> {
//!         self.inner.base_ref()
//!     }
//!
//!     fn base_mut(&mut self) -> &mut Base<Kind> {
//!         self.inner.base_mut()
//!     }
//! }
//!
//! /// Forward for object methods
//! ///
//! /// This allows us to access methods related to `url`, `generator`, `start_time`, `duration`,
//! /// and more directly from the PublicKey struct
//! impl<Inner, Kind> AsObject<Kind> for PublicKey<Inner>
//! where
//!     Inner: AsObject<Kind>,
//! {
//!     fn object_ref(&self) -> &Object<Kind> {
//!         self.inner.object_ref()
//!     }
//!
//!     fn object_mut(&mut self) -> &mut Object<Kind> {
//!         self.inner.object_mut()
//!     }
//! }
//!
//! /// Forward for ActivityPub actor methods
//! ///
//! /// This allows us to access methods related to `inbox`, `outbox`, `following`, `followers`,
//! /// `liked`, `streams`, `endpoints`, and more directly from the PublicKey struct
//! impl<Inner1, Inner2> AsApActor<Inner2> for PublicKey<Inner1>
//! where
//!     Inner1: AsApActor<Inner2>,
//! {
//!     fn ap_actor_ref(&self) -> &ApActor<Inner2> {
//!         self.inner.ap_actor_ref()
//!     }
//!
//!     fn ap_actor_mut(&mut self) -> &mut ApActor<Inner2> {
//!         self.inner.ap_actor_mut()
//!     }
//! }
//!
//!
//! /// If we want to be able to extend from our own type, we'll need to forward some
//! /// implementations, and create some traits
//!
//! /// Make it easy for downstreams to get an Unparsed
//! impl<Inner> UnparsedMut for PublicKey<Inner>
//! where
//!     Inner: UnparsedMut,
//! {
//!     fn unparsed_mut(&mut self) -> &mut Unparsed {
//!         self.inner.unparsed_mut()
//!     }
//! }
//!
//! /// Create our own extensible trait
//! pub trait AsPublicKey<Inner> {
//!     fn public_key_ref(&self) -> &PublicKey<Inner>;
//!     fn public_key_mut(&mut self) -> &mut PublicKey<Inner>;
//! }
//!
//! /// Implement it
//! impl<Inner> AsPublicKey<Inner> for PublicKey<Inner> {
//!     fn public_key_ref(&self) -> &Self {
//!         self
//!     }
//!
//!     fn public_key_mut(&mut self) -> &mut Self {
//!         self
//!     }
//! }
//!
//! /// And now create helper methods
//! pub trait PublicKeyExt<Inner>: AsPublicKey<Inner> {
//!     /// Borrow the public key's ID
//!     fn key_id<'a>(&'a self) -> &'a Url
//!     where
//!         Inner: 'a,
//!     {
//!         &self.public_key_ref().public_key.id
//!     }
//!
//!     /// Set the public key's ID
//!     fn set_key_id(&mut self, id: Url) -> &mut Self {
//!         self.public_key_mut().public_key.id = id;
//!         self
//!     }
//!
//!     /// Borrow the public key's Owner
//!     fn key_owner<'a>(&'a self) -> &'a Url
//!     where
//!         Inner: 'a,
//!     {
//!         &self.public_key_ref().public_key.owner
//!     }
//!
//!     /// Set the public key's Owner
//!     fn set_key_owner(&mut self, owner: Url) -> &mut Self {
//!         self.public_key_mut().public_key.owner = owner;
//!         self
//!     }
//!
//!     /// Borrow the public key's PEM encoded value
//!     fn key_pem<'a>(&'a self) -> &'a str
//!     where
//!         Inner: 'a,
//!     {
//!         &self.public_key_ref().public_key.public_key_pem
//!     }
//!
//!     /// Set the public key's PEM encoded value
//!     ///
//!     /// In a real application, this might take a different type, such as RSA's RSAPublicKey, or
//!     /// OpenSSL's or Ring's version
//!     fn set_key_pem<T>(&mut self, pem: T) -> &mut Self
//!     where
//!         T: Into<String>,
//!     {
//!         self.public_key_mut().public_key.public_key_pem = pem.into();
//!         self
//!     }
//! }
//!
//! /// Finally, we'll automatically implement PublicKeyExt for any type implementing AsPublicKey
//! impl<T, Inner> PublicKeyExt<Inner> for T where T: AsPublicKey<Inner> {}
//!
//!
//! /// Now that eveything is implemented, we can use it like so:
//! use activitystreams::{actor::{kind::PersonType, Person}, uri};
//!
//! pub type ExtendedPerson = PublicKey<ApActor<Person>>;
//!
//! impl ExtendedPerson {
//!     pub fn new(inbox: Url, mut owner: Url) -> Self {
//!         let id = owner.clone();
//!         owner.set_fragment(Some("main-key"));
//!         PublicKey {
//!             public_key: PublicKeyValues {
//!                 id,
//!                 owner,
//!                 public_key_pem: String::new(),
//!             },
//!             inner: ApActor::new(inbox, Person::new()),
//!         }
//!     }
//! }
//!
//! fn main() -> Result<(), anyhow::Error> {
//!     let mut extended_person = ExtendedPerson::new(
//!         uri!("https://example.com/user/inbox"),
//!         uri!("https://example.com/user"),
//!     );
//!
//!     extended_person
//!         .set_kind(PersonType::Person)
//!         .set_id("https://example.com/user".parse()?)
//!         .set_name("Demo User")
//!         .set_preferred_username("user")
//!         .set_outbox("https://example.com/user/outbox".parse()?)
//!         .set_key_pem(
//!             "------ BEGIN PUBLIC KEY ------\nasdfasdfasdfasdfasdfasdf..."
//!         )
//!         .set_key_owner("https://example.com/user".parse()?)
//!         .set_key_id("https://example.com/user#main-key".parse()?);
//!
//!     let string = serde_json::to_string(&extended_person)?;
//!     println!("{}", string);
//!     Ok(())
//! }
//! ```

/// A trait granting mutable access to an Unparsed struct
///
/// This is required for easy manipulation of Unparsed from potentially deeply nested structures.
pub trait UnparsedMut {
    /// Get a mutable reference to Unparsed
    fn unparsed_mut(&mut self) -> &mut Unparsed;
}

/// A helper trait providing two methods, 'insert' and 'remove', that is auto-implemented for
/// UnparsedMut types.
///
/// These methods are provided for easily pulling values from and inserting values into the
/// Unparsed struct.
pub trait UnparsedMutExt: UnparsedMut {
    /// Remove a value from the Unparsed struct, provided it matches the expected type
    fn remove<T>(&mut self, key: &str) -> Result<T, serde_json::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_value(self.unparsed_mut().remove(key))
    }

    /// Insert a value into the Unparsed struct if the value isn't Null
    fn insert<T>(&mut self, key: &str, value: T) -> Result<&mut Self, serde_json::Error>
    where
        T: serde::ser::Serialize,
    {
        let value = serde_json::to_value(value)?;

        if !value.is_null() {
            self.unparsed_mut().insert(key.to_owned(), value);
        }

        Ok(self)
    }
}

/// The Unparsed struct itself,
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct Unparsed(std::collections::HashMap<String, serde_json::Value>);

impl Unparsed {
    pub(crate) fn remove(&mut self, key: &str) -> serde_json::Value {
        self.0.remove(key).unwrap_or(serde_json::Value::Null)
    }

    pub(crate) fn insert(&mut self, key: String, value: serde_json::Value) {
        self.0.insert(key, value);
    }
}

impl UnparsedMut for Unparsed {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self
    }
}

impl<T> UnparsedMutExt for T where T: UnparsedMut {}
