use crate::{
    client::{GetPocket, *},
    ApiRequestError,
};
use anyhow::{bail, format_err, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;
use thiserror::Error;

static ENDPOINT: &'static str = "https://getpocket.com/v3/get";

#[derive(Error, Debug)]
pub enum RetrievingError<'a> {
    #[error("Invalid Params: `{0}`")]
    InvalidParams(&'a str),
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

#[async_trait]
pub trait RetrievingExt {
    async fn list_of_items_with_params<'a>(
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
    ) -> Result<RecordItem>;

    async fn list_of_items_paginate(&self, offset: i32, count: i32) -> Result<RecordItem>;

    async fn list_of_items(&self) -> Result<RecordItem>;
}

#[async_trait]
impl RetrievingExt for GetPocket {
    async fn list_of_items_with_params<'a>(
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
            },
            None => bail!(RetrievingError::InvalidParams("No access_token")),
        };

        let client = &self.reqwester.client;
        let res = client.post(ENDPOINT).json(&params).send().await?;

        if let Err(err) = ApiRequestError::handler_status(res.status()) {
            bail!(err);
        }

        let res_body = &res.text().await?;

        let res_ser: RecordItem = serde_json::from_str(&res_body).map_err(|e| format_err!(e))?;

        Ok(res_ser)
    }

    async fn list_of_items_paginate(&self, offset: i32, count: i32) -> Result<RecordItem> {
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

    async fn list_of_items(&self) -> Result<RecordItem> {
        self.list_of_items_paginate(0, 25).await
    }
}
