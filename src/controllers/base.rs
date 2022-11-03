pub use telegram_bot::*;
pub use diesel::PgConnection;
pub use diesel::prelude::*;

pub(super) struct BaseController<'a> {
    pub(super) user: User,
    pub(super) chat: MessageChat,
    pub(super) api: Api,
    pub(super) db: &'a mut PgConnection
}
