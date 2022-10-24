use telegram_bot::{Api, User, MessageChat, SendMessage};
use async_trait::async_trait;
use super::Controller;

pub struct TextController {
    pub(super) text: String,
    pub(super) user: User,
    pub(super) chat: MessageChat,
    pub(super) api: Api
}

#[async_trait(?Send)]
impl<'a> Controller<'a> for TextController {
    async fn handle(&mut self) {

        // todo: Implement something more sophisticated

        let user_name = &self.user.first_name;
        let response = format!("Hey {}! You just wrote '{}'", user_name, &self.text);
        self.api.send(SendMessage::new(&self.chat, response)).await.ok();

    }
}
