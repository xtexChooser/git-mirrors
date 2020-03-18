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

use crate::{
    ext::Ext,
    link::{kind::*, properties::*, Link, LinkBox},
    Base, PropRefs,
};

#[cfg(feature = "types")]
/// A specialized Link that represents an @mention.
#[derive(Clone, Debug, Default, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Link)]
pub struct Mention {
    #[serde(rename = "type")]
    #[serde(alias = "objectType")]
    #[serde(alias = "verb")]
    kind: MentionType,

    /// Adds all valid link properties to this struct
    #[serde(flatten)]
    #[prop_refs]
    pub link_props: LinkProperties,
}
