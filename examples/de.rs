use activitystreams::{
    collection::apub::OrderedCollection,
    object::{streams::Page, ObjectBox},
};
use anyhow::Error;

fn main() -> Result<(), Error> {
    let collection_json = r#"{
      "type": "OrderedCollection",
      "id": "http://lemmy_alpha:8540/federation/c/main",
      "context": "https://www.w3.org/ns/activitystreams",
      "items": [
        {
          "type": "Page",
          "id": "http://lemmy_alpha:8540/federation/post/2",
          "attributedTo": "http://lemmy_alpha:8540/federation/u/2",
          "content": "test",
          "context": "https://www.w3.org/ns/activitystreams",
          "name": "test",
          "published": "2020-03-13T00:14:41.188634+00:00"
        },
        {
          "type": "Page",
          "id": "http://lemmy_alpha:8540/federation/post/1",
          "attributedTo": "http://lemmy_alpha:8540/federation/u/2",
          "context": "https://www.w3.org/ns/activitystreams",
          "name": "test",
          "published": "2020-03-13T00:13:56.311479+00:00"
        }
      ],
      "totalItems": 2
    }"#;

    let page_json = r#"{
      "type": "Page",
      "id": "http://lemmy_alpha:8540/federation/post/2",
      "attributedTo": "http://lemmy_alpha:8540/federation/u/2",
      "content": "test",
      "name": "test",
      "published": "2020-03-13T00:14:41.188634+00:00"
    }"#;

    let page: Page = serde_json::from_str(page_json)?;
    println!("{:#?}", page);
    let obox: ObjectBox = page.into();
    println!("{:#?}", obox);
    let obox_string = serde_json::to_string(&obox)?;
    println!("{}", obox_string);
    let obox: ObjectBox = serde_json::from_str(&obox_string)?;
    println!("{:#?}", obox);
    let collection: OrderedCollection = serde_json::from_str(collection_json)?;
    println!("{:#?}", collection);

    Ok(())
}
