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

#[cfg(feature = "types")]
use activitystreams_derive::PropRefs;
#[cfg(feature = "types")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "kinds")]
pub mod kind;
#[cfg(feature = "types")]
pub mod properties;
#[cfg(feature = "types")]
use self::kind::*;
#[cfg(feature = "types")]
use self::properties::*;

pub use activitystreams_traits::Link;

#[cfg(feature = "types")]
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LinkBox(pub Box<dyn Link>);

#[cfg(feature = "types")]
/// A specialized Link that represents an @mention.
#[derive(Clone, Debug, Default, Deserialize, PropRefs, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: MentionType,

    /// Adds all valid link properties to this struct
    #[serde(flatten)]
    #[activitystreams(Link)]
    pub link_props: LinkProperties,
}

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
