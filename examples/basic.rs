use activitystreams::{
    context,
    object::{ApObject, Video},
    prelude::*,
    uri,
};
use chrono::Duration;

fn main() -> Result<(), anyhow::Error> {
    let mut video = ApObject::new(Video::new());

    video
        .set_context(context())
        .set_id(uri!("https://example.com/@example/lions"))
        .set_media_type("video/webm".parse()?)
        .set_url(uri!("https://example.com/@example/lions/video.webm"))
        .set_summary("A cool video".to_owned())
        .set_duration(Duration::minutes(4) + Duration::seconds(20))
        .set_shares(uri!("https://example.com/@example/lions/video.webm#shares"));

    println!("Video, {:#?}", video);

    let s = serde_json::to_string(&video)?;

    println!("json, {}", s);

    let v: ApObject<Video> = serde_json::from_str(&s)?;

    println!("Video again, {:#?}", v);

    Ok(())
}
