use activitystreams_ext::{Ext1, UnparsedExtension};
use activitystreams::{
    actor::{ApActor, Person},
    context,
    prelude::*,
    primitives::XsdAnyUri,
    security,
    unparsed::UnparsedMutExt,
};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    public_key: PublicKeyInner,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyInner {
    id: XsdAnyUri,
    owner: XsdAnyUri,
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
    let actor = ApActor::new(
        "http://in.box".parse()?,
        "http://out.box".parse()?,
        Person::new(),
    );

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
