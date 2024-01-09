extern crate getpocket;

use getpocket::{adding::*, modifying::*, GetPocket};

#[path = "../tests/test_helper.rs"]
mod lib;

#[tokio::main]
async fn main() {
    let get_pocket: GetPocket = lib::init_get_pocket().await;
    let resp = get_pocket
        .add_item("https://getpocket.com/developer/docs/v3/add")
        .await
        .unwrap();
    let item_id: i64 = resp
        .item
        .get("item_id")
        .unwrap()
        .as_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let resp_archive = get_pocket.archive(item_id).await;
    assert!(resp_archive.is_ok());
    println!("archive {:#?}", resp_archive);

    let resp_readd = get_pocket.readd(item_id).await;
    assert!(resp_readd.is_ok());
    println!("readd {:#?}", resp_readd);

    let resp_favorite = get_pocket.favorite(item_id).await;
    assert!(resp_favorite.is_ok());
    println!("favorite {:#?}", resp_favorite);

    let resp_unfavorite = get_pocket.unfavorite(item_id).await;
    assert!(resp_unfavorite.is_ok());
    println!("unfavorite {:#?}", resp_unfavorite);

    let resp_delete = get_pocket.delete(item_id).await;
    assert!(resp_delete.is_ok());
    println!("delete {:#?}", resp_delete);
}
