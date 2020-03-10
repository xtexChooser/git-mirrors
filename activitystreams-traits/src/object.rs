/*
 * This file is part of ActivityStreams Traits.
 *
 * Copyright © 2020 Riley Trautman
 *
 * ActivityStreams Traits is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams Traits is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams Traits.  If not, see <http://www.gnu.org/licenses/>.
 */

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
