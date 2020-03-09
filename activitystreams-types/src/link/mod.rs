/*
 * This file is part of ActivityStreams Types.
 *
 * Copyright © 2018 Riley Trautman
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

use activitystreams_traits::Link;
use serde_derive::{Deserialize, Serialize};

pub mod kind;
pub mod properties;
use self::kind::*;
use self::properties::*;

/// The Link Extension Trait
///
/// This trait provides generic access to a link's properties
pub trait LinkExt: Link {
    fn props(&self) -> &LinkProperties;
    fn props_mut(&mut self) -> &mut LinkProperties;
}

/// A specialized Link that represents an @mention.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: MentionType,

    /// Adds all valid link properties to this struct
    #[serde(flatten)]
    pub link_props: LinkProperties,
}

impl Link for Mention {}
impl LinkExt for Mention {
    fn props(&self) -> &LinkProperties {
        &self.link_props
    }

    fn props_mut(&mut self) -> &mut LinkProperties {
        &mut self.link_props
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LinkBox(pub Box<dyn Link>);

impl LinkBox {
    pub fn is<T>(&self) -> bool
    where
        T: Link,
    {
        self.0.as_any().is::<T>()
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Link,
    {
        self.0.as_any().downcast_ref()
    }

    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Link,
    {
        self.0.as_any_mut().downcast_mut()
    }

    pub fn downcast<T>(self) -> Option<T>
    where
        T: Link,
    {
        let any: Box<dyn Any> = self;
        any.downcast()
    }
}

impl<T> From<T> for LinkBox
where
    T: Link,
{
    fn from(t: T) -> Self {
        LinkBox(Box::new(t))
    }
}
