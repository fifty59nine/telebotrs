use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum ContentType {
    Message,
    Command(String),
    Audio,
    Animation,
    Document,
    Game,
    Photo,
    Sticker,
    Video,
    VideoNote,
    Voice,
    Contact,
    Venue,
    Location,
    Poll,
    Dice,
    NewChatMember,
    LeftChatMember,
    Invoice,
    SuccessfulPayment,
    ConnectedWebsite,
    MessageAutoDeleteTimerChanged,
    MigrateFromChatId,
    MigrateToChatId,
    PinnedMessage,
    NewChatTitle,
    NewChatPhoto,
    DeleteChatPhoto,
    GroupChatCreated,
    PassportData,
    ProximityAlertTriggered,
    WebAppData,
    ForumTopicClosed,
    ForumTopicCreated,
    ForumTopicReopened,
    VideoChatScheduled,
    VideoChatStarted,
    VideoChatEnded,
    VideoChatParticipantsInvited,
    Any,
    Unknown,
}

impl ContentType {
    pub fn to_string(&self) -> String {
        format!("{:?}", self).to_string()
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
