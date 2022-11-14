use telegram_bot::SendMessage;
use async_trait::async_trait;
use diesel::PgConnection;
use super::{Controller, BaseController};
pub use std::str::FromStr;

use crate::schema::{*, users::dsl::*};
use crate::models::{User, NewUser};
use diesel::RunQueryDsl;
use diesel::prelude::*;

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
        let user_id_str = self.base.user.id.to_string();
        let user = users.filter(external_id.eq(&user_id_str));
        let res = user.load::<User>(self.base.db).ok();
        if let Some(user_vec) = res {
            if !user_vec.is_empty() {
                // TODO: log here and respond that already started
                return;
            }
        }

        let new_user = NewUser {
            name: &self.base.user.first_name,
            external_id: &user_id_str
        };

        let res = diesel::insert_into(users)
            .values(new_user)
            .execute(self.base.db);

        if let Err(err) = res {
            // TODO: log error here
            return;
        }

        // TODO: log and respond with greeting message
    }

    fn delete_user(&mut self) {
        let user_id_str = self.base.user.id.to_string();
        let user = users.filter(external_id.eq(user_id_str));
        let res = diesel::delete(user)
            .execute(self.base.db);

        if let Err(err) = res {
            // TODO: log error here
        }

        // TODO: log and respond with good bye message
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
