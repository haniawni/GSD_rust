#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Template;
use rocket::response::Redirect;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }
#[derive(Serialize)]
struct Context {
	name: String,
	foods: Vec<String>,
}

#[get("/")]
fn index() -> Redirect {
	Redirect::to("/Jebidiah/eat")
}

#[get("/<name>/eat")]
fn eat(name: String) -> Template {
	let context = Context{
		name: name,
		foods: vec!["Bagel", "Beef", "Banana"].iter().map(|s| s.to_string()).collect()
	};

	Template::render("ctl",&context)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, eat])
    .attach(Template::fairing())
    .launch();
} 