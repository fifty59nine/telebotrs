use crate::types::data_types::Chat;

/// MessageHeader contains message_id: i64, date: i64, chat: Chat
#[derive(Debug, Clone)]
pub struct MessageHeader {
    pub message_id: i64,
    pub date: i64,
    pub chat: Chat,
}
