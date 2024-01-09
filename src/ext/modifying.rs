// TODO: remove this derive after implementing the code
#![allow(unused_variables)]
// TODO: remove this derive after implementing the code
#![allow(dead_code)]

use crate::client::{GetPocket, RecordSendDirect};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModifyingError<'a> {
    #[error("Invalid Params: `{0}`")]
    InvalidParams(&'a str),
}

#[derive(Debug, Deserialize, Clone)]
pub struct RecordModified {
    pub is_success: bool,
    pub status: i32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

// #[derive(Debug, Serialize)]
// struct RequestParams<'a, T> {
//     consumer_key: &'a str,
//     access_token: &'a str,
//     actions: T,
// }

// impl<'a, T> RequestParams<'a, T> {
//     fn try_new(client: &'a GetPocket, actions: T) -> Result<Self> {
//         match &client.token.access_token {
//             Some(access_token) => Ok(Self {
//                 consumer_key: &client.consumer_key,
//                 access_token: access_token,
//                 actions,
//             }),
//             None => bail!(ModifyingError::InvalidParams("No access_token")),
//         }
//     }
// }

#[derive(Debug, Serialize)]
pub enum Action {
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "readd")]
    Readd,
    #[serde(rename = "favorite")]
    Favorite,
    #[serde(rename = "unfavorite")]
    Unfavorite,
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Debug, Serialize)]
pub struct RequestArchive {
    action: Action,
    /// The id of the item to perform the action on.
    item_id: i64,
    /// The time the action occurred. Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct RequestReadd {
    action: Action,
    /// The id of the item to perform the action on.
    item_id: i64,
    /// The time the action occurred. Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct RequestFavorite {
    action: Action,
    /// The id of the item to perform the action on.
    item_id: i64,
    /// The time the action occurred. Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct RequestUnfavorite {
    action: Action,
    /// The id of the item to perform the action on.
    item_id: i64,
    /// The time the action occurred. Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct RequestDelete {
    action: Action,
    /// The id of the item to perform the action on.
    item_id: i64,
    /// The time the action occurred. Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<i32>,
}

/// <https://getpocket.com/developer/docs/v3/modify>   
#[async_trait]
pub trait ModifyingExt {
    /// Move an item to the user's archive
    async fn archive(&self, item_id: i64) -> Result<RecordModified>;

    /// Re-add (unarchive) an item to the user's list
    async fn readd(&self, item_id: i64) -> Result<RecordModified>;

    /// Mark an item as a favorite
    async fn favorite(&self, item_id: i64) -> Result<RecordModified>;

    /// Remove an item from the user's favorites
    async fn unfavorite(&self, item_id: i64) -> Result<RecordModified>;

    /// Permanently remove an item from the user's account
    async fn delete(&self, item_id: i64) -> Result<RecordModified>;
}

#[async_trait]
impl ModifyingExt for GetPocket {
    /// Move an item to the user's archive
    async fn archive(&self, item_id: i64) -> Result<RecordModified> {
        let params = RequestArchive {
            action: Action::Archive,
            item_id,
            time: None,
        };

        let resp = &self.send(&[params]).await?;

        Ok(resp.into())
    }

    /// Move an item from the user's archive back into their unread list.
    async fn readd(&self, item_id: i64) -> Result<RecordModified> {
        let params = RequestReadd {
            action: Action::Readd,
            item_id,
            time: None,
        };

        let resp = &self.send(&[params]).await?;

        Ok(resp.into())
    }

    /// Mark an item as a favorite
    async fn favorite(&self, item_id: i64) -> Result<RecordModified> {
        let params = RequestFavorite {
            action: Action::Favorite,
            item_id,
            time: None,
        };

        let resp = &self.send(&[params]).await?;

        Ok(resp.into())
    }

    /// Remove an item from the user's favorites
    async fn unfavorite(&self, item_id: i64) -> Result<RecordModified> {
        let params = RequestUnfavorite {
            action: Action::Unfavorite,
            item_id,
            time: None,
        };

        let resp = &self.send(&[params]).await?;

        Ok(resp.into())
    }

    /// Permanently remove an item from the user's account
    async fn delete(&self, item_id: i64) -> Result<RecordModified> {
        let params = RequestDelete {
            action: Action::Delete,
            item_id,
            time: None,
        };

        let resp = &self.send(&[params]).await?;

        Ok(resp.into())
    }
}

impl From<&RecordSendDirect> for RecordModified {
    fn from(record: &RecordSendDirect) -> Self {
        match record {
            RecordSendDirect::Standart(record) => {
                let Some(first_action_result) = record.action_results.get(0) else {
                    return Self {
                        is_success: false,
                        status: record.status,
                        errors: vec![Some("No action results".to_string())],
                        data: None,
                    };
                };

                Self {
                    is_success: *first_action_result,
                    status: record.status,
                    errors: vec![],
                    data: None,
                }
            }
            RecordSendDirect::Extended(record) => Self {
                is_success: !record.action_errors.is_empty(),
                status: record.status,
                errors: record.action_errors.clone(),
                data: Some(record.action_results.clone()),
            },
        }
    }
}
