use telegram_bot::{Message, Api, SendMessage};
use diesel::PgConnection;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Controller<'a> {
    fn with(&mut self, db: &'a mut PgConnection, api: &Api) -> &mut Self where Self: Sized;
    async fn handle(&mut self);
}

pub struct DefaultController {
    message: Message,
    api: Option<Api>
}

impl DefaultController {
    pub fn new(message: Message) -> Self {
        Self {
            message: message,
            api: None
        }
    }

    const RESPONSE_MSG: &str = "Sorry, I don't understand you... Try sending some audio message or text";
}

#[async_trait(?Send)]
impl<'a> Controller<'a> for DefaultController {
    #[allow(unused_variables)]
    fn with(&mut self, db: &'a mut PgConnection, api: &Api) -> &mut Self where Self: Sized {
        self.api = Some(api.clone());
        self
    }

    async fn handle(&mut self) {
        assert!(self.api.is_some());
        let api = self.api.clone().unwrap();

        // todo: Implement something more sophisticated

        let user_name = &self.message.from.first_name;
        let response = format!("Hey {}! {}", user_name, DefaultController::RESPONSE_MSG);
        api.send(SendMessage::new(&self.message.chat, response)).await.ok();
    }
}

pub struct TextController {
    text: String,
    message: Message,
    api: Option<Api>
}

impl TextController {
    pub fn new(message: Message, text: String) -> Self {
        Self {
            text: text,
            message: message,
            api: None
        }
    }
}

#[async_trait(?Send)]
impl<'a> Controller<'a> for TextController {
    #[allow(unused_variables)]
    fn with(&mut self, db: &'a mut PgConnection, api: &Api) -> &mut Self where Self: Sized {
        self.api = Some(api.clone());
        self
    }

    async fn handle(&mut self) {
        assert!(self.api.is_some());
        let api = self.api.clone().unwrap();

        // todo: Implement something more sophisticated

        let user_name = &self.message.from.first_name;
        let response = format!("Hey {}! You just wrote '{}'", user_name, &self.text);
        api.send(SendMessage::new(&self.message.chat, response)).await.ok();
    }
}
