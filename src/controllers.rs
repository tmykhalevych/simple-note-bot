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

    pub fn with<'a>(self, db: &'a mut PgConnection, api: &Api) -> Option<Box<dyn Controller<'a>>> {
        if let UpdateKind::Message(message) = self.update.kind {
            match message.kind {
                MessageKind::Text { data, .. } =>
                    return Some(Box::new(TextController { text: data, user: message.from, chat: message.chat, api: api.clone() })),
                MessageKind::Voice { data } =>
                    return Some(Box::new(VoiceController { audio: data, user: message.from, chat: message.chat, api: api.clone() })),
                _ =>
                    return Some(Box::new(DefaultController { user: message.from, chat: message.chat, api: api.clone() }))
            }
        }
        
        // TODO: consider handling other update kinds (edit message, post, polls, etc.)
        None
    }
}
