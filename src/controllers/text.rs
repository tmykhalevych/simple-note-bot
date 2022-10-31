use telegram_bot::SendMessage;
use async_trait::async_trait;
use super::{Controller, BaseController};

pub struct TextController<'a> {
    pub(super) text: String,
    pub(super) base: BaseController<'a>
}

#[async_trait(?Send)]
impl<'a> Controller<'a> for TextController<'a> {
    async fn handle(&mut self) {

        // todo: Implement something more sophisticated

        let user_name = &self.base.user.first_name;
        let response = format!("Hey {}! You just wrote '{}'", user_name, &self.text);
        self.base.api.send(SendMessage::new(&self.base.chat, response)).await.ok();

    }
}
