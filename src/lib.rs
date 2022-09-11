// GET_POCKET_CONSUMER_KEY

/* #[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
mod client;

#[cfg(test)]
mod tests {
    use crate::client::GetPocket;

    #[tokio::test]
    async fn it_works() {
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

        // dbg!(&get_pocket.list_of_items_paginate(0, 10).await.unwrap());

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

        assert!(false);
    }

    fn pwd() -> anyhow::Result<String> {
        let mut path = std::env::current_exe()?;
        path.pop();

        Ok(path.to_string_lossy().to_string())
    }
}

/*
POST /v3/add HTTP/1.1
Host: getpocket.com
Content-Type: application/json; charset=UTF-8
X-Accept: application/json

{"url":"http:\/\/pocket.co\/s8Kga",
"title":"iTeaching: The New Pedagogy (How the iPad is Inspiring Better Ways of
Teaching)",
"time":1346976937,
"consumer_key":"1234-abcd1234abcd1234abcd1234",
"access_token":"5678defg-5678-defg-5678-defg56"}
*/
