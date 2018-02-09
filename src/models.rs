extern crate chrono;

use self::chrono::{DateTime, Utc};
use super::schema::ctl;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub complete_date: Option<DateTime<Utc>>,
    pub discrete: bool,
}

#[derive(Insertable)]
#[table_name="ctl"]
pub struct NewTask {
	pub name: String,
	pub discrete: bool,
}
