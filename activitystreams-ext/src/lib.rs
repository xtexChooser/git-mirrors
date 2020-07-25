//! # An extension API for activitystreams
//! _This crate provides Ext1, Ext2, Ext3, and Ext4 for adding extensions to ActivityStreams types_
//!
//! - Find the code on [git.asonix.dog](https://git.asonix.dog/Aardwolf/activitystreams)
//! - Read the docs on [docs.rs](https://docs.rs/activitystreams-ext)
//! - Join the matrix channel at [#activitypub:asonix.dog](https://matrix.to/#/!fAEcHyTUdAaKCzIKCt:asonix.dog?via=asonix.dog&via=matrix.org&via=t2bot.io)
//! - Hit me up on [mastodon](https://asonix.dog/@asonix)
//!
//! ## Usage
//!
//! First, add ActivityStreams to your dependencies
//! ```toml
//! [dependencies]
//! activitystreams = "0.7.0-alpha.0"
//! activitystreams-ext = "0.1.0-alpha.0"
//! ```
//!
//! For an example, we'll implement a PublicKey extension and demonstrate usage with Ext1
//! ```rust
//! use activitystreams_ext::{Ext1, UnparsedExtension};
//! use activitystreams::{
//!     actor::{ApActor, Person},
//!     context,
//!     prelude::*,
//!     primitives::XsdAnyUri,
//!     security,
//!     unparsed::UnparsedMutExt,
//! };
//!
//! #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct PublicKey {
//!     public_key: PublicKeyInner,
//! }
//!
//! #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct PublicKeyInner {
//!     id: XsdAnyUri,
//!     owner: XsdAnyUri,
//!     public_key_pem: String,
//! }
//!
//! impl<U> UnparsedExtension<U> for PublicKey
//! where
//!     U: UnparsedMutExt,
//! {
//!     type Error = serde_json::Error;
//!
//!     fn try_from_unparsed(unparsed_mut: &mut U) -> Result<Self, Self::Error> {
//!         Ok(PublicKey {
//!             public_key: unparsed_mut.remove("publicKey")?,
//!         })
//!     }
//!
//!     fn try_into_unparsed(self, unparsed_mut: &mut U) -> Result<(), Self::Error> {
//!         unparsed_mut.insert("publicKey", self.public_key)?;
//!         Ok(())
//!     }
//! }
//!
//! pub type ExtendedPerson = Ext1<ApActor<Person>, PublicKey>;
//!
//! fn main() -> Result<(), anyhow::Error> {
//!     let actor = ApActor::new(
//!         "http://in.box".parse()?,
//!         "http://out.box".parse()?,
//!         Person::new(),
//!     );
//!
//!     let mut person = Ext1::new(
//!         actor,
//!         PublicKey {
//!             public_key: PublicKeyInner {
//!                 id: "http://key.id".parse()?,
//!                 owner: "http://owner.id".parse()?,
//!                 public_key_pem: "asdfasdfasdf".to_owned(),
//!             },
//!         },
//!     );
//!
//!     person.set_context(context()).add_context(security());
//!
//!     let any_base = person.into_any_base()?;
//!     println!("any_base: {:#?}", any_base);
//!     let person = ExtendedPerson::from_any_base(any_base)?;
//!
//!     println!("person: {:#?}", person);
//!     Ok(())
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/activitystreams-ext/0.1.0-alpha.0/activitystreams_ext")]

use activitystreams::{
    base::{Base, Extends},
    unparsed::{UnparsedMut, UnparsedMutExt},
};

mod ext1;
mod ext2;
mod ext3;
mod ext4;

/// Transform types from and into the Unparsed structure
pub trait UnparsedExtension<U>
where
    U: UnparsedMutExt,
{
    type Error: std::error::Error;

    /// Generate Self from Unparsed
    fn try_from_unparsed(unparsed_mut: &mut U) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Insert Self into Unparsed
    fn try_into_unparsed(self, unparsed_mut: &mut U) -> Result<(), Self::Error>;
}

/// Extend a type with a single value
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Ext1<Inner, A> {
    #[serde(flatten)]
    pub ext_one: A,

    /// The type being extended
    #[serde(flatten)]
    pub inner: Inner,
}

/// Extend a type with two values
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Ext2<Inner, A, B> {
    #[serde(flatten)]
    pub ext_one: A,

    #[serde(flatten)]
    pub ext_two: B,

    /// The type being extended
    #[serde(flatten)]
    pub inner: Inner,
}

/// Extend a type with three values
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Ext3<Inner, A, B, C> {
    #[serde(flatten)]
    pub ext_one: A,

    #[serde(flatten)]
    pub ext_two: B,

    #[serde(flatten)]
    pub ext_three: C,

    /// The type being extended
    #[serde(flatten)]
    pub inner: Inner,
}

/// Extend a type with four values
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Ext4<Inner, A, B, C, D> {
    #[serde(flatten)]
    pub ext_one: A,

    #[serde(flatten)]
    pub ext_two: B,

    #[serde(flatten)]
    pub ext_three: C,

    #[serde(flatten)]
    pub ext_four: D,

    /// The type being extended
    #[serde(flatten)]
    pub inner: Inner,
}

impl<Inner, A> Ext1<Inner, A> {
    pub fn new(inner: Inner, ext_one: A) -> Self {
        Ext1 { inner, ext_one }
    }

    pub fn extend<B>(self, ext_two: B) -> Ext2<Inner, A, B> {
        Ext2 {
            inner: self.inner,
            ext_one: self.ext_one,
            ext_two,
        }
    }
}

impl<Inner, A, B> Ext2<Inner, A, B> {
    pub fn new(inner: Inner, ext_one: A, ext_two: B) -> Self {
        Ext2 {
            inner,
            ext_one,
            ext_two,
        }
    }

    pub fn extend<C>(self, ext_three: C) -> Ext3<Inner, A, B, C> {
        Ext3 {
            inner: self.inner,
            ext_one: self.ext_one,
            ext_two: self.ext_two,
            ext_three,
        }
    }
}

impl<Inner, A, B, C> Ext3<Inner, A, B, C> {
    pub fn new(inner: Inner, ext_one: A, ext_two: B, ext_three: C) -> Self {
        Ext3 {
            inner,
            ext_one,
            ext_two,
            ext_three,
        }
    }

    pub fn extend<D>(self, ext_four: D) -> Ext4<Inner, A, B, C, D> {
        Ext4 {
            inner: self.inner,
            ext_one: self.ext_one,
            ext_two: self.ext_two,
            ext_three: self.ext_three,
            ext_four,
        }
    }
}

impl<Inner, A, Kind, Error> Extends<Kind> for Ext1<Inner, A>
where
    Inner: Extends<Kind, Error = Error> + UnparsedMut,
    A: UnparsedExtension<Inner, Error = Error>,
    Error: From<serde_json::Error> + std::error::Error,
{
    type Error = Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        let mut inner = Inner::extends(base)?;
        let ext_one = A::try_from_unparsed(&mut inner)?;

        Ok(Ext1 { inner, ext_one })
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        let Ext1 { mut inner, ext_one } = self;

        ext_one.try_into_unparsed(&mut inner)?;
        inner.retracts()
    }
}

impl<Inner, A, B, Kind, Error> Extends<Kind> for Ext2<Inner, A, B>
where
    Inner: Extends<Kind, Error = Error> + UnparsedMut,
    A: UnparsedExtension<Inner, Error = Error>,
    B: UnparsedExtension<Inner, Error = Error>,
    Error: From<serde_json::Error> + std::error::Error,
{
    type Error = Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        let mut inner = Inner::extends(base)?;
        let ext_one = A::try_from_unparsed(&mut inner)?;
        let ext_two = B::try_from_unparsed(&mut inner)?;

        Ok(Ext2 {
            inner,
            ext_one,
            ext_two,
        })
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        let Ext2 {
            mut inner,
            ext_one,
            ext_two,
        } = self;

        ext_one.try_into_unparsed(&mut inner)?;
        ext_two.try_into_unparsed(&mut inner)?;
        inner.retracts()
    }
}

impl<Inner, A, B, C, Kind, Error> Extends<Kind> for Ext3<Inner, A, B, C>
where
    Inner: Extends<Kind, Error = Error> + UnparsedMut,
    A: UnparsedExtension<Inner, Error = Error>,
    B: UnparsedExtension<Inner, Error = Error>,
    C: UnparsedExtension<Inner, Error = Error>,
    Error: From<serde_json::Error> + std::error::Error,
{
    type Error = Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        let mut inner = Inner::extends(base)?;
        let ext_one = A::try_from_unparsed(&mut inner)?;
        let ext_two = B::try_from_unparsed(&mut inner)?;
        let ext_three = C::try_from_unparsed(&mut inner)?;

        Ok(Ext3 {
            inner,
            ext_one,
            ext_two,
            ext_three,
        })
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        let Ext3 {
            mut inner,
            ext_one,
            ext_two,
            ext_three,
        } = self;

        ext_one.try_into_unparsed(&mut inner)?;
        ext_two.try_into_unparsed(&mut inner)?;
        ext_three.try_into_unparsed(&mut inner)?;
        inner.retracts()
    }
}

impl<Inner, A, B, C, D, Kind, Error> Extends<Kind> for Ext4<Inner, A, B, C, D>
where
    Inner: Extends<Kind, Error = Error> + UnparsedMut,
    A: UnparsedExtension<Inner, Error = Error>,
    B: UnparsedExtension<Inner, Error = Error>,
    C: UnparsedExtension<Inner, Error = Error>,
    D: UnparsedExtension<Inner, Error = Error>,
    Error: From<serde_json::Error> + std::error::Error,
{
    type Error = Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        let mut inner = Inner::extends(base)?;
        let ext_one = A::try_from_unparsed(&mut inner)?;
        let ext_two = B::try_from_unparsed(&mut inner)?;
        let ext_three = C::try_from_unparsed(&mut inner)?;
        let ext_four = D::try_from_unparsed(&mut inner)?;

        Ok(Ext4 {
            inner,
            ext_one,
            ext_two,
            ext_three,
            ext_four,
        })
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        let Ext4 {
            mut inner,
            ext_one,
            ext_two,
            ext_three,
            ext_four,
        } = self;

        ext_one.try_into_unparsed(&mut inner)?;
        ext_two.try_into_unparsed(&mut inner)?;
        ext_three.try_into_unparsed(&mut inner)?;
        ext_four.try_into_unparsed(&mut inner)?;
        inner.retracts()
    }
}
