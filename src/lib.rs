mod client;
pub use client::GetPocket;

mod ext;
pub use ext::retrieving;
pub use ext::adding;
pub use ext::modifying;

mod request;
pub use request::ApiRequestError;

pub use client::{
    RecordItemContentType, RecordItemDetailType, RecordItemFavorite, RecordItemSort, RecordItemState, RecordItemTag,
};
