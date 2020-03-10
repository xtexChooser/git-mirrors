# ActivityStreams
__A set of Traits and Types that make up the Activity Streams specification__

- [Read the documentation on docs.rs](https://docs.rs/activitystreams)
- [Find the crate on crates.io](https://crates.io/crates/activitystreams)
- [Join the discussion on Matrix](https://matrix.to/#/!fAEcHyTUdAaKCzIKCt:asonix.dog?via=asonix.dog)

## Usage

### Basic usage
For basic use, add the following to your Cargo.toml
```toml
# Cargo.toml

activitystreams = "0.4"
```

And then use it in your project
```rust
use activitystreams::object::Video;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut v = Video::default();

    v.as_mut()
        .set_context_xsd_any_uri("https://www.w3.org/ns/activitystreams")?
        .set_id("https://example.com/@example/lions")?
        .set_url_xsd_any_uri("https://example.com/@example/lions/video.webm")?
        .set_name_xsd_string("My Cool Video")?
        .set_summary_xsd_string("A video about some cool lions")?
        .set_media_type("video/webm")?
        .set_duration("PT4M20S")?;

    println!("Video, {:#?}", v);

    let s = serde_json::to_string(&v)?;

    println!("json, {}", s);

    let v: Video = serde_json::from_str(&s)?;

    println!("Video again, {:#?}", v);

    Ok(())
}
```

### Intermediate Usage

```rust
use activitystreams::{
    context,
    object::{
        properties::{
            ObjectProperties,
            ProfileProperties
        },
        Profile,
    },
    primitives::XsdAnyUri,
    Actor,
    Object,
};
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Persona {
    #[serde(rename = "@context")]
    context: XsdAnyUri,

    #[serde(rename = "type")]
    kind: String,
}

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

fn main() -> Result<(), anyhow::Error> {
    let mut profile = Profile::default();

    let pprops: &mut ProfileProperties = profile.as_mut();

    pprops.set_describes_object_box(Persona {
        context: context(),
        kind: "Persona".to_owned(),
    })?;

    let oprops: &mut ObjectProperties = profile.as_mut();
    oprops.set_context_xsd_any_uri(context())?;

    let profile_string = serde_json::to_string(&profile)?;

    let profile: Profile = serde_json::from_str(&profile_string)?;

    Ok(())
}
```

### Advanced Usage
Add the required crates to your `Cargo.toml`
```toml
# Cargo.toml

activitystreams = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

And then in your project
```rust
use activitystreams::{
    context,
    link::{
        properties::LinkProperties,
        LinkExt,
    },
    Link,
    Object,
    PropRefs,
    UnitString
};

/// Using the UnitString derive macro
///
/// This macro implements Serialize and Deserialize for the given type, making this type
/// represent the string "SomeKind" in JSON.
#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(MyLink)]
pub struct MyKind;

properties! {
    MyLink {
        docs [ "Document MyLinkProperties" ],

        required_key {
            docs [ "Document the required key" ],
            types [ String ],
            functional,
            required,
        }
    }
}

/// Using the Properties derive macro
///
/// This macro generates getters and setters for the associated fields.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, PropRefs)]
#[serde(rename_all = "camelCase")]
pub struct MyLink {
    /// Use the UnitString MyKind to enforce the type of the object by "SomeKind"
    pub kind: MyKind,

    #[activitystreams(Link)]
    pub link_props: LinkProperties,

    #[activitystreams(None)]
    pub my_link_props: MyLinkProperties,
}

fn run() -> Result<(), anyhow::Error> {
    let mut my_link = MyLink::default();

    let mprops: &mut MyLinkProperties = my_link.as_mut();
    mprops.set_required_key("hey")?;

    let lprops: &mut LinkProperties = my_link.as_mut();
    lprops.set_context_xsd_any_uri(context)?;

    let my_link_string = serde_json::to_string(&my_link)?;

    let my_link: MyLink = serde_json::from_str(&my_link_string)?;
    let mprops: &MyLinkProperties = my_link.as_ref();

    println!("{}", mprops.get_required_key());

    Ok(())
}
```

## Contributing
Feel free to open issues for anything you find an issue with. Please note that any contributed code will be licensed under the GPLv3.

## License

Copyright © 2018 Riley Trautman

ActivityStreams is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

ActivityStreams is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details. This file is part of ActivityStreams.

You should have received a copy of the GNU General Public License along with ActivityStreams. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
