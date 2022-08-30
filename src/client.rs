use anyhow::Result;

struct TokenError {
    http_status: u32,
    x_error_code: u32,
    x_error: &'static str,
}

impl TokenError {
    const fn new(http_status: u32, x_error_code: u32, x_error: &'static str) -> TokenError {
        Self {
            http_status,
            x_error_code,
            x_error,
        }
    }

    fn is_error(&self, code: u32) -> Option<Self> {
       None
    }
}

const GET_TOKEN_ERROR: [TokenError; 7] = [
    TokenError::new(400, 138, "Missing consumer key"),
    TokenError::new(400, 140, "Missing redirect url"),
    TokenError::new(403, 152, "Invalid consumer key"),
    TokenError::new(500, 199, "Pocket server issue"),
    TokenError::new(501, 199, "Pocket server issue"),
    TokenError::new(502, 199, "Pocket server issue"),
    TokenError::new(503, 199, "Pocket server issue"),
];

struct GetPocket {
    consumer_key: String,
    redirect_uri: String,
}

impl GetPocket {
    pub async fn get_token(&self) -> Result<String>{
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert("consumer_key", &self.consumer_key);
        map.insert("redirect_uri", &self.redirect_uri);

        let url = "https://getpocket.com/v3/oauth/request";

        let client = reqwest::Client::new();
        let res = client.post(url).json(&map).send().await?;

        // GET_TOKEN_ERROR

        Ok(String::from(""))
    }
}
