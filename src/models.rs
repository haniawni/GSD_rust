extern crate chrono;

use self::chrono::{DateTime, Utc};
// use diesel::dsl::*;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub complete_date: Option<DateTime<Utc>>,
    pub discrete: bool,
}