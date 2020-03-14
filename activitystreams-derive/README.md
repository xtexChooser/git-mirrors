# ActivityStreams Derive
__derive macros for ActivityStreams__

- [Read the documentation on docs.rs](https://docs.rs/activitystreams-derive)
- [Find the crate on crates.io](https://crates.io/crates/activitystreams-derive)
- [Hit me up on Mastodon](https://asonix.dog/@asonix)

## Usage
Add the required crates to your `Cargo.toml`
```toml
# Cargo.toml

activitystreams = "0.5.0-alpha.3"
serde = { version = "1.0", features = ["derive"] }
```

And then in your project
```rust
// derive macros
use activitystreams::{
    properties,
    PropRefs,
    UnitString
};
// traits
use activitystreams::Object;
// properties
use activitystreams::object::properties::ObjectProperties;

/// Using the UnitString derive macro
///
/// This macro implements Serialize and Deserialize for the given type, making this type
/// represent the string "SomeKind" in JSON.
#[derive(Clone, Debug, Default, UnitString)]
#[unit_string(SomeKind)]
pub struct MyKind;

properties! {
    My {
        docs [
            "Using the properties macro",
            "",
            "This macro generates getters and setters for the associated fields.",
        ],

        kind {
            docs [
                "Use the UnitString MyKind to enforce the type of the object by \"SomeKind\"",
                "",
                "Rename to/from 'type' when serializing/deserializing",
            ],

            types [
                MyKind,
            ],
            functional,
            required,
            rename("type"),
        },

        required_key {
            docs [
                "Derive getters and setters for required_key with String type.",
                "",
                "In the Activity Streams spec, 'functional' means there can only be one item for",
                "this key. This means all fields not labeled 'functional' can also be",
                "serialized/deserialized as Vec<T>.",
                "",
                "'required' here means that the field must be present, otherwise, it's"
                "represented as an Option<T>",
            ],
            types [
                String,
            ],
            functional,
            required,
        },
    }
}

#[derive(Clone, Default, PropRefs, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[prop_refs(Object)]
pub struct My {
    /// Derive AsRef<MyProperties> and AsMut<MyProperties>
    #[serde(flatten)]
    #[prop_refs]
    my_properties: MyProperties,

    /// Derive AsRef<ObjectProperties> and AsMut<ObjectProperties>
    ///
    /// as well as the Object trait
    #[serde(flatten)]
    #[prop_refs]
    properties: ObjectProperties,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut my = My::default();

    let mprops: &mut MyProperties = my.as_mut();
    mprops.set_required_key("Hello")?;

    let mprops: &MyProperties = my.as_ref();
    assert_eq!(mprops.get_required_key(), "Hello");
    Ok(())
}
```

## Contributing
Feel free to open issues for anything you find an issue with. Please note that any contributed code will be licensed under the GPLv3.

## License

Copyright © 2020 Riley Trautman

ActivityStreams Derive is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

ActivityStreams Derive is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details. This file is part of ActivityStreams Derive.

You should have received a copy of the GNU General Public License along with ActivityStreams Derive. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
