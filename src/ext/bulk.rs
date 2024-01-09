use crate::{
    client::GetPocket,
    ext::{modifying::*, tags::*},
};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BulkRecordModified {
    pub action_results: Vec<bool>,
    pub status: i32,
}

pub struct BulkRequestAdd {
    pub action: String,
    /// The id of the item to perform the action on.
    pub item_id: i32,
    /// A Twitter status id; this is used to show tweet attribution.
    pub ref_id: Option<i32>,
    /// A comma-delimited list of one or more tags.
    pub tags: Option<String>,
    /// The time the action occurred. Unix epoch in milliseconds
    pub time: Option<i32>,
    /// The title of the item.
    pub title: Option<String>,
    /// The url of the item; provide this only if you do not have an item_id.
    pub url: Option<String>,
}

#[async_trait]
pub trait BulkExt {
    async fn bulk_modify<T>(&self, params: &[T]) -> Result<BulkRecordModified>;

    async fn bulk_add(&self, params: &[BulkRequestAdd]) -> Result<BulkRecordModified>;

    async fn bulk_archive(&self, params: &[RequestArchive]) -> Result<BulkRecordModified>;

    async fn bulk_readd(&self, params: &[RequestReadd]) -> Result<BulkRecordModified>;

    async fn bulk_favorite(&self, params: &[RequestFavorite]) -> Result<BulkRecordModified>;

    async fn bulk_unfavorite(&self, params: &[RequestUnfavorite]) -> Result<BulkRecordModified>;

    async fn bulk_delete(&self, params: &[RequestDelete]) -> Result<BulkRecordModified>;

    async fn bulk_tags_add(&self, params: &[RequestAddTags]) -> Result<BulkRecordModified>;

    async fn bulk_tags_remove(&self, params: &[RequestRemoveTags]) -> Result<BulkRecordModified>;

    async fn bulk_tags_replace(&self, params: &[RequestReplaceTags]) -> Result<BulkRecordModified>;

    async fn bulk_tags_clear(&self, params: &[RequestClearTags]) -> Result<BulkRecordModified>;

    async fn bulk_tag_rename(&self, params: &[RequestRenameTags]) -> Result<BulkRecordModified>;

    async fn bulk_tag_delete(&self, params: &[RequestDeleteTags]) -> Result<BulkRecordModified>;
}

#[async_trait]
impl BulkExt for GetPocket {
    async fn bulk_modify<T>(&self, _params: &[T]) -> Result<BulkRecordModified> {
        unimplemented!()
    }

    async fn bulk_add(&self, _params: &[BulkRequestAdd]) -> Result<BulkRecordModified> {
        unimplemented!()
    }

    async fn bulk_archive(&self, _params: &[RequestArchive]) -> Result<BulkRecordModified> {
        unimplemented!()
    }

    async fn bulk_readd(&self, _params: &[RequestReadd]) -> Result<BulkRecordModified> {
        unimplemented!()
    }

    async fn bulk_favorite(&self, _params: &[RequestFavorite]) -> Result<BulkRecordModified> {
        unimplemented!()
    }

    async fn bulk_unfavorite(&self, _params: &[RequestUnfavorite]) -> Result<BulkRecordModified> {
        unimplemented!()
    }

    async fn bulk_delete(&self, _params: &[RequestDelete]) -> Result<BulkRecordModified> {
        unimplemented!()
    }

    async fn bulk_tags_add(&self, _params: &[RequestAddTags]) -> Result<BulkRecordModified> {
        unimplemented!()
    }

    async fn bulk_tags_remove(&self, _params: &[RequestRemoveTags]) -> Result<BulkRecordModified> {
        unimplemented!()
    }

    async fn bulk_tags_replace(
        &self,
        _params: &[RequestReplaceTags],
    ) -> Result<BulkRecordModified> {
        unimplemented!()
    }

    async fn bulk_tags_clear(&self, _params: &[RequestClearTags]) -> Result<BulkRecordModified> {
        unimplemented!()
    }

    async fn bulk_tag_rename(&self, _params: &[RequestRenameTags]) -> Result<BulkRecordModified> {
        unimplemented!()
    }

    async fn bulk_tag_delete(&self, _params: &[RequestDeleteTags]) -> Result<BulkRecordModified> {
        unimplemented!()
    }
}
