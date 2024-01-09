// TODO: remove this derive after implementing the code
#![allow(unused_variables)]
// TODO: remove this derive after implementing the code
#![allow(dead_code)]

use crate::client::GetPocket;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TagsError<'a> {
    #[error("Invalid Params: `{0}`")]
    InvalidParams(&'a str),
}

#[derive(Debug, Deserialize)]
pub struct RecordModified {
    pub action_results: Vec<bool>,
    pub status: i32,
}

struct RequestParams<T> {
    consumer_key: String,
    access_token: String,
    actions: T,
}

pub struct RequestAddTags {
    /// The id of the item to perform the action on.
    item_id: i32,
    /// A comma-delimited list of one or more tags.
    tags: String,
    /// The time the action occurred. Unix epoch in milliseconds
    time: Option<i32>,
}

pub struct RequestRemoveTags {
    /// The id of the item to perform the action on.
    item_id: i32,
    /// A comma-delimited list of one or more tags.
    tags: String,
    /// The time the action occurred. Unix epoch in milliseconds
    time: Option<i32>,
}

pub struct RequestReplaceTags {
    /// The id of the item to perform the action on.
    item_id: i32,
    /// A comma-delimited list of one or more tags.
    tags: String,
    /// The time the action occurred. Unix epoch in milliseconds
    time: Option<i32>,
}

pub struct RequestClearTags {
    /// The id of the item to perform the action on.
    item_id: i32,
    /// The time the action occurred. Unix epoch in milliseconds
    time: Option<i32>,
}

pub struct RequestRenameTags {
    /// The tag name that will be replaced.
    old_tag: String,
    /// The new tag name that will be added.
    new_tag: String,
    /// The time the action occurred. Unix epoch in milliseconds
    time: Option<i32>,
}

pub struct RequestDeleteTags {
    /// The tag name that will be deleted.
    tag: String,
    /// The time the action occurred. Unix epoch in milliseconds
    time: Option<i32>,
}

#[async_trait]
pub trait TagsExt {
    async fn tags_add(&self, params: &RequestAddTags) -> Result<RecordModified>;

    async fn tags_remove(&self, params: &RequestRemoveTags) -> Result<RecordModified>;

    async fn tags_replace(&self, params: &RequestReplaceTags) -> Result<RecordModified>;

    async fn tags_clear(&self, params: &RequestClearTags) -> Result<RecordModified>;

    async fn tag_rename(&self, params: &RequestRenameTags) -> Result<RecordModified>;

    async fn tag_delete(&self, params: &RequestDeleteTags) -> Result<RecordModified>;
}

#[async_trait]
impl TagsExt for GetPocket {
    /// Add one or more tags to an item
    async fn tags_add(&self, _params: &RequestAddTags) -> Result<RecordModified> {
        unimplemented!()
    }

    /// Remove one or more tags from an item
    async fn tags_remove(&self, _params: &RequestRemoveTags) -> Result<RecordModified> {
        unimplemented!()
    }

    /// Replace all of the tags for an item with one or more provided tags
    async fn tags_replace(&self, _params: &RequestReplaceTags) -> Result<RecordModified> {
        unimplemented!()
    }

    /// Remove all tags from an item
    async fn tags_clear(&self, _params: &RequestClearTags) -> Result<RecordModified> {
        unimplemented!()
    }

    /// Rename a tag; this affects all items with this tag
    async fn tag_rename(&self, _params: &RequestRenameTags) -> Result<RecordModified> {
        unimplemented!()
    }

    /// Delete a tag; this affects all items with this tag
    async fn tag_delete(&self, _params: &RequestDeleteTags) -> Result<RecordModified> {
        unimplemented!()
    }
}
