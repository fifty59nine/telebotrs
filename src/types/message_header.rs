use crate::types::data_types::Chat;
use serde::{Deserialize, Serialize};

/// MessageHeader contains message_id: i64, date: i64, chat: Chat
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageHeader {
    pub message_id: i64,
    pub date: i64,
    pub chat: Chat,
}
