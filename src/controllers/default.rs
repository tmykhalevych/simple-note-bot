use telegram_bot::{User, Api, MessageChat, SendMessage};
use async_trait::async_trait;
use super::Controller;

pub struct DefaultController {
    pub(super) user: User,
    pub(super) chat: MessageChat,
    pub(super) api: Api
}

impl DefaultController {
    const RESPONSE_MSG: &str = "Sorry, I don't understand you... Try sending some audio message or text";
}

#[async_trait(?Send)]
impl<'a> Controller<'a> for DefaultController {
    async fn handle(&mut self) {
        let user_name = &self.user.first_name;
        let response = format!("Hey {}! {}", user_name, DefaultController::RESPONSE_MSG);
        self.api.send(SendMessage::new(&self.chat, response)).await.ok();
    }
}
