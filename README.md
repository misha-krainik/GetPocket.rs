```rust
fn main() {
        let consumer_key = std::env::var("GET_POCKET_CONSUMER_KEY").expect("ENV must be set");
        let redirect_url = std::env::var("GET_POCKET_REDIRECT_URL").expect("ENV must be set");
        let db = rocksdb::DB::open_default(&format!("{}/database", pwd().unwrap())).unwrap();

        let get_pocket = match db.get("access_token").unwrap() {
            Some(access_token) => {
                let access_token = String::from_utf8(access_token).unwrap();
                let pocket = GetPocket::new(consumer_key, redirect_url, access_token)
                    .await
                    .expect("Cannot init GetPocket instance");
                pocket
            }
            None => {
                let pocket = GetPocket::init(consumer_key, redirect_url, |access_token| {
                    db.put("access_token", access_token).unwrap();
                })
                .await
                .map_err(|e| {
                    eprintln!("{:?}", &e);
                    e
                })
                .expect("Cannot init GetPocket instance");

                pocket
            }
        };

        // dbg!(&get_pocket.list_of_items_paginate(0, 10).await.unwrap());

        // dbg!(&get_pocket.list_of_items_with_params(
        //     RecordItemState::All,
        //     RecordItemFavorite::All,
        //     RecordItemTag::All,
        //     RecordItemContentType::All,
        //     RecordItemSort::All,
        //     RecordItemDetailType::All,
        //     None,
        //     None,
        //     None,
        //     0,
        //     25,
        // ).await.unwrap());

        dbg!(&get_pocket.list_of_items().await.unwrap());
}
```

```rust
// RecordItem {
//     status: 1,
//     complete: Some(
//         1,
//     ),
//     error: None,
//     since: 9999999999,
//     list: {
//         "3302700367": Object {
//             "excerpt": String("What is"),
//             "favorite": String("0"),
//             "given_title": String(""),
//             "given_url": String("https://www.site.com/path/"),
//             "has_image": String("1"),
//             "has_video": String("0"),
//             "is_article": String("1"),
//             "is_index": String("0"),
//             "item_id": String("9999999999"),
//             "lang": String("en"),
//             "listen_duration_estimate": Number(9999),
//             "resolved_id": String("9999999999"),
//             "resolved_title": String("Top 25"),
//             "resolved_url": String("https://www.site.com/path/"),
//             "sort_id": Number(0),
//             "status": String("0"),
//             "time_added": String("9999999999"),
//             "time_favorited": String("0"),
//             "time_read": String("0"),
//             "time_to_read": Number(99),
//             "time_updated": String("9999999999"),
//             "top_image_url": String("https://www.site.com/path.jpg"),
//             "word_count": String("99"),
//         },
//     },
// }
```
