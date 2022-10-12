use telegram_bot::{Message, Api, SendMessage};
use async_trait::async_trait;
use super::Controller;

pub struct TextController {
    pub text: String,
    pub message: Message,
    pub api: Api
}

#[async_trait(?Send)]
impl<'a> Controller<'a> for TextController {
    async fn handle(&mut self) {

        // todo: Implement something more sophisticated

        let user_name = &self.message.from.first_name;
        let response = format!("Hey {}! You just wrote '{}'", user_name, &self.text);
        self.api.send(SendMessage::new(&self.message.chat, response)).await.ok();
    }
}
