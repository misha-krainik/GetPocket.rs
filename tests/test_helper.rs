use getpocket::GetPocket;
use lazy_static::lazy_static;
use std::{sync::Mutex, thread, time};
use webbrowser;

lazy_static! {
    #[derive(Debug)]
    static ref GETPOCKET_INSTANCE: Mutex<Option<GetPocket>> = Mutex::new(None);
}

use std::{path, fs, env};

pub async fn init_get_pocket() -> GetPocket {
    let consumer_key = env::var("GET_POCKET_CONSUMER_KEY").expect("ENV must be set");
    let redirect_url = env::var("GET_POCKET_REDIRECT_URL").expect("ENV must be set");
    let tmp_dir = std::env::temp_dir().display().to_string();
    let cfg_path = format!("{}/get_pocket_access_token", tmp_dir);
    match path::Path::new(&cfg_path).exists() {
        true => {
            let access_token: String = fs::read_to_string(cfg_path).unwrap();
            GetPocket::new(consumer_key, redirect_url, access_token)
                .await
                .unwrap()
        }
        false => {
            GetPocket::init(
                consumer_key,
                redirect_url,
                |access_token| {
                    fs::write(cfg_path, access_token).unwrap();
                },
                |auth_url| {
                    let ret = webbrowser::open(auth_url).is_ok();

                    let wait_time = time::Duration::from_millis(6000);
                    thread::sleep(wait_time);

                    Ok(ret)
                },
            )
            .await
            .unwrap()
        }
    }
}
