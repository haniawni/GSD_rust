use rocket_contrib::Template;

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

	Template::render("ctl",&context)
}