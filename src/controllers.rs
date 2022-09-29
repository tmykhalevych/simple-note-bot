// use crate::models::*;
use telegram_bot::{Message, Api, SendMessage};
use diesel::PgConnection;
use async_trait::async_trait;

// fn create_user<'a>(user: &'a User) -> Result<(), &'static str>;
// fn destroy_user<'a>(user: &'a User) -> Result<(), &'static str>;

#[async_trait(?Send)]
pub trait Controller<'a> {
    fn create_with(db: &'a mut PgConnection, api: &Api) -> Self;
    async fn handle(&mut self, message: Message);
}

pub struct DefaultController {
    api: Api
}

impl DefaultController {
    const RESPONSE_MSG: &str = "Sorry, I don't understand you... Try sending some audio message or text";
}

#[async_trait(?Send)]
impl<'a> Controller<'a> for DefaultController {
    #[allow(unused_variables)]
    fn create_with(db: &'a mut PgConnection, api: &Api) -> Self
    {
        Self {
            api: api.clone()
        }
    }

    async fn handle(&mut self, message: Message) {
        // todo: Implement something more sophisticated
        let user_name = &message.from.first_name;
        let response = format!("Hey {}! {}", user_name, DefaultController::RESPONSE_MSG);
        self.api.send(SendMessage::new(message.chat, response)).await.ok();
    }
}
