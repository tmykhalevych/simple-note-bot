pub mod default;
pub mod text;
pub mod voice;

use default::*;
use text::*;
use voice::*;
use telegram_bot::{Update, MessageKind, UpdateKind, Api};
use diesel::PgConnection;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Controller<'a> {
    async fn handle(&mut self);
}

pub struct Builder {
    update: Update
}

impl Builder {
    pub fn new(update: Update) -> Self {
        Self {
            update: update
        }
    }

    pub fn with<'a>(self, db: &'a mut PgConnection, api: &Api) -> Result<Box<dyn Controller<'a>>, String> {
        if let UpdateKind::Message(message) = self.update.kind {
            match &message.kind {
                MessageKind::Text { data, .. } => Ok(Box::new(TextController { text: data.clone(), message: message, api: api.clone() })),
                MessageKind::Voice { data } => Ok(Box::new(VoiceController { audio: data.clone(), message: message, api: api.clone() })),
                _ => Ok(Box::new(DefaultController { message: message, api: api.clone() }))
            }
        }
        else {
            Err("The update in not a message".to_owned())
        }
    }
}
