pub mod schema;
pub mod models;

#[macro_use] extern crate prettytable;
#[macro_use] extern crate diesel;
#[macro_use] extern crate clap;

use prettytable::Table;
use clap::App;
use clockit::{clean_tasks, get_task, task_exists, establish_connection, create_task, delete_task, change_task_state, models::State, get_task_with_state, get_tasks};



fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();
    let connection = establish_connection();

    if matches.is_present("TASKNAME"){

        let input : &str = matches.value_of("TASKNAME").unwrap();

        if matches.is_present("start") {
            if task_exists(&connection, input) {
                let task = get_task(&connection, input).unwrap();
                if task.state.to_char() == State::Created.to_char() || task.state.to_char() == State::Paused.to_char() {
                    change_task_state(&connection, input, "started").unwrap();
                    println!("{} started", task.label);
                } else {
                    println!("{} is in state {} which doesn't allow it to be modified for started", task.label, task.state.to_sql());
                }
            } else {
                create_task(&connection, input);
                println!("Task {} created", input);
                change_task_state(&connection, input, "started").unwrap();
                println!("Task {} started", input);
            }
        }

        if matches.is_present("pause") {
            if task_exists(&connection, input){
                let task = get_task(&connection, input).unwrap();
                if task.state.to_char() == State::Started.to_char() {
                    change_task_state(&connection, input, "paused").unwrap();
                    println!("{} paused", task.label);
                }else{
                    println!("{} is in state {} which doesn't allow it to be paused", task.label, task.state.to_sql());
                }
            }
        }

        if matches.is_present("end") {
           let task = get_task(&connection, input).unwrap();
           if task.state.to_char() == State::Paused.to_char() || task.state.to_char() == State::Started.to_char() {
                change_task_state(&connection, input, "ended").unwrap();
                println!("{} ended", task.label);
            } else {
                println!("{} is in state {} which doesn't allow it to be paused", task.label, task.state.to_sql());
            }
        }

        if matches.is_present("show") {
            match get_task(&connection, input){
                Some(t) => println!("{}", t),
                None => println!("None task found with this name")
            }
        }

        if matches.is_present("delete") {
            match delete_task(&connection, input){
                Ok(_) => println!("{} deleted", input),
                Err(t) => println!("Error : {}", t),
            }
        }

    }

    if matches.is_present("show") {
        let mut table = Table::new();

        table.add_row(row!["Task", "Ellapsed time", "Statut", "Started on", "Ended on"]);
        if let Some(t) = get_tasks(&connection){
            t.iter().for_each(|x| {println!("{} :: {}", x.time, x.ellapsed_time()); table.add_row(row![x.label, x.readable_ellapsed_time(), x.state.to_sql(), x.begin_dt, x.end_dt]);});
        }else {
            println!("No task being registered");
        };
        table.printstd();
    }

    if matches.is_present("running") {
        match get_task_with_state(&connection, "started"){
            Some(t) => t.iter().for_each(|x| println!("{}", x)),
            None => println!("No task currently running"),
        };
    }

    if matches.is_present("clean") {
        println!("{} tasks deleted", clean_tasks(&connection));
    }
}
