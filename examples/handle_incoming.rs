use activitystreams::{activity::ActorAndObject, prelude::*};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum AcceptedTypes {
    Accept,
    Announce,
    Create,
    Delete,
    Follow,
    Reject,
    Update,
    Undo,
}

pub type AcceptedActivity = ActorAndObject<AcceptedTypes>;

pub fn handle_activity(activity: AcceptedActivity) -> Result<(), anyhow::Error> {
    println!("Actor: {:?}", activity.actor());
    println!("Object: {:?}", activity.object());

    match activity.kind() {
        Some(AcceptedTypes::Accept) => println!("Accept"),
        Some(AcceptedTypes::Announce) => println!("Announce"),
        Some(AcceptedTypes::Create) => println!("Create"),
        Some(AcceptedTypes::Delete) => println!("Delete"),
        Some(AcceptedTypes::Follow) => println!("Follow"),
        Some(AcceptedTypes::Reject) => println!("Reject"),
        Some(AcceptedTypes::Update) => println!("Update"),
        Some(AcceptedTypes::Undo) => println!("Undo"),
        None => return Err(anyhow::Error::msg("No activity type provided")),
    }

    Ok(())
}

static EXAMPLE_JSON: &str = r#"{"actor":"https://asonix.dog/users/asonix","object":"https://asonix.dog/users/asonix/posts/1","type":"Announce"}"#;

fn main() -> Result<(), anyhow::Error> {
    handle_activity(serde_json::from_str(EXAMPLE_JSON)?)
}
