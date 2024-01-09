mod test_helper;

use getpocket::{
    adding::AddingExt, modifying::ModifyingExt, retrieving::RetrievingExt, GetPocket,
    RecordItemContentType, RecordItemDetailType, RecordItemFavorite, RecordItemSort,
    RecordItemState, RecordItemTag,
};
use serde::{Deserialize, Serialize};
use tokio::test;

#[test]
async fn test_retrieve_all_archive_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    // Test case: Retrieve all archive items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::Archive,
            RecordItemFavorite::default(),
            RecordItemTag::default(),
            RecordItemContentType::default(),
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_retrieve_all_unread_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    // Test case: Retrieve all unread items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::Unread,
            RecordItemFavorite::default(),
            RecordItemTag::default(),
            RecordItemContentType::default(),
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_retrieve_all_state_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    // Test case: Retrieve all items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::All,
            RecordItemFavorite::default(),
            RecordItemTag::default(),
            RecordItemContentType::default(),
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_retrieve_favorited_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    // Test case: Retrieve favorited items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::Favorited,
            RecordItemTag::default(),
            RecordItemContentType::default(),
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_retrieve_unfavorited_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    // Test case: Retrieve unfavorited items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::Unfavorited,
            RecordItemTag::default(),
            RecordItemContentType::default(),
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_retrieve_untagged_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    // Test case: Retrieve untagged items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::default(),
            RecordItemTag::Untagged,
            RecordItemContentType::default(),
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_retrieve_tagged_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    let _ = get_pocket
        .add_item_with_params(
            "https://www.rust-lang.org/",
            Some("Rust Programming Language"),
            Some(&["rust", "programming", "language"]),
            None,
        )
        .await;

    // Test case: Retrieve tagged items
    let resp = get_pocket
        .list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::default(),
            RecordItemTag::TagName("rust"),
            RecordItemContentType::default(),
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            0,
            1,
        )
        .await;

    assert!(resp.is_ok());
}

#[test]
async fn test_retrieve_all_content_type_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    // Test case: Retrieve all content type items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::default(),
            RecordItemTag::default(),
            RecordItemContentType::All,
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_retrieve_video_content_type_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    // Test case: Retrieve video content type items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::default(),
            RecordItemTag::default(),
            RecordItemContentType::Video,
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_retrieve_article_content_type_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    // Test case: Retrieve article content type items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::default(),
            RecordItemTag::default(),
            RecordItemContentType::Article,
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_retrieve_image_content_type_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    let _ = get_pocket
        .add_item_with_params(
            "https://www.mozilla.org/media/img/pocket/pocket-logo-light-mode.9a20614bbcba.svg",
            None,
            None,
            None,
        )
        .await;

    // Test case: Retrieve image content type items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::default(),
            RecordItemTag::default(),
            RecordItemContentType::Image,
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_retrieve_simple_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    // Test case: Retrieve simple items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::default(),
            RecordItemTag::default(),
            RecordItemContentType::default(),
            RecordItemSort::default(),
            RecordItemDetailType::Simple,
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_retrieve_complete_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    // Test case: Retrieve complete items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::default(),
            RecordItemTag::default(),
            RecordItemContentType::default(),
            RecordItemSort::default(),
            RecordItemDetailType::Complete,
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_retrieve_all_items() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    // Test case: Retrieve all items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::default(),
            RecordItemTag::default(),
            RecordItemContentType::default(),
            RecordItemSort::default(),
            RecordItemDetailType::default(),
            None,
            None,
            None,
            0,
            1,
        )
        .await;
    assert!(r.is_ok());
}

#[test]
async fn test_send_params_direct() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;

    #[derive(Debug, Serialize, Deserialize)]
    struct MyStruct {
        action: String,
        item_id: String,
        time: String,
    }
    let my_struct = MyStruct {
        action: String::from("favorite"),
        item_id: String::from("229279689"),
        time: String::from("1348853312"),
    };

    let r = get_pocket.send(&[my_struct]).await;

    assert!(r.is_ok());
}

#[test]
async fn test_archive_item() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;
    let item_id = add_new_item(&get_pocket, "https://getpocket.com/developer/docs/v3/add").await;
    let resp_archive = get_pocket.archive(item_id).await;

    assert!(resp_archive.is_ok());

    let _ = get_pocket.delete(item_id).await;
}

#[test]
async fn test_readd_item() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;
    let item_id = add_new_item(&get_pocket, "https://getpocket.com/developer/docs/v3/add").await;
    let _ = get_pocket.archive(item_id).await;

    let resp_readd = get_pocket.readd(item_id).await;
    assert!(resp_readd.is_ok());

    let _ = get_pocket.delete(item_id).await;
}

#[test]
async fn test_favorite_item() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;
    let item_id = add_new_item(&get_pocket, "https://getpocket.com/developer/docs/v3/add").await;

    let resp_favorite = get_pocket.favorite(item_id).await;
    assert!(resp_favorite.is_ok());

    let _ = get_pocket.delete(item_id).await;
}

#[test]
async fn test_unfavorite_item() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;
    let item_id = add_new_item(&get_pocket, "https://getpocket.com/developer/docs/v3/add").await;

    let _ = get_pocket.favorite(item_id).await;

    let resp_unfavorite = get_pocket.unfavorite(item_id).await;
    assert!(resp_unfavorite.is_ok());

    let _ = get_pocket.delete(item_id).await;
}

#[test]
async fn test_delete_item() {
    let get_pocket: GetPocket = test_helper::init_get_pocket().await;
    let item_id = add_new_item(&get_pocket, "https://getpocket.com/developer/docs/v3/add").await;
    let resp_delete = get_pocket.delete(item_id).await;
    assert!(resp_delete.is_ok());
}

pub async fn add_new_item(get_pocket: &GetPocket, url: &str) -> i64 {
    let resp = get_pocket.add_item(url).await.unwrap();
    let item_id: i64 = resp
        .item
        .get("item_id")
        .unwrap()
        .as_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    item_id
}
