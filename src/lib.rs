#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;


pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{NewTask, Task};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn append_ctl(conn: &PgConnection, name: &str, discrete: bool) -> Task {
	use schema::ctl;
	let new_task = NewTask {
		name: name,
		discrete: discrete
	};

	diesel::insert_into(ctl::table)
		.values(&new_task)
		.get_result(conn)
		.expect("Error Inserting new task to CTL")
}