mod test_helper;

use getpocket::{
    retrieving::RetrievingExt, GetPocket, RecordItemContentType, RecordItemDetailType,
    RecordItemFavorite, RecordItemSort, RecordItemState, RecordItemTag,
};
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

    // Test case: Retrieve tagged items
    let r = get_pocket
        .list_of_items_with_params(
            RecordItemState::default(),
            RecordItemFavorite::default(),
            RecordItemTag::TagName("article"),
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