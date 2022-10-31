use telegram_bot::{Voice, SendMessage};
use async_trait::async_trait;
use diesel::PgConnection;
use super::{Controller, BaseController};

pub struct VoiceController<'a> {
    pub(super) audio: Voice,
    pub(super) base: BaseController<'a>
}

#[async_trait(?Send)]
impl<'a> Controller<'a> for VoiceController<'a> {
    async fn handle(&mut self) {
        // TODO: implement
        // 1) get user from db
        // 2) add voice message to user collection
        // 3) respond user with correct message
    }
}
