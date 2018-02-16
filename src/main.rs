#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;

use rocket_contrib::Template;
use rocket::response::Redirect;

mod db;
mod eat;

#[get("/")]
fn index() -> Redirect {
	Redirect::to("/Jebidiah/eat")
}

fn main() {
    rocket::ignite()
    .manage(db::init_pool())
    .mount("/", routes![index, eat::eat])
    .attach(Template::fairing())
    .launch();
} 