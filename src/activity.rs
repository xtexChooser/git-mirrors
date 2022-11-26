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
pub trait AsActivity<Kind>: markers::Activity {
    /// Immutable borrow of `Activity<Kind>`
    fn activity_ref(&self) -> &Activity<Kind>;

    /// Mutable borrow of `Activity<Kind>`
    fn activity_mut(&mut self) -> &mut Activity<Kind>;
}

/// Implementation trait for deriving Actor and Object methods for a type
///
/// Any type implementing ActorAndObjectRef will automatically gain methods provided by
/// `ActorAndObjectRefExt`
pub trait ActorAndObjectRef: markers::Activity {
    /// Immutable borrow of actor field
    fn actor_field_ref(&self) -> &OneOrMany<AnyBase>;

    /// Immutable borrow of object field
    fn object_field_ref(&self) -> &OneOrMany<AnyBase>;

    /// Mutable borrow of actor field
    fn actor_field_mut(&mut self) -> &mut OneOrMany<AnyBase>;

    /// Mutable borrow of object field
    fn object_field_mut(&mut self) -> &mut OneOrMany<AnyBase>;
}

/// Implementation trait for deriving Target methods for a type
///
/// Any type implementing TargetRef will automatically gain methods provided by `TargetRefExt`
pub trait TargetRef: markers::Activity {
    /// Immutable borrow of target field
    fn target_field_ref(&self) -> &OneOrMany<AnyBase>;

    /// Mutable borrow of target field
    fn target_field_mut(&mut self) -> &mut OneOrMany<AnyBase>;
}

/// Implementation trait for deriving Origin methods for a type
///
/// Any type implementing OriginRef will automatically gain methods provided by
/// `OriginRefExt`
pub trait OriginRef: markers::Activity {
    /// Immutable borrow of origin field
    fn origin_field_ref(&self) -> &OneOrMany<AnyBase>;

    /// Mutable borrow of origin field
    fn origin_field_mut(&mut self) -> &mut OneOrMany<AnyBase>;
}

/// Implementation trait for deriving Target methods for a type
///
/// Any type implementing OptTargetRef will automatically gain methods provided by
/// `OptTargetRefExt`
pub trait OptTargetRef: markers::Activity {
    /// Immutable borrow of target field
    fn target_field_ref(&self) -> &Option<OneOrMany<AnyBase>>;

    /// Mutable borrow of target field
    fn target_field_mut(&mut self) -> &mut Option<OneOrMany<AnyBase>>;
}

/// Implementation trait for deriving Origin methods for a type
///
/// Any type implementing OptOriginRef will automatically gain methods provided by
/// `OptOriginRefExt`
pub trait OptOriginRef: markers::Activity {
    /// Immutable borrow of origin field
    fn origin_field_ref(&self) -> &Option<OneOrMany<AnyBase>>;

    /// Mutable borrow of origin field
    fn origin_field_mut(&mut self) -> &mut Option<OneOrMany<AnyBase>>;
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
pub trait ActivityExt<Kind>: AsActivity<Kind> {
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
        Kind: 'a,
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
        Kind: 'a,
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
pub trait ActorAndObjectRefExt: ActorAndObjectRef {
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
    fn actor<Kind>(&self) -> Result<&OneOrMany<AnyBase>, CheckError>
    where
        Self: BaseExt<Kind>,
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
        self.actor_field_ref()
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
        *self.actor_field_mut() = actor.into().into();
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
        *self.actor_field_mut() = v.into();
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
        self.actor_field_mut().add(actor.into());
        self
    }

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
    fn object<Kind>(&self) -> Result<&OneOrMany<AnyBase>, CheckError>
    where
        Self: BaseExt<Kind>,
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
        self.object_field_ref()
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
        *self.object_field_mut() = object.into().into();
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
        *self.object_field_mut() = v.into();
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
        self.object_field_mut().add(object.into());
        self
    }
}

/// Helper methods for interacting with Activity types with a target field
///
/// Documentation for the target field can be found on the `Invite` struct
pub trait TargetRefExt: TargetRef {
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
        self.target_field_ref()
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
        *self.target_field_mut() = target.into().into();
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
        *self.target_field_mut() = v.into();
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
        self.target_field_mut().add(target.into());
        self
    }
}

/// Helper methods for interacting with Activity types with an origin
///
/// Documentation for the origin field can be found on the `Arrive` struct
pub trait OriginRefExt: OriginRef {
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
        self.origin_field_ref()
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
        *self.origin_field_mut() = origin.into().into();
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
        *self.origin_field_mut() = v.into();
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
        self.origin_field_mut().add(origin.into());
        self
    }
}

/// Helper methods for interacting with Activity types with an optional target field
///
/// Documentation for the target field can be found on the
/// `ActorAndObjectOptTarget` struct
pub trait OptTargetRefExt: OptTargetRef {
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
        self.target_field_ref().as_ref()
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
        *self.target_field_mut() = Some(target.into().into());
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
        *self.target_field_mut() = Some(v.into());
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
        let c = match self.target_field_mut().take() {
            Some(mut c) => {
                c.add(target.into());
                c
            }
            None => vec![target.into()].into(),
        };
        *self.target_field_mut() = Some(c);
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
        self.target_field_mut().take()
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
        *self.target_field_mut() = None;
        self
    }
}

/// Helper methods for interacting with Activity types with an optional origin field
///
/// Documentation for the origin field can be found on the
/// `Delete` struct
pub trait OptOriginRefExt: OptOriginRef {
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
        self.origin_field_ref().as_ref()
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
        *self.origin_field_mut() = Some(origin.into().into());
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
        *self.origin_field_mut() = Some(v.into());
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
        let c = match self.origin_field_mut().take() {
            Some(mut c) => {
                c.add(origin.into());
                c
            }
            None => vec![origin.into()].into(),
        };
        *self.origin_field_mut() = Some(c);
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
        self.origin_field_mut().take()
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
        *self.origin_field_mut() = None;
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
/// This is just an alias for `Object<AcceptType>` because there's no fields inherent to
/// Accept that aren't already present on an ActorAndObject.
pub type Accept = ActorAndObject<AcceptType>;

/// Indicates that the actor has added the object to the target.
///
/// If the target property is not explicitly specified, the target would need to be determined
/// implicitly by context. The origin can be used to identify the context from which the object originated.
///
/// This is just an alias for `Object<AddType>` because there's no fields inherent to
/// Add that aren't already present on an ActorAndObjectOptOriginAndTarget.
pub type Add = ActorAndObjectOptOriginAndTarget<AddType>;

/// Indicates that the actor is blocking the object.
///
/// Blocking is a stronger form of Ignore. The typical use is to support social systems that allow
/// one user to block activities or content of other users. The target and origin typically have no
/// defined meaning.
///
/// This is just an alias for `Object<BlockType>` because there's no fields inherent to
/// Block that aren't already present on an ActorAndObject.
pub type Block = ActorAndObject<BlockType>;

/// Indicates that the actor has created the object.
///
/// This is just an alias for `Object<CreateType>` because there's no fields inherent to
/// Create that aren't already present on an ActorAndObject.
pub type Create = ActorAndObject<CreateType>;

/// Indicates that the actor dislikes the object.
///
/// This is just an alias for `Object<DislikeType>` because there's no fields inherent to
/// Dislike that aren't already present on an ActorAndObject.
pub type Dislike = ActorAndObject<DislikeType>;

/// Indicates that the actor is "flagging" the object.
///
/// Flagging is defined in the sense common to many social platforms as reporting content as being
/// inappropriate for any number of reasons.
///
/// This is just an alias for `Object<FlagType>` because there's no fields inherent to
/// Flag that aren't already present on an ActorAndObject.
pub type Flag = ActorAndObject<FlagType>;

/// Indicates that the actor is "following" the object.
///
/// Following is defined in the sense typically used within Social systems in which the actor is
/// interested in any activity performed by or on the object. The target and origin typically have
/// no defined meaning.
///
/// This is just an alias for `Object<FollowType>` because there's no fields inherent to Follow
/// that aren't already present on an ActorAndObject.
pub type Follow = ActorAndObject<FollowType>;

/// Indicates that the actor is ignoring the object.
///
/// The target and origin typically have no defined meaning.
///
/// This is just an alias for `Object<IgnoreType>` because there's no fields inherent to Ignore
/// that aren't already present on an ActorAndObject.
pub type Ignore = ActorAndObject<IgnoreType>;

/// Indicates that the actor has joined the object.
///
/// The target and origin typically have no defined meaning
///
/// This is just an alias for `Object<JoinType>` because there's no fields inherent to Join that
/// aren't already present on an ActorAndObject.
pub type Join = ActorAndObject<JoinType>;

/// Indicates that the actor has left the object.
///
/// The target and origin typically have no meaning.
///
/// This is just an alias for `Object<LeaveType>` because there's no fields inherent to Leave that
/// aren't already present on an ActorAndObject.
pub type Leave = ActorAndObject<LeaveType>;

/// Indicates that the actor likes, recommends or endorses the object.
///
/// The target and origin typically have no defined meaning.
///
/// This is just an alias for `Object<LikeType>` because there's no fields inherent to Like that
/// aren't already present on an ActorAndObject.
pub type Like = ActorAndObject<LikeType>;

/// Indicates that the actor has listened to the object.
///
/// This is just an alias for `Object<ListenType>` because there's no fields inherent to Listen
/// that aren't already present on an ActorAndObject.
pub type Listen = ActorAndObject<ListenType>;

/// Indicates that the actor has read the object.
///
/// This is just an alias for `Object<ReadType>` because there's no fields inherent to Read that
/// aren't already present on an ActorAndObject.
pub type Read = ActorAndObject<ReadType>;

/// Indicates that the actor is rejecting the object.
///
/// The target and origin typically have no defined meaning.
///
/// This is just an alias for `Object<RejectType>` because there's no fields inherent to Reject
/// that aren't already present on an ActorAndObject.
pub type Reject = ActorAndObject<RejectType>;

/// A specialization of Accept indicating that the acceptance is tentative.
///
/// This is just an alias for `Object<TentativeAcceptType>` because there's no fields inherent to
/// TentativeAccept that aren't already present on an ActorAndObject.
pub type TentativeAccept = ActorAndObject<TentativeAcceptType>;

/// A specialization of Reject in which the rejection is considered tentative.
///
/// This is just an alias for `Object<TentativeRejectType>` because there's no fields inherent to
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
/// This is just an alias for `Object<UndoType>` because there's no fields inherent to
/// Undo that aren't already present on an ActorAndObject.
pub type Undo = ActorAndObject<UndoType>;

/// Indicates that the actor has updated the object.
///
/// Note, however, that this vocabulary does not define a mechanism for describing the actual set
/// of modifications made to object.
///
/// The target and origin typically have no defined meaning.
///
/// This is just an alias for `Object<UpdateType>` because there's no fields inherent to
/// Update that aren't already present on an ActorAndObject.
pub type Update = ActorAndObject<UpdateType>;

/// Indicates that the actor has viewed the object.
///
/// This is just an alias for `Object<ViewType>` because there's no fields inherent to
/// View that aren't already present on an ActorAndObject.
pub type View = ActorAndObject<ViewType>;

/// Indicates that the actor is calling the target's attention the object.
///
/// The origin typically has no defined meaning.
///
/// This is just an alias for `Object<AnnounceType>` because there's no fields inherent to
/// Announce that aren't already present on an ActorAndObjectOptTarget.
pub type Announce = ActorAndObjectOptTarget<AnnounceType>;

/// Indicates that the actor is offering the object.
///
/// If specified, the target indicates the entity to which the object is being offered.
///
/// This is just an alias for `Object<OfferType>` because there's no fields inherent to
/// Offer that aren't already present on an ActorAndObjectOptTarget.
pub type Offer = ActorAndObjectOptTarget<OfferType>;

/// Indicates that the actor has moved object from origin to target.
///
/// If the origin or target are not specified, either can be determined by context.
///
/// This is just an alias for `Object<MoveType>` because there's no fields inherent to
/// Move that aren't already present on an ActorAndObjectOptOriginAndTarget.
pub type Move = ActorAndObjectOptOriginAndTarget<MoveType>;

/// Indicates that the actor is removing the object.
///
/// If specified, the origin indicates the context from which the object is being removed.
///
/// This is just an alias for `Object<RemoveType>` because there's no fields inherent to
/// Remove that aren't already present on an ActorAndObjectOptOriginAndTarget.
pub type Remove = ActorAndObjectOptOriginAndTarget<RemoveType>;

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

/// Activity with actor and object properties
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorAndObject<Kind> {
    /// Describes one or more entities that either performed or are expected to perform the
    /// activity.
    ///
    /// Any single activity can have multiple actors. The actor MAY be specified using an indirect
    /// Link.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    actor: OneOrMany<AnyBase>,

    /// When used within an Activity, describes the direct object of the activity.
    ///
    /// For instance, in the activity "John added a movie to his wishlist", the object of the
    /// activity is the movie added.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    object: OneOrMany<AnyBase>,

    /// base fields and unparsed json ends up here
    #[serde(flatten)]
    inner: Activity<Kind>,
}

/// An IntransitiveActivity that indicates that the actor has arrived at the location.
///
/// The origin can be used to identify the context from which the actor originated. The target
/// typically has no defined meaning.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Arrive {
    /// Describes one or more entities that either performed or are expected to perform the
    /// activity.
    ///
    /// Any single activity can have multiple actors. The actor MAY be specified using an indirect
    /// Link.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    actor: OneOrMany<AnyBase>,

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
    inner: Activity<ArriveType>,
}

/// A specialization of Offer in which the actor is extending an invitation for the object to the
/// target.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
    /// Describes one or more entities that either performed or are expected to perform the
    /// activity.
    ///
    /// Any single activity can have multiple actors. The actor MAY be specified using an indirect
    /// Link.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    actor: OneOrMany<AnyBase>,

    /// When used within an Activity, describes the direct object of the activity.
    ///
    /// For instance, in the activity "John added a movie to his wishlist", the object of the
    /// activity is the movie added.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    object: OneOrMany<AnyBase>,

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
    inner: Activity<InviteType>,
}

/// Indicates that the actor has deleted the object.
///
/// If specified, the origin indicates the context from which the object was deleted.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Delete {
    /// Describes one or more entities that either performed or are expected to perform the
    /// activity.
    ///
    /// Any single activity can have multiple actors. The actor MAY be specified using an indirect
    /// Link.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    actor: OneOrMany<AnyBase>,

    /// When used within an Activity, describes the direct object of the activity.
    ///
    /// For instance, in the activity "John added a movie to his wishlist", the object of the
    /// activity is the movie added.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    object: OneOrMany<AnyBase>,

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
    inner: Activity<DeleteType>,
}

/// Activity with actor, object, and optional origin and target properties
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorAndObjectOptOriginAndTarget<Kind> {
    /// Describes one or more entities that either performed or are expected to perform the
    /// activity.
    ///
    /// Any single activity can have multiple actors. The actor MAY be specified using an indirect
    /// Link.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    actor: OneOrMany<AnyBase>,

    /// When used within an Activity, describes the direct object of the activity.
    ///
    /// For instance, in the activity "John added a movie to his wishlist", the object of the
    /// activity is the movie added.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    object: OneOrMany<AnyBase>,

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
    inner: Activity<Kind>,
}

/// Activity with actor, object, and optional target properties
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorAndObjectOptTarget<Kind> {
    /// Describes one or more entities that either performed or are expected to perform the
    /// activity.
    ///
    /// Any single activity can have multiple actors. The actor MAY be specified using an indirect
    /// Link.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    actor: OneOrMany<AnyBase>,

    /// When used within an Activity, describes the direct object of the activity.
    ///
    /// For instance, in the activity "John added a movie to his wishlist", the object of the
    /// activity is the movie added.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    object: OneOrMany<AnyBase>,

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
    inner: Activity<Kind>,
}

/// Indicates that the actor is traveling to target from origin.
///
/// Travel is an IntransitiveObject whose actor specifies the direct object. If the target or
/// origin are not specified, either can be determined by context.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Travel {
    /// Describes one or more entities that either performed or are expected to perform the
    /// activity.
    ///
    /// Any single activity can have multiple actors. The actor MAY be specified using an indirect
    /// Link.
    ///
    /// - Range: Object | Link
    /// - Functional: false
    actor: OneOrMany<AnyBase>,

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
    inner: Activity<TravelType>,
}

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
        ActorAndObject {
            actor: actor.into(),
            object: object.into(),
            inner: Activity::new(),
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
        ActorAndObject {
            actor: actor.into(),
            object: object.into(),
            inner: Activity::new_none_type(),
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
        (self.actor, self.object, self.inner)
    }

    fn extending(object: Object<Kind>) -> Result<Self, serde_json::Error> {
        let mut inner = Activity::extending(object)?;

        let actor = inner.remove("actor")?;
        let object = inner.remove("object")?;

        Ok(ActorAndObject {
            actor,
            object,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<Kind>, serde_json::Error> {
        let ActorAndObject {
            actor,
            object,
            mut inner,
        } = self;

        inner.insert("actor", actor)?.insert("object", object)?;

        inner.retracting()
    }
}

impl Arrive {
    /// Create a new Arrive Activity
    ///
    /// ```rust
    /// use activitystreams::activity::Arrive;
    ///
    /// let activity = Arrive::new(vec![], vec![]);
    /// ```
    pub fn new<T, U>(actor: T, origin: U) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        U: Into<OneOrMany<AnyBase>>,
    {
        Arrive {
            actor: actor.into(),
            origin: origin.into(),
            inner: Activity::new(),
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
        (self.actor, self.origin, self.inner)
    }

    fn extending(object: Object<ArriveType>) -> Result<Self, serde_json::Error> {
        let mut inner = Activity::extending(object)?;

        let actor = inner.remove("actor")?;
        let origin = inner.remove("origin")?;

        Ok(Arrive {
            actor,
            origin,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<ArriveType>, serde_json::Error> {
        let Arrive {
            actor,
            origin,
            mut inner,
        } = self;

        inner.insert("actor", actor)?.insert("origin", origin)?;

        inner.retracting()
    }
}

impl Invite {
    /// Create a new Invite Activity
    ///
    /// ```rust
    /// use activitystreams::activity::Invite;
    ///
    /// let activity = Invite::new(vec![], vec![], vec![]);
    /// ```
    pub fn new<T, U, V>(actor: T, object: U, target: V) -> Self
    where
        T: Into<OneOrMany<AnyBase>>,
        U: Into<OneOrMany<AnyBase>>,
        V: Into<OneOrMany<AnyBase>>,
    {
        Invite {
            actor: actor.into(),
            object: object.into(),
            target: target.into(),
            inner: Activity::new(),
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
        (self.actor, self.object, self.target, self.inner)
    }

    fn extending(object: Object<InviteType>) -> Result<Self, serde_json::Error> {
        let mut inner = Activity::extending(object)?;

        let actor = inner.remove("actor")?;
        let object = inner.remove("object")?;
        let target = inner.remove("target")?;

        Ok(Invite {
            actor,
            object,
            target,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<InviteType>, serde_json::Error> {
        let Invite {
            actor,
            object,
            target,
            mut inner,
        } = self;

        inner
            .insert("actor", actor)?
            .insert("object", object)?
            .insert("target", target)?;

        inner.retracting()
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
        Delete {
            actor: actor.into(),
            object: object.into(),
            origin: None,
            inner: Activity::new(),
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
        (self.actor, self.object, self.origin, self.inner)
    }

    fn extending(object: Object<DeleteType>) -> Result<Self, serde_json::Error> {
        let mut inner = Activity::extending(object)?;

        let actor = inner.remove("actor")?;
        let object = inner.remove("object")?;
        let origin = inner.remove("origin")?;

        Ok(Delete {
            actor,
            object,
            origin,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<DeleteType>, serde_json::Error> {
        let Delete {
            actor,
            object,
            origin,
            mut inner,
        } = self;

        inner
            .insert("actor", actor)?
            .insert("object", object)?
            .insert("origin", origin)?;

        inner.retracting()
    }
}

impl<Kind> ActorAndObjectOptOriginAndTarget<Kind> {
    /// Create a new ActorAndObjectOptOriginAndTarget Activity
    ///
    /// ```rust
    /// use activitystreams::activity::ActorAndObjectOptOriginAndTarget;
    ///
    /// let activity = ActorAndObjectOptOriginAndTarget::<String>::new(
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
        ActorAndObjectOptOriginAndTarget {
            actor: actor.into(),
            object: object.into(),
            origin: None,
            target: None,
            inner: Activity::new(),
        }
    }

    /// Create a new ActorAndObjectOptOriginAndTarget with `None` for it's `kind` property
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
        ActorAndObjectOptOriginAndTarget {
            actor: actor.into(),
            object: object.into(),
            origin: None,
            target: None,
            inner: Activity::new_none_type(),
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
            self.actor,
            self.object,
            self.origin,
            self.target,
            self.inner,
        )
    }

    fn extending(object: Object<Kind>) -> Result<Self, serde_json::Error> {
        let mut inner = Activity::extending(object)?;

        let actor = inner.remove("actor")?;
        let object = inner.remove("object")?;
        let origin = inner.remove("origin")?;
        let target = inner.remove("target")?;

        Ok(ActorAndObjectOptOriginAndTarget {
            actor,
            object,
            origin,
            target,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<Kind>, serde_json::Error> {
        let ActorAndObjectOptOriginAndTarget {
            actor,
            object,
            origin,
            target,
            mut inner,
        } = self;

        inner
            .insert("actor", actor)?
            .insert("object", object)?
            .insert("origin", origin)?
            .insert("target", target)?;

        inner.retracting()
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
        ActorAndObjectOptTarget {
            actor: actor.into(),
            object: object.into(),
            target: None,
            inner: Activity::new(),
        }
    }

    /// Create a new ActorAndObjectOptTarget with `None` for it's `kind` property
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
        ActorAndObjectOptTarget {
            actor: actor.into(),
            object: object.into(),
            target: None,
            inner: Activity::new_none_type(),
        }
    }

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
        (self.actor, self.object, self.target, self.inner)
    }

    fn extending(object: Object<Kind>) -> Result<Self, serde_json::Error> {
        let mut inner = Activity::extending(object)?;

        let actor = inner.remove("actor")?;
        let object = inner.remove("object")?;
        let target = inner.remove("target")?;

        Ok(ActorAndObjectOptTarget {
            actor,
            object,
            target,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<Kind>, serde_json::Error> {
        let ActorAndObjectOptTarget {
            actor,
            object,
            target,
            mut inner,
        } = self;

        inner
            .insert("actor", actor)?
            .insert("object", object)?
            .insert("target", target)?;

        inner.retracting()
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
        Travel {
            actor: actor.into(),
            origin: None,
            target: None,
            inner: Activity::new(),
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
        (self.actor, self.origin, self.target, self.inner)
    }

    fn extending(object: Object<TravelType>) -> Result<Self, serde_json::Error> {
        let mut inner = Activity::extending(object)?;

        let actor = inner.remove("actor")?;
        let origin = inner.remove("origin")?;
        let target = inner.remove("target")?;

        Ok(Travel {
            actor,
            origin,
            target,
            inner,
        })
    }

    fn retracting(self) -> Result<Object<TravelType>, serde_json::Error> {
        let Travel {
            actor,
            origin,
            target,
            mut inner,
        } = self;

        inner
            .insert("actor", actor)?
            .insert("origin", origin)?
            .insert("target", target)?;

        inner.retracting()
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

impl<Kind> markers::Base for ActorAndObject<Kind> {}
impl<Kind> markers::Object for ActorAndObject<Kind> {}
impl<Kind> markers::Activity for ActorAndObject<Kind> {}

impl<Kind> markers::Base for ActorAndObjectOptTarget<Kind> {}
impl<Kind> markers::Object for ActorAndObjectOptTarget<Kind> {}
impl<Kind> markers::Activity for ActorAndObjectOptTarget<Kind> {}

impl<Kind> markers::Base for ActorAndObjectOptOriginAndTarget<Kind> {}
impl<Kind> markers::Object for ActorAndObjectOptOriginAndTarget<Kind> {}
impl<Kind> markers::Activity for ActorAndObjectOptOriginAndTarget<Kind> {}

impl markers::Base for Arrive {}
impl markers::Object for Arrive {}
impl markers::Activity for Arrive {}
impl markers::IntransitiveActivity for Arrive {}

impl markers::Base for Invite {}
impl markers::Object for Invite {}
impl markers::Activity for Invite {}

impl markers::Base for Delete {}
impl markers::Object for Delete {}
impl markers::Activity for Delete {}

impl markers::Base for Travel {}
impl markers::Object for Travel {}
impl markers::Activity for Travel {}
impl markers::IntransitiveActivity for Travel {}

impl markers::Base for Question {}
impl markers::Object for Question {}
impl markers::Activity for Question {}
impl markers::IntransitiveActivity for Question {}

impl<Inner> markers::Activity for ApObject<Inner> where Inner: markers::Activity {}
impl<Inner> markers::IntransitiveActivity for ApObject<Inner> where
    Inner: markers::IntransitiveActivity
{
}

impl<Kind> Extends<Kind> for Activity<Kind> {
    type Error = serde_json::Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
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

impl<Kind> Extends<Kind> for ActorAndObject<Kind> {
    type Error = serde_json::Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Kind> TryFrom<Object<Kind>> for ActorAndObject<Kind> {
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl<Kind> TryFrom<ActorAndObject<Kind>> for Object<Kind> {
    type Error = serde_json::Error;

    fn try_from(activity: ActorAndObject<Kind>) -> Result<Self, Self::Error> {
        activity.retracting()
    }
}

impl Extends<ArriveType> for Arrive {
    type Error = serde_json::Error;

    fn extends(base: Base<ArriveType>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<ArriveType>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl TryFrom<Object<ArriveType>> for Arrive {
    type Error = serde_json::Error;

    fn try_from(object: Object<ArriveType>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl TryFrom<Arrive> for Object<ArriveType> {
    type Error = serde_json::Error;

    fn try_from(arrive: Arrive) -> Result<Self, Self::Error> {
        arrive.retracting()
    }
}

impl Extends<InviteType> for Invite {
    type Error = serde_json::Error;

    fn extends(base: Base<InviteType>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<InviteType>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl TryFrom<Object<InviteType>> for Invite {
    type Error = serde_json::Error;

    fn try_from(object: Object<InviteType>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl TryFrom<Invite> for Object<InviteType> {
    type Error = serde_json::Error;

    fn try_from(invite: Invite) -> Result<Self, Self::Error> {
        invite.retracting()
    }
}

impl Extends<DeleteType> for Delete {
    type Error = serde_json::Error;

    fn extends(base: Base<DeleteType>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<DeleteType>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl TryFrom<Object<DeleteType>> for Delete {
    type Error = serde_json::Error;

    fn try_from(object: Object<DeleteType>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl TryFrom<Delete> for Object<DeleteType> {
    type Error = serde_json::Error;

    fn try_from(delete: Delete) -> Result<Self, Self::Error> {
        delete.retracting()
    }
}

impl<Kind> Extends<Kind> for ActorAndObjectOptOriginAndTarget<Kind> {
    type Error = serde_json::Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Kind> TryFrom<Object<Kind>> for ActorAndObjectOptOriginAndTarget<Kind> {
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl<Kind> TryFrom<ActorAndObjectOptOriginAndTarget<Kind>> for Object<Kind> {
    type Error = serde_json::Error;

    fn try_from(activity: ActorAndObjectOptOriginAndTarget<Kind>) -> Result<Self, Self::Error> {
        activity.retracting()
    }
}

impl<Kind> Extends<Kind> for ActorAndObjectOptTarget<Kind> {
    type Error = serde_json::Error;

    fn extends(base: Base<Kind>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<Kind>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl<Kind> TryFrom<Object<Kind>> for ActorAndObjectOptTarget<Kind> {
    type Error = serde_json::Error;

    fn try_from(object: Object<Kind>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl<Kind> TryFrom<ActorAndObjectOptTarget<Kind>> for Object<Kind> {
    type Error = serde_json::Error;

    fn try_from(activity: ActorAndObjectOptTarget<Kind>) -> Result<Self, Self::Error> {
        activity.retracting()
    }
}

impl Extends<TravelType> for Travel {
    type Error = serde_json::Error;

    fn extends(base: Base<TravelType>) -> Result<Self, Self::Error> {
        let inner = Object::extends(base)?;
        Self::extending(inner)
    }

    fn retracts(self) -> Result<Base<TravelType>, Self::Error> {
        let inner = self.retracting()?;
        inner.retracts()
    }
}

impl TryFrom<Object<TravelType>> for Travel {
    type Error = serde_json::Error;

    fn try_from(object: Object<TravelType>) -> Result<Self, Self::Error> {
        Self::extending(object)
    }
}

impl TryFrom<Travel> for Object<TravelType> {
    type Error = serde_json::Error;

    fn try_from(travel: Travel) -> Result<Self, Self::Error> {
        travel.retracting()
    }
}

impl Extends<QuestionType> for Question {
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

impl<Kind> UnparsedMut for ActorAndObject<Kind> {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl UnparsedMut for Arrive {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl UnparsedMut for Invite {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl UnparsedMut for Delete {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Kind> UnparsedMut for ActorAndObjectOptOriginAndTarget<Kind> {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Kind> UnparsedMut for ActorAndObjectOptTarget<Kind> {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl UnparsedMut for Travel {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl UnparsedMut for Question {
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

impl<Kind> AsBase<Kind> for Activity<Kind> {
    fn base_ref(&self) -> &Base<Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Kind> {
        self.inner.base_mut()
    }
}

impl<Kind> AsObject<Kind> for Activity<Kind> {
    fn object_ref(&self) -> &Object<Kind> {
        &self.inner
    }

    fn object_mut(&mut self) -> &mut Object<Kind> {
        &mut self.inner
    }
}

impl<Kind> AsActivity<Kind> for Activity<Kind> {
    fn activity_ref(&self) -> &Activity<Kind> {
        self
    }

    fn activity_mut(&mut self) -> &mut Activity<Kind> {
        self
    }
}

impl<Kind> AsBase<Kind> for ActorAndObject<Kind> {
    fn base_ref(&self) -> &Base<Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Kind> {
        self.inner.base_mut()
    }
}

impl<Kind> AsObject<Kind> for ActorAndObject<Kind> {
    fn object_ref(&self) -> &Object<Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Kind> {
        self.inner.object_mut()
    }
}

impl<Kind> AsActivity<Kind> for ActorAndObject<Kind> {
    fn activity_ref(&self) -> &Activity<Kind> {
        &self.inner
    }

    fn activity_mut(&mut self) -> &mut Activity<Kind> {
        &mut self.inner
    }
}

impl<Kind> ActorAndObjectRef for ActorAndObject<Kind> {
    fn actor_field_ref(&self) -> &OneOrMany<AnyBase> {
        &self.actor
    }

    fn object_field_ref(&self) -> &OneOrMany<AnyBase> {
        &self.object
    }

    fn actor_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        &mut self.actor
    }

    fn object_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        &mut self.object
    }
}

impl<Kind> AsBase<Kind> for ActorAndObjectOptTarget<Kind> {
    fn base_ref(&self) -> &Base<Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Kind> {
        self.inner.base_mut()
    }
}

impl<Kind> AsObject<Kind> for ActorAndObjectOptTarget<Kind> {
    fn object_ref(&self) -> &Object<Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Kind> {
        self.inner.object_mut()
    }
}

impl<Kind> AsActivity<Kind> for ActorAndObjectOptTarget<Kind> {
    fn activity_ref(&self) -> &Activity<Kind> {
        &self.inner
    }

    fn activity_mut(&mut self) -> &mut Activity<Kind> {
        &mut self.inner
    }
}

impl<Kind> ActorAndObjectRef for ActorAndObjectOptTarget<Kind> {
    fn actor_field_ref(&self) -> &OneOrMany<AnyBase> {
        &self.actor
    }

    fn object_field_ref(&self) -> &OneOrMany<AnyBase> {
        &self.object
    }

    fn actor_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        &mut self.actor
    }

    fn object_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        &mut self.object
    }
}

impl<Kind> OptTargetRef for ActorAndObjectOptTarget<Kind> {
    fn target_field_ref(&self) -> &Option<OneOrMany<AnyBase>> {
        &self.target
    }

    fn target_field_mut(&mut self) -> &mut Option<OneOrMany<AnyBase>> {
        &mut self.target
    }
}

impl<Kind> AsBase<Kind> for ActorAndObjectOptOriginAndTarget<Kind> {
    fn base_ref(&self) -> &Base<Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Kind> {
        self.inner.base_mut()
    }
}

impl<Kind> AsObject<Kind> for ActorAndObjectOptOriginAndTarget<Kind> {
    fn object_ref(&self) -> &Object<Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Kind> {
        self.inner.object_mut()
    }
}

impl<Kind> AsActivity<Kind> for ActorAndObjectOptOriginAndTarget<Kind> {
    fn activity_ref(&self) -> &Activity<Kind> {
        &self.inner
    }

    fn activity_mut(&mut self) -> &mut Activity<Kind> {
        &mut self.inner
    }
}

impl<Kind> ActorAndObjectRef for ActorAndObjectOptOriginAndTarget<Kind> {
    fn actor_field_ref(&self) -> &OneOrMany<AnyBase> {
        &self.actor
    }

    fn object_field_ref(&self) -> &OneOrMany<AnyBase> {
        &self.object
    }

    fn actor_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        &mut self.actor
    }

    fn object_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        &mut self.object
    }
}

impl<Kind> OptTargetRef for ActorAndObjectOptOriginAndTarget<Kind> {
    fn target_field_ref(&self) -> &Option<OneOrMany<AnyBase>> {
        &self.target
    }

    fn target_field_mut(&mut self) -> &mut Option<OneOrMany<AnyBase>> {
        &mut self.target
    }
}

impl<Kind> OptOriginRef for ActorAndObjectOptOriginAndTarget<Kind> {
    fn origin_field_ref(&self) -> &Option<OneOrMany<AnyBase>> {
        &self.origin
    }

    fn origin_field_mut(&mut self) -> &mut Option<OneOrMany<AnyBase>> {
        &mut self.origin
    }
}

impl AsBase<ArriveType> for Arrive {
    fn base_ref(&self) -> &Base<ArriveType> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<ArriveType> {
        self.inner.base_mut()
    }
}

impl AsObject<ArriveType> for Arrive {
    fn object_ref(&self) -> &Object<ArriveType> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<ArriveType> {
        self.inner.object_mut()
    }
}

impl AsActivity<ArriveType> for Arrive {
    fn activity_ref(&self) -> &Activity<ArriveType> {
        &self.inner
    }

    fn activity_mut(&mut self) -> &mut Activity<ArriveType> {
        &mut self.inner
    }
}

impl OriginRef for Arrive {
    fn origin_field_ref(&self) -> &OneOrMany<AnyBase> {
        &self.origin
    }

    fn origin_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        &mut self.origin
    }
}

impl AsBase<InviteType> for Invite {
    fn base_ref(&self) -> &Base<InviteType> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<InviteType> {
        self.inner.base_mut()
    }
}

impl AsObject<InviteType> for Invite {
    fn object_ref(&self) -> &Object<InviteType> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<InviteType> {
        self.inner.object_mut()
    }
}

impl AsActivity<InviteType> for Invite {
    fn activity_ref(&self) -> &Activity<InviteType> {
        &self.inner
    }

    fn activity_mut(&mut self) -> &mut Activity<InviteType> {
        &mut self.inner
    }
}

impl ActorAndObjectRef for Invite {
    fn actor_field_ref(&self) -> &OneOrMany<AnyBase> {
        &self.actor
    }

    fn object_field_ref(&self) -> &OneOrMany<AnyBase> {
        &self.object
    }

    fn actor_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        &mut self.actor
    }

    fn object_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        &mut self.object
    }
}

impl TargetRef for Invite {
    fn target_field_ref(&self) -> &OneOrMany<AnyBase> {
        &self.target
    }

    fn target_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        &mut self.target
    }
}

impl AsBase<DeleteType> for Delete {
    fn base_ref(&self) -> &Base<DeleteType> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<DeleteType> {
        self.inner.base_mut()
    }
}

impl AsObject<DeleteType> for Delete {
    fn object_ref(&self) -> &Object<DeleteType> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<DeleteType> {
        self.inner.object_mut()
    }
}

impl AsActivity<DeleteType> for Delete {
    fn activity_ref(&self) -> &Activity<DeleteType> {
        &self.inner
    }

    fn activity_mut(&mut self) -> &mut Activity<DeleteType> {
        &mut self.inner
    }
}

impl ActorAndObjectRef for Delete {
    fn actor_field_ref(&self) -> &OneOrMany<AnyBase> {
        &self.actor
    }

    fn object_field_ref(&self) -> &OneOrMany<AnyBase> {
        &self.object
    }

    fn actor_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        &mut self.actor
    }

    fn object_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        &mut self.object
    }
}

impl OptOriginRef for Delete {
    fn origin_field_ref(&self) -> &Option<OneOrMany<AnyBase>> {
        &self.origin
    }

    fn origin_field_mut(&mut self) -> &mut Option<OneOrMany<AnyBase>> {
        &mut self.origin
    }
}

impl AsBase<TravelType> for Travel {
    fn base_ref(&self) -> &Base<TravelType> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<TravelType> {
        self.inner.base_mut()
    }
}

impl AsObject<TravelType> for Travel {
    fn object_ref(&self) -> &Object<TravelType> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<TravelType> {
        self.inner.object_mut()
    }
}

impl AsActivity<TravelType> for Travel {
    fn activity_ref(&self) -> &Activity<TravelType> {
        self.inner.activity_ref()
    }

    fn activity_mut(&mut self) -> &mut Activity<TravelType> {
        self.inner.activity_mut()
    }
}

impl OptTargetRef for Travel {
    fn target_field_ref(&self) -> &Option<OneOrMany<AnyBase>> {
        &self.target
    }

    fn target_field_mut(&mut self) -> &mut Option<OneOrMany<AnyBase>> {
        &mut self.target
    }
}

impl OptOriginRef for Travel {
    fn origin_field_ref(&self) -> &Option<OneOrMany<AnyBase>> {
        &self.origin
    }

    fn origin_field_mut(&mut self) -> &mut Option<OneOrMany<AnyBase>> {
        &mut self.origin
    }
}

impl AsBase<QuestionType> for Question {
    fn base_ref(&self) -> &Base<QuestionType> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<QuestionType> {
        self.inner.base_mut()
    }
}

impl AsObject<QuestionType> for Question {
    fn object_ref(&self) -> &Object<QuestionType> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<QuestionType> {
        self.inner.object_mut()
    }
}

impl AsActivity<QuestionType> for Question {
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

impl<Inner, Kind> AsActivity<Kind> for ApObject<Inner>
where
    Inner: AsActivity<Kind>,
{
    fn activity_ref(&self) -> &Activity<Kind> {
        self.inner().activity_ref()
    }

    fn activity_mut(&mut self) -> &mut Activity<Kind> {
        self.inner_mut().activity_mut()
    }
}

impl<Inner> ActorAndObjectRef for ApObject<Inner>
where
    Inner: ActorAndObjectRef,
{
    fn actor_field_ref(&self) -> &OneOrMany<AnyBase> {
        self.inner().actor_field_ref()
    }

    fn object_field_ref(&self) -> &OneOrMany<AnyBase> {
        self.inner().object_field_ref()
    }

    fn actor_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        self.inner_mut().actor_field_mut()
    }

    fn object_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        self.inner_mut().object_field_mut()
    }
}

impl<Inner> TargetRef for ApObject<Inner>
where
    Inner: TargetRef,
{
    fn target_field_ref(&self) -> &OneOrMany<AnyBase> {
        self.inner().target_field_ref()
    }

    fn target_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        self.inner_mut().target_field_mut()
    }
}

impl<Inner> OriginRef for ApObject<Inner>
where
    Inner: OriginRef,
{
    fn origin_field_ref(&self) -> &OneOrMany<AnyBase> {
        self.inner().origin_field_ref()
    }

    fn origin_field_mut(&mut self) -> &mut OneOrMany<AnyBase> {
        self.inner_mut().origin_field_mut()
    }
}

impl<Inner> OptTargetRef for ApObject<Inner>
where
    Inner: OptTargetRef,
{
    fn target_field_ref(&self) -> &Option<OneOrMany<AnyBase>> {
        self.inner().target_field_ref()
    }

    fn target_field_mut(&mut self) -> &mut Option<OneOrMany<AnyBase>> {
        self.inner_mut().target_field_mut()
    }
}

impl<Inner> OptOriginRef for ApObject<Inner>
where
    Inner: OptOriginRef,
{
    fn origin_field_ref(&self) -> &Option<OneOrMany<AnyBase>> {
        self.inner().origin_field_ref()
    }

    fn origin_field_mut(&mut self) -> &mut Option<OneOrMany<AnyBase>> {
        self.inner_mut().origin_field_mut()
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

impl<T, Kind> ActivityExt<Kind> for T where T: AsActivity<Kind> {}
impl<T> ActorAndObjectRefExt for T where T: ActorAndObjectRef {}
impl<T> TargetRefExt for T where T: TargetRef {}
impl<T> OriginRefExt for T where T: OriginRef {}
impl<T> OptTargetRefExt for T where T: OptTargetRef {}
impl<T> OptOriginRefExt for T where T: OptOriginRef {}
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
