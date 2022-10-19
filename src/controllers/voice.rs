use telegram_bot::{Message, Voice, Api, SendMessage};
use async_trait::async_trait;
use super::Controller;

pub struct VoiceController {
    pub audio: Voice,
    pub message: Message,
    pub api: Api
}

#[async_trait(?Send)]
impl<'a> Controller<'a> for VoiceController {
    async fn handle(&mut self) {
        // todo: Implement something more sophisticated

        let user_name = &self.message.from.first_name;
        let response = format!("Hey {}! Got your audio message", user_name);
        self.api.send(SendMessage::new(&self.message.chat, response)).await.ok();
    }
}
