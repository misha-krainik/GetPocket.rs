### use get_pocket;

###### Status: minimal working version

Rust crate to https://getpocket.com/developer/docs/overview

```
$cargo add getpocket

```

##### List of items

```rust
fn main() {
        let consumer_key = std::env::var("GET_POCKET_CONSUMER_KEY").expect("ENV must be set");
        let redirect_url = std::env::var("GET_POCKET_REDIRECT_URL").expect("ENV must be set");
        let access_token = std::env::var("GET_POCKET_ACCESS_TOKEN").expect("ENV must be set");

        let get_pocket = GetPocket::new(consumer_key, redirect_url, access_token)
                    .await
                    .expect("Cannot init GetPocket instance");

        dbg!(&get_pocket.list_of_items().await.unwrap());
}
```
##### Response: List of items

```rust
RecordItem {
    status: 1,
    complete: Some(
        1,
    ),
    error: None,
    since: 9999999999,
    list: {
        "3302700367": Object {
            "excerpt": String("What is"),
            "favorite": String("0"),
            "given_title": String(""),
            "given_url": String("https://www.site.com/path/"),
            "has_image": String("1"),
            "has_video": String("0"),
            "is_article": String("1"),
            "is_index": String("0"),
            "item_id": String("9999999999"),
            "lang": String("en"),
            "listen_duration_estimate": Number(9999),
            "resolved_id": String("9999999999"),
            "resolved_title": String("Top 25"),
            "resolved_url": String("https://www.site.com/path/"),
            "sort_id": Number(0),
            "status": String("0"),
            "time_added": String("9999999999"),
            "time_favorited": String("0"),
            "time_read": String("0"),
            "time_to_read": Number(99),
            "time_updated": String("9999999999"),
            "top_image_url": String("https://www.site.com/path.jpg"),
            "word_count": String("99"),
        },
    },
}
```
##### Add new item

```rust
fn main() {
        let consumer_key = std::env::var("GET_POCKET_CONSUMER_KEY").expect("ENV must be set");
        let redirect_url = std::env::var("GET_POCKET_REDIRECT_URL").expect("ENV must be set");
        let access_token = std::env::var("GET_POCKET_ACCESS_TOKEN").expect("ENV must be set");

        let get_pocket = GetPocket::new(consumer_key, redirect_url, access_token)
                    .await
                    .expect("Cannot init GetPocket instance");

        dbg!(&get_pocket.add_item("https://getpocket.com/developer/docs/v3/add").await.unwrap());
}
```
##### Response: Add new item

```rust
RecordAdded {
    item: {
        "authors": Array [],
        "content_length": String("9999"),
        "date_published": String("0000-00-00 00:00:00"),
        "date_resolved": String("9999-12-31 23:59:59"),
        "domain_id": String("9999999"),
        "domain_metadata": Object {
            "greyscale_logo": String("https://logo.clearbit.com/getpocket.com?size=800&greyscale=true"),
            "logo": String("https://logo.clearbit.com/getpocket.com?size=800"),
            "name": String("Pocket"),
        },
        "encoding": String("utf-8"),
        "excerpt": String("Allowing users to add articles, videos, images and URLs to Pocket is most likely the first type of integration that you’ll want to build into your application. Adding items to Pocket is easy. In order to use the /v3/add endpoint, your consumer key must have the \"Add\" permission."),
        "extended_item_id": String("999999999"),
        "given_url": String("https://getpocket.com/developer/docs/v3/add"),
        "has_image": String("0"),
        "has_video": String("0"),
        "images": Array [],
        "innerdomain_redirect": String("0"),
        "is_article": String("1"),
        "is_index": String("0"),
        "item_id": String("999999999"),
        "lang": String("en"),
        "login_required": String("0"),
        "mime_type": String("text/html"),
        "normal_url": String("http://getpocket.com/developer/docs/v3/add"),
        "origin_domain_id": String("9999999"),
        "resolved_id": String("999999999"),
        "resolved_normal_url": String("http://getpocket.com/developer/docs/v3/add"),
        "resolved_url": String("https://getpocket.com/developer/docs/v3/add"),
        "response_code": String("200"),
        "time_first_parsed": String("0"),
        "time_to_read": Number(9),
        "title": String("Pocket"),
        "used_fallback": String("0"),
        "videos": Array [],
        "word_count": String("999"),
    },
    status: 1,
}
```
##### ACCESS TOKEN

``` rust
async fn main() {
    let consumer_key = std::env::var("GET_POCKET_CONSUMER_KEY").expect("ENV must be set");
    let redirect_url = std::env::var("GET_POCKET_REDIRECT_URL").expect("ENV must be set");

    let pocket = GetPocket::init(consumer_key, redirect_url, |access_token| {
                // ! save ACCESS_TOKEN for next requests
                println!("{}",  access_token);
            })
            .await
            .expect("Cannot init GetPocket instance");
}
```
##### FROM EXAMPLES

```rust
extern crate getpocket;
use getpocket::GetPocket;

#[tokio::main]
async fn main() {
    let get_pocket: GetPocket = init_get_pocket().await;

    dbg!(&get_pocket.list_of_items_with_params(
        RecordItemState::All,
        RecordItemFavorite::All,
        RecordItemTag::All,
        RecordItemContentType::All,
        RecordItemSort::All,
        RecordItemDetailType::All,
        None,
        None,
        None,
        0,
        25,
    ).await.unwrap());
}

async fn init_get_pocket() -> GetPocket {
    let consumer_key = std::env::var("GET_POCKET_CONSUMER_KEY").expect("ENV must be set");
    let redirect_url = std::env::var("GET_POCKET_REDIRECT_URL").expect("ENV must be set");
    let mut current_path = std::env::current_exe().unwrap();
    current_path.pop();

    let db = rocksdb::DB::open_default(&format!("{}/database", current_path.display())).unwrap();

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
            .expect("Cannot init GetPocket instance");

            pocket
        }
    };

    get_pocket
}
```
#### ARCHIVE ITEM

```rust

#[tokio::main]
async fn main() {
    let get_pocket = init_get_pocket().await;

    dbg!(&get_pocket
        .bulk_modify_raw_params("actions=%5B%7B%22action%22%3A%22archive%22%2C%22time%22%3A1348853312%2C%22item_id%22%3A229279689%7D%5D")
        .await
        .unwrap());
}
```
##### Response: archive item

```rust
RecordModified {
    action_results: [
        true,
    ],
    status: 1,
}
```
##### RUN OTHER EXAMPLES

```
$ cargo run --example list
```
