extern crate getpocket;

use getpocket::{modifying::ModifyingExt, GetPocket};

#[path = "../tests/test_helper.rs"]
mod lib;

#[tokio::main]
async fn main() {
    let get_pocket: GetPocket = lib::init_get_pocket().await;

    /*
    [
      {
        "action"   : "archive",
        "item_id"  : "229279689",
        "time"     : "1348853312"
      }
    ]
    */

    dbg!(&get_pocket
        .bulk_modify_raw_params("actions=%5B%7B%22action%22%3A%22archive%22%2C%22time%22%3A1348853312%2C%22item_id%22%3A229279689%7D%5D")
        .await
        .unwrap());
}
