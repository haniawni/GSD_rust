extern crate GSD;
extern crate diesel;

use self::GSD::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use GSD::schema::ctl::dsl::*;

    let connection = establish_connection();
    let results = ctl  //.filter(published.eq(true))
        .limit(5)
        .load::<Task>(&connection)
        .expect("Error loading CTL");

    println!("Displaying {} tasks", results.len());
    for task in results {
        println!("{}:  {}", task.id, task.name);
    }
}