use crate::types::data_types::{Chat, User};

#[derive(Debug, Clone)]
pub struct TextMessage {
    pub message_id: i64,
    pub date: i64,
    pub chat: Chat,
    pub from: User,
    pub text: String,
}

impl TextMessage {
    pub fn new(
        message_id: i64,
        date: i64,
        chat: Chat,
        from: User,
        text: impl Into<String>,
    ) -> Self {
        TextMessage {
            message_id,
            date,
            chat,
            from,
            text: text.into(),
        }
    }
}
