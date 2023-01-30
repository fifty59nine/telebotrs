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

impl TryFrom<Message> for TextMessage {
    type Error = String;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let ct = value.content_type.clone().unwrap();
        if matches!(ct, ContentType::Message) || matches!(ct, ContentType::Command(_)) {
            Err(format!(
                "Expected \"Message with content_type Message or Commands\", found: {:?} ",
                value.content_type.unwrap()
            ))
        } else {
            Ok(TextMessage::new(
                value.message_id,
                value.date,
                value.chat,
                value.from.unwrap_or_default(),
                value.text.unwrap(),
            ))
        }
    }
}
