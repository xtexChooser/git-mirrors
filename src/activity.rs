//! Types and traits for dealing with Activity attributes
//!
//! ```rust
//! # fn main() -> Result<(), anyhow::Error> {
//! use activitystreams::{
//!     activity::Create,
//!     context,
//!     prelude::*,
//!     iri,
//! };
//!
//! let mut create = Create::new(
//!     iri!("https://example.com/actors/abcd"),
//!     iri!("https://example.com/notes/1234"),
//! );
//!
//! create
//!     .set_result(iri!("https://example.com/"))
//!     .set_instrument(iri!("https://example.com/"))
//!     .set_id(iri!("https://example.com/activities/abcd"))
//!     .set_context(context());
//! # Ok(())
//! # }
//! ```
use crate::{
    base::{AnyBase, AsBase, Base, Extends},
    checked::CheckError,
    markers,
    object::{ApObject, AsObject, Object},
    prelude::BaseExt,
    primitives::{Either, OneOrMany, XsdBoolean, XsdDateTime},
    unparsed::{Unparsed, UnparsedMut, UnparsedMutExt},
};
use iri_string::types::IriString;
use std::convert::TryFrom;
use time::OffsetDateTime;

pub use activitystreams_kinds::activity as kind;

use self::kind::*;

/// Implementation trait for deriving Activity methods for a type
///
/// Any type implementing AsObject will automatically gain methods provided by ActivityExt
pub trait AsActivity: markers::Activity {
    type Kind;

    /// Immutable borrow of `Activity<Kind>`
    fn activity_ref(&self) -> &Activity<Self::Kind>;

    /// Mutable borrow of `Activity<Kind>`
    fn activity_mut(&mut self) -> &mut Activity<Self::Kind>;
}

/// Implementation trait for deriving Actor and Object methods for a type
///
/// Any type implementing AsActivityActor will automatically gain methods provided by
/// `AsActivityActorExt`
pub trait AsActivityActor: markers::Activity {
    type Inner;

    fn activity_actor_ref(&self) -> &ActivityActor<Self::Inner>;

    fn activity_actor_mut(&mut self) -> &mut ActivityActor<Self::Inner>;
}

/// Implementation trait for deriving Actor and Object methods for a type
///
/// Any type implementing AsActivityObject will automatically gain methods provided by
/// `AsActivityObjectExt`
pub trait AsActivityObject: markers::Activity {
    type Inner;

    fn activity_object_ref(&self) -> &ActivityObject<Self::Inner>;

    fn activity_object_mut(&mut self) -> &mut ActivityObject<Self::Inner>;
}

/// Implementation trait for deriving Target methods for a type
///
/// Any type implementing AsTarget will automatically gain methods provided by `AsTargetExt`
pub trait AsTarget: markers::Activity {
    type Inner;

    fn target_ref(&self) -> &Target<Self::Inner>;

    fn target_mut(&mut self) -> &mut Target<Self::Inner>;
}

/// Implementation trait for deriving Origin methods for a type
///
/// Any type implementing AsOrigin will automatically gain methods provided by
/// `AsOriginExt`
pub trait AsOrigin: markers::Activity {
    type Inner;

    fn origin_ref(&self) -> &Origin<Self::Inner>;

    fn origin_mut(&mut self) -> &mut Origin<Self::Inner>;
}

/// Implementation trait for deriving Target methods for a type
///
/// Any type implementing AsOptTarget will automatically gain methods provided by
/// `AsOptTargetExt`
pub trait AsOptTarget: markers::Activity {
    type Inner;

    fn opt_target_ref(&self) -> &OptTarget<Self::Inner>;

    fn opt_target_mut(&mut self) -> &mut OptTarget<Self::Inner>;
}

/// Implementation trait for deriving Origin methods for a type
///
/// Any type implementing AsOptOrigin will automatically gain methods provided by
/// `AsOptOriginExt`
pub trait AsOptOrigin: markers::Activity {
    type Inner;

    fn opt_origin_ref(&self) -> &OptOrigin<Self::Inner>;

    fn opt_origin_mut(&mut self) -> &mut OptOrigin<Self::Inner>;
}

/// Implementation trait for deriving Question methods for a type
///
/// Any type implementing AsQuestion will automatically gain methods provided by
/// `QuestionExt`
pub trait AsQuestion: markers::Activity {
    /// Immutable borrow of Question
    fn question_ref(&self) -> &Question;

    /// Mutable borrow of Question
    fn question_mut(&mut self) -> &mut Question;
}

/// Helper methods for interacting with Activity types
///
/// This trait represents methods valid for any ActivityStreams Activity
///
/// Documentation for the fields related to these methods can be found on the `Activity`
/// struct
pub trait ActivityExt: AsActivity {
    /// Fetch the result for the current activity
    ///
    /// ```rust
    /// # use activitystreams::activity::Question;
    /// # let mut question = Question::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(result) = question.result() {
    ///     println!("{:?}", result);
    /// }
    /// ```
    fn result<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Self::Kind: 'a,
    {
        self.activity_ref().result.as_ref()
    }

    /// Set the result for the current activity
    ///
    /// This overwrites the contents of result
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question.set_result(iri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_result<T>(&mut self, result: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.activity_mut().result = Some(result.into().into());
        self
    }

    /// Set many results for the current activity
    ///
    /// This overwrites the contents of result
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question.set_many_results(vec![
    ///     iri!("https://example.com/one"),
    ///     iri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_results<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.activity_mut().result = Some(v.into());
        self
    }

    /// Add a result to the current activity
    ///
    /// This does not overwrite the contents of result, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::{prelude::*, iri};
    /// # use activitystreams::{activity::Question};
    /// # let mut question = Question::new();
    ///
    /// question
    ///     .add_result(iri!("https://example.com/one"))
    ///     .add_result(iri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_result<T>(&mut self, result: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let c = match self.activity_mut().result.take() {
            Some(mut c) => {
                c.add(result.into());
                c
            }
            None => vec![result.into()].into(),
        };
        self.activity_mut().result = Some(c);
        self
    }

    /// Take the result from the current activity, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::activity::Question;
    /// # let mut question = Question::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(result) = question.take_result() {
    ///     println!("{:?}", result);
    /// }
    /// ```
    fn take_result(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.activity_mut().result.take()
    }

    /// Delete the result from the current activity
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    /// # question.set_result(iri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(question.result().is_some());
    /// question.delete_result();
    /// assert!(question.result().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_result(&mut self) -> &mut Self {
        self.activity_mut().result = None;
        self
    }

    /// Fetch the instrument for the current activity
    ///
    /// ```rust
    /// # use activitystreams::activity::Question;
    /// # let mut question = Question::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(instrument) = question.instrument() {
    ///     println!("{:?}", instrument);
    /// }
    /// ```
    fn instrument<'a>(&'a self) -> Option<&'a OneOrMany<AnyBase>>
    where
        Self::Kind: 'a,
    {
        self.activity_ref().instrument.as_ref()
    }

    /// Set the instrument for the current activity
    ///
    /// This overwrites the contents of instrument
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question.set_instrument(iri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_instrument<T>(&mut self, instrument: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.activity_mut().instrument = Some(instrument.into().into());
        self
    }

    /// Set many instruments for the current activity
    ///
    /// This overwrites the contents of instrument
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question.set_many_instruments(vec![
    ///     iri!("https://example.com/one"),
    ///     iri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_instruments<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.activity_mut().instrument = Some(v.into());
        self
    }

    /// Add a instrument to the current activity
    ///
    /// This does not overwrite the contents of instrument, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question
    ///     .add_instrument(iri!("https://example.com/one"))
    ///     .add_instrument(iri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_instrument<T>(&mut self, instrument: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let c = match self.activity_mut().instrument.take() {
            Some(mut c) => {
                c.add(instrument.into());
                c
            }
            None => vec![instrument.into()].into(),
        };
        self.activity_mut().instrument = Some(c);
        self
    }

    /// Take the instrument from the current activity, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::activity::Question;
    /// # let mut question = Question::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(instrument) = question.take_instrument() {
    ///     println!("{:?}", instrument);
    /// }
    /// ```
    fn take_instrument(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.activity_mut().instrument.take()
    }

    /// Delete the instrument from the current activity
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    /// # question.set_instrument(iri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(question.instrument().is_some());
    /// question.delete_instrument();
    /// assert!(question.instrument().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_instrument(&mut self) -> &mut Self {
        self.activity_mut().instrument = None;
        self
    }
}

/// Helper methods for interacting with Activity types with actor and object fields
///
/// Documentation for the fields related to these methods can be found on the
/// `ActorAndObject` struct
pub trait AsActivityActorExt: AsActivityActor {
    /// Fetch the actor for the current activity
    ///
    /// ```rust
    /// # use activitystreams::{context, activity::Create};
    /// # let mut create = Create::new(context(), context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// let actor_ref = create.actor();
    /// println!("{:?}", actor_ref);
    /// ```
    fn actor(&self) -> Result<&OneOrMany<AnyBase>, CheckError>
    where
        Self: BaseExt,
    {
        let actor = self.actor_unchecked();

        for any_base in actor {
            let id = any_base.id().ok_or(CheckError)?;
            self.check_authority(id)?;
        }

        Ok(actor)
    }

    /// Fetch the actor for the current activity
    ///
    /// ```rust
    /// # use activitystreams::{context, activity::Create};
    /// # let mut create = Create::new(context(), context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// let actor_ref = create.actor_unchecked();
    /// println!("{:?}", actor_ref);
    /// ```
    fn actor_unchecked(&self) -> &OneOrMany<AnyBase> {
        &self.activity_actor_ref().actor
    }

    /// Check if the actor's ID is `id`
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{context, activity::Create, iri};
    /// # let mut create = Create::new(context(), context());
    /// use activitystreams::prelude::*;
    ///
    /// create.set_actor(iri!("https://example.com"));
    ///
    /// assert!(create.actor_is(&iri!("https://example.com")));
    /// # Ok(())
    /// # }
    /// ```
    fn actor_is(&self, id: &IriString) -> bool {
        self.actor_unchecked().is_single_id(id)
    }

    /// Set the actor for the current activity
    ///
    /// This overwrites the contents of actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Create, iri};
    /// # let mut create = Create::new(context(), context());
    ///
    /// create.set_actor(iri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_actor<T>(&mut self, actor: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.activity_actor_mut().actor = actor.into().into();
        self
    }

    /// Set many actors for the current activity
    ///
    /// This overwrites the contents of actor
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Create, iri};
    /// # let mut create = Create::new(context(), context());
    ///
    /// create.set_many_actors(vec![
    ///     iri!("https://example.com/one"),
    ///     iri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_actors<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.activity_actor_mut().actor = v.into();
        self
    }

    /// Add a actor to the current activity
    ///
    /// This does not overwrite the contents of actor, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Create, iri};
    /// # let mut create = Create::new(context(), context());
    ///
    /// create
    ///     .add_actor(iri!("https://example.com/one"))
    ///     .add_actor(iri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_actor<T>(&mut self, actor: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.activity_actor_mut().actor.add(actor.into());
        self
    }
}

pub trait AsActivityObjectExt: AsActivityObject {
    /// Fetch the object for the current activity
    ///
    /// ```rust
    /// # use activitystreams::{context, activity::Create};
    /// # let mut create = Create::new(context(), context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// let object_ref = create.object();
    /// println!("{:?}", object_ref);
    /// ```
    fn object(&self) -> Result<&OneOrMany<AnyBase>, CheckError>
    where
        Self: BaseExt,
    {
        let object = self.object_unchecked();

        for any_base in object {
            let id = any_base.id().ok_or(CheckError)?;
            self.check_authority(id)?;
        }

        Ok(object)
    }

    /// Fetch the object for the current activity
    ///
    /// ```rust
    /// # use activitystreams::{context, activity::Create};
    /// # let mut create = Create::new(context(), context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// let object_ref = create.object_unchecked();
    /// println!("{:?}", object_ref);
    /// ```
    fn object_unchecked(&self) -> &OneOrMany<AnyBase> {
        &self.activity_object_ref().object
    }

    /// Check if the object's ID is `id`
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{context, activity::Create, iri};
    /// # let mut create = Create::new(context(), context());
    /// use activitystreams::prelude::*;
    ///
    /// create.set_object(iri!("https://example.com"));
    ///
    /// assert!(create.object_is(&iri!("https://example.com")));
    /// # Ok(())
    /// # }
    /// ```
    fn object_is(&self, id: &IriString) -> bool {
        self.object_unchecked().is_single_id(id)
    }

    /// Set the object for the current activity
    ///
    /// This overwrites the contents of object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Create, iri};
    /// # let mut create = Create::new(context(), context());
    ///
    /// create.set_object(iri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_object<T>(&mut self, object: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.activity_object_mut().object = object.into().into();
        self
    }

    /// Set many objects for the current activity
    ///
    /// This overwrites the contents of object
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Create, iri};
    /// # let mut create = Create::new(context(), context());
    ///
    /// create.set_many_objects(vec![
    ///     iri!("https://example.com/one"),
    ///     iri!("https://example.com/two"),
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
        self.activity_object_mut().object = v.into();
        self
    }

    /// Add a object to the current activity
    ///
    /// This does not overwrite the contents of object, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Create, iri};
    /// # let mut create = Create::new(context(), context());
    ///
    /// create
    ///     .add_object(iri!("https://example.com/one"))
    ///     .add_object(iri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_object<T>(&mut self, object: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.activity_object_mut().object.add(object.into());
        self
    }
}

/// Helper methods for interacting with Activity types with a target field
///
/// Documentation for the target field can be found on the `Invite` struct
pub trait AsTargetExt: AsTarget {
    /// Fetch the target for the current activity
    ///
    /// ```rust
    /// # use activitystreams::{context, activity::Invite};
    /// # let mut invite = Invite::new(context(), context(), context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// let target_ref = invite.target();
    /// println!("{:?}", target_ref);
    /// ```
    fn target(&self) -> &OneOrMany<AnyBase> {
        &self.target_ref().target
    }

    /// Set the target for the current activity
    ///
    /// This overwrites the contents of target
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Invite, iri};
    /// # let mut invite = Invite::new(context(), context(), context());
    ///
    /// invite.set_target(iri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_target<T>(&mut self, target: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.target_mut().target = target.into().into();
        self
    }

    /// Set many targets for the current activity
    ///
    /// This overwrites the contents of target
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Invite, iri};
    /// # let mut invite = Invite::new(context(), context(), context());
    ///
    /// invite.set_many_targets(vec![
    ///     iri!("https://example.com/one"),
    ///     iri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_targets<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.target_mut().target = v.into();
        self
    }

    /// Add a target to the current activity
    ///
    /// This does not overwrite the contents of target, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Invite, iri};
    /// # let mut invite = Invite::new(context(), context(), context());
    ///
    /// invite
    ///     .add_target(iri!("https://example.com/one"))
    ///     .add_target(iri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_target<T>(&mut self, target: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.target_mut().target.add(target.into());
        self
    }
}

/// Helper methods for interacting with Activity types with an origin
///
/// Documentation for the origin field can be found on the `Arrive` struct
pub trait AsOriginExt: AsOrigin {
    /// Fetch the origin for the current activity
    ///
    /// ```rust
    /// # use activitystreams::{context, activity::Arrive};
    /// # let mut arrive = Arrive::new(context(), context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// let origin_ref = arrive.origin();
    /// println!("{:?}", origin_ref);
    /// ```
    fn origin(&self) -> &OneOrMany<AnyBase> {
        &self.origin_ref().origin
    }

    /// Set the origin for the current activity
    ///
    /// This overwrites the contents of origin
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Arrive, iri};
    /// # let mut arrive = Arrive::new(context(), context());
    ///
    /// arrive.set_origin(iri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_origin<T>(&mut self, origin: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.origin_mut().origin = origin.into().into();
        self
    }

    /// Set many origins for the current activity
    ///
    /// This overwrites the contents of origin
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Arrive, iri};
    /// # let mut arrive = Arrive::new(context(), context());
    ///
    /// arrive.set_many_origins(vec![
    ///     iri!("https://example.com/one"),
    ///     iri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_origins<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.origin_mut().origin = v.into();
        self
    }

    /// Add a origin to the current activity
    ///
    /// This does not overwrite the contents of origin, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Arrive, iri};
    /// # let mut arrive = Arrive::new(context(), context());
    ///
    /// arrive
    ///     .add_origin(iri!("https://example.com/one"))
    ///     .add_origin(iri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_origin<T>(&mut self, origin: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.origin_mut().origin.add(origin.into());
        self
    }
}

/// Helper methods for interacting with Activity types with an optional target field
///
/// Documentation for the target field can be found on the
/// `OptTarget` struct
pub trait AsOptTargetExt: AsOptTarget {
    /// Fetch the target for the current activity
    ///
    /// ```rust
    /// # use activitystreams::{context, activity::Announce};
    /// # let mut announce = Announce::new(context(), context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(target_ref) = announce.target() {
    ///     println!("{:?}", target_ref);
    /// }
    /// ```
    fn target(&self) -> Option<&OneOrMany<AnyBase>> {
        self.opt_target_ref().target.as_ref()
    }

    /// Set the target for the current activity
    ///
    /// This overwrites the contents of target
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Announce, iri};
    /// # let mut announce = Announce::new(context(), context());
    ///
    /// announce.set_target(iri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_target<T>(&mut self, target: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.opt_target_mut().target = Some(target.into().into());
        self
    }

    /// Set many targets for the current activity
    ///
    /// This overwrites the contents of target
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Announce, iri};
    /// # let mut announce = Announce::new(context(), context());
    ///
    /// announce.set_many_targets(vec![
    ///     iri!("https://example.com/one"),
    ///     iri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_targets<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.opt_target_mut().target = Some(v.into());
        self
    }

    /// Add a target to the current activity
    ///
    /// This does not overwrite the contents of target, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Announce, iri};
    /// # let mut announce = Announce::new(context(), context());
    ///
    /// announce
    ///     .add_target(iri!("https://example.com/one"))
    ///     .add_target(iri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_target<T>(&mut self, target: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let c = match self.opt_target_mut().target.take() {
            Some(mut c) => {
                c.add(target.into());
                c
            }
            None => vec![target.into()].into(),
        };
        self.opt_target_mut().target = Some(c);
        self
    }

    /// Take a target from the current activity, leaving nothing
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{
    /// #   context,
    /// #   activity::Announce,
    /// # };
    /// # let mut announce = Announce::new(context(), context());
    ///
    /// if let Some(target) = announce.take_target() {
    ///     println!("{:?}", target);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn take_target(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.opt_target_mut().target.take()
    }

    /// Delete a target from the current activity
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{
    /// #   context,
    /// #   activity::Announce,
    /// # };
    /// # let mut announce = Announce::new(context(), context());
    /// # announce.set_target(context());
    ///
    /// assert!(announce.target().is_some());
    /// announce.delete_target();
    /// assert!(announce.target().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_target(&mut self) -> &mut Self {
        self.opt_target_mut().target = None;
        self
    }
}

/// Helper methods for interacting with Activity types with an optional origin field
///
/// Documentation for the origin field can be found on the
/// `Delete` struct
pub trait AsOptOriginExt: AsOptOrigin {
    /// Fetch the origin for the current activity
    ///
    /// ```rust
    /// # use activitystreams::{context, activity::Delete};
    /// # let mut delete = Delete::new(context(), context());
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(origin_ref) = delete.origin() {
    ///     println!("{:?}", origin_ref);
    /// }
    /// ```
    fn origin(&self) -> Option<&OneOrMany<AnyBase>> {
        self.opt_origin_ref().origin.as_ref()
    }

    /// Set the origin for the current activity
    ///
    /// This overwrites the contents of origin
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Delete, iri};
    /// # let mut delete = Delete::new(context(), context());
    ///
    /// delete.set_origin(iri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_origin<T>(&mut self, origin: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.opt_origin_mut().origin = Some(origin.into().into());
        self
    }

    /// Set many origins for the current activity
    ///
    /// This overwrites the contents of origin
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Delete, iri};
    /// # let mut delete = Delete::new(context(), context());
    ///
    /// delete.set_many_origins(vec![
    ///     iri!("https://example.com/one"),
    ///     iri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_origins<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.opt_origin_mut().origin = Some(v.into());
        self
    }

    /// Add a origin to the current activity
    ///
    /// This does not overwrite the contents of origin, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Delete, iri};
    /// # let mut delete = Delete::new(context(), context());
    ///
    /// delete
    ///     .add_origin(iri!("https://example.com/one"))
    ///     .add_origin(iri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_origin<T>(&mut self, origin: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let c = match self.opt_origin_mut().origin.take() {
            Some(mut c) => {
                c.add(origin.into());
                c
            }
            None => vec![origin.into()].into(),
        };
        self.opt_origin_mut().origin = Some(c);
        self
    }

    /// Take a origin from the current activity, leaving nothing
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Delete};
    /// # let mut delete = Delete::new(context(), context());
    ///
    /// if let Some(origin) = delete.take_origin() {
    ///     println!("{:?}", origin);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn take_origin(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.opt_origin_mut().origin.take()
    }

    /// Delete a origin from the current activity
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{context, activity::Delete};
    /// # let mut delete = Delete::new(context(), context());
    /// # delete.set_origin(context());
    ///
    /// assert!(delete.origin().is_some());
    /// delete.delete_origin();
    /// assert!(delete.origin().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_origin(&mut self) -> &mut Self {
        self.opt_origin_mut().origin = None;
        self
    }
}

/// Helper methods for interacting with Question types
///
/// This trait represents methods valid for an ActivityStreams Question
///
/// Documentation for the fields related to these methods can be found on the `Question`
/// struct
pub trait QuestionExt: AsQuestion {
    /// Fetch the one_of field for the current activity
    ///
    /// ```rust
    /// # use activitystreams::activity::Question;
    /// # let mut question = Question::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(one_of) = question.one_of() {
    ///     println!("{:?}", one_of);
    /// }
    /// ```
    fn one_of(&self) -> Option<&OneOrMany<AnyBase>> {
        self.question_ref().one_of.as_ref()
    }

    /// Set the one_of field for the current activity
    ///
    /// This overwrites the contents of one_of
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question.set_one_of(iri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_one_of<T>(&mut self, one_of: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.question_mut().one_of = Some(one_of.into().into());
        self
    }

    /// Set many one_of items for the current activity
    ///
    /// This overwrites the contents of one_of
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question.set_many_one_ofs(vec![
    ///     iri!("https://example.com/one"),
    ///     iri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_one_ofs<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.question_mut().one_of = Some(v.into());
        self
    }

    /// Add a one_of to the current activity
    ///
    /// This does not overwrite the contents of one_of, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question
    ///     .add_one_of(iri!("https://example.com/one"))
    ///     .add_one_of(iri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_one_of<T>(&mut self, one_of: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let v = match self.question_mut().one_of.take() {
            Some(mut v) => {
                v.add(one_of.into());
                v
            }
            None => vec![one_of.into()].into(),
        };
        self.question_mut().one_of = Some(v);
        self
    }

    /// Take the one_of field from the current activity, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::activity::Question;
    /// # let mut question = Question::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(one_of) = question.take_one_of() {
    ///     println!("{:?}", one_of);
    /// }
    /// ```
    fn take_one_of(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.question_mut().one_of.take()
    }

    /// Delete the one_of field from the current activity
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    /// # question.set_one_of(iri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(question.one_of().is_some());
    /// question.delete_one_of();
    /// assert!(question.one_of().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_one_of(&mut self) -> &mut Self {
        self.question_mut().one_of = None;
        self
    }

    /// Fetch the any_of field for the current activity
    ///
    /// ```rust
    /// # use activitystreams::activity::Question;
    /// # let mut question = Question::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(any_of) = question.any_of() {
    ///     println!("{:?}", any_of);
    /// }
    /// ```
    fn any_of(&self) -> Option<&OneOrMany<AnyBase>> {
        self.question_ref().any_of.as_ref()
    }

    /// Set the any_of field for the current activity
    ///
    /// This overwrites the contents of any_of
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question.set_any_of(iri!("https://example.com"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_any_of<T>(&mut self, any_of: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.question_mut().any_of = Some(any_of.into().into());
        self
    }

    /// Set many any_of items for the current activity
    ///
    /// This overwrites the contents of any_of
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question.set_many_any_ofs(vec![
    ///     iri!("https://example.com/one"),
    ///     iri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_any_ofs<I, T>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let v: Vec<_> = items.into_iter().map(Into::into).collect();
        self.question_mut().any_of = Some(v.into());
        self
    }

    /// Add an any_of to the current activity
    ///
    /// This does not overwrite the contents of any_of, only appends an item
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question
    ///     .add_any_of(iri!("https://example.com/one"))
    ///     .add_any_of(iri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_any_of<T>(&mut self, any_of: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let v = match self.question_mut().any_of.take() {
            Some(mut v) => {
                v.add(any_of.into());
                v
            }
            None => vec![any_of.into()].into(),
        };
        self.question_mut().any_of = Some(v);
        self
    }

    /// Take the any_of field from the current activity, leaving nothing
    ///
    /// ```rust
    /// # use activitystreams::activity::Question;
    /// # let mut question = Question::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(any_of) = question.take_any_of() {
    ///     println!("{:?}", any_of);
    /// }
    /// ```
    fn take_any_of(&mut self) -> Option<OneOrMany<AnyBase>> {
        self.question_mut().any_of.take()
    }

    /// Delete the any_of field from the current activity
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    /// # question.set_any_of(iri!("https://example.com"));
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(question.any_of().is_some());
    /// question.delete_any_of();
    /// assert!(question.any_of().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_any_of(&mut self) -> &mut Self {
        self.question_mut().any_of = None;
        self
    }

    /// Fetch the closed field for the current activity
    ///
    /// ```rust
    /// # use activitystreams::activity::Question;
    /// # let mut question = Question::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(closed) = question.closed() {
    ///     println!("{:?}", closed);
    /// }
    /// ```
    fn closed(&self) -> Option<Either<&OneOrMany<AnyBase>, Either<OffsetDateTime, bool>>> {
        self.question_ref().closed.as_ref().map(|either| {
            either
                .as_ref()
                .map_right(|r| r.as_ref().map_left(|l| *l.as_datetime()).map_right(|r| r.0))
        })
    }

    /// Set the closed field for the current activity
    ///
    /// This overwrites the contents of any_of
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question.set_closed_base(iri!("https://example.com/one"));
    /// # Ok(())
    /// # }
    /// ```
    fn set_closed_base<T>(&mut self, closed: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        self.question_mut().closed = Some(Either::Left(OneOrMany::from_one(closed.into())));
        self
    }

    /// Set many closed items for the current activity
    ///
    /// This overwrites the contents of any_of
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question.set_many_closed_bases(vec![
    ///     iri!("https://example.com/one"),
    ///     iri!("https://example.com/two"),
    /// ]);
    /// # Ok(())
    /// # }
    /// ```
    fn set_many_closed_bases<I, T>(&mut self, closed: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<AnyBase>,
    {
        let many = OneOrMany::from_many(closed.into_iter().map(|t| t.into()).collect());
        self.question_mut().closed = Some(Either::Left(many));
        self
    }

    /// Set the closed field as a date
    ///
    /// This overwrites the contents of any_of
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question.set_closed_date(time::OffsetDateTime::now_utc());
    /// # Ok(())
    /// # }
    /// ```
    fn set_closed_date(&mut self, closed: OffsetDateTime) -> &mut Self {
        self.question_mut().closed = Some(Either::Right(Either::Left(closed.into())));
        self
    }

    /// Set the closed field as a boolean
    ///
    /// This overwrites the contents of any_of
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question.set_closed_bool(true);
    /// # Ok(())
    /// # }
    /// ```
    fn set_closed_bool(&mut self, closed: bool) -> &mut Self {
        self.question_mut().closed = Some(Either::Right(Either::Right(closed.into())));
        self
    }

    /// Add an object or link to the closed field
    ///
    /// This overwrites the contents of any_of
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::prelude::*;
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    ///
    /// question
    ///     .add_closed_base(iri!("https://example.com/one"))
    ///     .add_closed_base(iri!("https://example.com/two"));
    /// # Ok(())
    /// # }
    /// ```
    fn add_closed_base<T>(&mut self, closed: T) -> &mut Self
    where
        T: Into<AnyBase>,
    {
        let one_or_many = match self.question_mut().closed.take() {
            Some(Either::Left(mut one_or_many)) => {
                one_or_many.add(closed.into());
                one_or_many
            }
            _ => OneOrMany::from_one(closed.into()),
        };

        self.question_mut().closed = Some(Either::Left(one_or_many));
        self
    }

    /// Take the closed field from the current activity
    ///
    /// ```rust
    /// # use activitystreams::activity::Question;
    /// # let mut question = Question::new();
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// if let Some(closed) = question.take_closed() {
    ///     println!("{:?}", closed);
    /// }
    /// ```
    fn take_closed(&mut self) -> Option<Either<OneOrMany<AnyBase>, Either<OffsetDateTime, bool>>> {
        self.question_mut()
            .closed
            .take()
            .map(|either| either.map_right(|r| r.map(|date| date.into(), |b| b.into())))
    }

    /// Remove the closed field from the current activity
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # use activitystreams::{activity::Question, iri};
    /// # let mut question = Question::new();
    /// # question.set_closed_bool(true);
    /// #
    /// use activitystreams::prelude::*;
    ///
    /// assert!(question.closed().is_some());
    /// question.delete_closed();
    /// assert!(question.closed().is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn delete_closed(&mut self) -> &mut Self {
        self.question_mut().closed = None;
        self
    }
}

/// Indicates that the actor accepts the object.
///
/// The target property can be used in certain circumstances to indicate the context into which the
/// object has been accepted.
///
/// This is just an alias for `ActorAndObject<AcceptType>` because there's no fields inherent to
/// Accept that aren't already present on an ActorAndObject.
pub type Accept = ActorAndObject<AcceptType>;

/// Indicates that the actor has added the object to the target.
///
/// If the target property is not explicitly specified, the target would need to be determined
/// implicitly by context. The origin can be used to identify the context from which the object originated.
///
/// This is just an alias for `ActorAndObjectOptOriginAndTarget<AddType>` because there's no fields inherent to
/// Add that aren't already present on an OptOrigin.
pub type Add = ActorAndObjectOptOriginAndTarget<AddType>;

/// Indicates that the actor is blocking the object.
///
/// Blocking is a stronger form of Ignore. The typical use is to support social systems that allow
/// one user to block activities or content of other users. The target and origin typically have no
/// defined meaning.
///
/// This is just an alias for `ActorAndObject<BlockType>` because there's no fields inherent to
/// Block that aren't already present on an ActorAndObject.
pub type Block = ActorAndObject<BlockType>;

/// Indicates that the actor has created the object.
///
/// This is just an alias for `ActorAndObject<CreateType>` because there's no fields inherent to
/// Create that aren't already present on an ActorAndObject.
pub type Create = ActorAndObject<CreateType>;

/// Indicates that the actor dislikes the object.
///
/// This is just an alias for `ActorAndObject<DislikeType>` because there's no fields inherent to
/// Dislike that aren't already present on an ActorAndObject.
pub type Dislike = ActorAndObject<DislikeType>;

/// Indicates that the actor is "flagging" the object.
///
/// Flagging is defined in the sense common to many social platforms as reporting content as being
/// inappropriate for any number of reasons.
///
/// This is just an alias for `ActorAndObject<FlagType>` because there's no fields inherent to
/// Flag that aren't already present on an ActorAndObject.
pub type Flag = ActorAndObject<FlagType>;

/// Indicates that the actor is "following" the object.
///
/// Following is defined in the sense typically used within Social systems in which the actor is
/// interested in any activity performed by or on the object. The target and origin typically have
/// no defined meaning.
///
/// This is just an alias for `ActorAndObject<FollowType>` because there's no fields inherent to Follow
/// that aren't already present on an ActorAndObject.
pub type Follow = ActorAndObject<FollowType>;

/// Indicates that the actor is ignoring the object.
///
/// The target and origin typically have no defined meaning.
///
/// This is just an alias for `ActorAndObject<IgnoreType>` because there's no fields inherent to Ignore
/// that aren't already present on an ActorAndObject.
pub type Ignore = ActorAndObject<IgnoreType>;

/// Indicates that the actor has joined the object.
///
/// The target and origin typically have no defined meaning
///
/// This is just an alias for `ActorAndObject<JoinType>` because there's no fields inherent to Join that
/// aren't already present on an ActorAndObject.
pub type Join = ActorAndObject<JoinType>;

/// Indicates that the actor has left the object.
///
/// The target and origin typically have no meaning.
///
/// This is just an alias for `ActorAndObject<LeaveType>` because there's no fields inherent to Leave that
/// aren't already present on an ActorAndObject.
pub type Leave = ActorAndObject<LeaveType>;

/// Indicates that the actor likes, recommends or endorses the object.
///
/// The target and origin typically have no defined meaning.
///
/// This is just an alias for `ActorAndObject<LikeType>` because there's no fields inherent to Like that
/// aren't already present on an ActorAndObject.
pub type Like = ActorAndObject<LikeType>;

/// Indicates that the actor has listened to the object.
///
/// This is just an alias for `ActorAndObject<ListenType>` because there's no fields inherent to Listen
/// that aren't already present on an ActorAndObject.
pub type Listen = ActorAndObject<ListenType>;

/// Indicates that the actor has read the object.
///
/// This is just an alias for `ActorAndObject<ReadType>` because there's no fields inherent to Read that
/// aren't already present on an ActorAndObject.
pub type Read = ActorAndObject<ReadType>;

/// Indicates that the actor is rejecting the object.
///
/// The target and origin typically have no defined meaning.
///
/// This is just an alias for `ActorAndObject<RejectType>` because there's no fields inherent to Reject
/// that aren't already present on an ActorAndObject.
pub type Reject = ActorAndObject<RejectType>;

/// A specialization of Accept indicating that the acceptance is tentative.
///
/// This is just an alias for `ActorAndObject<TentativeAcceptType>` because there's no fields inherent to
/// TentativeAccept that aren't already present on an ActorAndObject.
pub type TentativeAccept = ActorAndObject<TentativeAcceptType>;

/// A specialization of Reject in which the rejection is considered tentative.
///
/// This is just an alias for `ActorAndObject<TentativeRejectType>` because there's no fields inherent to
/// TentativeReject that aren't already present on an ActorAndObject.
pub type TentativeReject = ActorAndObject<TentativeRejectType>;

/// Indicates that the actor is undoing the object.
///
/// In most cases, the object will be an Activity describing some previously performed action (for
/// instance, a person may have previously "liked" an article but, for whatever reason, might
/// choose to undo that like at some later point in time).
///
/// The target and origin typically have no defined meaning.
///
/// This is just an alias for `ActorAndObject<UndoType>` because there's no fields inherent to
/// Undo that aren't already present on an ActorAndObject.
pub type Undo = ActorAndObject<UndoType>;

/// Indicates that the actor has updated the object.
///
/// Note, however, that this vocabulary does not define a mechanism for describing the actual set
/// of modifications made to object.
///
/// The target and origin typically have no defined meaning.
///
/// This is just an alias for `ActorAndObject<UpdateType>` because there's no fields inherent to
/// Update that aren't already present on an ActorAndObject.
pub type Update = ActorAndObject<UpdateType>;

/// Indicates that the actor has viewed the object.
///
/// This is just an alias for `ActorAndObject<ViewType>` because there's no fields inherent to
/// View that aren't already present on an ActorAndObject.
pub type View = ActorAndObject<ViewType>;

/// Indicates that the actor is calling the target's attention the object.
///
/// The origin typically has no defined meaning.
///
/// This is just an alias for `ActorAndObjectOptTarget<AnnounceType>` because there's no fields inherent to
/// Announce that aren't already present on an OptTarget.
pub type Announce = ActorAndObjectOptTarget<AnnounceType>;

/// Indicates that the actor is offering the object.
///
/// If specified, the target indicates the entity to which the object is being offered.
///
/// This is just an alias for `ActorAndObjectOptTarget<OfferType>` because there's no fields inherent to
/// Offer that aren't already present on an OptTarget.
pub type Offer = ActorAndObjectOptTarget<OfferType>;

/// Indicates that the actor has moved object from origin to target.
///
/// If the origin or target are not specified, either can be determined by context.
///
/// This is just an alias for `ActorAndObject<MoveType>` because there's no fields inherent to
/// Move that aren't already present on an OptOrigin.
pub type Move = ActorAndObjectOptOrigin<MoveType>;

/// Indicates that the actor is removing the object.
///
/// If specified, the origin indicates the context from which the object is being removed.
///
/// This is just an alias for `ActorAndObject<RemoveType>` because there's no fields inherent to
/// Remove that aren't already present on an OptOrigin.
pub type Remove = ActorAndObjectOptOrigin<RemoveType>;

/// An IntransitiveActivity that indicates that the actor has arrived at the location.
///
/// The origin can be used to identify the context from which the actor originated. The target
/// typically has no defined meaning.
pub type Arrive = ActorAndOrigin<ArriveType>;

/// A specialization of Offer in which the actor is extending an invitation for the object to the
/// target.
pub type Invite = Target<ActorAndObject<InviteType>>;

/// Indicates that the actor has deleted the object.
///
/// If specified, the origin indicates the context from which the object was deleted.
pub type Delete = ActorAndObjectOptOrigin<DeleteType>;

/// Indicates that the actor is traveling to target from origin.
///
/// Travel is an IntransitiveObject whose actor specifies the direct object. If the target or
/// origin are not specified, either can be determined by context.
pub type Travel = OptOrigin<OptTarget<ActivityActor<Activity<TravelType>>>>;

/// Represents a question being asked.
///
/// Question objects are an extension of IntransitiveActivity. That is, the Question object is an
/// Activity, but the direct object is the question itself and therefore it would not contain an
/// object property.
///
/// Either of the anyOf and oneOf properties MAY be used to express possible answers, but a
/// Question object MUST NOT have both properties.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    /// Identifies an exclusive option for a Question.
    ///
    /// Use of one_of implies that the Question can have only a single answer. To indicate that a
    /// Question can have multiple answers, use any_of.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    one_of: Option<OneOrMany<AnyBase>>,

    /// Identifies an inclusive option for a Question.
    ///
    /// Use of any_of implies that the Question can have multiple answers. To indicate that a
    /// Question can have only one answer, use one_of.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    any_of: Option<OneOrMany<AnyBase>>,

    /// Indicates that a question has been closed, and answers are no longer accepted.
    ///
    /// - Range: Object | Link | xsd:datetime | xsd:boolean
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    closed: Option<Either<OneOrMany<AnyBase>, Either<XsdDateTime, XsdBoolean>>>,

    /// base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Activity<QuestionType>,
}

/// Activity with actor and object properties
pub type ActorAndObject<Kind> = ActivityActor<ActivityObject<Activity<Kind>>>;

/// Activity with actor and origin properties
pub type ActorAndOrigin<Kind> = Origin<ActivityActor<Activity<Kind>>>;

/// Activity with actor and object properties, and optional origin and target properties
pub type ActorAndObjectOptOriginAndTarget<Kind> = OptOrigin<OptTarget<ActorAndObject<Kind>>>;

/// Activity with actor and object properties, and and optional origin property
pub type ActorAndObjectOptOrigin<Kind> = OptOrigin<ActorAndObject<Kind>>;

/// Activity with actor and object properties, and and optional target property
pub type ActorAndObjectOptTarget<Kind> = OptTarget<ActorAndObject<Kind>>;

/// Activity objects are specializations of the base Object type that provide information about
/// actions that have either already occurred, are in the process of occurring, or may occur in the
/// future.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity<Kind> {
    /// Describes the result of the activity.
    ///
    /// For instance, if a particular action results in the creation of a new resource, the result
    /// property can be used to describe that new resource.
    ///
    /// - Range: Object | Link
    /// - Funcitonal: false
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<OneOrMany<AnyBase>>,

    /// Identifies one or more objects used (or to be used) in the completion of an Activity.
    ///
    /// - Range: Object | Link
    /// - Funcitonal: false
    #[serde(skip_serializing_if = "Option::is_none")]
    instrument: Option<OneOrMany<AnyBase>>,

    /// base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Object<Kind>,
}

/// Activity with an actor property
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityActor<Inner> {
    actor: OneOrMany<AnyBase>,

    /// base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Inner,
}

/// Activity with an object property
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityObject<Inner> {
    object: OneOrMany<AnyBase>,

    /// base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Inner,
}

/// Activity with an origin property
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Origin<Inner> {
    /// Describes an indirect object of the activity from which the activity is directed.
    ///
    /// The precise meaning of the origin is the object of the English preposition "from". For
    /// instance, in the activity "John moved an item to List B from List A", the origin of the
    /// activity is "List A".
    ///
    /// - Range: Object | Link
    /// - Functional: false
    origin: OneOrMany<AnyBase>,

    /// base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Inner,
}

/// Activity with an optional origin property
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptOrigin<Inner> {
    /// Describes an indirect object of the activity from which the activity is directed.
    ///
    /// The precise meaning of the origin is the object of the English preposition "from". For
    /// instance, in the activity "John moved an item to List B from List A", the origin of the
    /// activity is "List A".
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<OneOrMany<AnyBase>>,

    /// base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Inner,
}

/// Activity with a target property
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Target<Inner> {
    /// Describes the indirect object, or target, of the activity.
    ///
    /// The precise meaning of the target is largely dependent on the type of action being
    /// described but will often be the object of the English preposition "to". For instance, in
    /// the activity "John added a movie to his wishlist", the target of the activity is John's
    /// wishlist. An activity can have more than one target
    ///
    /// - Range: Object | Link
    /// - Functional: false
    target: OneOrMany<AnyBase>,

    /// base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Inner,
}

/// Activity with an optional target property
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptTarget<Inner> {
    /// Describes the indirect object, or target, of the activity.
    ///
    /// The precise meaning of the target is largely dependent on the type of action being
    /// described but will often be the object of the English preposition "to". For instance, in
    /// the activity "John added a movie to his wishlist", the target of the activity is John's
    /// wishlist. An activity can have more than one target
    ///
    /// - Range: Object | Link
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<OneOrMany<AnyBase>>,

    /// base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Inner,
}

impl<Kind> Activity<Kind> {
    /// Create a new Activity
    ///
    /// ```rust
    /// use activitystreams::activity::Activity;
    ///
    /// let activity = Activity::<String>::new();
    /// ```
    pub fn new() -> Self
    where
        Kind: Default,
    {
        Activity {
            result: None,
            instrument: None,
            inner: Object::new(),
        }
    }

    /// Create a new activity with `None` for it's `kind` property
    ///
    /// This means that no `type` field will be present in serialized JSON
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::activity::Activity;
    ///
    /// let activity = Activity::<()>::new_none_type();
    ///
    /// let s = serde_json::to_string(&activity)?;
    ///
    /// assert_eq!(s, "{}");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_none_type() -> Self {
        Activity {
            result: None,
            instrument: None,
            inner: Object::new_none_type(),
        }
    }

    fn extending(mut inner: Object<Kind>) -> Result<Self, serde_json::Error> {
        let result = inner.remove("result")?;
        let instrument = inner.remove("instrument")?;

        Ok(Activity {
            result,
            instrument,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<Kind>, serde_json::Error> {
        let Activity {
            result,
            instrument,
            mut inner,
        } = self;

        inner
            .insert("result", result)?
            .insert("instrument", instrument)?;

        Ok(inner)
    }
}

impl<Inner> ActivityActor<Inner> {
    fn extending(mut inner: Inner) -> Result<Self, serde_json::Error>
    where
        Inner: UnparsedMut,
    {
        let actor = inner.remove("actor")?;

        Ok(ActivityActor { actor, inner })
    }

    fn retracting(self) -> Result<Inner, serde_json::Error>
    where
        Inner: UnparsedMut,
    {
        let ActivityActor { actor, mut inner } = self;

        inner.insert("actor", actor)?;

        Ok(inner)
    }
}

impl<Inner> ActivityObject<Inner> {
    fn extending(mut inner: Inner) -> Result<Self, serde_json::Error>
    where
        Inner: UnparsedMut,
    {
        let object = inner.remove("object")?;

        Ok(ActivityObject { object, inner })
    }

    fn retracting(self) -> Result<Inner, serde_json::Error>
    where
        Inner: UnparsedMut,
    {
        let ActivityObject { object, mut inner } = self;

        inner.insert("object", object)?;

        Ok(inner)
    }
}

impl Arrive {
    pub fn new<T, U>(actor: T, origin: U) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        U: Into<OneOrMany<AnyBase>>,
    {
        Origin {
            origin: origin.into(),
            inner: ActivityActor {
                actor: actor.into(),
                inner: Activity::new(),
            },
        }
    }

    /// Deconstruct the Arrive into its parts
    ///
    /// ```rust
    /// use activitystreams::activity::Arrive;
    ///
    /// let activity = Arrive::new(vec![], vec![]);
    ///
    /// let (actor, origin, activity) = activity.into_parts();
    /// ```
    pub fn into_parts(self) -> (OneOrMany<AnyBase>, OneOrMany<AnyBase>, Activity<ArriveType>) {
        (self.inner.actor, self.origin, self.inner.inner)
    }
}

impl Invite {
    pub fn new<T, U, V>(actor: T, object: U, target: V) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        U: Into<OneOrMany<AnyBase>>,
        V: Into<OneOrMany<AnyBase>>,
    {
        Target {
            target: target.into(),
            inner: ActivityActor {
                actor: actor.into(),
                inner: ActivityObject {
                    object: object.into(),
                    inner: Activity::new(),
                },
            },
        }
    }

    /// Deconstruct the Invite into its parts
    ///
    /// ```rust
    /// use activitystreams::activity::Invite;
    ///
    /// let activity = Invite::new(vec![], vec![], vec![]);
    ///
    /// let (actor, object, target, activity) = activity.into_parts();
    /// ```
    pub fn into_parts(
        self,
    ) -> (
        OneOrMany<AnyBase>,
        OneOrMany<AnyBase>,
        OneOrMany<AnyBase>,
        Activity<InviteType>,
    ) {
        (
            self.inner.actor,
            self.inner.inner.object,
            self.target,
            self.inner.inner.inner,
        )
    }
}

impl Delete {
    /// Create a new Delete Activity
    ///
    /// ```rust
    /// use activitystreams::activity::Delete;
    ///
    /// let activity = Delete::new(vec![], vec![]);
    /// ```
    pub fn new<T, U>(actor: T, object: U) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        U: Into<OneOrMany<AnyBase>>,
    {
        OptOrigin {
            origin: None,
            inner: ActivityActor {
                actor: actor.into(),
                inner: ActivityObject {
                    object: object.into(),
                    inner: Activity::new(),
                },
            },
        }
    }

    /// Deconstruct the Delete into its parts
    ///
    /// ```rust
    /// use activitystreams::activity::Delete;
    ///
    /// let activity = Delete::new(vec![], vec![]);
    ///
    /// let (actor, object, origin, activity) = activity.into_parts();
    /// ```
    pub fn into_parts(
        self,
    ) -> (
        OneOrMany<AnyBase>,
        OneOrMany<AnyBase>,
        Option<OneOrMany<AnyBase>>,
        Activity<DeleteType>,
    ) {
        (
            self.inner.actor,
            self.inner.inner.object,
            self.origin,
            self.inner.inner.inner,
        )
    }
}

impl Travel {
    /// Create a new Travel Activity
    ///
    /// ```rust
    /// use activitystreams::activity::Travel;
    ///
    /// let activity = Travel::new(vec![]);
    /// ```
    pub fn new<T>(actor: T) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
    {
        OptOrigin {
            origin: None,
            inner: OptTarget {
                target: None,
                inner: ActivityActor {
                    actor: actor.into(),
                    inner: Activity::new(),
                },
            },
        }
    }

    #[allow(clippy::type_complexity)]
    /// Deconstruct the Travel into its parts
    ///
    /// ```rust
    /// use activitystreams::activity::Travel;
    ///
    /// let activity = Travel::new(vec![]);
    ///
    /// let (actor, origin, target, activity) = activity.into_parts();
    /// ```
    pub fn into_parts(
        self,
    ) -> (
        OneOrMany<AnyBase>,
        Option<OneOrMany<AnyBase>>,
        Option<OneOrMany<AnyBase>>,
        Activity<TravelType>,
    ) {
        (
            self.inner.inner.actor,
            self.origin,
            self.inner.target,
            self.inner.inner.inner,
        )
    }
}

impl<Kind> ActorAndObject<Kind> {
    /// Create a new ActorAndObject Activity
    ///
    /// ```rust
    /// use activitystreams::activity::ActorAndObject;
    ///
    /// let activity = ActorAndObject::<String>::new(vec![], vec![]);
    /// ```
    pub fn new<T, U>(actor: T, object: U) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        U: Into<OneOrMany<AnyBase>>,
        Kind: Default,
    {
        ActivityActor {
            actor: actor.into(),
            inner: ActivityObject {
                object: object.into(),
                inner: Activity::new(),
            },
        }
    }

    /// Create a new ActorAndObject with `None` for it's `kind` property
    ///
    /// This means that no `type` field will be present in serialized JSON
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::activity::ActorAndObject;
    ///
    /// let activity = ActorAndObject::<()>::new_none_type(vec![], vec![]);
    ///
    /// let s = serde_json::to_string(&activity)?;
    ///
    /// assert_eq!(s, r#"{"actor":[],"object":[]}"#);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_none_type<T, U>(actor: T, object: U) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        U: Into<OneOrMany<AnyBase>>,
    {
        ActivityActor {
            actor: actor.into(),
            inner: ActivityObject {
                object: object.into(),
                inner: Activity::new_none_type(),
            },
        }
    }

    /// Deconstruct the ActorAndObject into its parts
    ///
    /// ```rust
    /// use activitystreams::activity::ActorAndObject;
    ///
    /// let activity = ActorAndObject::<String>::new(vec![], vec![]);
    ///
    /// let (actor, object, activity) = activity.into_parts();
    /// ```
    pub fn into_parts(self) -> (OneOrMany<AnyBase>, OneOrMany<AnyBase>, Activity<Kind>) {
        (self.actor, self.inner.object, self.inner.inner)
    }
}

impl<Kind> ActorAndObjectOptTarget<Kind> {
    /// Create a new ActorAndObjectOptTarget Activity
    ///
    /// ```rust
    /// use activitystreams::activity::ActorAndObjectOptTarget;
    ///
    /// let activity = ActorAndObjectOptTarget::<String>::new(
    ///     vec![],
    ///     vec![]
    /// );
    /// ```
    pub fn new<T, U>(actor: T, object: U) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        U: Into<OneOrMany<AnyBase>>,
        Kind: Default,
    {
        OptTarget {
            target: None,
            inner: ActivityActor {
                actor: actor.into(),
                inner: ActivityObject {
                    object: object.into(),
                    inner: Activity::new(),
                },
            },
        }
    }

    /// Create a new ActorAndObject with `None` for it's `kind` property
    ///
    /// This means that no `type` field will be present in serialized JSON
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::activity::ActorAndObjectOptTarget;
    ///
    /// let activity = ActorAndObjectOptTarget::<()>::new_none_type(vec![], vec![]);
    ///
    /// let s = serde_json::to_string(&activity)?;
    ///
    /// assert_eq!(s, r#"{"actor":[],"object":[]}"#);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_none_type<T, U>(actor: T, object: U) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        U: Into<OneOrMany<AnyBase>>,
    {
        OptTarget {
            target: None,
            inner: ActivityActor {
                actor: actor.into(),
                inner: ActivityObject {
                    object: object.into(),
                    inner: Activity::new_none_type(),
                },
            },
        }
    }

    #[allow(clippy::type_complexity)]
    /// Deconstruct the ActorAndObjectOptTarget into its parts
    ///
    /// ```rust
    /// use activitystreams::activity::ActorAndObjectOptTarget;
    ///
    /// let activity = ActorAndObjectOptTarget::<String>::new(vec![], vec![]);
    ///
    /// let (actor, object, target, activity) = activity.into_parts();
    /// ```
    pub fn into_parts(
        self,
    ) -> (
        OneOrMany<AnyBase>,
        OneOrMany<AnyBase>,
        Option<OneOrMany<AnyBase>>,
        Activity<Kind>,
    ) {
        (
            self.inner.actor,
            self.inner.inner.object,
            self.target,
            self.inner.inner.inner,
        )
    }
}

impl<Kind> ActorAndObjectOptOriginAndTarget<Kind> {
    /// Create a new ActorAndObject Activity
    ///
    /// ```rust
    /// use activitystreams::activity::ActorAndObjectOptOriginAndTarget;
    ///
    /// let activity = ActorAndObjectOptOriginAndTarget::<String>::new(vec![], vec![]);
    /// ```
    pub fn new<T, U>(actor: T, object: U) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        U: Into<OneOrMany<AnyBase>>,
        Kind: Default,
    {
        OptOrigin {
            origin: None,
            inner: OptTarget {
                target: None,
                inner: ActivityActor {
                    actor: actor.into(),
                    inner: ActivityObject {
                        object: object.into(),
                        inner: Activity::new(),
                    },
                },
            },
        }
    }

    /// Create a new ActorAndObject with `None` for it's `kind` property
    ///
    /// This means that no `type` field will be present in serialized JSON
    ///
    /// ```rust
    /// # fn main() -> Result<(), anyhow::Error> {
    /// use activitystreams::activity::ActorAndObjectOptOriginAndTarget;
    ///
    /// let activity = ActorAndObjectOptOriginAndTarget::<()>::new_none_type(vec![], vec![]);
    ///
    /// let s = serde_json::to_string(&activity)?;
    ///
    /// assert_eq!(s, r#"{"actor":[],"object":[]}"#);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_none_type<T, U>(actor: T, object: U) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        U: Into<OneOrMany<AnyBase>>,
    {
        OptOrigin {
            origin: None,
            inner: OptTarget {
                target: None,
                inner: ActivityActor {
                    actor: actor.into(),
                    inner: ActivityObject {
                        object: object.into(),
                        inner: Activity::new_none_type(),
                    },
                },
            },
        }
    }

    #[allow(clippy::type_complexity)]
    /// Deconstruct the ActorAndObjectOptOriginAndTarget into its parts
    ///
    /// ```rust
    /// use activitystreams::activity::ActorAndObjectOptOriginAndTarget;
    ///
    /// let activity = ActorAndObjectOptOriginAndTarget::<String>::new(vec![], vec![]);
    ///
    /// let (actor, object, origin, target, activity) = activity.into_parts();
    /// ```
    pub fn into_parts(
        self,
    ) -> (
        OneOrMany<AnyBase>,
        OneOrMany<AnyBase>,
        Option<OneOrMany<AnyBase>>,
        Option<OneOrMany<AnyBase>>,
        Activity<Kind>,
    ) {
        (
            self.inner.inner.actor,
            self.inner.inner.inner.object,
            self.origin,
            self.inner.target,
            self.inner.inner.inner.inner,
        )
    }
}

impl<Inner> Origin<Inner> {
    fn extending(mut inner: Inner) -> Result<Self, serde_json::Error>
    where
        Inner: UnparsedMut,
    {
        let origin = inner.remove("origin")?;

        Ok(Origin { origin, inner })
    }

    fn retracting(self) -> Result<Inner, serde_json::Error>
    where
        Inner: UnparsedMut,
    {
        let Origin { origin, mut inner } = self;

        inner.insert("origin", origin)?;

        Ok(inner)
    }
}

impl<Inner> OptOrigin<Inner> {
    fn extending(mut inner: Inner) -> Result<Self, serde_json::Error>
    where
        Inner: UnparsedMut,
    {
        let origin = inner.remove("origin")?;

        Ok(OptOrigin { origin, inner })
    }

    fn retracting(self) -> Result<Inner, serde_json::Error>
    where
        Inner: UnparsedMut,
    {
        let OptOrigin { origin, mut inner } = self;

        inner.insert("origin", origin)?;

        Ok(inner)
    }
}

impl<Inner> Target<Inner> {
    fn extending(mut inner: Inner) -> Result<Self, serde_json::Error>
    where
        Inner: UnparsedMut,
    {
        let target = inner.remove("target")?;

        Ok(Target { target, inner })
    }

    fn retracting(self) -> Result<Inner, serde_json::Error>
    where
        Inner: UnparsedMut,
    {
        let Target { target, mut inner } = self;

        inner.insert("target", target)?;

        Ok(inner)
    }
}

impl<Inner> OptTarget<Inner> {
    fn extending(mut inner: Inner) -> Result<Self, serde_json::Error>
    where
        Inner: UnparsedMut,
    {
        let target = inner.remove("target")?;

        Ok(OptTarget { target, inner })
    }

    fn retracting(self) -> Result<Inner, serde_json::Error>
    where
        Inner: UnparsedMut,
    {
        let OptTarget { target, mut inner } = self;

        inner.insert("target", target)?;

        Ok(inner)
    }
}

impl Question {
    /// Create a new Question Activity
    ///
    /// ```rust
    /// use activitystreams::activity::Question;
    ///
    /// let activity = Question::new();
    /// ```
    pub fn new() -> Self {
        Question {
            one_of: None,
            any_of: None,
            closed: None,
            inner: Activity::new(),
        }
    }

    /// Deconstruct the Question into its parts
    ///
    /// ```rust
    /// use activitystreams::activity::Question;
    ///
    /// let activity = Question::new();
    ///
    /// let (one_of, any_of, activity) = activity.into_parts();
    /// ```
    pub fn into_parts(
        self,
    ) -> (
        Option<OneOrMany<AnyBase>>,
        Option<OneOrMany<AnyBase>>,
        Activity<QuestionType>,
    ) {
        (self.one_of, self.any_of, self.inner)
    }

    fn extending(object: Object<QuestionType>) -> Result<Self, serde_json::Error> {
        let mut inner = Activity::extending(object)?;

        let one_of = inner.remove("oneOf")?;
        let any_of = inner.remove("anyOf")?;
        let closed = inner.remove("closed")?;

        Ok(Question {
            one_of,
            any_of,
            closed,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<QuestionType>, serde_json::Error> {
        let Question {
            one_of,
            any_of,
            closed,
            mut inner,
        } = self;

        inner
            .insert("oneOf", one_of)?
            .insert("anyOf", any_of)?
            .insert("closed", closed)?;

        inner.retracting()
    }
}

impl<Kind> markers::Base for Activity<Kind> {}
impl<Kind> markers::Object for Activity<Kind> {}
impl<Kind> markers::Activity for Activity<Kind> {}

impl<Inner> markers::Base for ActivityActor<Inner> where Inner: markers::Base {}
impl<Inner> markers::Object for ActivityActor<Inner> where Inner: markers::Object {}
impl<Inner> markers::Activity for ActivityActor<Inner> where Inner: markers::Activity {}
impl<Inner> markers::IntransitiveActivity for ActivityActor<Inner> where
    Inner: markers::IntransitiveActivity
{
}

impl<Inner> markers::Base for ActivityObject<Inner> where Inner: markers::Base {}
impl<Inner> markers::Object for ActivityObject<Inner> where Inner: markers::Object {}
impl<Inner> markers::Activity for ActivityObject<Inner> where Inner: markers::Activity {}
impl<Inner> markers::IntransitiveActivity for ActivityObject<Inner> where
    Inner: markers::IntransitiveActivity
{
}

impl<Inner> markers::Base for Target<Inner> where Inner: markers::Base {}
impl<Inner> markers::Object for Target<Inner> where Inner: markers::Object {}
impl<Inner> markers::Activity for Target<Inner> where Inner: markers::Activity {}
impl<Inner> markers::IntransitiveActivity for Target<Inner> where
    Inner: markers::IntransitiveActivity
{
}

impl<Inner> markers::Base for OptTarget<Inner> where Inner: markers::Base {}
impl<Inner> markers::Object for OptTarget<Inner> where Inner: markers::Object {}
impl<Inner> markers::Activity for OptTarget<Inner> where Inner: markers::Activity {}
impl<Inner> markers::IntransitiveActivity for OptTarget<Inner> where
    Inner: markers::IntransitiveActivity
{
}

impl<Inner> markers::Base for Origin<Inner> where Inner: markers::Base {}
impl<Inner> markers::Object for Origin<Inner> where Inner: markers::Object {}
impl<Inner> markers::Activity for Origin<Inner> where Inner: markers::Activity {}
impl<Inner> markers::IntransitiveActivity for Origin<Inner> where
    Inner: markers::IntransitiveActivity
{
}

impl<Inner> markers::Base for OptOrigin<Inner> where Inner: markers::Base {}
impl<Inner> markers::Object for OptOrigin<Inner> where Inner: markers::Object {}
impl<Inner> markers::Activity for OptOrigin<Inner> where Inner: markers::Activity {}
impl<Inner> markers::IntransitiveActivity for OptOrigin<Inner> where
    Inner: markers::IntransitiveActivity
{
}

impl markers::Base for Question {}
impl markers::Object for Question {}
impl markers::Activity for Question {}

impl markers::IntransitiveActivity for Arrive {}
impl markers::IntransitiveActivity for Travel {}
impl markers::IntransitiveActivity for Question {}

impl<Inner> markers::Activity for ApObject<Inner> where Inner: markers::Activity {}
impl<Inner> markers::IntransitiveActivity for ApObject<Inner> where
    Inner: markers::IntransitiveActivity
{
}

impl<Kind> Extends for Activity<Kind> {
    type Kind = Kind;

    type Error = serde_json::Error;

    fn extends(base: Base<Self::Kind>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Self::Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Kind> TryFrom<Object<Kind>> for Activity<Kind> {
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl<Kind> TryFrom<Activity<Kind>> for Object<Kind> {
    type Error = serde_json::Error;

    fn try_from(activity: Activity<Kind>) -> Result<Self, Self::Error> {
        activity.retracting()
    }
}

impl<Inner> Extends for ActivityActor<Inner>
where
    Inner: Extends<Error = serde_json::Error> + UnparsedMut,
{
    type Kind = Inner::Kind;

    type Error = serde_json::Error;

    fn extends(base: Base<Self::Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Self::Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Inner, Kind> TryFrom<Object<Kind>> for ActivityActor<Inner>
where
    Inner: TryFrom<Object<Kind>, Error = serde_json::Error> + UnparsedMut,
{
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::try_from(object)?;
        Self::extending(inner)
    }
}

impl<Inner, Kind> TryFrom<ActivityActor<Inner>> for Object<Kind>
where
    Object<Kind>: TryFrom<Inner, Error = serde_json::Error>,
    Inner: UnparsedMut,
{
    type Error = serde_json::Error;

    fn try_from(activity: ActivityActor<Inner>) -> Result<Self, Self::Error> {
        let inner = activity.retracting()?;
        TryFrom::try_from(inner)
    }
}

impl<Inner> Extends for ActivityObject<Inner>
where
    Inner: Extends<Error = serde_json::Error> + UnparsedMut,
{
    type Kind = Inner::Kind;

    type Error = serde_json::Error;

    fn extends(base: Base<Self::Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Self::Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Inner, Kind> TryFrom<Object<Kind>> for ActivityObject<Inner>
where
    Inner: TryFrom<Object<Kind>, Error = serde_json::Error> + UnparsedMut,
{
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::try_from(object)?;
        Self::extending(inner)
    }
}

impl<Inner, Kind> TryFrom<ActivityObject<Inner>> for Object<Kind>
where
    Object<Kind>: TryFrom<Inner, Error = serde_json::Error>,
    Inner: UnparsedMut,
{
    type Error = serde_json::Error;

    fn try_from(activity: ActivityObject<Inner>) -> Result<Self, Self::Error> {
        let inner = activity.retracting()?;
        TryFrom::try_from(inner)
    }
}

impl<Inner> Extends for Origin<Inner>
where
    Inner: Extends<Error = serde_json::Error> + UnparsedMut,
{
    type Kind = Inner::Kind;

    type Error = serde_json::Error;

    fn extends(base: Base<Self::Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Self::Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Inner, Kind> TryFrom<Object<Kind>> for Origin<Inner>
where
    Inner: TryFrom<Object<Kind>, Error = serde_json::Error> + UnparsedMut,
{
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::try_from(object)?;
        Self::extending(inner)
    }
}

impl<Inner, Kind> TryFrom<Origin<Inner>> for Object<Kind>
where
    Object<Kind>: TryFrom<Inner, Error = serde_json::Error>,
    Inner: UnparsedMut,
{
    type Error = serde_json::Error;

    fn try_from(activity: Origin<Inner>) -> Result<Self, Self::Error> {
        let inner = activity.retracting()?;
        TryFrom::try_from(inner)
    }
}

impl<Inner> Extends for OptOrigin<Inner>
where
    Inner: Extends<Error = serde_json::Error> + UnparsedMut,
{
    type Kind = Inner::Kind;

    type Error = serde_json::Error;

    fn extends(base: Base<Self::Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Self::Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Inner, Kind> TryFrom<Object<Kind>> for OptOrigin<Inner>
where
    Inner: TryFrom<Object<Kind>, Error = serde_json::Error> + UnparsedMut,
{
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::try_from(object)?;
        Self::extending(inner)
    }
}

impl<Inner, Kind> TryFrom<OptOrigin<Inner>> for Object<Kind>
where
    Object<Kind>: TryFrom<Inner, Error = serde_json::Error>,
    Inner: UnparsedMut,
{
    type Error = serde_json::Error;

    fn try_from(activity: OptOrigin<Inner>) -> Result<Self, Self::Error> {
        let inner = activity.retracting()?;
        TryFrom::try_from(inner)
    }
}

impl<Inner> Extends for Target<Inner>
where
    Inner: Extends<Error = serde_json::Error> + UnparsedMut,
{
    type Kind = Inner::Kind;

    type Error = serde_json::Error;

    fn extends(base: Base<Self::Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Self::Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Inner, Kind> TryFrom<Object<Kind>> for Target<Inner>
where
    Inner: TryFrom<Object<Kind>, Error = serde_json::Error> + UnparsedMut,
{
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::try_from(object)?;
        Self::extending(inner)
    }
}

impl<Inner, Kind> TryFrom<Target<Inner>> for Object<Kind>
where
    Object<Kind>: TryFrom<Inner, Error = serde_json::Error>,
    Inner: UnparsedMut,
{
    type Error = serde_json::Error;

    fn try_from(activity: Target<Inner>) -> Result<Self, Self::Error> {
        let inner = activity.retracting()?;
        TryFrom::try_from(inner)
    }
}

impl<Inner> Extends for OptTarget<Inner>
where
    Inner: Extends<Error = serde_json::Error> + UnparsedMut,
{
    type Kind = Inner::Kind;

    type Error = serde_json::Error;

    fn extends(base: Base<Self::Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Self::Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Inner, Kind> TryFrom<Object<Kind>> for OptTarget<Inner>
where
    Inner: TryFrom<Object<Kind>, Error = serde_json::Error> + UnparsedMut,
{
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        let inner = Inner::try_from(object)?;
        Self::extending(inner)
    }
}

impl<Inner, Kind> TryFrom<OptTarget<Inner>> for Object<Kind>
where
    Object<Kind>: TryFrom<Inner, Error = serde_json::Error>,
    Inner: UnparsedMut,
{
    type Error = serde_json::Error;

    fn try_from(activity: OptTarget<Inner>) -> Result<Self, Self::Error> {
        let inner = activity.retracting()?;
        TryFrom::try_from(inner)
    }
}

impl Extends for Question {
    type Kind = QuestionType;

    type Error = serde_json::Error;

    fn extends(base: Base<QuestionType>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<QuestionType>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl TryFrom<Object<QuestionType>> for Question {
    type Error = serde_json::Error;

    fn try_from(object: Object<QuestionType>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl TryFrom<Question> for Object<QuestionType> {
    type Error = serde_json::Error;

    fn try_from(question: Question) -> Result<Self, Self::Error> {
        question.retracting()
    }
}

impl<Kind> UnparsedMut for Activity<Kind> {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Inner> UnparsedMut for ActivityActor<Inner>
where
    Inner: UnparsedMut,
{
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Inner> UnparsedMut for ActivityObject<Inner>
where
    Inner: UnparsedMut,
{
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Inner> UnparsedMut for Origin<Inner>
where
    Inner: UnparsedMut,
{
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Inner> UnparsedMut for OptOrigin<Inner>
where
    Inner: UnparsedMut,
{
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Inner> UnparsedMut for Target<Inner>
where
    Inner: UnparsedMut,
{
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Inner> UnparsedMut for OptTarget<Inner>
where
    Inner: UnparsedMut,
{
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl UnparsedMut for Question {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Kind> AsBase for Activity<Kind> {
    type Kind = Kind;

    fn base_ref(&self) -> &Base<Self::Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Self::Kind> {
        self.inner.base_mut()
    }
}

impl<Kind> AsObject for Activity<Kind> {
    type Kind = Kind;

    fn object_ref(&self) -> &Object<Self::Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Self::Kind> {
        self.inner.object_mut()
    }
}

impl<Kind> AsActivity for Activity<Kind> {
    type Kind = Kind;

    fn activity_ref(&self) -> &Activity<Self::Kind> {
        self
    }

    fn activity_mut(&mut self) -> &mut Activity<Self::Kind> {
        self
    }
}

impl<Inner> AsBase for ActivityActor<Inner>
where
    Inner: AsBase,
{
    type Kind = Inner::Kind;

    fn base_ref(&self) -> &Base<Self::Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Self::Kind> {
        self.inner.base_mut()
    }
}

impl<Inner> AsObject for ActivityActor<Inner>
where
    Inner: AsObject,
{
    type Kind = Inner::Kind;

    fn object_ref(&self) -> &Object<Self::Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Self::Kind> {
        self.inner.object_mut()
    }
}

impl<Inner> AsActivity for ActivityActor<Inner>
where
    Inner: AsActivity,
{
    type Kind = Inner::Kind;

    fn activity_ref(&self) -> &Activity<Self::Kind> {
        self.inner.activity_ref()
    }

    fn activity_mut(&mut self) -> &mut Activity<Self::Kind> {
        self.inner.activity_mut()
    }
}

impl<Inner> AsActivityActor for ActivityActor<Inner>
where
    Inner: markers::Activity,
{
    type Inner = Inner;

    fn activity_actor_ref(&self) -> &ActivityActor<Self::Inner> {
        self
    }

    fn activity_actor_mut(&mut self) -> &mut ActivityActor<Self::Inner> {
        self
    }
}

impl<Inner> AsActivityObject for ActivityActor<Inner>
where
    Inner: AsActivityObject,
{
    type Inner = Inner::Inner;

    fn activity_object_ref(&self) -> &ActivityObject<Self::Inner> {
        self.inner.activity_object_ref()
    }

    fn activity_object_mut(&mut self) -> &mut ActivityObject<Self::Inner> {
        self.inner.activity_object_mut()
    }
}

impl<Inner> AsTarget for ActivityActor<Inner>
where
    Inner: AsTarget,
{
    type Inner = Inner::Inner;

    fn target_ref(&self) -> &Target<Self::Inner> {
        self.inner.target_ref()
    }

    fn target_mut(&mut self) -> &mut Target<Self::Inner> {
        self.inner.target_mut()
    }
}

impl<Inner> AsOptTarget for ActivityActor<Inner>
where
    Inner: AsOptTarget,
{
    type Inner = Inner::Inner;

    fn opt_target_ref(&self) -> &OptTarget<Self::Inner> {
        self.inner.opt_target_ref()
    }

    fn opt_target_mut(&mut self) -> &mut OptTarget<Self::Inner> {
        self.inner.opt_target_mut()
    }
}

impl<Inner> AsOrigin for ActivityActor<Inner>
where
    Inner: AsOrigin,
{
    type Inner = Inner::Inner;

    fn origin_ref(&self) -> &Origin<Self::Inner> {
        self.inner.origin_ref()
    }

    fn origin_mut(&mut self) -> &mut Origin<Self::Inner> {
        self.inner.origin_mut()
    }
}

impl<Inner> AsOptOrigin for ActivityActor<Inner>
where
    Inner: AsOptOrigin,
{
    type Inner = Inner::Inner;

    fn opt_origin_ref(&self) -> &OptOrigin<Self::Inner> {
        self.inner.opt_origin_ref()
    }

    fn opt_origin_mut(&mut self) -> &mut OptOrigin<Self::Inner> {
        self.inner.opt_origin_mut()
    }
}

impl<Inner> AsBase for ActivityObject<Inner>
where
    Inner: AsBase,
{
    type Kind = Inner::Kind;

    fn base_ref(&self) -> &Base<Self::Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Self::Kind> {
        self.inner.base_mut()
    }
}

impl<Inner> AsObject for ActivityObject<Inner>
where
    Inner: AsObject,
{
    type Kind = Inner::Kind;

    fn object_ref(&self) -> &Object<Self::Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Self::Kind> {
        self.inner.object_mut()
    }
}

impl<Inner> AsActivity for ActivityObject<Inner>
where
    Inner: AsActivity,
{
    type Kind = Inner::Kind;

    fn activity_ref(&self) -> &Activity<Self::Kind> {
        self.inner.activity_ref()
    }

    fn activity_mut(&mut self) -> &mut Activity<Self::Kind> {
        self.inner.activity_mut()
    }
}

impl<Inner> AsActivityObject for ActivityObject<Inner>
where
    Inner: markers::Activity,
{
    type Inner = Inner;

    fn activity_object_ref(&self) -> &ActivityObject<Self::Inner> {
        self
    }

    fn activity_object_mut(&mut self) -> &mut ActivityObject<Self::Inner> {
        self
    }
}

impl<Inner> AsActivityActor for ActivityObject<Inner>
where
    Inner: AsActivityActor,
{
    type Inner = Inner::Inner;

    fn activity_actor_ref(&self) -> &ActivityActor<Self::Inner> {
        self.inner.activity_actor_ref()
    }

    fn activity_actor_mut(&mut self) -> &mut ActivityActor<Self::Inner> {
        self.inner.activity_actor_mut()
    }
}

impl<Inner> AsTarget for ActivityObject<Inner>
where
    Inner: AsTarget,
{
    type Inner = Inner::Inner;

    fn target_ref(&self) -> &Target<Self::Inner> {
        self.inner.target_ref()
    }

    fn target_mut(&mut self) -> &mut Target<Self::Inner> {
        self.inner.target_mut()
    }
}

impl<Inner> AsOptTarget for ActivityObject<Inner>
where
    Inner: AsOptTarget,
{
    type Inner = Inner::Inner;

    fn opt_target_ref(&self) -> &OptTarget<Self::Inner> {
        self.inner.opt_target_ref()
    }

    fn opt_target_mut(&mut self) -> &mut OptTarget<Self::Inner> {
        self.inner.opt_target_mut()
    }
}

impl<Inner> AsOrigin for ActivityObject<Inner>
where
    Inner: AsOrigin,
{
    type Inner = Inner::Inner;

    fn origin_ref(&self) -> &Origin<Self::Inner> {
        self.inner.origin_ref()
    }

    fn origin_mut(&mut self) -> &mut Origin<Self::Inner> {
        self.inner.origin_mut()
    }
}

impl<Inner> AsOptOrigin for ActivityObject<Inner>
where
    Inner: AsOptOrigin,
{
    type Inner = Inner::Inner;

    fn opt_origin_ref(&self) -> &OptOrigin<Self::Inner> {
        self.inner.opt_origin_ref()
    }

    fn opt_origin_mut(&mut self) -> &mut OptOrigin<Self::Inner> {
        self.inner.opt_origin_mut()
    }
}

impl<Inner> AsBase for Target<Inner>
where
    Inner: AsBase,
{
    type Kind = Inner::Kind;

    fn base_ref(&self) -> &Base<Self::Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Self::Kind> {
        self.inner.base_mut()
    }
}

impl<Inner> AsObject for Target<Inner>
where
    Inner: AsObject,
{
    type Kind = Inner::Kind;

    fn object_ref(&self) -> &Object<Self::Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Self::Kind> {
        self.inner.object_mut()
    }
}

impl<Inner> AsActivity for Target<Inner>
where
    Inner: AsActivity,
{
    type Kind = Inner::Kind;

    fn activity_ref(&self) -> &Activity<Self::Kind> {
        self.inner.activity_ref()
    }

    fn activity_mut(&mut self) -> &mut Activity<Self::Kind> {
        self.inner.activity_mut()
    }
}

impl<Inner> AsActivityObject for Target<Inner>
where
    Inner: AsActivityObject,
{
    type Inner = Inner::Inner;

    fn activity_object_ref(&self) -> &ActivityObject<Self::Inner> {
        self.inner.activity_object_ref()
    }

    fn activity_object_mut(&mut self) -> &mut ActivityObject<Self::Inner> {
        self.inner.activity_object_mut()
    }
}

impl<Inner> AsActivityActor for Target<Inner>
where
    Inner: AsActivityActor,
{
    type Inner = Inner::Inner;

    fn activity_actor_ref(&self) -> &ActivityActor<Self::Inner> {
        self.inner.activity_actor_ref()
    }

    fn activity_actor_mut(&mut self) -> &mut ActivityActor<Self::Inner> {
        self.inner.activity_actor_mut()
    }
}

impl<Inner> AsTarget for Target<Inner>
where
    Inner: markers::Activity,
{
    type Inner = Inner;

    fn target_ref(&self) -> &Target<Self::Inner> {
        self
    }

    fn target_mut(&mut self) -> &mut Target<Self::Inner> {
        self
    }
}

impl<Inner> AsOrigin for Target<Inner>
where
    Inner: AsOrigin,
{
    type Inner = Inner::Inner;

    fn origin_ref(&self) -> &Origin<Self::Inner> {
        self.inner.origin_ref()
    }

    fn origin_mut(&mut self) -> &mut Origin<Self::Inner> {
        self.inner.origin_mut()
    }
}

impl<Inner> AsOptOrigin for Target<Inner>
where
    Inner: AsOptOrigin,
{
    type Inner = Inner::Inner;

    fn opt_origin_ref(&self) -> &OptOrigin<Self::Inner> {
        self.inner.opt_origin_ref()
    }

    fn opt_origin_mut(&mut self) -> &mut OptOrigin<Self::Inner> {
        self.inner.opt_origin_mut()
    }
}

impl<Inner> AsBase for OptTarget<Inner>
where
    Inner: AsBase,
{
    type Kind = Inner::Kind;

    fn base_ref(&self) -> &Base<Self::Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Self::Kind> {
        self.inner.base_mut()
    }
}

impl<Inner> AsObject for OptTarget<Inner>
where
    Inner: AsObject,
{
    type Kind = Inner::Kind;

    fn object_ref(&self) -> &Object<Self::Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Self::Kind> {
        self.inner.object_mut()
    }
}

impl<Inner> AsActivity for OptTarget<Inner>
where
    Inner: AsActivity,
{
    type Kind = Inner::Kind;

    fn activity_ref(&self) -> &Activity<Self::Kind> {
        self.inner.activity_ref()
    }

    fn activity_mut(&mut self) -> &mut Activity<Self::Kind> {
        self.inner.activity_mut()
    }
}

impl<Inner> AsActivityObject for OptTarget<Inner>
where
    Inner: AsActivityObject,
{
    type Inner = Inner::Inner;

    fn activity_object_ref(&self) -> &ActivityObject<Self::Inner> {
        self.inner.activity_object_ref()
    }

    fn activity_object_mut(&mut self) -> &mut ActivityObject<Self::Inner> {
        self.inner.activity_object_mut()
    }
}

impl<Inner> AsActivityActor for OptTarget<Inner>
where
    Inner: AsActivityActor,
{
    type Inner = Inner::Inner;

    fn activity_actor_ref(&self) -> &ActivityActor<Self::Inner> {
        self.inner.activity_actor_ref()
    }

    fn activity_actor_mut(&mut self) -> &mut ActivityActor<Self::Inner> {
        self.inner.activity_actor_mut()
    }
}

impl<Inner> AsOptTarget for OptTarget<Inner>
where
    Inner: markers::Activity,
{
    type Inner = Inner;

    fn opt_target_ref(&self) -> &OptTarget<Self::Inner> {
        self
    }

    fn opt_target_mut(&mut self) -> &mut OptTarget<Self::Inner> {
        self
    }
}

impl<Inner> AsOrigin for OptTarget<Inner>
where
    Inner: AsOrigin,
{
    type Inner = Inner::Inner;

    fn origin_ref(&self) -> &Origin<Self::Inner> {
        self.inner.origin_ref()
    }

    fn origin_mut(&mut self) -> &mut Origin<Self::Inner> {
        self.inner.origin_mut()
    }
}

impl<Inner> AsOptOrigin for OptTarget<Inner>
where
    Inner: AsOptOrigin,
{
    type Inner = Inner::Inner;

    fn opt_origin_ref(&self) -> &OptOrigin<Self::Inner> {
        self.inner.opt_origin_ref()
    }

    fn opt_origin_mut(&mut self) -> &mut OptOrigin<Self::Inner> {
        self.inner.opt_origin_mut()
    }
}

impl<Inner> AsActivity for Origin<Inner>
where
    Inner: AsActivity,
{
    type Kind = Inner::Kind;

    fn activity_ref(&self) -> &Activity<Self::Kind> {
        self.inner.activity_ref()
    }

    fn activity_mut(&mut self) -> &mut Activity<Self::Kind> {
        self.inner.activity_mut()
    }
}

impl<Inner> AsBase for Origin<Inner>
where
    Inner: AsBase,
{
    type Kind = Inner::Kind;

    fn base_ref(&self) -> &Base<Self::Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Self::Kind> {
        self.inner.base_mut()
    }
}

impl<Inner> AsObject for Origin<Inner>
where
    Inner: AsObject,
{
    type Kind = Inner::Kind;

    fn object_ref(&self) -> &Object<Self::Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Self::Kind> {
        self.inner.object_mut()
    }
}

impl<Inner> AsActivityObject for Origin<Inner>
where
    Inner: AsActivityObject,
{
    type Inner = Inner::Inner;

    fn activity_object_ref(&self) -> &ActivityObject<Self::Inner> {
        self.inner.activity_object_ref()
    }

    fn activity_object_mut(&mut self) -> &mut ActivityObject<Self::Inner> {
        self.inner.activity_object_mut()
    }
}

impl<Inner> AsActivityActor for Origin<Inner>
where
    Inner: AsActivityActor,
{
    type Inner = Inner::Inner;

    fn activity_actor_ref(&self) -> &ActivityActor<Self::Inner> {
        self.inner.activity_actor_ref()
    }

    fn activity_actor_mut(&mut self) -> &mut ActivityActor<Self::Inner> {
        self.inner.activity_actor_mut()
    }
}

impl<Inner> AsTarget for Origin<Inner>
where
    Inner: AsTarget,
{
    type Inner = Inner::Inner;

    fn target_ref(&self) -> &Target<Self::Inner> {
        self.inner.target_ref()
    }

    fn target_mut(&mut self) -> &mut Target<Self::Inner> {
        self.inner.target_mut()
    }
}

impl<Inner> AsOptTarget for Origin<Inner>
where
    Inner: AsOptTarget,
{
    type Inner = Inner::Inner;

    fn opt_target_ref(&self) -> &OptTarget<Self::Inner> {
        self.inner.opt_target_ref()
    }

    fn opt_target_mut(&mut self) -> &mut OptTarget<Self::Inner> {
        self.inner.opt_target_mut()
    }
}

impl<Inner> AsOrigin for Origin<Inner>
where
    Inner: markers::Activity,
{
    type Inner = Inner;

    fn origin_ref(&self) -> &Origin<Self::Inner> {
        self
    }

    fn origin_mut(&mut self) -> &mut Origin<Self::Inner> {
        self
    }
}

impl<Inner> AsBase for OptOrigin<Inner>
where
    Inner: AsBase,
{
    type Kind = Inner::Kind;

    fn base_ref(&self) -> &Base<Self::Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Self::Kind> {
        self.inner.base_mut()
    }
}

impl<Inner> AsObject for OptOrigin<Inner>
where
    Inner: AsObject,
{
    type Kind = Inner::Kind;

    fn object_ref(&self) -> &Object<Self::Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Self::Kind> {
        self.inner.object_mut()
    }
}

impl<Inner> AsActivity for OptOrigin<Inner>
where
    Inner: AsActivity,
{
    type Kind = Inner::Kind;

    fn activity_ref(&self) -> &Activity<Self::Kind> {
        self.inner.activity_ref()
    }

    fn activity_mut(&mut self) -> &mut Activity<Self::Kind> {
        self.inner.activity_mut()
    }
}

impl<Inner> AsActivityObject for OptOrigin<Inner>
where
    Inner: AsActivityObject,
{
    type Inner = Inner::Inner;

    fn activity_object_ref(&self) -> &ActivityObject<Self::Inner> {
        self.inner.activity_object_ref()
    }

    fn activity_object_mut(&mut self) -> &mut ActivityObject<Self::Inner> {
        self.inner.activity_object_mut()
    }
}

impl<Inner> AsActivityActor for OptOrigin<Inner>
where
    Inner: AsActivityActor,
{
    type Inner = Inner::Inner;

    fn activity_actor_ref(&self) -> &ActivityActor<Self::Inner> {
        self.inner.activity_actor_ref()
    }

    fn activity_actor_mut(&mut self) -> &mut ActivityActor<Self::Inner> {
        self.inner.activity_actor_mut()
    }
}

impl<Inner> AsTarget for OptOrigin<Inner>
where
    Inner: AsTarget,
{
    type Inner = Inner::Inner;

    fn target_ref(&self) -> &Target<Self::Inner> {
        self.inner.target_ref()
    }

    fn target_mut(&mut self) -> &mut Target<Self::Inner> {
        self.inner.target_mut()
    }
}

impl<Inner> AsOptTarget for OptOrigin<Inner>
where
    Inner: AsOptTarget,
{
    type Inner = Inner::Inner;

    fn opt_target_ref(&self) -> &OptTarget<Self::Inner> {
        self.inner.opt_target_ref()
    }

    fn opt_target_mut(&mut self) -> &mut OptTarget<Self::Inner> {
        self.inner.opt_target_mut()
    }
}

impl<Inner> AsOptOrigin for OptOrigin<Inner>
where
    Inner: markers::Activity,
{
    type Inner = Inner;

    fn opt_origin_ref(&self) -> &OptOrigin<Self::Inner> {
        self
    }

    fn opt_origin_mut(&mut self) -> &mut OptOrigin<Self::Inner> {
        self
    }
}

impl AsActivity for Question {
    type Kind = QuestionType;

    fn activity_ref(&self) -> &Activity<QuestionType> {
        &self.inner
    }

    fn activity_mut(&mut self) -> &mut Activity<QuestionType> {
        &mut self.inner
    }
}

impl AsQuestion for Question {
    fn question_ref(&self) -> &Question {
        self
    }

    fn question_mut(&mut self) -> &mut Question {
        self
    }
}

impl<Inner> AsActivity for ApObject<Inner>
where
    Inner: AsActivity,
{
    type Kind = Inner::Kind;

    fn activity_ref(&self) -> &Activity<Self::Kind> {
        self.inner().activity_ref()
    }

    fn activity_mut(&mut self) -> &mut Activity<Self::Kind> {
        self.inner_mut().activity_mut()
    }
}

impl<Inner> AsActivityActor for ApObject<Inner>
where
    Inner: AsActivityActor,
{
    type Inner = Inner::Inner;

    fn activity_actor_ref(&self) -> &ActivityActor<Self::Inner> {
        self.inner().activity_actor_ref()
    }

    fn activity_actor_mut(&mut self) -> &mut ActivityActor<Self::Inner> {
        self.inner_mut().activity_actor_mut()
    }
}

impl<Inner> AsActivityObject for ApObject<Inner>
where
    Inner: AsActivityObject,
{
    type Inner = Inner::Inner;

    fn activity_object_ref(&self) -> &ActivityObject<Self::Inner> {
        self.inner().activity_object_ref()
    }

    fn activity_object_mut(&mut self) -> &mut ActivityObject<Self::Inner> {
        self.inner_mut().activity_object_mut()
    }
}

impl<Inner> AsTarget for ApObject<Inner>
where
    Inner: AsTarget,
{
    type Inner = Inner::Inner;

    fn target_ref(&self) -> &Target<Self::Inner> {
        self.inner().target_ref()
    }

    fn target_mut(&mut self) -> &mut Target<Self::Inner> {
        self.inner_mut().target_mut()
    }
}

impl<Inner> AsOrigin for ApObject<Inner>
where
    Inner: AsOrigin,
{
    type Inner = Inner::Inner;

    fn origin_ref(&self) -> &Origin<Self::Inner> {
        self.inner().origin_ref()
    }

    fn origin_mut(&mut self) -> &mut Origin<Self::Inner> {
        self.inner_mut().origin_mut()
    }
}

impl<Inner> AsOptTarget for ApObject<Inner>
where
    Inner: AsOptTarget,
{
    type Inner = Inner::Inner;

    fn opt_target_ref(&self) -> &OptTarget<Self::Inner> {
        self.inner().opt_target_ref()
    }

    fn opt_target_mut(&mut self) -> &mut OptTarget<Self::Inner> {
        self.inner_mut().opt_target_mut()
    }
}

impl<Inner> AsOptOrigin for ApObject<Inner>
where
    Inner: AsOptOrigin,
{
    type Inner = Inner::Inner;

    fn opt_origin_ref(&self) -> &OptOrigin<Self::Inner> {
        self.inner().opt_origin_ref()
    }

    fn opt_origin_mut(&mut self) -> &mut OptOrigin<Self::Inner> {
        self.inner_mut().opt_origin_mut()
    }
}

impl<Inner> AsQuestion for ApObject<Inner>
where
    Inner: AsQuestion,
{
    fn question_ref(&self) -> &Question {
        self.inner().question_ref()
    }

    fn question_mut(&mut self) -> &mut Question {
        self.inner_mut().question_mut()
    }
}

impl<T> ActivityExt for T where T: AsActivity {}
impl<T> AsActivityActorExt for T where T: AsActivityActor {}
impl<T> AsActivityObjectExt for T where T: AsActivityObject {}
impl<T> AsTargetExt for T where T: AsTarget {}
impl<T> AsOriginExt for T where T: AsOrigin {}
impl<T> AsOptTargetExt for T where T: AsOptTarget {}
impl<T> AsOptOriginExt for T where T: AsOptOrigin {}
impl<T> QuestionExt for T where T: AsQuestion {}

impl<Kind> Default for Activity<Kind>
where
    Kind: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Question {
    fn default() -> Self {
        Self::new()
    }
}
