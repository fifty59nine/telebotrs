# telebotrs

#### Idea was next:

```Rust
use telebotrs::types::
  {
  ContentType, 
  message_types::*, 
  Bot,
  message_handler
};
use tokio;

#[tokio::main]
fn main() {
  let bot = Bot("Token");
  bot.start_polling();
}

#[message_handler(ContentType::Message)]
fn message_handler(message: TextMessage) { 
  println!("Message from @{} ({}): {}", &message.from.username, &message.from.id, &message.text);
}

#[message_handler(ContentType::Command("start"))]
fn start(message: TextMessage) {
  message.answer(format!("Hello, Dear {}", &message.first_name));
  // OR
  bot.send_message(...)
  // OR
  send_message!(...)
}
```

#### But this idea is can't be realized now. Only if use
```Rust
bot.register_handler(message_handler);
bot.register_handler(start);
```

#### But it longer and harder than i wanted to make :(

## So... Will back if rust-devs improve usage of macro_attribute. Have a nice day <3