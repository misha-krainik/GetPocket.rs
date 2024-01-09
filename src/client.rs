#![allow(dead_code)]
use crate::ApiRequestError;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_qs as qs;
use thiserror::Error;

pub static ENDPOINT: &'static str = "https://getpocket.com/v3/send";

const RATE_LIMIT_HEADERS: [(&str, &str); 6] = [
    ("X-Limit-User-Limit", "Current rate limit enforced per user"),
    (
        "X-Limit-User-Remaining",
        "Number of calls remaining before hitting user's rate limit",
    ),
    (
        "X-Limit-User-Reset",
        "Seconds until user's rate limit resets",
    ),
    (
        "X-Limit-Key-Limit",
        "Current rate limit enforced per consumer key",
    ),
    (
        "X-Limit-Key-Remaining",
        "Number of calls remaining before hitting consumer key's rate limit",
    ),
    (
        "X-Limit-Key-Reset:",
        "Seconds until consumer key rate limit resets",
    ),
];

#[derive(Debug, Error)]
pub enum ClientError<'a> {
    #[error("{0}")]
    JsonError(serde_json::Error),
    #[error("There was an issue with the parameters. `{0}`")]
    InvalidParams(&'a str),
    #[error("Access to this resource is restricted")]
    AccessDenied,
    #[error("Token authentication failed. Please check your token and try again.")]
    TokenError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardResponse {
    pub action_results: Vec<bool>,
    pub action_errors: Vec<Option<String>>,
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedResponse {
    pub action_results: serde_json::Value,
    pub action_errors: Vec<Option<String>>,
    pub status: i32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RecordSendDirect {
    Standart(StandardResponse),
    Extended(ExtendedResponse),
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct Token {
    pub code: Option<String>,
    pub access_token: Option<String>,
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
}

#[derive(Debug, Clone)]
pub struct Reqwester {
    pub client: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct GetPocket {
    pub consumer_key: String,
    pub redirect_uri: String,
    pub token: Token,
    pub reqwester: Reqwester,
}

impl GetPocket {
    pub async fn init<F, C>(
        consumer_key: String,
        redirect_uri: String,
        store_fn: F,
        opener_fn: C,
    ) -> Result<Self>
    where
        F: for<'a> FnOnce(&'a str),
        C: for<'b> FnOnce(&'b str) -> Result<bool>,
    {
        let token = Token::new();

        let reqwester = Self::init_reqwester();

        let mut get_pocket = Self {
            consumer_key,
            redirect_uri,
            reqwester,
            token,
        };

        get_pocket
            .get_access_token_manual_open(opener_fn, None)
            .await?;

        if let Some(ref access_token) = get_pocket.token.access_token {
            store_fn(access_token);

            Ok(get_pocket)
        } else {
            bail!(ClientError::TokenError);
        }
    }

    pub async fn new(
        consumer_key: String,
        redirect_uri: String,
        access_token: String,
    ) -> Result<Self> {
        let mut token = Token::new();

        token.set_access_token(&access_token);

        let reqwester = Self::init_reqwester();

        let get_pocket = Self {
            consumer_key,
            redirect_uri,
            reqwester,
            token,
        };

        Ok(get_pocket)
    }

    pub async fn send<T>(&self, params: T) -> Result<RecordSendDirect>
    where
        T: Serialize,
    {
        #[derive(Serialize)]
        struct RequestParams<'a, T> {
            consumer_key: &'a str,
            access_token: &'a str,
            actions: T,
        }

        let access_token = match &self.token.access_token {
            Some(access_token) => access_token,
            None => bail!(ClientError::TokenError),
        };

        let consumer_key = &self.consumer_key;

        let req_param = RequestParams {
            consumer_key,
            access_token,
            actions: params,
        };

        let urlencoded = qs::to_string(&req_param)?;

        let params = format!("{ENDPOINT}?{urlencoded}");

        let client = &self.reqwester.client;
        let res = client.post(&params).send().await?;

        if let Err(err) = ApiRequestError::handler_status(res.status()) {
            bail!(err);
        }

        let res_body = &res.text().await?;

        let res_ser: Result<RecordSendDirect, serde_json::Error> = serde_json::from_str(&res_body);

        match res_ser {
            Ok(res_ser) => Ok(res_ser),
            Err(err) => Err(ClientError::JsonError(err).into()),
        }
    }

    async fn token_code(&mut self) -> Result<String> {
        if let Some(access_token) = &self.token.access_token {
            return Ok(access_token.clone());
        }

        let endpoint = "https://getpocket.com/v3/oauth/request";

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
        let res = client.post(endpoint).json(&map).send().await?;

        if let Err(err) = ApiRequestError::handler_status(res.status()) {
            bail!(err);
        }

        res.json::<RequestCode>()
            .await
            .map(|request_code| request_code.code)
            .map_err(Into::into)
    }

    async fn get_access_token_manual_open<F>(
        &mut self,
        f: F,
        redirect_uri: Option<&str>,
    ) -> Result<&mut Self>
    where
        F: for<'b> FnOnce(&'b str) -> Result<bool>,
    {
        let code = self.token_code().await?;

        let redirect_uri = match redirect_uri {
            Some(redirect_uri) => redirect_uri,
            None => "https://getpocket.com",
        };

        let is_save = f(&format!(
            "https://getpocket.com/auth/authorize?request_token={code}&redirect_uri={redirect_uri}"
        ))?;

        if is_save {
            self.token.set_code(&code);
            self.get_request_access_token().await?;

            Ok(self)
        } else {
            bail!(ClientError::InvalidParams(
                "No token was provided from the callback function."
            ));
        }
    }

    fn init_reqwester() -> Reqwester {
        use reqwest::header;

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
            .build()
            .unwrap();

        Reqwester { client }
    }

    async fn get_request_access_token(&mut self) -> Result<&mut Self> {
        let endpoint = "https://getpocket.com/v3/oauth/authorize";

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
            None => bail!(ClientError::TokenError),
        };

        #[derive(Deserialize)]
        struct RequestAccessToken {
            access_token: String,
        }

        let client = &self.reqwester.client;
        let res = client.post(endpoint).json(&map).send().await?;

        if let Err(err) = ApiRequestError::handler_status(res.status()) {
            bail!(err);
        }

        match res.json::<RequestAccessToken>().await {
            Ok(RequestAccessToken { access_token }) => self.token.set_access_token(&access_token),
            Err(err) => Err(err)?,
        }

        Ok(self)
    }
}

#[derive(Default)]
pub enum RecordItemState {
    All,
    #[default]
    Unread,
    Archive,
}

#[derive(Default)]
pub enum RecordItemFavorite {
    #[default]
    All,
    Unfavorited,
    Favorited,
}

#[derive(Default)]
pub enum RecordItemTag<'a> {
    #[default]
    All,
    TagName(&'a str),
    Untagged,
}

#[derive(Default)]
pub enum RecordItemContentType {
    #[default]
    All,
    Article,
    Video,
    Image,
}

#[derive(Default)]
pub enum RecordItemSort {
    #[default]
    All,
    Newest,
    Oldest,
    Title,
    Site,
}

#[derive(Default)]
pub enum RecordItemDetailType {
    #[default]
    All,
    Simple,
    Complete,
}
