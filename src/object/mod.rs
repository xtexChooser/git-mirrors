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

//! Namespace for Object types

#[cfg(feature = "types")]
pub mod apub;
#[cfg(feature = "kinds")]
pub mod kind;
#[cfg(feature = "types")]
pub mod properties;
#[cfg(feature = "types")]
pub mod streams;

use std::any::Any;

/// Describes an object of any kind.
///
/// The Object type serves as the base type for most of the other kinds of objects defined in the
/// Activity Vocabulary, including other Core types such as `Activity`, `IntransitiveActivity`,
/// `Collection` and `OrderedCollection`.
#[typetag::serde(tag = "type")]
pub trait Object: std::fmt::Debug {
    /// Provide an as_any method to allow for borrowed downcasting.
    ///
    /// This is useful since Objects can be deserialized generically via typetag
    fn as_any(&self) -> &dyn Any;

    /// Provide an as_any method to allow for mutably borrowed downcasting.
    ///
    /// This is useful since Objects can be deserialized generically via typetag
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Provide a duplicate method to allow for cloning type objects.
    ///
    /// This is useful since Objects can be deserialized generically via typetag
    fn duplicate(&self) -> Box<dyn Object>;
}

#[cfg(feature = "types")]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct ObjectBox(pub Box<dyn Object>);

#[cfg(feature = "types")]
impl ObjectBox {
    pub fn is<T>(&self) -> bool
    where
        T: Object + 'static,
    {
        self.0.as_any().is::<T>()
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Object + 'static,
    {
        self.0.as_any().downcast_ref()
    }

    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Object + 'static,
    {
        self.0.as_any_mut().downcast_mut()
    }
}

#[cfg(feature = "types")]
impl Clone for ObjectBox {
    fn clone(&self) -> Self {
        ObjectBox(self.0.duplicate())
    }
}

#[cfg(feature = "types")]
impl<T> From<T> for ObjectBox
where
    T: Object + 'static,
{
    fn from(t: T) -> Self {
        ObjectBox(Box::new(t))
    }
}
