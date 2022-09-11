extern crate getpocket;
use getpocket::GetPocket;

#[tokio::main]
async fn main() {
    let get_pocket: GetPocket = init_get_pocket().await;

    dbg!(get_pocket.list_of_items().await.unwrap());

    // dbg!(&get_pocket.list_of_items_paginate(0, 1).await.unwrap());

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
            .map_err(|e| {
                eprintln!("{:?}", &e);
                e
            })
            .expect("Cannot init GetPocket instance");

            pocket
        }
    };

    get_pocket
}
