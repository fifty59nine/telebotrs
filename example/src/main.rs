use telebotrs::{
    message_handler,
    types::{
        data_types::{Chat, User},
        message_types::TextMessage,
        ContentType::Message,
    },
    Bot,
};

fn main() {
    let bot = Bot::new();
}

#[message_handler(Message)]
fn msg_hndl(msg: TextMessage) {
    println!("Message from {}: {}", msg.from.username, msg.text);
}
