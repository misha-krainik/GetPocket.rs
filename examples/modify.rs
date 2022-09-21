extern crate getpocket;
use getpocket::GetPocket;
use webbrowser;
use std::{thread, time};

#[tokio::main]
async fn main() {
    let get_pocket = init_get_pocket().await;

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
