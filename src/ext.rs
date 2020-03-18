//! Defining extensibility in the ActivityStreams spec

use crate::{
    activity::{Activity, IntransitiveActivity},
    actor::Actor,
    collection::{Collection, CollectionPage},
    link::Link,
    object::Object,
    Base,
};
use std::fmt::Debug;

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
