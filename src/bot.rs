use crate::types::{message_parse_object::Root, Message};
use reqwest::Url;

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

        loop {
            let messages_result = self.get_updates().await;
            match messages_result {
                Err(e) => {
                    if none_stop_handler {
                        println!("Exception on getUpdates in start_polling method: {e}");
                    } else {
                        panic!("Exception on getUpdates in start_polling method: {e}");
                    }
                    println!("Sleeping exception timeout (10s)");
                    std::thread::sleep(std::time::Duration::from_secs(10));
                }
                Ok(mut messages) => {
                    // Here we work with messages which accepted from api
                    for msg in messages.iter_mut() {
                        msg.set_content_type();
                        let msg_data = msg.clone();
                        println!(
                            "Message from {} ({}): {} | {:?}",
                            msg_data.chat.username,
                            msg_data.chat.id,
                            msg_data.text.unwrap_or("NO TEXT".to_string()),
                            msg_data.content_type.unwrap()
                        );
                    }
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(interval_handler));
        }

        // TODO:
        // 1) getUpdates
        // 2) Parse answer to the message_parse_object
        // 3) write objects like document, audio etc.
        // 4) send to message_handler
    }

    async fn get_updates(&mut self) -> Result<Vec<Message>, anyhow::Error> {
        let mut data: Vec<(String, String)> = Vec::new();
        if self.last_update_id != 0 {
            // Offset value, like last msg
            let offset = &self.last_update_id + 1;
            data.push(("offset".to_string(), offset.to_string()))
        }

        let updates = self.request("getUpdates", data).await;

        // Check request for errors
        if let Err(n) = updates {
            return Err(n.into());
        } else {
            let root_result: Result<Root, serde_json::Error> =
                serde_json::from_str(&updates.unwrap());
            match root_result {
                Err(n) => {
                    return Err(n.into());
                }
                Ok(n) => {
                    // Here all parsed good. Working with Root object
                    if let Some(last) = &n.result.last() {
                        // updating last_id aka offset
                        self.last_update_id = last.update_id;
                    }
                    let mut message_array: Vec<Message> = Vec::with_capacity(n.result.len());
                    n.result
                        .iter()
                        .for_each(|obj| message_array.push(obj.message.clone()));
                    return Ok(message_array);
                }
            }
        }
    }

    async fn request(
        &self,
        method: impl Into<String>,
        data: Vec<(String, String)>,
    ) -> Result<String, reqwest::Error> {
        let method_name = method.into();
        let base_url = Url::parse_with_params::<Vec<(String, String)>, String, String>(
            format!("https://api.telegram.org/{}/{}", &self.token, &method_name).as_str(),
            data,
        )
        .unwrap();
        let response = reqwest::get(base_url).await?.text().await?;

        Ok(response)
    }
}
