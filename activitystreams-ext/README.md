# ActivityStreams Ext
_This crate provides Ext1, Ext2, Ext3, and Ext4 for adding extensions to ActivityStreams types_

- Find the code on [git.asonix.dog](https://git.asonix.dog/Aardwolf/activitystreams)
- Read the docs on [docs.rs](https://docs.rs/activitystreams-ext)
- Join the matrix channel at [#activitypub:asonix.dog](https://matrix.to/#/!fAEcHyTUdAaKCzIKCt:asonix.dog?via=asonix.dog&via=matrix.org&via=t2bot.io)
- Hit me up on [mastodon](https://asonix.dog/@asonix)

## Usage

First, add ActivityStreams to your dependencies
```toml
[dependencies]
activitystreams = "0.7.0-alpha.3"
activitystreams-ext = "0.1.0-alpha.2"
```

For an example, we'll implement a PublicKey extension and demonstrate usage with Ext1
```rust
use activitystreams::{
    actor::{ApActor, Person},
    context,
    prelude::*,
    security,
    unparsed::UnparsedMutExt,
    url::Url,
};
use activitystreams_ext::{Ext1, UnparsedExtension};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    public_key: PublicKeyInner,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyInner {
    id: Url,
    owner: Url,
    public_key_pem: String,
}

impl<U> UnparsedExtension<U> for PublicKey
where
    U: UnparsedMutExt,
{
    type Error = serde_json::Error;

    fn try_from_unparsed(unparsed_mut: &mut U) -> Result<Self, Self::Error> {
        Ok(PublicKey {
            public_key: unparsed_mut.remove("publicKey")?,
        })
    }

    fn try_into_unparsed(self, unparsed_mut: &mut U) -> Result<(), Self::Error> {
        unparsed_mut.insert("publicKey", self.public_key)?;
        Ok(())
    }
}

pub type ExtendedPerson = Ext1<ApActor<Person>, PublicKey>;

fn main() -> Result<(), anyhow::Error> {
    let actor = ApActor::new("http://in.box".parse()?, Person::new());

    let mut person = Ext1::new(
        actor,
        PublicKey {
            public_key: PublicKeyInner {
                id: "http://key.id".parse()?,
                owner: "http://owner.id".parse()?,
                public_key_pem: "asdfasdfasdf".to_owned(),
            },
        },
    );

    person.set_context(context()).add_context(security());

    let any_base = person.into_any_base()?;
    println!("any_base: {:#?}", any_base);
    let person = ExtendedPerson::from_any_base(any_base)?;

    println!("person: {:#?}", person);
    Ok(())
}
```

## Contributing
Feel free to open issues for anything you find an issue with. Please note that any contributed code will be licensed under the GPLv3.

## License

Copyright © 2020 Riley Trautman

ActivityStreams is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

ActivityStreams is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details. This file is part of ActivityStreams.

You should have received a copy of the GNU General Public License along with ActivityStreams. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
