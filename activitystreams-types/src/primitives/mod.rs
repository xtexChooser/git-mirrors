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

//! Types defined as 'primitives' are used as building-blocks for ActivityStreams objects.
//!
//! For example, an `Object` may have a `summary` field, which is defined as a range of
//! `xsd:string` and `rdf:langString`. As code, this is represented as an enum that either
//! contains an `XsdString` or an `RdfLangString`.
//!
//! ```rust
//! use activitystreams_types::primitives::{RdfLangString, XsdString};
//!
//! /// Define a terminating enum for the Summary field in Object Properties
//! ///
//! /// In this case, terminating means it does not contain child elements.
//! pub enum ObjectPropertiesSummaryTermEnum {
//!     XsdString(XsdString),
//!     RdfLangString(RdfLangString),
//! }
//!
//! /// Since summary isn't functional, we can either have a single string, or multiple strings.
//! pub enum ObjectPropertiesSummaryEnum {
//!     Term(ObjectPropertiesSummaryTermEnum),
//!     Array(Vec<ObjectPropertiesSummaryTermEnum>),
//! }
//!
//! /// Define an excerpt from the ObjectProperties struct
//! pub struct ObjectProperties {
//!     // ...
//!
//!     /// Since summary isn't a required field, it's stored as an option
//!     summary: Option<ObjectPropertiesSummaryEnum>,
//!
//!     // ...
//! }
//! #
//! # fn main() {}
//! ```

mod length;
mod mime_media_type;
mod rdf_lang_string;
mod xsd_any_uri;
mod xsd_datetime;
mod xsd_duration;
mod xsd_float;
mod xsd_non_negative_float;
mod xsd_non_negative_integer;
mod xsd_string;

pub use self::{
    length::Length,
    mime_media_type::{MimeMediaType, MimeMediaTypeError},
    rdf_lang_string::RdfLangString,
    xsd_any_uri::{XsdAnyUri, XsdAnyUriError},
    xsd_datetime::{XsdDateTime, XsdDateTimeError},
    xsd_duration::{XsdDuration, XsdDurationError},
    xsd_float::{XsdFloat, XsdFloatError},
    xsd_non_negative_float::{XsdNonNegativeFloat, XsdNonNegativeFloatError},
    xsd_non_negative_integer::{XsdNonNegativeInteger, XsdNonNegativeIntegerError},
    xsd_string::XsdString,
};
