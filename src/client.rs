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

    pub async fn list_of_items_with_params<'a>(
        &self,
        state: RecordItemState,
        favorite: RecordItemFavorite,
        tag: RecordItemTag<'a>,
        content_type: RecordItemContentType,
        sort: RecordItemSort,
        detail_type: RecordItemDetailType,
        search: Option<&'a str>,
        domain: Option<&'a str>,
        since: Option<&i32>,
        offset: i32,
        count: i32,
    ) -> Result<RecordItem> {
        let endpoint = "https://getpocket.com/v3/get";

        #[derive(Serialize)]
        struct RequestParams<'a> {
            consumer_key: &'a str,
            access_token: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            state: Option<&'a str>, // ItemState,
            #[serde(skip_serializing_if = "Option::is_none")]
            favorite: Option<i32>, // RecordItemfavorite,
            #[serde(skip_serializing_if = "Option::is_none")]
            tag: Option<&'a str>, // RecordItemTag,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "contentType")]
            content_type: Option<&'a str>, // RecordItemContentType,
            #[serde(skip_serializing_if = "Option::is_none")]
            sort: Option<&'a str>, // RecordItemSort,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "detailType")]
            detail_type: Option<&'a str>, // RecordItemDetailType,
            #[serde(skip_serializing_if = "Option::is_none")]
            search: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            domain: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            since: Option<&'a i32>,
            offset: i32,
            count: i32,
        }
        let params = match &self.token.access_token {
            Some(access_token) => RequestParams {
                consumer_key: &self.consumer_key,
                access_token,
                state: match state {
                    RecordItemState::All => Some("all"),
                    RecordItemState::Unread => None, // default
                    RecordItemState::Archive => Some("archive"),
                },
                favorite: match favorite {
                    RecordItemFavorite::All => None,
                    RecordItemFavorite::Unfavorited => Some(0),
                    RecordItemFavorite::Favorited => Some(1),
                },
                tag: match tag {
                    RecordItemTag::All => None,
                    RecordItemTag::TagName(tag) => Some(tag),
                    RecordItemTag::Untagged => Some("_untagged_"),
                },
                content_type: match content_type {
                    RecordItemContentType::All => None,
                    RecordItemContentType::Article => Some("article"),
                    RecordItemContentType::Video => Some("video"),
                    RecordItemContentType::Image => Some("image"),
                },
                sort: match sort {
                    RecordItemSort::All => None,
                    RecordItemSort::Newest => Some("newest"),
                    RecordItemSort::Oldest => Some("oldest"),
                    RecordItemSort::Title => Some("title"),
                    RecordItemSort::Site => Some("site"),
                },
                detail_type: match detail_type {
                    RecordItemDetailType::All => None,
                    RecordItemDetailType::Simple => Some("simple"),
                    RecordItemDetailType::Complete => Some("complete"),
                },
                search: match search {
                    Some(search) if !search.is_empty() => Some(search),
                    _ => None,
                },
                domain: match domain {
                    Some(domain) if !domain.is_empty() => Some(domain),
                    _ => None,
                },
                since: match since {
                    Some(since) if *since >= 0 => Some(since),
                    _ => None,
                },
                offset,
                count,
                // offset: match offset {
                //     0..=i32::MAX => offset,
                //     _ => bail!("Offset is not a positive"),
                // },
                // count: match count {
                //     0..=i32::MAX => offset,
                //     _ => bail!("Count is not a positive"),
                // },
            },
            None => bail!("No access_token"),
        };

        let client = &self.reqwester.client;
        let res = client.post(endpoint).json(&params).send().await?;
        let res_body = &res.text().await?;

        let res_ser: RecordItem = serde_json::from_str(&res_body).map_err(|e| format_err!(e))?;

        Ok(res_ser)
    }

    pub async fn list_of_items_paginate(&self, offset: i32, count: i32) -> Result<RecordItem> {
        self.list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::default(),
            RecordItemTag::default(),
            RecordItemContentType::default(),
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            offset,
            count,
        )
        .await
    }

    /// retrieving a user's data
    pub async fn list_of_items(&self) -> Result<RecordItem> {
        self.list_of_items_paginate(0, 25).await
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
    pub async fn bulk_add(&self, params: &[BulkRequestRecordAdd]) -> Result<()> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_archive(&self, params: serde_json::Value) -> Result<()> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_readd(&self, params: serde_json::Value) -> Result<()> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_favorite(&self, params: serde_json::Value) -> Result<()> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_unfavorite(&self, params: serde_json::Value) -> Result<()> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_delete(&self, params: serde_json::Value) -> Result<()> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_tags_add(&self, params: serde_json::Value) -> Result<()> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_tags_remove(&self, params: serde_json::Value) -> Result<()> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_tags_replace(&self, params: serde_json::Value) -> Result<()> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_tags_clear(&self, params: serde_json::Value) -> Result<()> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_tag_rename(&self, params: serde_json::Value) -> Result<()> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    pub async fn bulk_tag_delete(&self, params: serde_json::Value) -> Result<()> {
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
pub struct BulkRequestRecordAdd {
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

// #[derive(Debug, Deserialize)]
// pub struct RecordModAction<'a> {
//     pub action: &'a str, // TODO: structured
//     pub params: Option<Map<String, String>>,
//     pub time: Option<i32>,
// }
