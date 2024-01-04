use crate::{
    client::GetPocket,
    ApiRequestError,
};
use std::collections::BTreeMap as Map;
use anyhow::{bail, format_err, Result};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use thiserror::Error;

static ENDPOINT: &'static str = "https://getpocket.com/v3/add";

#[derive(Error, Debug)]
pub enum AddingError<'a> {
    #[error("Invalid Params: `{0}`")]
    InvalidParams(&'a str),
}

#[derive(Debug, Deserialize)]
pub struct RecordAdded {
    pub item: Map<String, serde_json::Value>,
    pub status: i32,
}

#[derive(Serialize)]
pub struct RequestParams<'a> {
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

#[async_trait]
pub trait AddingExt {
    async fn add_item_with_params<'a>(
        &self,
        url: &'a str,
        title: Option<&'a str>,
        tags: Option<&[&'a str]>,
        tweet_id: Option<&'a str>,
    ) -> Result<RecordAdded>;

    async fn add_item<'a>(&self, url: &'a str) -> Result<RecordAdded>;
}

#[async_trait]
impl AddingExt for GetPocket {
    async fn add_item_with_params<'a>(
        &self,
        url: &'a str,
        title: Option<&'a str>,
        tags: Option<&[&'a str]>,
        tweet_id: Option<&'a str>,
    ) -> Result<RecordAdded> {
        let params = match &self.token.access_token {
            Some(access_token) => RequestParams {
                consumer_key: &self.consumer_key,
                access_token,
                url,
                title,
                tags,
                tweet_id,
            },
            None => bail!(AddingError::InvalidParams("No access_token")),
        };

        let client = &self.reqwester.client;
        let res = client.post(ENDPOINT).json(&params).send().await?;

        if let Err(err) = ApiRequestError::handler_status(res.status()) {
            bail!(err);
        }

        let res_body = &res.text().await?;

        let res_ser: RecordAdded = serde_json::from_str(&res_body).map_err(|e| format_err!(e))?;

        Ok(res_ser)
    }

    async fn add_item<'a>(&self, url: &'a str) -> Result<RecordAdded> {
        self.add_item_with_params(url, None, None, None).await
    }
}
