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

//! Namespace for Collection types

#[cfg(feature = "types")]
pub mod apub;
#[cfg(feature = "kinds")]
pub mod kind;
#[cfg(feature = "types")]
pub mod properties;
#[cfg(feature = "types")]
pub mod streams;

use crate::object::Object;

/// A Collection is a subtype of `Object` that represents ordered or unordered sets of `Object` or
/// `Link` instances.
///
/// The items within a Collection can be ordered or unordered. The OrderedCollection type MAY be
/// used to identify a Collection whose items are always ordered. In the JSON serialization, the
/// unordered items of a Collection are represented using the items property while ordered items
/// are represented using the orderedItems property.
///
/// `UnorderedCollection` and `OrderedCollection` types are provided by the `activitystreams-types`
/// crate.
pub trait Collection: Object {}

/// Used to represent distinct subsets of items from a Collection.
///
/// A `Collection` can contain a large number of items. Often, it becomes impractical for an
/// implementation to serialize every item contained by a `Collection` using the items (or
/// `ordered_items`) property alone. In such cases, the items within a `Collection` can be divided
/// into distinct subsets or "pages". A page is identified using the `CollectionPage` type.
///
/// `UnorderedCollectionPage` and `OrderedCollectionPage` types are provied by the
/// `activitystreams-types` crate.
pub trait CollectionPage: Collection {}

#[cfg(feature = "types")]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct CollectionBox(pub Box<dyn Object>);

#[cfg(feature = "types")]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct CollectionPageBox(pub Box<dyn Object>);

#[cfg(feature = "types")]
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

#[cfg(feature = "types")]
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

#[cfg(feature = "types")]
impl Clone for CollectionBox {
    fn clone(&self) -> Self {
        CollectionBox(self.0.duplicate())
    }
}

#[cfg(feature = "types")]
impl Clone for CollectionPageBox {
    fn clone(&self) -> Self {
        CollectionPageBox(self.0.duplicate())
    }
}

#[cfg(feature = "types")]
impl<T> From<T> for CollectionBox
where
    T: Collection + 'static,
{
    fn from(t: T) -> Self {
        CollectionBox(Box::new(t))
    }
}

#[cfg(feature = "types")]
impl<T> From<T> for CollectionPageBox
where
    T: CollectionPage + 'static,
{
    fn from(t: T) -> Self {
        CollectionPageBox(Box::new(t))
    }
}
