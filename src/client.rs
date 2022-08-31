use anyhow::Result;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Debug)]
struct Token {
    code: String,
}

#[derive(Serialize)]
struct GetTokenRequest<'a> {
    consumer_key: &'a str,
    redirect_uri: &'a str,
}

// #[derive(Serialize)]
// struct GetPocketRequstHeaders {
//     #[serde(rename(serialize = "Content-Type"))]
//     content_type: String,
//     #[serde(rename(serialize = "X-Accept"))]
//     x_accept: String,
// }

// impl Default for GetPocketRequstHeaders {
//     fn default() -> Self {
//         let content_type = "";
//         let x_accept = "";

//         GetPocketRequstHeaders {
//             content_type,
//             x_accept,
//         }
//     }
// }

#[derive(Debug)]
pub struct GetPocket {
    consumer_key: String,
    redirect_uri: String,
    token: Option<Token>,
}

impl GetPocket {
    pub fn new(consumer_key: String, redirect_uri: String) -> Self {
        Self {
            consumer_key,
            redirect_uri,
            token: None,
        }
    }

    pub async fn get_token(&mut self) -> Result<String> {
        use reqwest::header;
        use std::collections::HashMap;

        let mut map = GetTokenRequest {
            consumer_key: &self.consumer_key,
            redirect_uri: &self.redirect_uri,
        };

        let url = "https://getpocket.com/v3/oauth/request";

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

        let res = client.post(url).json(&map).send().await?;

        match res.json::<Token>().await {
            Ok(code) => self.token = Some(code),
            Err(err) => Err(err)?,
        }

        dbg!(&self);

        Ok(String::from(""))
    }
}
