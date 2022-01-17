# Unreleased
- implement `IntoIterator` for `&OneOrMany<T>` and `&mut OneOrMany<T>`
- add `check` function for verifying an IRI's authority
- add `BaseExt::check_authority` for verifying an IRI's authority against an object's ID
- add back checked `Base::id`, `Activity::actor`, `Activity::object`, `Actor::inbox`,
  `Actor::outbox`, `Actor::following`, `Actor::followers`, `Actor::liked`, `Actor::streams`,
  `Actor::endpoints`

# 0.7.0-alpha.14
- switch to iri-string from url
- remove `checked` variations of methods, rename `unchecked`

# 0.7.0-alpha.13
- re-export url functions, kind macro from activitystreams-kinds library

# 0.7.0-alpha.12
- re-export kinds from new activitystreams-kinds library

# 0.7.0-alpha.11
- Add `to_owned` to `OneOrMany<&'a AnyString>`
- Update summary and content to return `OneOrMany<&'a AnyString>`
- Implement as_single_xsd_string and as_single_rdf_lang_string for `OneOrMany<&'a AnyString>`
- Change Add from aliasing `ActorAndObject` to `ActorAndObjectOptOriginAndTarget`

# 0.7.0-alpha.10
- Fix extraction of `image` and `icon` when creating Objects from AnyBase

# 0.7.0-alpha.9
- Add default impls for many object kinds
- Add `extend` method on AnyBase
- Clippy nits

# 0.7.0-alpha.8
- Add `from_arbitrary_json` to AnyBase

# 0.7.0-alpha.7
- implement Extends for Base

# 0.7.0-alpha.6
- Add Actor and AsApActor impls for ApObject

# 0.7.0-alpha.5
- Add orderedItems field to collections
- Document URL functions from crate root

# 0.7.0-alpha.4
- Clean up unneeded `.into()` calls
- Remove redundant `into_iter` method on OneOrMany.
- Add `new_none_type` constructors to create activitystreams constructs without setting the `type`
    field
- Don't serialize a collection's `items` field if it's `None`
- Rename plural methods
    - `src/actor.rs`: set_streams -> set_stream, add_streams -> add_stream
    - `src/collections.rs`: set_items -> set_item, add_items -> add_item
    - `src/object.rs`: set_replies -> set_reply, add_replies -> add_reply

# 0.7.0-alpha.3
- Add `.into_parts()` for some types where it makes sense
    - All activity subtypes
    - ApActor
    - ApObject

# 0.7.0-alpha.2
- Add `.iter()`, `.iter_mut()` and `.into_iter()` for `OneOrMany<T>`

# 0.7.0-alpha.1
- The `items` methods for collections now deal in options

# 0.7.0-alpha.0
- Complete rewrite from 0.6.x, [check the docs](https://docs.rs/activitystreams)
