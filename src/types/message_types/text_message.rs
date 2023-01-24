use crate::types::{
    data_types::{Chat, User},
    ContentType, Message,
};

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

impl From<Message> for TextMessage {
    fn from(value: Message) -> Self {
        let CT = value.content_type.clone().unwrap();
        if matches!(CT, ContentType::Message) || matches!(CT, ContentType::Command(_)) {
            panic!(
                "Expected \"Message with content_type Message or Commands\", found: {:?} ",
                value.content_type.unwrap()
            );
        } else {
            TextMessage::new(
                value.message_id,
                value.date,
                value.chat,
                value.from.unwrap_or_default(),
                value.text.unwrap(),
            )
        }
    }
}
