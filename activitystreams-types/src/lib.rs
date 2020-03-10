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

//! ActivityStreams Types
//!
//! This crate defines the base set of types from the Activity Streams specification.
//!
//! ## Example Usage
//! ```rust
//! use activitystreams_types::{context, link::Mention};
//!
//! fn run() -> Result<(), anyhow::Error> {
//!     /// A Mention is the only predefined Link type in the Activity Streams spec
//!     let mut mention = Mention::default();
//!     mention.as_mut().set_context_xsd_any_uri(context())?;
//!
//!     let mention_string = serde_json::to_string(&mention)?;
//!
//!     let mention: Mention = serde_json::from_str(&mention_string)?;
//!
//!     Ok(())
//! }
//! #
//! # fn main() {
//! #     run().unwrap();
//! # }
//! ```

/// The context associated with all of the Activity Streams types defined in the crate.
pub fn context() -> crate::primitives::XsdAnyUri {
    "https://www.w3.org/ns/activitystreams".parse().unwrap()
}

pub mod activity;
pub mod actor;
pub mod collection;
pub mod link;
pub mod object;
pub mod primitives;
