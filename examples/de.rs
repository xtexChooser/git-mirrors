use activitystreams::{
    collection::OrderedCollection,
    object::{ApObject, Page},
    prelude::*,
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

    let page: ApObject<Page> = serde_json::from_str(page_json)?;
    println!("{:#?}", page);
    let mut collection: ApObject<OrderedCollection> = serde_json::from_str(collection_json)?;
    println!("{:#?}", collection);

    let v: Vec<ApObject<Page>> = collection
        .items()
        .clone()
        .many()
        .into_iter()
        .flatten()
        .filter_map(|any_base| any_base.take_base())
        .map(|base| base.solidify().and_then(|o| o.extend()))
        .collect::<Result<Vec<_>, _>>()?;

    println!("{:#?}", v);
    let v = v
        .into_iter()
        .map(|o| o.into_any_base())
        .collect::<Result<Vec<_>, _>>()?;

    collection.set_many_items(v);

    Ok(())
}
