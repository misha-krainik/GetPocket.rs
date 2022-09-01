use anyhow::Result;
use reqwest::header;
use serde::{Deserialize, Serialize};
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
    request_token: Option<String>,
    access_token: Option<String>,
}

impl<'a> Token {
    fn new() -> Self {
        Self::default()
    }

    fn set_request_token(&mut self, request_token: &str) {
        let request_token = request_token.to_string();
        self.request_token = Some(request_token);
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

        let get_pocket = Self {
            consumer_key,
            redirect_uri,
            reqwester,
            token: Token::default(),
        };

        Ok(get_pocket)
    }

    pub async fn get_request_token(&mut self) -> Result<&mut Self> {
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
                let mut token = Token::new();
                let RequestCode { code } = code;
                token.set_request_token(&code);
                self.token = token.finish();
            }
            Err(err) => Err(err)?,
        }

        Ok(self)
    }

    pub async fn get_access_token(&mut self) -> Result<&mut Self> {
        let url = "https://getpocket.com/v3/oauth/authorize";

        #[derive(Serialize)]
        struct RequestParams<'a> {
            consumer_key: &'a str,
            code: &'a str,
        }

        let map = match &self.token.request_token {
            Some(code) => RequestParams {
                consumer_key: &self.consumer_key,
                code,
            },
            None => return Err(anyhow::anyhow!("request_token uninitialized")),
        };

        let client = &self.reqwester.client;
        let res = client.post(url).json(&map).send().await?;

        dbg!(&res.text().await);

        Ok(self)
    }
}
