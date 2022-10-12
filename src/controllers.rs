pub mod default;
pub mod text;

use default::*;
use text::*;
use telegram_bot::{Message, Api};
use diesel::PgConnection;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Controller<'a> {
    async fn handle(&mut self);
}

pub struct Builder {
    message: Message
}

impl Builder {
    pub fn new(message: Message) -> Self {
        Self {
            message: message
        }
    }

    pub fn with<'a>(self, db: &'a mut PgConnection, api: &Api) -> Box<dyn Controller<'a>> {
        // todo: temporary code here
        if true {
            Box::new(DefaultController { message: self.message, api: api.clone() })
        }
        else {
            Box::new(TextController { text: "".to_owned(), message: self.message, api: api.clone() }) 
        }
    }
}
