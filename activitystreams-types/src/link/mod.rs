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

//! Namespace for Link types

use activitystreams_derive::PropRefs;
use activitystreams_traits::Link;
use serde::{Deserialize, Serialize};

pub mod kind;
pub mod properties;
use self::kind::*;
use self::properties::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LinkBox(pub Box<dyn Link>);

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

impl Clone for LinkBox {
    fn clone(&self) -> Self {
        LinkBox(self.0.duplicate())
    }
}

impl<T> From<T> for LinkBox
where
    T: Link + 'static,
{
    fn from(t: T) -> Self {
        LinkBox(Box::new(t))
    }
}
