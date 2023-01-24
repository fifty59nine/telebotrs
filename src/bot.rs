use crate::types::{message_parse_object::Root, ContentType, Message};

pub struct Bot {
    token: String,
    last_update_id: i64,
}

impl Bot {
    pub fn new(token: impl Into<String>) -> Self {
        Bot {
            token: token.into(),
            last_update_id: 0,
        }
    }

    /// Use this method to receive incoming updates using long polling
    /// `interval` - sleep time in ms between geting updates (Default: 20)
    /// `none_stop` - don't stopping bot on error (Default: true)
    pub async fn start_polling(&mut self, interval: Option<u64>, none_stop: Option<bool>) {
        let interval_handler = match interval {
            Some(n) => n,
            None => 20,
        };
        let none_stop_handler = match none_stop {
            Some(n) => n,
            None => true,
        };

        println!("Handling updates with {} interval", &interval_handler);
        loop {
            let messages = self.get_updates(&none_stop_handler).await;
            for msg in messages {
                println!(
                    "Message from {} ({}): {}",
                    msg.chat.username,
                    msg.chat.id,
                    msg.text.unwrap()
                );
            }

            std::thread::sleep(std::time::Duration::from_millis(interval_handler));
        }

        // TODO:
        // 1) getUpdates
        // 2) Parse answer to the message_parse_object
        // 3) write objects like document, audio etc.
        // 4) send to message_handler
    }

    async fn get_updates(&mut self, none_stop: &bool) -> Vec<Message> {
        let updates = self.request("getUpdates").await;
        match updates {
            Ok(update) => {
                let root_result: Result<Root, serde_json::Error> = serde_json::from_str(&update);
                match root_result {
                    Err(n) => {
                        if none_stop.to_owned() {
                            println!("Exception on getUpdates while parsing update object: {}", n);
                        } else {
                            panic!("Exception on getUpdates while parsing update object: {}", n);
                        }
                        return vec![];
                    }
                    Ok(n) => {
                        // Here all parsed good. Working with Root object
                        if let Some(last) = &n.result.last() {
                            self.last_update_id = last.update_id;
                        }
                        let mut message_array: Vec<Message> = Vec::with_capacity(n.result.len());
                        n.result
                            .iter()
                            .for_each(|obj| message_array.push(obj.message.clone()));
                        return message_array;
                    }
                }
            }
            Err(n) => {
                if none_stop.to_owned() {
                    println!("Exception on getUpdates: {}", n);
                } else {
                    panic!("Exception on getUpdates: {}", n);
                }
                return vec![];
            }
        }
    }

    async fn request(&self, method: impl Into<String>) -> Result<String, reqwest::Error> {
        let method_name = method.into();
        let mut base_url = format!("https://api.telegram.org/{}/{}", &self.token, &method_name);
        if &method_name == "getUpdates" && self.last_update_id != 0 {
            base_url.push_str(format!("?offset={}", &self.last_update_id + 1).as_str());
        }
        let response = reqwest::get(base_url).await?.text().await?;

        Ok(response)
    }

    fn get_message_content_type(message: &Message) -> ContentType {
        if let Some(txt) = message.text {
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
        }
        todo!("Continue!")
    }
}
