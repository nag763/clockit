extern crate clockit;
extern crate diesel;

use self::diesel::prelude::*;
use self::clockit::*;
use std::env::args;

fn main() {
    use clockit::schema::tasks::dsl::{tasks, label};

    let id = args().nth(1).expect("two arg rqrd")
        .parse::<String>().expect("Invalid");


    let input = args().nth(2).expect("two arg rqrd")
        .parse::<String>().expect("Invalid");

    let connection = establish_connection();


    let _ = diesel::update(tasks.find(id))
        .set(label.eq(input))
        .execute(&connection)
        .unwrap_or_else(|_| panic!("Unable to find task"));

    println!("Modified");
}
