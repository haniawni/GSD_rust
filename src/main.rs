#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
#[macro_use] extern crate diesel;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;
extern crate chrono;
extern crate get_stuff_done;

mod db;
mod models;


use rocket_contrib::{Template, Json};
use rocket::response::Redirect;
use db::DbConn;
use self::models::*;
use self::diesel::prelude::*;
use self::get_stuff_done::*;



// ~~~~~~~~~ ROOT ROUTES

#[get("/")]
fn index() -> Redirect {
	Redirect::to("/Jebidiah/eat")
}

// ~~~~~~~~~~ CTL/Execution Routes

#[get("/ctl", format = "application/json")]
pub fn get_ctl(conn: DbConn) -> Json<Vec<Task>> {
	use get_stuff_done::schema::ctl::dsl::*;

    let results = ctl.filter(complete_date.is_null())
        .load::<Task>(DbConn)
        .expect("Error loading CTL")
        .map(|task| Json(task));

        return !Json;
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