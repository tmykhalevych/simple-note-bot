use crate::schema::{notes, users};
use diesel::prelude::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub external_id: String
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = notes)]
pub struct Note {
    pub id: i32,
    pub user_id: i32,
    pub url: String,
    pub timestamp: chrono::NaiveDateTime,
    pub published: bool
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub external_id: &'a str
}

#[derive(Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote<'a> {
    pub user_id: i32,
    pub url: &'a str,
    pub timestamp: chrono::NaiveDateTime
}
