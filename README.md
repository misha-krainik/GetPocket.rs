# GetPocket API crate

Official GetPocket API docs
https://getpocket.com/developer/docs/overview

[![Build](https://github.com/mikhail-krainik/getpocket/actions/workflows/rust.yml/badge.svg)](https://github.com/mikhail-krainik/getpocket/actions/workflows/rust.yml) [![Publish](https://github.com/misha-krainik/GetPocket.rs/actions/workflows/release.yml/badge.svg?branch=master)](https://github.com/misha-krainik/GetPocket.rs/actions/workflows/release.yml)

### Crates.io

https://crates.io/crates/getpocket

### Documentation

https://docs.rs/getpocket/latest/getpocket/

### How to use

```shell
cargo add getpocket
```

```rust
use getpocket::{adding::AddingExt, GetPocket}; 

let get_pocket = GetPocket::new(consumer_key, redirect_url, access_token).await?;
get_pocket.add_item("https://getpocket.com/developer/docs/v3/add").await?;
```

The GetPocket token can be obtained through the built-in `GetPocket::init` method, or you can use the ready-made method for executing requests.

### Context

- [List of items](#list-of-items)
- [Add new item](#add-new-item)
- [Modify items](https://github.com/misha-krainik/GetPocket.rs/blob/master/examples/modify.rs)
- [Using the direct GetPocket API](#using-the-direct-getpocket-api)
- [Run examples](#run-examples)
- [Dependencies](#dependencies)
- [Features](#features)

### List of items

```rust
fn main() {
    let consumer_key = std::env::var("GET_POCKET_CONSUMER_KEY").expect("ENV must be set");
    let redirect_url = std::env::var("GET_POCKET_REDIRECT_URL").expect("ENV must be set");
    let access_token = std::env::var("GET_POCKET_ACCESS_TOKEN").expect("ENV must be set");

    let get_pocket = GetPocket::new(consumer_key, redirect_url, access_token)
        .await
        .expect("Cannot init GetPocket instance");

    let _ = get_pocket.list_of_items().await.unwrap();
}
```

### Response: List of items

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

### Add new item

```rust
fn main() {
    let consumer_key = std::env::var("GET_POCKET_CONSUMER_KEY").expect("ENV must be set");
    let redirect_url = std::env::var("GET_POCKET_REDIRECT_URL").expect("ENV must be set");
    let access_token = std::env::var("GET_POCKET_ACCESS_TOKEN").expect("ENV must be set");

    let get_pocket = GetPocket::new(consumer_key, redirect_url, access_token)
        .await
        .expect("Cannot init GetPocket instance");

    let _ = get_pocket.add_item("https://getpocket.com/developer/docs/v3/add").await.unwrap();
}
```

### Response: Add new item

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

### Access token

``` rust
async fn main() {
    let consumer_key = std::env::var("GET_POCKET_CONSUMER_KEY").expect("ENV must be set");
    let redirect_url = std::env::var("GET_POCKET_REDIRECT_URL").expect("ENV must be set");

    // This is necessary to set one times and save the token
    let pocket = GetPocket::init(consumer_key, redirect_url, |access_token| {
                // ! save ACCESS_TOKEN for next requests
                println!("Access token is {}",  access_token);
            }, |auth_url| {
                // ! redirect the user to Pocket WEB site to authorize this application's request token
                println!("Open in browser next URL {}",  auth_url);
            })
            .await
            .expect("Cannot init GetPocket instance");
}
```

### Possible use case

```rust
extern crate getpocket;
use getpocket::GetPocket;
use webbrowser;
use std::{thread, time};

#[tokio::main]
async fn main() {
    let get_pocket: GetPocket = init_get_pocket().await;

    let _ = get_pocket.list_of_items_with_params(
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
    ).await.unwrap();
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
            let pocket = GetPocket::init(
                consumer_key,
                redirect_url,
                |access_token| {
                    db.put("access_token", access_token).unwrap();
                },
                |auth_url| {
                    let ret = webbrowser::open(auth_url).is_ok();

                    let wait_time = time::Duration::from_millis(6000);
                    thread::sleep(wait_time);

                    Ok(ret)
                },
            )
            .await
            .expect("Cannot init GetPocket instance");

            pocket
        }
    };

    get_pocket
}
```

### Using the direct GetPocket API

Currently, the crate is in an early stage and is actively under development. If you haven't found the method you need, we recommend utilizing the `send` function to send a request to the GetPocket API. This function includes automatic error handling and transfers all necessary tokens for the request. Rest assured, this method will persist in future versions of the crate. You need not worry that new releases will replace or alter it."

```rust
use serde::{Serialize, Deserialize};
use serde_urlencoded;

#[derive(Debug, Serialize, Deserialize)]
struct MyStruct {
    action: String,
    item_id: String,
    time: String,
}

fn main() {
    let my_struct = MyStruct {
        action: String::from("favorite"),
        item_id: String::from("229279689"),
        time: String::from("1348853312"),
    };

    let get_pocket = GetPocket::new(consumer_key, redirect_url, access_token)
        .await
        .expect("Cannot init GetPocket instance");

    let _ = get_pocket
        .send(&[url_encoded_string])
        .await;
}

```

### Response: Archive item

```rust
RecordModified {
    action_results: [
        true,
    ],
    status: 1,
}
```

### Run examples

```shell
cargo run --example list
```

```shell
cargo run --example add
```

```shell
cargo run --example modify
```

### Run tests

```shell
cargo test
```

### Dependencies

* tokio 
* reqwest
* async-trait
* serde 
* serde_json
* serde_qs 
* anyhow
* thiserror

### Features

[Article View](https://getpocket.com/developer/docs/v3/article-view) API and [Preferences](https://getpocket.com/developer/docs/v3/preferences-api) API (WIP)

```toml
[dependencies]
getpocket = { version = "*", features = ["extended"] }
```
