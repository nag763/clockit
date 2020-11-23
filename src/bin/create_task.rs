extern crate clockit;
extern crate diesel;

use clockit::*;

use std::env::args;

fn main() {
    let connection = establish_connection();

    let id = args().nth(1).expect("one arg rqrd")
        .parse::<String>().expect("Invalid");
    
    create_task(&connection, &id);
}
