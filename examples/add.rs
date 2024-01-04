extern crate getpocket;

use getpocket::{adding::AddingExt, GetPocket}; 

#[path = "../tests/test_helper.rs"]
mod lib;

#[tokio::main]
async fn main() {
    let get_pocket: GetPocket = lib::init_get_pocket().await;

    dbg!(&get_pocket
        .add_item("https://getpocket.com/developer/docs/v3/add")
        .await
        .unwrap());
}
