use telegram_bot::{Voice, User, MessageChat, Api, SendMessage};
use async_trait::async_trait;
use super::Controller;

pub struct VoiceController {
    pub(super) audio: Voice,
    pub(super) user: User,
    pub(super) chat: MessageChat,
    pub(super) api: Api
}

#[async_trait(?Send)]
impl<'a> Controller<'a> for VoiceController {
    async fn handle(&mut self) {
        // todo: Implement something more sophisticated

        let user_name = &self.user.first_name;
        let response = format!("Hey {}! Got your audio message", user_name);
        self.api.send(SendMessage::new(&self.chat, response)).await.ok();
    }
}
