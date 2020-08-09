# Unreleased
- Rename plural methods
    - src/actor.rs set_streams -> set_stream, add_streams -> add_stream
    - src/collections.rs set_items -> set_item, add_items -> add_item
    - src/object.rs set_replies -> set_reply, add_replies -> add_reply

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
