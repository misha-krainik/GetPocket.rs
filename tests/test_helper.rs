use getpocket::GetPocket;
use lazy_static::lazy_static;
use std::{sync::Mutex, thread, time};
use webbrowser;

lazy_static! {
    #[derive(Debug)]
    static ref GETPOCKET_INSTANCE: Mutex<Option<GetPocket>> = Mutex::new(None);
}

pub async fn init_get_pocket() -> GetPocket {
    let instance_lock = GETPOCKET_INSTANCE.try_lock();
    if let Ok(ref instance) = instance_lock {
        if let Some(instance) = instance.as_ref() {
            return instance.clone();
        }
    }

    let get_pocket = init_get_pocket_with_dbpath(None).await;
    let get_pocket_clone = get_pocket.clone();
    std::thread::spawn(move || {
        let mut instance = GETPOCKET_INSTANCE.lock().unwrap();
        *instance = Some(get_pocket_clone);
    });
    get_pocket
}

pub async fn init_get_pocket_with_dbpath(current_path: Option<&str>) -> GetPocket {
    let consumer_key = std::env::var("GET_POCKET_CONSUMER_KEY").expect("ENV must be set");
    let redirect_url = std::env::var("GET_POCKET_REDIRECT_URL").expect("ENV must be set");
    let current_path = current_path
        .map(|p| p.to_string())
        .unwrap_or_else(|| std::env::temp_dir().display().to_string());

    let db = rocksdb::DB::open_default(&format!("{}/database", current_path)).unwrap();
    
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
