use anyhow::Result;
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::{thread, time};
use webbrowser;

//        use std::collections::HashMap;

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

    fn is_error(&self, _code: u32) -> Option<Self> {
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

#[derive(Deserialize, Default, Clone, Debug)]
struct Token {
    code: Option<String>,
    access_token: Option<String>,
}

impl<'a> Token {
    fn new() -> Self {
        Self::default()
    }

    fn set_code(&mut self, code: &str) {
        let code = code.to_string();
        self.code = Some(code);
    }

    fn set_access_token(&mut self, access_token: &str) {
        let access_token = access_token.to_string();
        self.access_token = Some(access_token);
    }

    fn finish(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug)]
struct Reqwester {
    client: reqwest::Client,
}

#[derive(Debug)]
pub struct GetPocket {
    consumer_key: String,
    redirect_uri: String,
    token: Token,
    reqwester: Reqwester,
}

impl GetPocket {
    pub fn new(consumer_key: String, redirect_uri: String) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json; charset=UTF-8"),
        );
        headers.insert(
            "X-Accept",
            header::HeaderValue::from_static("application/json"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let reqwester = Reqwester { client };

        let mut token = Token::new();

        if let Ok(access_token) = std::env::var("GET_POCKET_ACCESS_TOKEN") {
            dbg!("GOT");
            token.set_access_token(&access_token)
        }

        let get_pocket = Self {
            consumer_key,
            redirect_uri,
            reqwester,
            token,
        };

        Ok(get_pocket)
    }

    pub fn save_access_token(&self) -> bool {
        if let Some(ref access_token) = self.token.access_token {
            // Sets the environment variable key for the currently running process !!!
            std::env::set_var("GET_POCKET_ACCESS_TOKEN", access_token);
            true
        } else {
            false
        }
    }

    pub async fn get_access_token(&mut self) -> Result<&mut Self> {
        if self.token.access_token.is_some() {
            dbg!("access_token exists");
            return Ok(self);
        }

        let url = "https://getpocket.com/v3/oauth/request";

        #[derive(Serialize)]
        struct RequestParams<'a> {
            consumer_key: &'a str,
            redirect_uri: &'a str,
        }

        #[derive(Deserialize)]
        struct RequestCode {
            code: String,
        }

        let map = RequestParams {
            consumer_key: &self.consumer_key,
            redirect_uri: &self.redirect_uri,
        };

        let client = &self.reqwester.client;
        let res = client.post(url).json(&map).send().await?;

        // HTTP/1.1 200 OK
        // Content-Type: application/json
        // Status: 200 OK
        //
        // TODO: check into GET_TOKEN_ERROR

        match res.json::<RequestCode>().await {
            Ok(code) => {
                let RequestCode { code } = code;
                if webbrowser::open(&format!("https://getpocket.com/auth/authorize?request_token={code}&redirect_uri=https://getpocket.com")).is_ok() {
                    let wait_time = time::Duration::from_millis(6000);
                    thread::sleep(wait_time);

                    self.token.set_code(&code);
                    self.get_request_access_token().await?;
                }
            }
            Err(err) => Err(err)?,
        }

        Ok(self)
    }

    async fn get_request_access_token(&mut self) -> Result<&mut Self> {
        let url = "https://getpocket.com/v3/oauth/authorize";

        #[derive(Debug, Serialize)]
        struct RequestParams<'a> {
            consumer_key: &'a str,
            code: &'a str,
        }

        let map = match &self.token.code {
            Some(code) => RequestParams {
                consumer_key: &self.consumer_key,
                code,
            },
            None => return Err(anyhow::anyhow!("request_token uninitialized")),
        };

        #[derive(Deserialize)]
        struct RequestAccessToken {
            access_token: String,
        }

        // dbg!(&serde_json::to_string(&map));

        let client = &self.reqwester.client;
        let res = client.post(url).json(&map).send().await?;

        // dbg!(&res.text().await);

        match res.json::<RequestAccessToken>().await {
            Ok(RequestAccessToken { access_token }) =>
                self.token.set_access_token(&access_token),
            Err(err) => Err(err)?,
        }

        Ok(self)
    }

    pub async fn list_of_items(&self) -> Result<serde_json::Value> {
        let url = "https://getpocket.com/v3/get";

        #[derive(Debug, Serialize)]
        struct RequestParams<'a> {
            consumer_key: &'a str,
            access_token: &'a str,
            count: i32,
        }
        let params = match &self.token.access_token {
            Some(access_token) => RequestParams{
                access_token,
                consumer_key: &self.consumer_key,
                count: 10,
            },
            None => return Err(anyhow::anyhow!("No access_token"))
        };

        let client = &self.reqwester.client;
        let res = client.post(url).json(&params).send().await?;

        dbg!(&res.text().await);

        Ok(serde_json::json![{}])
    }
}
