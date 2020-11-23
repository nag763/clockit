pub mod models;
pub mod schema;


#[macro_use] extern crate diesel;
use self::models::{Task, SQLTask};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

///Establishs the connection with the sqli db
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

///Get a task saved in db
pub fn get_task<'a>(conn : &SqliteConnection, input : &str) -> Option<Task> {
    use self::schema::tasks::dsl::*;
    match tasks
        .filter(label.eq(input))
        .first::<SQLTask>(conn)
        {
            Ok(t) => Some(t.as_task()),
            _ => None
        }
}

///Get a task saved with the given state
pub fn get_task_with_state<'a>(conn : &SqliteConnection, criteria : &str) -> Option<Vec::<Task>> {
    use self::schema::tasks::dsl::*;
    match tasks
        .filter(state.eq(criteria))
        .load::<SQLTask>(conn)
        {
            Ok(t) if t.len() != 0 => Some(t.iter().map(|x| x.as_task()).collect()),
            _ => None
        }
}

///Get all tasks saved
pub fn get_tasks<'a>(conn : &SqliteConnection) -> Option<Vec::<Task>> {
    use self::schema::tasks::dsl::*;
    match tasks
        .order_by(created_on.desc())
        .load::<SQLTask>(conn)
        {
            Ok(t) if t.len() != 0 => Some(t.iter().map(|x| x.as_task()).collect()),
            _ => None
        }
}

///Create a task with the given name
pub fn create_task<'a>(conn: &SqliteConnection, label : &str) -> usize {
    use schema::tasks;

    let new_task = SQLTask::from_task(Task::new(label));

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)
        .expect("Error saving new task")
}

///Change the state of a saved task
pub fn change_task_state<'a>(conn: &SqliteConnection, input : &str, new_state :&str) -> Result<(), String> {
    use self::schema::tasks::dsl::*;
    use models::State;

    let sql_state : String = match State::from_str(new_state){
        Ok(s) => s.to_sql().to_string(),
        Err(e) => return Err(e),
    };

    match diesel::update(tasks.find(input)).set(state.eq(sql_state)).execute(conn){
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

///Return true if the tasks exists
pub fn task_exists<'a>(conn: &SqliteConnection, input : &str) -> bool {
    use self::schema::tasks::dsl::*;

    let results = tasks
        .find(input)
        .load::<SQLTask>(conn)
        .expect("Err");

    match results.len() {
        0 => false,
        _ => true
    }
}

///Delete all ended tasks with passed date time
pub fn clean_tasks<'a>(conn: &SqliteConnection) -> usize {
    use chrono::{Local};
    use self::schema::tasks::dsl::*;
    use models::State;

    diesel::delete(
            tasks
            .filter(end_dt.lt(Local::now().timestamp() as i32))
            .filter(state.like(State::Ended.to_sql()))
            )
        .execute(conn)
        .expect("Error while processing statement")
}

///Delete a task with the given name
pub fn delete_task<'a>(conn: &SqliteConnection, input :&str) -> Result<(), String> {

    use self::schema::tasks::dsl::*;

    match diesel::delete(tasks.find(input))
        .execute(conn)
        .expect("Error while processing statement")
        {
            x if x == 0 => Err(format!("{} not found", input)),
            x if x == 1 => Ok(()),
            _ => Err("Database error".into()),
        }
}
