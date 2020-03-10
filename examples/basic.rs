use activitystreams::{
    object::Video,
    primitives::{MimeMediaType, XsdAnyUri, XsdDuration},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut v = Video::default();

    v.as_mut()
        .set_context_xsd_any_uri("https://www.w3.org/ns/activitystreams".parse::<XsdAnyUri>()?)
        .set_id("https://example.com/@example/lions".parse::<XsdAnyUri>()?)
        .set_url_xsd_any_uri("https://example.com/@example/lions/video.webm".parse::<XsdAnyUri>()?)
        .set_name_xsd_string("My Cool Video")
        .set_summary_xsd_string("A video about some cool lions")
        .set_media_type("video/webm".parse::<MimeMediaType>()?)
        .set_duration("PT4M20S".parse::<XsdDuration>()?);

    println!("Video, {:#?}", v);

    let s = serde_json::to_string(&v)?;

    println!("json, {}", s);

    let v: Video = serde_json::from_str(&s)?;

    println!("Video again, {:#?}", v);

    Ok(())
}
