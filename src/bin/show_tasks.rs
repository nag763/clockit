extern crate diesel;
extern crate clockit;

use self::models::*;
use diesel::prelude::*;
use clockit::*;

fn main() {
    use self::schema::tasks::dsl::*;

    let connection = establish_connection();
    let results = tasks
        .load::<SQLTask>(&connection)
        .expect("Error loading tasks");

    println!("Displaying {} tasks", results.len());
    for task in results {
        println!("{}", task.label);
        println!("-----------\n");
    }
}
