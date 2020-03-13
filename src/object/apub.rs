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
    object::{kind::*, properties::*, Object, ObjectBox},
    PropRefs,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ApImageBox(pub Box<Image>);

#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct Article {
    #[serde(rename = "type")]
    kind: ArticleType,

    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct Audio {
    #[serde(rename = "type")]
    kind: AudioType,

    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct Document {
    #[serde(rename = "type")]
    kind: DocumentType,

    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct Event {
    #[serde(rename = "type")]
    kind: EventType,

    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct Image {
    #[serde(rename = "type")]
    kind: ImageType,

    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct Note {
    #[serde(rename = "type")]
    kind: NoteType,

    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct Page {
    #[serde(rename = "type")]
    kind: PageType,

    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct Place {
    #[serde(rename = "type")]
    kind: PlaceType,

    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub place_props: PlaceProperties,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct Profile {
    #[serde(rename = "type")]
    kind: ProfileType,

    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub profile_props: ProfileProperties,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct Relationship {
    #[serde(rename = "type")]
    kind: RelationshipType,

    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub relationship_props: RelationshipProperties,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct Tombstone {
    #[serde(rename = "type")]
    kind: TombstoneType,

    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub tombstone_props: TombstoneProperties,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct Video {
    #[serde(rename = "type")]
    kind: VideoType,

    #[serde(flatten)]
    #[prop_refs]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    #[prop_refs]
    pub ap_object_props: ApObjectProperties,
}

impl From<Image> for ApImageBox {
    fn from(i: Image) -> Self {
        ApImageBox(Box::new(i))
    }
}

impl From<ApImageBox> for Image {
    fn from(i: ApImageBox) -> Self {
        *i.0
    }
}
