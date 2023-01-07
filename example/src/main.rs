use telebotrs::{
    message_handler,
    types::{message_types::TextMessage, ContentType::Message,
        data_types::{Chat, User}
    },
};


fn main() {
    let _txtmsg = TextMessage::new(0, 0, Chat::new(0, "Test", "Test", "test"), User::new(0, false, "Test", "Test", "UA"), "Hello bot!"); 
}

#[message_handler(Message)]
fn msg_hndl(msg: TextMessage) {
    println!("Message from {}: {}", msg.from.username, msg.text);
}
