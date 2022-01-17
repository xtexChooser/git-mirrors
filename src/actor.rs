//! Types and traits for dealing with Actor attributes
//!
//! ```rust
//! # fn main() -> Result<(), anyhow::Error> {
//! use activitystreams::{
//!     actor::{ApActor, Person},
//!     prelude::*,
//!     iri,
//! };
//!
//! let mut person = ApActor::new(
//!     iri!("https://example.com/actor/inbox"),
//!     Person::new(),
//! );
//!
//! person
//!     .set_outbox(iri!("https://example.com/actor/outbox"))
//!     .set_following(iri!("https://example.com/actor/following"))
//!     .set_followers(iri!("https://example.com/actor/followers"));
//! #
//! # Ok(())
//! # }
//! ```
use crate::{
    base::{AsBase, Base, Extends},
    checked::CheckError,
    markers,
    object::{ApObject, AsApObject, AsObject, Object},
    prelude::BaseExt,
    primitives::OneOrMany,
    unparsed::{Unparsed, UnparsedMut, UnparsedMutExt},
};
use iri_string::types::IriString;

pub use activitystreams_kinds::actor as kind;

use self::kind::*;

/// Implementation trait for deriving ActivityPub Actor methods for a type
///
/// Any type implementing AsObject will automatically gain methods provided by ApActorExt
pub trait AsApActor<Inner>: markers::Actor {
    /// Immutable borrow of `ApActor<Inner>`
    fn ap_actor_ref(&self) -> &ApActor<Inner>;

    /// Mutable borrow of `ApActor<Inner>`
    fn ap_actor_mut(&mut self) -> &mut ApActor<Inner>;
}

/// Helper methods for interacting with ActivityPub Actor types
///
/// This trait represents methods valid for any ActivityPub Actor.
///
/// Documentation for the fields related to these methods can be found on the `ApActor` struct
pub trait ApActorExt<Inner>: AsApActor<Inner> {
    /// Fetch the inbox for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// let inbox_ref = person.inbox();
    /// ```
    fn inbox<'a, Kind>(&'a self) -> Result<&'a IriString, CheckError>
    where
        Inner: 'a,
        Self: BaseExt<Kind>,
    {
        let inbox = self.inbox_unchecked();
        self.check_authority(inbox)
    }

    /// Fetch the inbox for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// let inbox_ref = person.inbox_unchecked();
    /// ```
    fn inbox_unchecked<'a>(&'a self) -> &'a IriString
    where
        Inner: 'a,
    {
        &self.ap_actor_ref().inbox
    }

    /// Fetch a mutable referece to the current actor's inbox
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// let inbox_mut = person.inbox_mut();
    /// ```
    fn inbox_mut<'a>(&'a mut self) -> &'a mut IriString
    where
        Inner: 'a,
    {
        &mut self.ap_actor_mut().inbox
    }

    /// Set the inbox for the current actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// person.set_inbox(iri!("https://example.com/inbox"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_inbox(&mut self, inbox: IriString) -> &mut Self {
        self.ap_actor_mut().inbox = inbox;
        self
    }

    /// Fetch the outbox for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// let outbox_ref = person.outbox();
    /// ```
    fn outbox<'a, Kind>(&'a self) -> Result<Option<&'a IriString>, CheckError>
    where
        Inner: 'a,
        Self: BaseExt<Kind>,
    {
        self.outbox_unchecked()
            .map(|outbox| self.check_authority(outbox))
            .transpose()
    }

    /// Fetch the outbox for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// let outbox_ref = person.outbox_unchecked();
    /// ```
    fn outbox_unchecked<'a>(&'a self) -> Option<&'a IriString>
    where
        Inner: 'a,
    {
        self.ap_actor_ref().outbox.as_ref()
    }

    /// Mutably fetch the outbox for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(outbox) = person.outbox_mut() {
    ///     println!("{:?}", outbox);
    /// }
    /// ```
    fn outbox_mut<'a>(&'a mut self) -> Option<&'a mut IriString>
    where
        Inner: 'a,
    {
        self.ap_actor_mut().outbox.as_mut()
    }

    /// Set the outbox for the current actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// person.set_outbox(iri!("https://example.com/outbox"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_outbox(&mut self, outbox: IriString) -> &mut Self {
        self.ap_actor_mut().outbox = Some(outbox);
        self
    }

    /// Take the outbox from the current actor, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(outbox) = person.take_outbox() {
    ///     println!("{:?}", outbox);
    /// }
    /// ```
    fn take_outbox(&mut self) -> Option<IriString> {
        self.ap_actor_mut().outbox.take()
    }

    /// Delete the outbox from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// # person.set_outbox(iri!("https://example.com/outbox"));
    /// use activitystreams::prelude::*;
    ///
    /// assert!(person.outbox_unchecked().is_some());
    /// person.delete_outbox();
    /// assert!(person.outbox_unchecked().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_outbox(&mut self) -> &mut Self {
        self.ap_actor_mut().outbox = None;
        self
    }

    /// Fetch the following link for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Ok(following) = person.following() {
    ///     println!("{:?}", following);
    /// }
    /// ```
    fn following<'a, Kind>(&'a self) -> Result<Option<&'a IriString>, CheckError>
    where
        Inner: 'a,
        Self: BaseExt<Kind>,
    {
        self.following_unchecked()
            .map(|following| self.check_authority(following))
            .transpose()
    }

    /// Fetch the following link for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(following) = person.following_unchecked() {
    ///     println!("{:?}", following);
    /// }
    /// ```
    fn following_unchecked<'a>(&'a self) -> Option<&'a IriString>
    where
        Inner: 'a,
    {
        self.ap_actor_ref().following.as_ref()
    }

    /// Mutably fetch the following link for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(following) = person.following_mut() {
    ///     println!("{:?}", following);
    /// }
    /// ```
    fn following_mut<'a>(&'a mut self) -> Option<&'a mut IriString>
    where
        Inner: 'a,
    {
        self.ap_actor_mut().following.as_mut()
    }

    /// Set the following link for the current actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// person.set_following(iri!("https://example.com/following"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_following(&mut self, following: IriString) -> &mut Self {
        self.ap_actor_mut().following = Some(following);
        self
    }

    /// Take the following link from the current actor, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(following) = person.take_following() {
    ///     println!("{:?}", following);
    /// }
    /// ```
    fn take_following(&mut self) -> Option<IriString> {
        self.ap_actor_mut().following.take()
    }

    /// Delete the following link from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// # person.set_following(iri!("https://example.com/following"));
    /// use activitystreams::prelude::*;
    ///
    /// assert!(person.following_unchecked().is_some());
    /// person.delete_following();
    /// assert!(person.following_unchecked().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_following(&mut self) -> &mut Self {
        self.ap_actor_mut().following = None;
        self
    }

    /// Fetch the followers link for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Ok(followers) = person.followers() {
    ///     println!("{:?}", followers);
    /// }
    /// ```
    fn followers<'a, Kind>(&'a self) -> Result<Option<&'a IriString>, CheckError>
    where
        Inner: 'a,
        Self: BaseExt<Kind>,
    {
        self.followers_unchecked()
            .map(|followers| self.check_authority(followers))
            .transpose()
    }

    /// Fetch the followers link for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(followers) = person.followers_unchecked() {
    ///     println!("{:?}", followers);
    /// }
    /// ```
    fn followers_unchecked<'a>(&'a self) -> Option<&'a IriString>
    where
        Inner: 'a,
    {
        self.ap_actor_ref().followers.as_ref()
    }

    /// Mutably fetch the followers link for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(followers) = person.followers_mut() {
    ///     println!("{:?}", followers);
    /// }
    /// ```
    fn followers_mut<'a>(&'a mut self) -> Option<&'a mut IriString>
    where
        Inner: 'a,
    {
        self.ap_actor_mut().followers.as_mut()
    }

    /// Set the followers link for the current actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// person.set_followers(iri!("https://example.com/followers"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_followers(&mut self, followers: IriString) -> &mut Self {
        self.ap_actor_mut().followers = Some(followers);
        self
    }

    /// Take the followers link from the current actor, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(followers) = person.take_followers() {
    ///     println!("{:?}", followers);
    /// }
    /// ```
    fn take_followers(&mut self) -> Option<IriString> {
        self.ap_actor_mut().followers.take()
    }

    /// Delete the followers link from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// # person.set_followers(iri!("https://example.com/followers"));
    /// use activitystreams::prelude::*;
    ///
    /// assert!(person.followers_unchecked().is_some());
    /// person.delete_followers();
    /// assert!(person.followers_unchecked().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_followers(&mut self) -> &mut Self {
        self.ap_actor_mut().followers = None;
        self
    }

    /// Fetch the liked link for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Ok(liked) = person.liked() {
    ///     println!("{:?}", liked);
    /// }
    /// ```
    fn liked<'a, Kind>(&'a self) -> Result<Option<&'a IriString>, CheckError>
    where
        Inner: 'a,
        Self: BaseExt<Kind>,
    {
        self.liked_unchecked()
            .map(|liked| self.check_authority(liked))
            .transpose()
    }

    /// Fetch the liked link for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(liked) = person.liked_unchecked() {
    ///     println!("{:?}", liked);
    /// }
    /// ```
    fn liked_unchecked<'a>(&'a self) -> Option<&'a IriString>
    where
        Inner: 'a,
    {
        self.ap_actor_ref().liked.as_ref()
    }

    /// Mutably fetch the liked link for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(liked) = person.liked_mut() {
    ///     println!("{:?}", liked);
    /// }
    /// ```
    fn liked_mut<'a>(&'a mut self) -> Option<&'a mut IriString>
    where
        Inner: 'a,
    {
        self.ap_actor_mut().liked.as_mut()
    }

    /// Set the liked link for the current actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// person.set_stream(iri!("https://example.com/liked"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_liked(&mut self, liked: IriString) -> &mut Self {
        self.ap_actor_mut().liked = Some(liked);
        self
    }

    /// Take the liked link from the current actor, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(liked) = person.take_liked() {
    ///     println!("{:?}", liked);
    /// }
    /// ```
    fn take_liked(&mut self) -> Option<IriString> {
        self.ap_actor_mut().liked.take()
    }

    /// Delete the liked link from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// # person.set_liked(iri!("https://example.com/liked"));
    /// use activitystreams::prelude::*;
    ///
    /// assert!(person.liked_unchecked().is_some());
    /// person.delete_liked();
    /// assert!(person.liked_unchecked().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_liked(&mut self) -> &mut Self {
        self.ap_actor_mut().liked = None;
        self
    }

    /// Fetch the streams links for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Ok(streams) = person.streams() {
    ///     println!("{:?}", streams);
    /// }
    /// ```
    fn streams<'a, Kind>(&'a self) -> Result<Option<OneOrMany<&'a IriString>>, CheckError>
    where
        Inner: 'a,
        Self: BaseExt<Kind>,
    {
        if let Some(streams) = self.streams_unchecked() {
            for id in &streams {
                self.check_authority(id)?;
            }

            Ok(Some(streams))
        } else {
            Ok(None)
        }
    }

    /// Fetch the streams links for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(streams) = person.streams_unchecked() {
    ///     println!("{:?}", streams);
    /// }
    /// ```
    fn streams_unchecked<'a>(&'a self) -> Option<OneOrMany<&'a IriString>>
    where
        Inner: 'a,
    {
        self.ap_actor_ref().streams.as_ref().map(|o| o.as_ref())
    }

    /// Mutably fetch the streams links for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(mut streams) = person.streams_mut() {
    ///     for url in streams.iter_mut() {
    ///         // whatever
    ///     }
    ///     println!("{:?}", streams);
    /// }
    /// ```
    fn streams_mut<'a>(&'a mut self) -> Option<OneOrMany<&'a mut IriString>>
    where
        Inner: 'a,
    {
        self.ap_actor_mut().streams.as_mut().map(|o| o.as_mut())
    }

    /// Set the streams links for the current actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// person.set_stream(iri!("https://example.com/streams"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_stream(&mut self, streams: IriString) -> &mut Self {
        self.ap_actor_mut().streams = Some(streams.into());
        self
    }

    /// Set many streams links for the current actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// person.set_many_streams(vec![
    ///     iri!("https://example.com/streams1"),
    ///     iri!("https://example.com/streams2"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_streams<I, U>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = U>,
        U: Into<IriString>,
    {
        let v: Vec<IriString> = items.into_iter().map(|u| u.into()).collect();
        self.ap_actor_mut().streams = Some(v.into());
        self
    }

    /// Add a streams link for the current actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// person
    ///     .add_stream(iri!("https://example.com/streams1"))
    ///     .add_stream(iri!("https://example.com/streams2"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_stream(&mut self, stream: IriString) -> &mut Self {
        let v = match self.ap_actor_mut().streams.take() {
            Some(mut v) => {
                v.add(stream);
                v
            }
            None => vec![stream].into(),
        };
        self.ap_actor_mut().streams = Some(v);
        self
    }

    /// Take the streams links from the current actor, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(streams) = person.take_streams() {
    ///     println!("{:?}", streams);
    /// }
    /// ```
    fn take_streams(&mut self) -> Option<OneOrMany<IriString>> {
        self.ap_actor_mut().streams.take()
    }

    /// Delete the streams links from the current object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// # person.set_stream(iri!("https://example.com/streams"));
    /// use activitystreams::prelude::*;
    ///
    /// assert!(person.streams_unchecked().is_some());
    /// person.delete_streams();
    /// assert!(person.streams_unchecked().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_streams(&mut self) -> &mut Self {
        self.ap_actor_mut().streams = None;
        self
    }

    /// Fetch the preferred_username for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(preferred_username) = person.preferred_username() {
    ///     println!("{:?}", preferred_username);
    /// }
    /// ```
    fn preferred_username<'a>(&'a self) -> Option<&'a str>
    where
        Inner: 'a,
    {
        self.ap_actor_ref().preferred_username.as_deref()
    }

    /// Set the preferred_username for the current actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// person.set_preferred_username("user123");
    /// # Ok(())
    /// # }
    /// ```
    fn set_preferred_username<T>(&mut self, string: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.ap_actor_mut().preferred_username = Some(string.into());
        self
    }

    /// Take the preferred_username from the current actor, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(preferred_username) = person.take_preferred_username() {
    ///     println!("{:?}", preferred_username);
    /// }
    /// ```
    fn take_preferred_username(&mut self) -> Option<String> {
        self.ap_actor_mut().preferred_username.take()
    }

    /// Delete the preferred_username from the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// # person.set_preferred_username("hey");
    /// use activitystreams::prelude::*;
    ///
    /// assert!(person.preferred_username().is_some());
    /// person.delete_preferred_username();
    /// assert!(person.preferred_username().is_none());
    /// ```
    fn delete_preferred_username(&mut self) -> &mut Self {
        self.ap_actor_mut().preferred_username = None;
        self
    }

    /// Fetch the endpoints for the current actor, erroring if the Endpoints' domains do not
    /// match the ID's domain
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Endpoints, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// # person.set_id(context()).set_endpoints(Endpoints {
    /// #   shared_inbox: Some(context()),
    /// #   ..Default::default()
    /// # });
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(endpoints) = person.endpoints()? {
    ///     println!("{:?}", endpoints);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn endpoints<'a, Kind>(&'a self) -> Result<Option<Endpoints<&'a IriString>>, CheckError>
    where
        Self: BaseExt<Kind>,
        Inner: 'a,
        Kind: 'a,
    {
        if let Some(endpoints) = self.endpoints_unchecked() {
            let authority_opt = self.id_unchecked().and_then(|id| id.authority_components());

            let mut any_failed = false;

            any_failed |= endpoints
                .proxy_url
                .map(|u| u.authority_components() != authority_opt)
                .unwrap_or(false);
            any_failed |= endpoints
                .oauth_authorization_endpoint
                .map(|u| u.authority_components() != authority_opt)
                .unwrap_or(false);
            any_failed |= endpoints
                .oauth_token_endpoint
                .map(|u| u.authority_components() != authority_opt)
                .unwrap_or(false);
            any_failed |= endpoints
                .provide_client_key
                .map(|u| u.authority_components() != authority_opt)
                .unwrap_or(false);
            any_failed |= endpoints
                .sign_client_key
                .map(|u| u.authority_components() != authority_opt)
                .unwrap_or(false);
            any_failed |= endpoints
                .shared_inbox
                .map(|u| u.authority_components() != authority_opt)
                .unwrap_or(false);

            if any_failed {
                return Err(CheckError);
            }

            return Ok(Some(endpoints));
        }

        Ok(None)
    }

    /// Fetch the endpoints for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(endpoints) = person.endpoints_unchecked() {
    ///     println!("{:?}", endpoints);
    /// }
    /// ```
    fn endpoints_unchecked<'a>(&'a self) -> Option<Endpoints<&'a IriString>>
    where
        Inner: 'a,
    {
        self.ap_actor_ref().endpoints.as_ref().map(|e| e.as_ref())
    }

    /// Mutably fetch the endpoints for the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(mut endpoints) = person.endpoints_mut() {
    ///     if let Some(url) = endpoints.shared_inbox.as_mut() {
    ///         // whatever
    ///     }
    ///     println!("{:?}", endpoints);
    /// }
    /// ```
    fn endpoints_mut<'a>(&'a mut self) -> Option<Endpoints<&'a mut IriString>>
    where
        Inner: 'a,
    {
        self.ap_actor_mut().endpoints.as_mut().map(|e| e.as_mut())
    }

    /// Set the endpoints for the current actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{actor::{ApActor, Endpoints, Person}, context, iri};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// person.set_endpoints(Endpoints {
    ///     shared_inbox: Some(iri!("https://example.com/inbox")),
    ///     ..Default::default()
    /// });
    /// # Ok(())
    /// # }
    /// ```
    fn set_endpoints(&mut self, endpoints: Endpoints<IriString>) -> &mut Self {
        self.ap_actor_mut().endpoints = Some(endpoints);
        self
    }

    /// Take the endpoints from the current actor, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(endpoints) = person.take_endpoints() {
    ///     println!("{:?}", endpoints);
    /// }
    /// ```
    fn take_endpoints(&mut self) -> Option<Endpoints<IriString>> {
        self.ap_actor_mut().endpoints.take()
    }

    /// Delete the endpoints from the current actor
    ///
    /// ```rust
    /// # use activitystreams::{actor::{ApActor, Person}, context};
    /// # let mut person = ApActor::new(context(), Person::new());
    /// # person.set_endpoints(Default::default());
    /// use activitystreams::prelude::*;
    ///
    /// assert!(person.endpoints_unchecked().is_some());
    /// person.delete_endpoints();
    /// assert!(person.endpoints_unchecked().is_none());
    /// ```
    fn delete_endpoints(&mut self) -> &mut Self {
        self.ap_actor_mut().endpoints = None;
        self
    }
}

/// Describes a software application.
///
/// This is just an alias for `Actor<ApplicationType>` because there's no fields inherent to
/// Application that aren't already present on an Actor.
pub type Application = Actor<ApplicationType>;

/// Represents a formal or informal collective of Actors.
///
/// This is just an alias for `Actor<GroupType>` because there's no fields inherent to
/// Group that aren't already present on an Actor.
pub type Group = Actor<GroupType>;

/// Represents an organization.
///
/// This is just an alias for `Actor<OrganizationType>` because there's no fields inherent to
/// Organization that aren't already present on an Actor.
pub type Organization = Actor<OrganizationType>;

/// Represents an individual person.
///
/// This is just an alias for `Actor<PersonType>` because there's no fields inherent to
/// Person that aren't already present on an Actor.
pub type Person = Actor<PersonType>;

/// Represents a service of any kind.
///
/// This is just an alias for `Actor<ServiceType>` because there's no fields inherent to
/// Service that aren't already present on an Actor.
pub type Service = Actor<ServiceType>;

/// Actor types are Object types that are capable of performing activities.
///
/// This specification intentionally defines Actors in only the most generalized way, stopping
/// short of defining semantically specific properties for each. All Actor objects are
/// specializations of Object and inherit all of the core properties common to all Objects.
/// External vocabularies can be used to express additional detail not covered by the Activity
/// Vocabulary. VCard [vcard-rdf SHOULD be used to provide additional metadata for Person, Group,
/// and Organization instances.
///
/// While implementations are free to introduce new types of Actors beyond those defined by the
/// Activity Vocabulary, interoperability issues can arise when applications rely too much on
/// extension types that are not recognized by other implementations. Care should be taken to not
/// unduly overlap with or duplicate the existing Actor types.
///
/// When an implementation uses an extension type that overlaps with a core vocabulary type, the
/// implementation MUST also specify the core vocabulary type. For instance, some vocabularies
/// (e.g. VCard) define their own types for describing people. An implementation that wishes, for
/// example, to use a vcard:Individual as an Actor MUST also identify that Actor as a Person.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApActor<Inner> {
    /// A reference to an [ActivityStreams] OrderedCollection comprised of all the messages received by the actor.
    ///
    /// - Range: xsd:anyUri
    /// - Functional: true
    inbox: IriString,

    /// An ActivityStreams] OrderedCollection comprised of all the messages produced by the actor.
    ///
    /// - Range: xsd:anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    outbox: Option<IriString>,

    /// A link to an [ActivityStreams] collection of the actors that this actor is following.
    ///
    /// - Range: xsd:anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    following: Option<IriString>,

    /// A link to an [ActivityStreams] collection of the actors that follow this actor.
    ///
    /// - Range: xsd:anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    followers: Option<IriString>,

    /// A link to an [ActivityStreams] collection of objects this actor has liked.
    ///
    /// - Range: xsd:anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    liked: Option<IriString>,

    /// A list of supplementary Collections which may be of interest.
    ///
    /// - Range: xsd:anyUri
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    streams: Option<OneOrMany<IriString>>,

    /// A short username which may be used to refer to the actor, with no uniqueness guarantees.
    ///
    /// - Range: xsd:string
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_username: Option<String>,

    /// A json object which maps additional (typically server/domain-wide) endpoints which may be
    /// useful either for this actor or someone referencing this actor.
    ///
    /// This mapping may be nested inside the actor document as the value or may be a link to a
    /// JSON-LD document with these properties.
    ///
    /// - Range: Endpoint
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<Endpoints<IriString>>,

    /// base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Inner,
}

/// A json object which maps additional (typically server/domain-wide) endpoints which may be
/// useful either for this actor or someone referencing this actor.
///
/// This mapping may be nested inside the actor document as the value or may be a link to a
/// JSON-LD document with these properties.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoints<T> {
    /// Endpoint URI so this actor's clients may access remote ActivityStreams objects which
    /// require authentication to access.
    ///
    /// To use this endpoint, the client posts an x-www-form-urlencoded id parameter with the
    /// value being the id of the requested ActivityStreams object.
    ///
    /// - Range: anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<T>,

    /// If OAuth 2.0 bearer tokens [RFC6749](https://tools.ietf.org/html/rfc6749)
    /// [RFC6750](https://tools.ietf.org/html/rfc6750) are being used for authenticating client to
    /// server interactions, this endpoint specifies a URI at which a browser-authenticated user
    /// may obtain a new authorization grant.
    ///
    /// - Range: anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_authorization_endpoint: Option<T>,

    /// If OAuth 2.0 bearer tokens [RFC6749](https://tools.ietf.org/html/rfc6749)
    /// [RFC6750](https://tools.ietf.org/html/rfc6750) are being used for authenticating client to
    /// server interactions, this endpoint specifies a URI at which a client may acquire an access
    /// token.
    ///
    /// - Range: anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_token_endpoint: Option<T>,

    /// If Linked Data Signatures and HTTP Signatures are being used for authentication and
    /// authorization, this endpoint specifies a URI at which browser-authenticated users may
    /// authorize a client's public key for client to server interactions.
    ///
    /// - Range: anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provide_client_key: Option<T>,

    /// If Linked Data Signatures and HTTP Signatures are being used for authentication and
    /// authorization, this endpoint specifies a URI at which a client key may be signed by the
    /// actor's key for a time window to act on behalf of the actor in interacting with foreign
    /// servers.
    ///
    /// - Range: anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_client_key: Option<T>,

    /// An optional endpoint used for wide delivery of publicly addressed activities and
    /// activities sent to followers.
    ///
    /// shared_inbox endpoints SHOULD also be publicly readable OrderedCollection objects
    /// containing objects addressed to the Public special collection. Reading from the
    /// shared_inbox endpoint MUST NOT present objects which are not addressed to the Public
    /// endpoint.
    ///
    /// - Range: anyUri
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_inbox: Option<T>,
}

/// A simple type to create an Actor out of any Object
///
/// ```rust
/// # use activitystreams::{object::Object, actor::Actor, prelude::*};
/// let object = Object::<String>::new();
/// let mut actor = Actor(object);
/// actor.set_kind("MyCustomActor".into());
/// ```
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct Actor<Kind>(pub Object<Kind>);

impl<Kind> Actor<Kind> {
    /// Create a new Actor
    ///
    /// ```rust
    /// # use activitystreams::actor::Actor;
    /// let actor = Actor::<String>::new();
    /// ```
    pub fn new() -> Self
    where
        Kind: Default,
    {
        Actor(Object::new())
    }

    /// Create a new actor with `None` for it's `kind` property
    ///
    /// This means that no `type` field will be present in serialized JSON
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::actor::Actor;
    ///
    /// let actor = Actor::<()>::new_none_type();
    ///
    /// let s = serde_json::to_string(&actor)?;
    ///
    /// assert_eq!(s, "{}");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_none_type() -> Self {
        Actor(Object::new_none_type())
    }
}

impl<Inner> ApActor<Inner> {
    /// Create a new ActivityPub Actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::{actor::{ApActor, Person}, iri};
    ///
    /// let actor = ApActor::new(
    ///     iri!("https://example.com/inbox"),
    ///     Person::new(),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(inbox: IriString, inner: Inner) -> Self
    where
        Inner: markers::Actor,
    {
        ApActor {
            inbox,
            outbox: None,
            following: None,
            followers: None,
            liked: None,
            streams: None,
            preferred_username: None,
            endpoints: None,
            inner,
        }
    }

    #[allow(clippy::type_complexity)]
    /// Deconstruct the ApActor into its parts
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::{actor::{ApActor, Person}, iri};
    ///
    /// let actor = ApActor::new(iri!("https://inbox.url"), Person::new());
    ///
    /// let (
    ///     inbox,
    ///     outbox,
    ///     following,
    ///     followers,
    ///     liked,
    ///     streams,
    ///     preferred_username,
    ///     endpoints,
    ///     person,
    /// ) = actor.into_parts();
    /// # Ok(())
    /// # }
    /// ```
    pub fn into_parts(
        self,
    ) -> (
        IriString,
        Option<IriString>,
        Option<IriString>,
        Option<IriString>,
        Option<IriString>,
        Option<OneOrMany<IriString>>,
        Option<String>,
        Option<Endpoints<IriString>>,
        Inner,
    ) {
        (
            self.inbox,
            self.outbox,
            self.following,
            self.followers,
            self.liked,
            self.streams,
            self.preferred_username,
            self.endpoints,
            self.inner,
        )
    }

    fn extending(mut inner: Inner) -> Result<Self, serde_json::Error>
    where
        Inner: UnparsedMut + markers::Actor,
    {
        let inbox = inner.remove("inbox")?;
        let outbox = inner.remove("outbox")?;
        let following = inner.remove("following")?;
        let followers = inner.remove("followers")?;
        let liked = inner.remove("liked")?;
        let streams = inner.remove("streams")?;
        let preferred_username = inner.remove("preferredUsername")?;
        let endpoints = inner.remove("endpoints")?;

        Ok(ApActor {
            inbox,
            outbox,
            following,
            followers,
            liked,
            streams,
            preferred_username,
            endpoints,
            inner,
        })
    }

    fn retracting(self) -> Result<Inner, serde_json::Error>
    where
        Inner: UnparsedMut + markers::Actor,
    {
        let ApActor {
            inbox,
            outbox,
            following,
            followers,
            liked,
            streams,
            preferred_username,
            endpoints,
            mut inner,
        } = self;

        inner
            .insert("endpoints", endpoints)?
            .insert("preferredUsername", preferred_username)?
            .insert("streams", streams)?
            .insert("liked", liked)?
            .insert("followers", followers)?
            .insert("following", following)?
            .insert("outbox", outbox)?
            .insert("inbox", inbox)?;

        Ok(inner)
    }
}

impl<T> Endpoints<T> {
    /// Borrow the current Endpoints struct
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::{actor::Endpoints, iri};
    /// use iri_string::types::IriString;
    ///
    /// let uri = iri!("https://example.com");
    ///
    /// let endpoints: Endpoints<IriString> = Endpoints {
    ///     shared_inbox: Some(uri.clone()),
    ///     ..Default::default()
    /// };
    ///
    /// assert_eq!(endpoints.as_ref().shared_inbox, Some(&uri));
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_ref(&self) -> Endpoints<&T> {
        Endpoints {
            proxy_url: self.proxy_url.as_ref(),
            oauth_authorization_endpoint: self.oauth_authorization_endpoint.as_ref(),
            oauth_token_endpoint: self.oauth_token_endpoint.as_ref(),
            provide_client_key: self.provide_client_key.as_ref(),
            sign_client_key: self.sign_client_key.as_ref(),
            shared_inbox: self.shared_inbox.as_ref(),
        }
    }

    /// Mutably borrow the endpoints struct
    pub fn as_mut(&mut self) -> Endpoints<&mut T> {
        Endpoints {
            proxy_url: self.proxy_url.as_mut(),
            oauth_authorization_endpoint: self.oauth_authorization_endpoint.as_mut(),
            oauth_token_endpoint: self.oauth_token_endpoint.as_mut(),
            provide_client_key: self.provide_client_key.as_mut(),
            sign_client_key: self.sign_client_key.as_mut(),
            shared_inbox: self.shared_inbox.as_mut(),
        }
    }

    /// Map the URLs in Endpoints from T to U
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::{actor::Endpoints, iri};
    /// use iri_string::types::IriString;
    ///
    /// let endpoints: Endpoints<IriString> = Endpoints {
    ///     shared_inbox: Some(iri!("https://example.com")),
    ///     ..Default::default()
    /// };
    ///
    /// let endpoint_strings = endpoints.map(|u| u.to_string());
    ///
    /// assert_eq!(endpoint_strings.shared_inbox, Some(String::from("https://example.com")));
    /// # Ok(())
    /// # }
    /// ```
    pub fn map<F, U>(self, f: F) -> Endpoints<U>
    where
        F: Fn(T) -> U + Copy,
    {
        Endpoints {
            proxy_url: self.proxy_url.map(f),
            oauth_authorization_endpoint: self.oauth_authorization_endpoint.map(f),
            oauth_token_endpoint: self.oauth_token_endpoint.map(f),
            provide_client_key: self.provide_client_key.map(f),
            sign_client_key: self.sign_client_key.map(f),
            shared_inbox: self.shared_inbox.map(f),
        }
    }
}

impl<T> Default for Endpoints<T> {
    fn default() -> Self {
        Endpoints {
            proxy_url: None,
            oauth_authorization_endpoint: None,
            oauth_token_endpoint: None,
            provide_client_key: None,
            sign_client_key: None,
            shared_inbox: None,
        }
    }
}

impl<Kind> markers::Base for Actor<Kind> {}
impl<Kind> markers::Object for Actor<Kind> {}
impl<Kind> markers::Actor for Actor<Kind> {}

impl<Inner> markers::Base for ApActor<Inner> where Inner: markers::Base {}
impl<Inner> markers::Object for ApActor<Inner> where Inner: markers::Object {}
impl<Inner> markers::Actor for ApActor<Inner> where Inner: markers::Actor {}

impl<Inner> markers::Actor for ApObject<Inner> where Inner: markers::Actor {}

impl<Inner, Kind, Error> Extends<Kind> for ApActor<Inner>
where
    Inner: Extends<Kind, Error = Error> + UnparsedMut + markers::Actor,
    Error: From<serde_json::Error> + std::error::Error,
{
    type Error = Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::extends(base)?;
        Ok(Self::extending(inner)?)
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Inner> UnparsedMut for ApActor<Inner>
where
    Inner: UnparsedMut,
{
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Inner, Kind> AsBase<Kind> for ApActor<Inner>
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

impl<Inner, Kind> AsObject<Kind> for ApActor<Inner>
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

impl<Inner> AsApActor<Inner> for ApActor<Inner>
where
    Inner: markers::Actor,
{
    fn ap_actor_ref(&self) -> &ApActor<Inner> {
        self
    }

    fn ap_actor_mut(&mut self) -> &mut ApActor<Inner> {
        self
    }
}

impl<Inner1, Inner2> AsApObject<Inner2> for ApActor<Inner1>
where
    Inner1: AsApObject<Inner2>,
{
    fn ap_object_ref(&self) -> &ApObject<Inner2> {
        self.inner.ap_object_ref()
    }

    fn ap_object_mut(&mut self) -> &mut ApObject<Inner2> {
        self.inner.ap_object_mut()
    }
}

impl<Kind> Extends<Kind> for Actor<Kind> {
    type Error = serde_json::Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        Object::extends(base).map(Actor)
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        self.0.retracts()
    }
}

impl<Kind> UnparsedMut for Actor<Kind> {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.0.unparsed_mut()
    }
}

impl<Kind> AsBase<Kind> for Actor<Kind> {
    fn base_ref(&self) -> &Base<Kind> {
        self.0.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Kind> {
        self.0.base_mut()
    }
}

impl<Kind> AsObject<Kind> for Actor<Kind> {
    fn object_ref(&self) -> &Object<Kind> {
        self.0.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Kind> {
        self.0.object_mut()
    }
}

impl<T, Inner> ApActorExt<Inner> for T where T: AsApActor<Inner> {}

impl<Inner1, Inner2> AsApActor<Inner2> for ApObject<Inner1>
where
    Inner1: AsApActor<Inner2>,
{
    fn ap_actor_ref(&self) -> &ApActor<Inner2> {
        self.inner().ap_actor_ref()
    }

    fn ap_actor_mut(&mut self) -> &mut ApActor<Inner2> {
        self.inner_mut().ap_actor_mut()
    }
}

impl<Kind> Default for Actor<Kind>
where
    Kind: Default,
{
    fn default() -> Self {
        Self::new()
    }
}
