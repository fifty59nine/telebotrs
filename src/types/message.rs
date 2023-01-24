use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::types::data_types::{
    Animation, Audio, Chat, Contact, Dice, Document, ForumTopicClosed, ForumTopicCreated,
    ForumTopicReopened, Game, Invoice, Location, MessageAutoDeleteTimerChanged, MessageEntity,
    PassportData, PhotoSize, Poll, ProximityAlertTriggered, Sticker, SuccessfulPayment, User,
    Venue, Video, VideoChatEnded, VideoChatParticipantsInvited, VideoChatScheduled,
    VideoChatStarted, VideoNote, Voice, WebAppData,
};
use crate::types::keyboard_types::InlineKeyboardMarkup;
use crate::types::ContentType;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub date: i64,
    pub chat: Chat,
    pub message_thread_id: Option<i64>,
    pub from: Option<User>,
    pub sender_chat: Option<Chat>,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<i64>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    pub forward_date: Option<i64>,
    pub is_topic_message: Option<bool>,
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<Message>>,
    pub via_bot: Option<User>,
    pub edit_date: Option<i64>,
    pub has_protected_content: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub contact: Option<Contact>,
    pub dice: Option<Dice>,
    pub game: Option<Game>,
    pub poll: Option<Poll>,
    pub venue: Option<Venue>,
    pub location: Option<Location>,
    pub new_chat_member: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<Message>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
    pub connected_website: Option<String>,
    pub passport_data: Option<PassportData>,
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    pub forum_topic_created: Option<ForumTopicCreated>,
    pub forum_topic_closed: Option<ForumTopicClosed>,
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    pub video_chat_started: Option<VideoChatStarted>,
    pub video_chat_ended: Option<VideoChatEnded>,
    pub video_chat_participants_invite: Option<VideoChatParticipantsInvited>,
    pub web_app_data: Option<WebAppData>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub content_type: Option<ContentType>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl Message {
    pub fn set_content_type(&mut self) {
        self.content_type = Some(Message::get_message_content_type(&self));
    }

    fn get_message_content_type(message: &Message) -> ContentType {
        if let Some(txt) = &message.text {
            if txt.starts_with('/') {
                ContentType::Command(txt.replace('/', ""))
            } else {
                ContentType::Message
            }
        } else if let Some(_) = message.audio {
            ContentType::Audio
        } else if let Some(_) = message.animation {
            ContentType::Animation
        } else if let Some(_) = message.document {
            ContentType::Document
        } else if let Some(_) = message.game {
            ContentType::Game
        } else if let Some(_) = message.photo {
            ContentType::Photo
        } else if let Some(_) = message.sticker {
            ContentType::Sticker
        } else if let Some(_) = message.video {
            ContentType::Video
        } else if let Some(_) = message.video_note {
            ContentType::VideoNote
        } else if let Some(_) = message.voice {
            ContentType::Voice
        } else if let Some(_) = message.contact {
            ContentType::Contact
        } else if let Some(_) = message.venue {
            ContentType::Venue
        } else if let Some(_) = message.location {
            ContentType::Location
        } else if let Some(_) = message.poll {
            ContentType::Poll
        } else if let Some(_) = message.dice {
            ContentType::Dice
        } else if let Some(_) = message.new_chat_member {
            ContentType::NewChatMember
        } else if let Some(_) = message.left_chat_member {
            ContentType::LeftChatMember
        } else if let Some(_) = message.invoice {
            ContentType::Invoice
        } else if let Some(_) = message.successful_payment {
            ContentType::SuccessfulPayment
        } else if let Some(_) = message.connected_website {
            ContentType::ConnectedWebsite
        } else if let Some(_) = message.message_auto_delete_timer_changed {
            ContentType::MessageAutoDeleteTimerChanged
        } else if let Some(_) = message.migrate_from_chat_id {
            ContentType::MigrateFromChatId
        } else if let Some(_) = message.migrate_to_chat_id {
            ContentType::MigrateToChatId
        } else if let Some(_) = message.pinned_message {
            ContentType::PinnedMessage
        } else if let Some(_) = message.new_chat_title {
            ContentType::NewChatTitle
        } else if let Some(_) = message.new_chat_member {
            ContentType::NewChatMember
        } else if let Some(_) = message.new_chat_photo {
            ContentType::NewChatPhoto
        } else if let Some(_) = message.delete_chat_photo {
            ContentType::DeleteChatPhoto
        } else if let Some(_) = message.group_chat_created {
            ContentType::GroupChatCreated
        } else if let Some(_) = message.passport_data {
            ContentType::PassportData
        } else if let Some(_) = message.proximity_alert_triggered {
            ContentType::ProximityAlertTriggered
        } else if let Some(_) = message.video_chat_scheduled {
            ContentType::VideoChatScheduled
        } else if let Some(_) = message.video_chat_started {
            ContentType::VideoChatStarted
        } else if let Some(_) = message.video_chat_ended {
            ContentType::VideoChatEnded
        } else if let Some(_) = message.video_chat_participants_invite {
            ContentType::VideoChatParticipantsInvited
        } else if let Some(_) = message.web_app_data {
            ContentType::WebAppData
        } else if let Some(_) = message.forum_topic_created {
            ContentType::ForumTopicCreated
        } else if let Some(_) = message.forum_topic_closed {
            ContentType::ForumTopicClosed
        } else if let Some(_) = message.forum_topic_reopened {
            ContentType::ForumTopicReopened
        } else {
            ContentType::Unknown
        }
    }
}
