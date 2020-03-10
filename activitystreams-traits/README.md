# ActivityStreams Traits
__Traits for Activity Streams__

- [Read the documentation on docs.rs](https://docs.rs/activitystreams-traits)
- [Find the crate on crates.io](https://crates.io/crates/activitystreams-traits)
- [Hit me up on Mastodon](https://asonix.dog/@asonix)

These traits don't provide any functionality other than anotations for types created in other
projects. See the `activitystreams-types` crate for examples of how these traits could be used.

## Examples

Add it to your `Cargo.toml`
```toml
activitystreams-traits = "0.3"
serde = { version = "1.0", features = ["derive"] }
typetag = "0.1.4"
```

Use it in your project
```rust
use activitystreams_traits::{Object, Actor};

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Persona {
    #[serde(rename = "type")]
    kind: String,
}

// TypeTag exists so user-defined types can be deserialized generically in activitystreams-types
#[typetag::serde]
impl Object for Persona {
    fn as_any(&self) -> &(dyn Any + 'static) {
        self
    }

    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) {
        self
    }

    fn duplicate(&self) -> Box<dyn Object + 'static> {
        Box::new(self.clone())
    }
}
impl Actor for Persona {}
```

## Contributing
Feel free to open issues for anything you find an issue with. Please note that any contributed code will be licensed under the GPLv3.

## License

Copyright © 2020 Riley Trautman

ActivityStreams Traits is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

ActivityStreams Traits is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details. This file is part of ActivityStreams Traits.

You should have received a copy of the GNU General Public License along with ActivityStreams Traits. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
