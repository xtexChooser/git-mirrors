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

//! ActivityStreams
//!
//! A set of Traits and Types that make up the Activity Streams specification
//!
//! ## Examples
//!
//! ### Basic
//!
//! ```rust
//! use activitystreams::{
//!     context,
//!     object::{
//!         properties::{
//!             ObjectProperties,
//!             ProfileProperties
//!         },
//!         streams::Profile,
//!     },
//!     primitives::XsdAnyUri,
//!     Actor,
//!     Object,
//! };
//! use serde::{Deserialize, Serialize};
//! use std::any::Any;
//!
//! #[derive(Clone, Debug, Default, Deserialize, Serialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct Persona {
//!     #[serde(rename = "@context")]
//!     context: XsdAnyUri,
//!
//!     #[serde(rename = "type")]
//!     kind: String,
//! }
//!
//! #[typetag::serde]
//! impl Object for Persona {
//!     fn as_any(&self) -> &(dyn Any + 'static) {
//!         self
//!     }
//!
//!     fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
//!         self
//!     }
//!
//!     fn duplicate(&self) -> Box<dyn Object + 'static> {
//!         Box::new(self.clone())
//!     }
//! }
//! impl Actor for Persona {}
//!
//! fn main() -> Result<(), anyhow::Error> {
//!     let mut profile = Profile::default();
//!
//!     let pprops: &mut ProfileProperties = profile.as_mut();
//!
//!     pprops.set_describes_object_box(Persona {
//!         context: context(),
//!         kind: "Persona".to_owned(),
//!     })?;
//!
//!     let oprops: &mut ObjectProperties = profile.as_mut();
//!     oprops.set_context_xsd_any_uri(context())?;
//!
//!     let profile_string = serde_json::to_string(&profile)?;
//!
//!     let profile: Profile = serde_json::from_str(&profile_string)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Advanced
//!
//! ```rust
//! use activitystreams::{
//!     properties,
//!     link::{
//!         properties::LinkProperties,
//!         Mention,
//!     },
//!     Link,
//!     PropRefs,
//!     UnitString,
//! };
//! use serde::{Deserialize, Serialize};
//!
//! /// Using the UnitString derive macro
//! ///
//! /// This macro implements Serialize and Deserialize for the given type, making this type
//! /// represent the string "MyLink" in JSON.
//! #[derive(Clone, Debug, Default, UnitString)]
//! #[activitystreams(MyLink)]
//! pub struct MyKind;
//!
//! properties! {
//!     My {
//!         docs [ "Defining our own properties struct called MyProperties" ],
//!
//!         required_key {
//!             docs [
//!                 "Our own required key field",
//!                 "",
//!                 "'types' defines the range of values that can be stored in required_key",
//!                 "",
//!                 "'functional' means there is at most one value for required_key",
//!                 "'required' means there is at least one value for required_key",
//!             ],
//!             types [ String ],
//!             functional,
//!             required,
//!         },
//!     }
//! }
//!
//! /// Using the Properties derive macro
//! ///
//! /// This macro generates getters and setters for the associated fields.
//! #[derive(Clone, Debug, Default, Deserialize, Serialize, PropRefs)]
//! #[serde(rename_all = "camelCase")]
//! pub struct My {
//!     /// Use the UnitString MyKind to enforce the type of the object by "MyLink"
//!     pub kind: MyKind,
//!
//!     /// Derive AsRef/AsMut for My -> MyProperties
//!     #[activitystreams(None)]
//!     pub my_properties: MyProperties,
//!
//!     /// Derive AsRef/AsMut/Link for My -> MyProperties
//!     #[activitystreams(Link)]
//!     pub link_properties: LinkProperties,
//! }
//!
//! fn main() -> Result<(), anyhow::Error> {
//!     let mut my_link = My::default();
//!
//!     let lprops: &mut MyProperties = my_link.as_mut();
//!     lprops.set_required_key("Hey")?;
//!
//!     let my_link_string = serde_json::to_string(&my_link)?;
//!
//!     let my_link: My = serde_json::from_str(&my_link_string)?;
//!
//!     Ok(())
//! }
//! ```

pub mod activity;
pub mod actor;
pub mod collection;
#[cfg(feature = "types")]
pub mod endpoint;
pub mod link;
pub mod object;
#[cfg(feature = "primitives")]
pub mod primitives;

pub use self::{
    activity::{Activity, IntransitiveActivity},
    actor::Actor,
    collection::{Collection, CollectionPage},
    link::Link,
    object::Object,
};

#[cfg(feature = "primitives")]
/// The context associated with all of the Activity Streams types defined in the crate.
pub fn context() -> crate::primitives::XsdAnyUri {
    "https://www.w3.org/ns/activitystreams".parse().unwrap()
}

#[cfg(feature = "derive")]
pub use activitystreams_derive::{properties, PropRefs, UnitString};
