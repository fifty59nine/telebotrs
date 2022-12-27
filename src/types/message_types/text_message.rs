use crate::types::{MessageHeader, data_types::{User, Chat}};

#[derive(Debug, Clone)]
pub struct TextMessage {
    pub message: MessageHeader,
    pub from: User,
    pub chat: Chat,
    pub text: String
}
