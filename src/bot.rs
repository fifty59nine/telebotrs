use crate::types::Message;

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
    pub async fn start_polling(&mut self, interval: Option<u32>, none_stop: Option<bool>) {
        let interval_handler = match interval {
            Some(n) => n,
            None => 20,
        };
        let none_stop_handler = match none_stop {
            Some(n) => n,
            None => true,
        };

        println!("Handling updates with {} interval", &interval_handler);

        // TODO:
        // 1) getUpdates
        // 2) Parse answer to the message_parse_object
        // 3) write objects like document, audio etc.
        // 4) send to message_handler
    }

    async fn get_updates(&self) -> Vec<Message> {
        todo!("getUpdates method");
        vec![]
    }
}
