use serde::{Deserialize, Serialize};
use specta::Type;
use strum_macros::Display;

#[derive(Type, Serialize, Deserialize, Display)]
pub enum EventNames {
    #[serde(rename = "POOL_CLIPBOARD_UPDATED")]
    #[strum(serialize = "POOL_CLIPBOARD_UPDATED")]
    PoolClipboardUpdated,
    #[serde(rename = "SOMETHING")]
    #[strum(serialize = "SOMETHING")]
    Something, //todo
}
