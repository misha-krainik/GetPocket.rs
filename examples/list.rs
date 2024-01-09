extern crate getpocket;

use getpocket::{retrieving::RetrievingExt, GetPocket};

#[path = "../tests/test_helper.rs"]
mod lib;

#[tokio::main]
async fn main() {
    let get_pocket: GetPocket = lib::init_get_pocket().await;

    dbg!(get_pocket.list_of_items().await.unwrap());
}
