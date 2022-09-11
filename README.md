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

        dbg!(&get_pocket.list_of_items().await.unwrap());
}
```
