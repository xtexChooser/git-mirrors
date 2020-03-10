/*
 * This file is part of ActivityStreams Types.
 *
 * Copyright © 2020 Riley Trautman
 *
 * ActivityStreams Types is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams Types is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams Types.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Namespace for Collection types

use activitystreams_derive::PropRefs;
use activitystreams_traits::{Collection, CollectionPage, Object};
use serde::{Deserialize, Serialize};

use crate::object::{properties::ObjectProperties, ObjectExt};

pub mod kind;
pub mod properties;
use self::kind::*;
use self::properties::*;

/// The Collection Extension Trait
///
/// This trait provides generic access to a collection's properties
pub trait CollectionExt: Collection {
    fn props(&self) -> &CollectionProperties;
    fn props_mut(&mut self) -> &mut CollectionProperties;
}

/// The Collection Page Extension Trait
///
/// This trait provides generic access to a collection page's properties
pub trait CollectionPageExt: CollectionPage {
    fn props(&self) -> &CollectionPageProperties;
    fn props_mut(&mut self) -> &mut CollectionPageProperties;
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CollectionBox(pub Box<dyn Object>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CollectionPageBox(pub Box<dyn Object>);

/// The default `Collection` type.
#[derive(Clone, Debug, Default, Deserialize, PropRefs, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnorderedCollection {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: CollectionType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[activitystreams(Object)]
    pub object_props: ObjectProperties,

    /// Adds all valid collection properties to this struct
    #[serde(flatten)]
    #[activitystreams(Collection)]
    pub collection_props: CollectionProperties,
}

/// A subtype of `Collection` in which members of the logical collection are assumed to always be
/// strictly ordered.
#[derive(Clone, Debug, Default, Deserialize, PropRefs, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: OrderedCollectionType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[activitystreams(Object)]
    pub object_props: ObjectProperties,

    /// Adds all valid collection properties to this struct
    #[serde(flatten)]
    #[activitystreams(Collection)]
    pub collection_props: CollectionProperties,
}

/// Used to represent distinct subsets of items from a `Collection`.
#[derive(Clone, Debug, Default, Deserialize, PropRefs, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnorderedCollectionPage {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: CollectionPageType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[activitystreams(Object)]
    pub object_props: ObjectProperties,

    /// Adds all valid collection properties to this struct
    #[serde(flatten)]
    #[activitystreams(Collection)]
    pub collection_props: CollectionProperties,

    /// Adds all valid collection page properties to this struct
    #[serde(flatten)]
    #[activitystreams(CollectionPage)]
    pub collection_page_props: CollectionPageProperties,
}

/// Used to represent ordered subsets of items from an `OrderedCollection`.
#[derive(Clone, Debug, Default, Deserialize, PropRefs, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPage {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: OrderedCollectionPageType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    #[activitystreams(Object)]
    pub object_props: ObjectProperties,

    /// Adds all valid collection properties to this struct
    #[serde(flatten)]
    #[activitystreams(Collection)]
    pub collection_props: CollectionProperties,

    /// Adds all valid collection page properties to this struct
    #[serde(flatten)]
    #[activitystreams(CollectionPage)]
    pub collection_page_props: CollectionPageProperties,

    /// Adds all valid ordered collection page properties to this struct
    #[serde(flatten)]
    #[activitystreams(None)]
    pub ordered_collection_page_props: OrderedCollectionPageProperties,
}

impl CollectionBox {
    pub fn is<T>(&self) -> bool
    where
        T: Collection + 'static,
    {
        self.0.as_any().is::<T>()
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Collection + 'static,
    {
        self.0.as_any().downcast_ref()
    }

    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Collection + 'static,
    {
        self.0.as_any_mut().downcast_mut()
    }
}

impl CollectionPageBox {
    pub fn is<T>(&self) -> bool
    where
        T: CollectionPage + 'static,
    {
        self.0.as_any().is::<T>()
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: CollectionPage + 'static,
    {
        self.0.as_any().downcast_ref()
    }

    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: CollectionPage + 'static,
    {
        self.0.as_any_mut().downcast_mut()
    }
}

impl Clone for CollectionBox {
    fn clone(&self) -> Self {
        CollectionBox(self.0.duplicate())
    }
}

impl Clone for CollectionPageBox {
    fn clone(&self) -> Self {
        CollectionPageBox(self.0.duplicate())
    }
}

impl<T> From<T> for CollectionBox
where
    T: Collection + 'static,
{
    fn from(t: T) -> Self {
        CollectionBox(Box::new(t))
    }
}

impl<T> From<T> for CollectionPageBox
where
    T: CollectionPage + 'static,
{
    fn from(t: T) -> Self {
        CollectionPageBox(Box::new(t))
    }
}
