use crate::{
    client::{GetPocket, *},
    ApiRequestError,
};
use anyhow::{bail, format_err, Result};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use thiserror::Error;

static ENDPOINT: &'static str = "https://getpocket.com/v3/send";

#[derive(Error, Debug)]
pub enum ModifyingError<'a> {
    #[error("Invalid Params: `{0}`")]
    InvalidParams(&'a str),
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

/// https://getpocket.com/developer/docs/v3/modify    
#[async_trait]
pub trait ModifyingExt {
    async fn bulk_modify_raw_params<'a>(&self, params: &'a str) -> Result<RecordModified>;

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    async fn bulk_add(&self, _params: &[BulkRecAdd]) -> Result<BulkRecAdded> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    async fn bulk_archive(&self, _params: &[BulkRecArchive]) -> Result<BulkRecArchived> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    async fn bulk_readd(&self, _params: &[BulkRecReadd]) -> Result<BulkRecReadded> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    async fn bulk_favorite(&self, _params: &[BulkRecFavorite]) -> Result<BulkRecFavorited> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    async fn bulk_unfavorite(
        &self,
        _params: &[BulkRecUnfovorite],
    ) -> Result<BulkRecUnfovorited> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    async fn bulk_delete(&self, _params: &[BulkRecDelete]) -> Result<BulkRecDeleted> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    async fn bulk_tags_add(&self, _params: &[BulkTagsAdd]) -> Result<BulkTagsAdded> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    async fn bulk_tags_remove(&self, _params: &[BulkTagsRemove]) -> Result<BulkTagsRemoved> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    async fn bulk_tags_replace(&self, _params: &[BulkTagsReplace]) -> Result<BulkTagsReplaced> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    async fn bulk_tags_clear(&self, _params: &[BulkTagsClear]) -> Result<BulkTagsCleared> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    async fn bulk_tag_rename(&self, _params: &[BulkTagsRename]) -> Result<BulkTagsRenamed> {
        unimplemented!()
    }

    // NOTE: function signature and code can be changed.
    #[cfg(feature = "unstable")]
    async fn bulk_tag_delete(&self, _params: &[BulkTagsDelete]) -> Result<BulkTagsDeleted> {
        unimplemented!()
    }
}

#[async_trait]
impl ModifyingExt for GetPocket {
    async fn bulk_modify_raw_params<'a>(&self, params: &'a str) -> Result<RecordModified> {
        #[derive(Serialize)]
        struct RequestParams<'a> {
            consumer_key: &'a str,
            access_token: &'a str,
        }

        let access_token = match &self.token.access_token {
            Some(access_token) => access_token,
            None => bail!(ClientError::TokenError),
        };

        let consumer_key = &self.consumer_key;

        let params =
            format!("{ENDPOINT}?{params}&access_token={access_token}&consumer_key={consumer_key}");

        let client = &self.reqwester.client;
        let res = client.post(&params).send().await?;

        if let Err(err) = ApiRequestError::handler_status(res.status()) {
            bail!(err);
        }

        let res_body = &res.text().await?;

        let res_ser: RecordModified =
            serde_json::from_str(&res_body).map_err(|e| format_err!(ClientError::JsonError(e)))?;

        Ok(res_ser)
    }
}