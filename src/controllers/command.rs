use telegram_bot::SendMessage;
use async_trait::async_trait;
use diesel::PgConnection;
use super::{Controller, BaseController};
pub use std::str::FromStr;

pub(super) enum Command {
    Start,
    Stop,
    GetNotesForToday,
    GetAllNotes
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "/start"               => Ok(Command::Start),
            "/stop"                => Ok(Command::Stop),
            "/get_notes_for_today" => Ok(Command::GetNotesForToday),
            "/get_all_notes"       => Ok(Command::GetAllNotes),
            _                      => Err(()),
        }
    }
}

pub struct CommandController<'a> {
    pub(super) command: Command,
    pub(super) base: BaseController<'a>
}

impl<'a> CommandController<'a> {
    fn create_user(&mut self) {
        // TODO: implement
    }

    fn delete_user(&mut self) {
        // TODO: implement
    }

    fn get_notes_for_today(&mut self) {
        // TODO: implement
    }

    fn get_all_notes(&mut self) {
        // TODO: implement
    }
}

#[async_trait(?Send)]
impl<'a> Controller<'a> for CommandController<'a> {
    async fn handle(&mut self) {
        match self.command {
            Command::Start            => self.create_user(),
            Command::Stop             => self.delete_user(),
            Command::GetNotesForToday => self.get_notes_for_today(),
            Command::GetAllNotes      => self.get_all_notes()
        }
    }
}
