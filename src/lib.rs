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

        let mut get_pocket = GetPocket::new(consumer_key, redirect_url);
        let r = get_pocket.get_token().await;

        assert!(false);

        // assert!(r.is_ok());
    }
}
