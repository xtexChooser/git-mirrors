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

//! Namespace for Link types

#[cfg(feature = "kinds")]
pub mod kind;
#[cfg(feature = "types")]
pub mod properties;
#[cfg(feature = "types")]
mod types;

#[cfg(feature = "types")]
pub use self::types::Mention;

use std::any::Any;

/// A Link is an indirect, qualified reference to a resource identified by a URL.
///
/// The fundamental model for links is established by
/// [[RFC5988](https://tools.ietf.org/html/rfc5988)]. Many of the properties defined by the
/// Activity Vocabulary allow values that are either instances of Object or Link. When a Link is
/// used, it establishes a qualified relation connecting the subject (the containing object) to the
/// resource identified by the href. Properties of the Link are properties of the reference as
/// opposed to properties of the resource.
#[typetag::serde(tag = "type")]
pub trait Link: std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn duplicate(&self) -> Box<dyn Link>;
}

#[cfg(feature = "types")]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct LinkBox(pub Box<dyn Link>);

#[cfg(feature = "types")]
impl LinkBox {
    pub fn is<T>(&self) -> bool
    where
        T: Link + 'static,
    {
        self.0.as_any().is::<T>()
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Link + 'static,
    {
        self.0.as_any().downcast_ref()
    }

    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Link + 'static,
    {
        self.0.as_any_mut().downcast_mut()
    }
}

#[cfg(feature = "types")]
impl Clone for LinkBox {
    fn clone(&self) -> Self {
        LinkBox(self.0.duplicate())
    }
}

#[cfg(feature = "types")]
impl<T> From<T> for LinkBox
where
    T: Link + 'static,
{
    fn from(t: T) -> Self {
        LinkBox(Box::new(t))
    }
}
