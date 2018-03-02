extern crate get_stuff_done;
extern crate diesel;
extern crate chrono;

use self::get_stuff_done::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use get_stuff_done::schema::ctl::dsl::*;

    let connection = establish_connection();
    let results = ctl.filter(complete_date.is_null())
        .limit(5)
        .load::<Task>(&connection)
        .expect("Error loading CTL");

    println!("Displaying {} tasks", results.len());
    for task in results {
        println!("{}:  {}", task.id, task.name);
    }
}