extern crate clockit;
use std::env::args;

fn main() {
    use clockit::{establish_connection, task_exists};

    let id = args().nth(1).expect("one arg rqrd")
        .parse::<String>().expect("Invalid");

    let connection = establish_connection();

    println!("{}", task_exists(&connection, &id));

}
