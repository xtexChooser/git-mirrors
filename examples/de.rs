use activitystreams::{
    collection::{properties::CollectionProperties, OrderedCollection},
    ext::Ext,
    object::{properties::ApObjectProperties, ObjectBox, Page},
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

    let page: Ext<Page, ApObjectProperties> = serde_json::from_str(page_json)?;
    println!("{:#?}", page);
    let obox = ObjectBox::from_concrete(page)?;
    println!("{:#?}", obox);
    let obox_string = serde_json::to_string(&obox)?;
    println!("{}", obox_string);
    let obox: ObjectBox = serde_json::from_str(&obox_string)?;
    println!("{:#?}", obox);
    let mut collection: OrderedCollection = serde_json::from_str(collection_json)?;
    println!("{:#?}", collection);

    let cprops: &CollectionProperties = collection.as_ref();
    let v: Vec<Ext<Page, ApObjectProperties>> = cprops
        .get_many_items_object_boxs()
        .unwrap()
        .map(|object_box| object_box.clone().to_concrete())
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    let cprops: &mut CollectionProperties = collection.as_mut();
    cprops.set_many_items_object_boxs(v.clone())?;

    println!("{:#?}", v);

    Ok(())
}
