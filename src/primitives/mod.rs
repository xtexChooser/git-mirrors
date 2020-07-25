//! Types creating the base for most ActivityStreams fields.
//!
//! These types are not themselves defined by ActivityStreams, but are referenced by the
//! specification.
//!
//! ```rust
//! use activitystreams::primitives::{AnyString, OneOrMany, Unit};
//!
//! let any_string = AnyString::from_xsd_string("hey");
//!
//! let one_or_many = OneOrMany::<i32>::from_one(1234);
//!
//! let cm = Unit::centimeters();
//! ```

mod any_string;
mod one_or_many;
mod rdf_lang_string;
mod serde_parse;
mod unit;
mod xsd_datetime;
mod xsd_duration;

pub use self::{
    any_string::AnyString,
    one_or_many::OneOrMany,
    rdf_lang_string::RdfLangString,
    unit::Unit,
    xsd_datetime::XsdDateTime,
    xsd_duration::{XsdDuration, XsdDurationError},
};

use self::serde_parse::SerdeParse;

/// An alias for the mime::Mime struct with serde compatibility
pub(crate) type MimeMediaType = SerdeParse<mime::Mime>;
