mod client;
pub use client::GetPocket;

mod ext;
pub use ext::adding;
pub use ext::modifying;
pub use ext::retrieving;

mod request;
pub use request::ApiRequestError;

pub use client::{
    RecordItemContentType, RecordItemDetailType, RecordItemFavorite, RecordItemSort,
    RecordItemState, RecordItemTag,
};
