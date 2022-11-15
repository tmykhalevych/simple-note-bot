pub mod default;
pub mod text;
pub mod voice;
pub mod command;
pub mod base;

use default::*;
use text::*;
use voice::*;
use command::*;
use base::*;
use crate::views::*;

use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Controller<'a> {
    async fn handle(&mut self);
    fn get_view(&self) -> Option<Box<dyn View>> {
        None
    }
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

    pub fn with<'a>(self, db: &'a mut PgConnection, api: &Api) -> Option<Box<dyn Controller<'a> + 'a>> {
        // TODO: remove useless copy here
        if let UpdateKind::Message(message) = self.update.kind.clone() {
            let base = BaseController {
                user: message.from,
                chat: message.chat,
                api: api.clone(),
                db: db
            };

            match message.kind {
                MessageKind::Text { data, .. } => return Some(self.on_text(data, base)),
                MessageKind::Voice { data }     => return Some(self.on_voice(data, base)),
                _                                      => return Some(self.default(base))
            }
        }
    
        // TODO: consider handling other update kinds (edit message, post, polls, etc.)
        None
    }

    fn on_text<'a>(self, text: String, base: BaseController<'a>) -> Box<dyn Controller<'a> + 'a> {
        if let Some(command) = Command::from_str(&text).ok() {
            return Box::new(CommandController { command: command, base: base });
            // TODO: consider handling unsupported commands here, e.g. reply with proper message
        }

        Box::new(TextController { text: text.clone(), base: base })
    }

    fn on_voice<'a>(self, audio: Voice, base: BaseController<'a>) -> Box<dyn Controller<'a> + 'a> {
        Box::new(VoiceController { audio: audio.clone(), base: base })
    }

    fn default<'a>(self, base: BaseController<'a>) -> Box<dyn Controller<'a> + 'a> {
        Box::new(DefaultController { base: base })
    }
}
