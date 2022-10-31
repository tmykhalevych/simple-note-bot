use telegram_bot::SendMessage;
use async_trait::async_trait;
use super::{Controller, BaseController};

pub struct DefaultController<'a> {
    pub(super) base: BaseController<'a>
}

impl<'a> DefaultController<'a> {
    const RESPONSE_MSG: &str = "Sorry, I don't understand you... Try sending some audio message or text";
}

#[async_trait(?Send)]
impl<'a> Controller<'a> for DefaultController<'a> {
    async fn handle(&mut self) {
        let user_name = &self.base.user.first_name;
        let response = format!("Hey {}! {}", user_name, DefaultController::RESPONSE_MSG);
        self.base.api.send(SendMessage::new(&self.base.chat, response)).await.ok();
    }
}
