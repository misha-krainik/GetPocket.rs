mod client;
pub use client::GetPocket;

mod ext;
pub use ext::retrieving;

mod request;
pub use request::ApiRequestError;

pub use client::{
    RecordAdded, RecordItem, RecordItemContentType, RecordItemDetailType, RecordItemFavorite,
    RecordItemSort, RecordItemState, RecordItemTag,
};
