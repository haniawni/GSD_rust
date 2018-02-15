extern crate get_stuff_done;
extern crate diesel;

use self::get_stuff_done::*;
use std::io::{stdin};

fn main() {
    let connection = establish_connection();

    println!("What is the task?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    println!("\nOk! Assuming it's not discrete.)\n");
    let discrete = false;

    let task = append_ctl(&connection, name, discrete);
    println!("\nSaved task {} with id {}", name, task.id);
}