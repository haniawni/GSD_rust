#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate dotenv;
extern crate r2d2;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2_diesel;
extern crate chrono;

pub mod schema;
pub mod models;
mod db;

use rocket_contrib::{Template, Json};
use rocket::response::Redirect;
use db::DbConn;
use self::models::*;
use self::diesel::prelude::*;



// ~~~~~~~~~ ROOT ROUTES

#[get("/")]
fn index() -> Redirect {
	Redirect::to("/Jebidiah/eat")
}

// ~~~~~~~~~~ CTL/Execution Routes

#[get("/ctl", format = "application/json")]
pub fn get_ctl(conn: &DbConn) -> Json<Vec<Task>> {
	use schema::ctl::dsl::*;

    let results = ctl.filter(complete_date.is_null())
        .load::<Task>(DbConn)
        .expect("Error loading CTL")
        .map(|task| Json(task));

        return results;
}

// ~~~~~~~~~~~ EAT ROUTES: testing Rocket
#[derive(Serialize)]
struct Context {
	name: String,
	foods: Vec<String>,
}

#[get("/<name>/eat")]
fn eat(name: String) -> Template {
	let context = Context{
		name: name,
		foods: vec!["Bagel", "Beef", "Banana"].iter().map(|s| s.to_string()).collect()
	};

	Template::render("eat",&context)
}


fn main() {
    rocket::ignite()
    .manage(db::init_pool())
    .mount("/", routes![index, eat, get_ctl])
    .attach(Template::fairing())
    .launch();
} 