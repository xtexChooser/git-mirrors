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

//! Namespace for Unit Structs that serialize to strings
use crate::UnitString;

/// A Unit Struct that represents the string "Article"
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Article)]
pub struct ArticleType;

/// A Unit Struct that represents the string "Audio"
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Audio)]
pub struct AudioType;

/// A Unit Struct that represents the string "Document"
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Document)]
pub struct DocumentType;

/// A Unit Struct that represents the string "Event"
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Event)]
pub struct EventType;

/// A Unit Struct that represents the string "Image"
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Image)]
pub struct ImageType;

/// A Unit Struct that represents the string "Note"
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Note)]
pub struct NoteType;

/// A Unit Struct that represents the string "Page"
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Page)]
pub struct PageType;

/// A Unit Struct that represents the string "Place"
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Place)]
pub struct PlaceType;

/// A Unit Struct that represents the string "Profile"
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Profile)]
pub struct ProfileType;

/// A Unit Struct that represents the string "Relationship"
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Relationship)]
pub struct RelationshipType;

/// A Unit Struct that represents the string "Tombstone"
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Tombstone)]
pub struct TombstoneType;

/// A Unit Struct that represents the string "Video"
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(Video)]
pub struct VideoType;
