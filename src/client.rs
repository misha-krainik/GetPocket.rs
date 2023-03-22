#![allow(dead_code)]
use anyhow::{bail, format_err, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;

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

#[derive(Debug)]
pub struct Reqwester {
    pub client: reqwest::Client,
}

#[derive(Debug)]
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

        get_pocket.get_access_token_manual_open(opener_fn).await?;

        if let Some(ref access_token) = get_pocket.token.access_token {
            store_fn(access_token);

            Ok(get_pocket)
        } else {
            Err(anyhow::format_err!(""))
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

        // HTTP/1.1 200 OK
        // Content-Type: application/json
        // Status: 200 OK
        //
        // TODO: check into GET_TOKEN_ERROR

        res.json::<RequestCode>()
            .await
            .map(|request_code| request_code.code)
            .map_err(Into::into)
    }

    async fn get_access_token_manual_open<F>(&mut self, f: F) -> Result<&mut Self>
    where
        F: for<'b> FnOnce(&'b str) -> Result<bool>,
    {
        let code = self.token_code().await?;

        let is_save = f(&format!("https://getpocket.com/auth/authorize?request_token={code}&redirect_uri=https://getpocket.com"))?;

        if is_save {
            self.token.set_code(&code);
            self.get_request_access_token().await?;

            Ok(self)
        } else {
            Err(format_err!("Request toke not accepted"))
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
            None => return Err(anyhow::anyhow!("request_token uninitialized")),
        };

        #[derive(Deserialize)]
        struct RequestAccessToken {
            access_token: String,
        }

        let client = &self.reqwester.client;
        let res = client.post(endpoint).json(&map).send().await?;

        match res.json::<RequestAccessToken>().await {
            Ok(RequestAccessToken { access_token }) => self.token.set_access_token(&access_token),
            Err(err) => Err(err)?,
        }

        Ok(self)
    }

    pub async fn add_item_with_params<'a>(
        &self,
        url: &'a str,
        title: Option<&'a str>,
        tags: Option<&[&'a str]>,
        tweet_id: Option<&'a str>,
    ) -> Result<RecordAdded> {
        let endpoint = "https://getpocket.com/v3/add";

        #[derive(Serialize)]
        struct RequestParams<'a> {
            consumer_key: &'a str,
            access_token: &'a str,
            url: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tags: Option<&'a [&'a str]>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tweet_id: Option<&'a str>,
        }

        let params = match &self.token.access_token {
            Some(access_token) => RequestParams {
                consumer_key: &self.consumer_key,
                access_token,
                url,
                title,
                tags,
                tweet_id,
            },
            None => bail!("No access_token"),
        };

        let client = &self.reqwester.client;
        let res = client.post(endpoint).json(&params).send().await?;
        let res_body = &res.text().await?;

        let res_ser: RecordAdded = serde_json::from_str(&res_body).map_err(|e| format_err!(e))?;

        Ok(res_ser)
    }

    /// adding a single item
    pub async fn add_item<'a>(&self, url: &'a str) -> Result<RecordAdded> {
        self.add_item_with_params(url, None, None, None).await
    }

    /// https://getpocket.com/developer/docs/v3/modify
    pub async fn bulk_modify_raw_params<'a>(&self, params: &'a str) -> Result<RecordModified> {
        let endpoint = "https://getpocket.com/v3/send";

        #[derive(Serialize)]
        struct RequestParams<'a> {
            consumer_key: &'a str,
            access_token: &'a str,
        }

        let access_token = match &self.token.access_token {
            Some(access_token) => access_token,
            None => bail!("No access_token"),
        };

        let consumer_key = &self.consumer_key;

        let params =
            format!("{endpoint}?{params}&access_token={access_token}&consumer_key={consumer_key}");

        let client = &self.reqwester.client;
        let res = client.post(&params).send().await?;
        let res_body = &res.text().await?;

        let res_ser: RecordModified =
            serde_json::from_str(&res_body).map_err(|e| format_err!(e))?;

        Ok(res_ser)
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_add(&self, _params: &[BulkRecAdd]) -> Result<BulkRecAdded> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_archive(&self, _params: &[BulkRecArchive]) -> Result<BulkRecArchived> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_readd(&self, _params: &[BulkRecReadd]) -> Result<BulkRecReadded> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_favorite(&self, _params: &[BulkRecFavorite]) -> Result<BulkRecFavorited> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_unfavorite(
        &self,
        _params: &[BulkRecUnfovorite],
    ) -> Result<BulkRecUnfovorited> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_delete(&self, _params: &[BulkRecDelete]) -> Result<BulkRecDeleted> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_tags_add(&self, _params: &[BulkTagsAdd]) -> Result<BulkTagsAdded> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_tags_remove(&self, _params: &[BulkTagsRemove]) -> Result<BulkTagsRemoved> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_tags_replace(&self, _params: &[BulkTagsReplace]) -> Result<BulkTagsReplaced> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_tags_clear(&self, _params: &[BulkTagsClear]) -> Result<BulkTagsCleared> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_tag_rename(&self, _params: &[BulkTagsRename]) -> Result<BulkTagsRenamed> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_tag_delete(&self, _params: &[BulkTagsDelete]) -> Result<BulkTagsDeleted> {
        unimplemented!()
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

#[derive(Debug, Deserialize)]
pub struct RecordItem {
    pub status: i32,
    #[serde(default)]
    pub complete: Option<i32>,
    pub error: Option<String>,
    pub since: i32,
    pub list: Map<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct RecordAdded {
    pub item: Map<String, serde_json::Value>,
    pub status: i32,
}

#[derive(Debug, Deserialize)]
pub struct RecordModified {
    pub action_results: Vec<bool>,
    pub status: i32,
}

#[cfg(feature = "unstable")]
pub struct BulkRecAdd {
    /// The id of the item to perform the action on.
    item_id: i32,
    /// A Twitter status id; this is used to show tweet attribution.
    ref_id: i32,
    /// A comma-delimited list of one or more tags.
    tags: Option<String>,
    /// The time the action occurred. Unix epoch in milliseconds
    time: Option<i32>,
    /// The title of the item.
    title: Option<String>,
    /// The url of the item; provide this only if you do not have an item_id.
    url: Option<String>,
}

#[cfg(feature = "unstable")]
pub struct BulkRecArchive;

#[cfg(feature = "unstable")]
pub struct BulkRecReadd;

#[cfg(feature = "unstable")]
pub struct BulkRecFavorite;

#[cfg(feature = "unstable")]
pub struct BulkRecUnfovorite;

#[cfg(feature = "unstable")]
pub struct BulkRecDelete;

#[cfg(feature = "unstable")]
pub struct BulkTagsAdd;

#[cfg(feature = "unstable")]
pub struct BulkTagsRemove;

#[cfg(feature = "unstable")]
pub struct BulkTagsReplace;

#[cfg(feature = "unstable")]
pub struct BulkTagsClear;

#[cfg(feature = "unstable")]
pub struct BulkTagsRename;

#[cfg(feature = "unstable")]
pub struct BulkTagsDelete;

#[cfg(feature = "unstable")]
pub struct BulkRecAdded;

#[cfg(feature = "unstable")]
pub struct BulkRecArchived;

#[cfg(feature = "unstable")]
pub struct BulkRecReadded;

#[cfg(feature = "unstable")]
pub struct BulkRecFavorited;

#[cfg(feature = "unstable")]
pub struct BulkRecUnfovorited;

#[cfg(feature = "unstable")]
pub struct BulkRecDeleted;

#[cfg(feature = "unstable")]
pub struct BulkTagsAdded;

#[cfg(feature = "unstable")]
pub struct BulkTagsRemoved;

#[cfg(feature = "unstable")]
pub struct BulkTagsReplaced;

#[cfg(feature = "unstable")]
pub struct BulkTagsCleared;

#[cfg(feature = "unstable")]
pub struct BulkTagsRenamed;

#[cfg(feature = "unstable")]
pub struct BulkTagsDeleted;