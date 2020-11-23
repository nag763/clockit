extern crate console;
use console::style;

use chrono::{Duration, DateTime, Local, Utc, NaiveDateTime};
use super::schema::tasks;

///A state modelize the task situation
pub enum State{
    Created,
    Started,
    Paused,
    Ended,
}

///A task is a dated and timed element
pub struct Task{
    pub label : String,
    pub time : Duration,
    pub created_on : DateTime<Local>,
    pub begin_dt : DateTime<Local>,
    pub end_dt : DateTime<Local>,
    pub state : State,
}

#[derive(Queryable, Insertable)]
#[table_name="tasks"]
///A sql task is the transition between what is saved in the db and
///what is used in rustlang
pub struct SQLTask{
    pub label : String,
    pub time : i32,
    pub created_on : i32,
    pub begin_dt : i32,
    pub end_dt : i32,
    pub state : String,
}

impl State {
    
    ///Return a unique char associed to the
    ///given state
    pub fn to_char(&self) -> char {
        match self {
            State::Created => 'C',
            State::Started => 'S',
            State::Paused => 'P',
            State::Ended => 'E'
        }
    }

    ///Returns the order associed to the
    ///task
    pub fn to_order(&self) -> &str {
        match self {
            State::Created => "create",
            State::Started => "start",
            State::Paused => "pause",
            State::Ended => "end"
        }

    }

    ///Returns the sql equivalent of the 
    ///given task
    pub fn to_sql(&self) -> &str {
        match self {
            State::Created => "created",
            State::Started => "started",
            State::Paused => "paused",
            State::Ended => "ended"
        
        }
    }
    
    ///Gets a state from the sql order
    pub fn from_sql(input : &str) -> Result<State, &'static str> {
        match input {
            "created" => Ok(State::Created),
            "started" => Ok(State::Started),
            "paused" => Ok(State::Paused),
            "ended" => Ok(State::Ended),
            _ => Err("Not in expected values")
        }
    }

    ///Returns a state from the string order
    pub fn from_str(input : &str) -> Result<State, String> {
        match input.to_lowercase().as_str() {
            "create" | "c" => Ok(State::Created),
            "paused" | "p" => Ok(State::Paused),
            "ended" | "e" => Ok(State::Ended),
            "started" | "s" => Ok(State::Started),
            _ => Err(format!("{} not found in possibilities", input))
        }
    }
}

impl Task {

    ///Create a new task from the given
    ///label
    pub fn new(label : &str) -> Task {
        Task {
            label : label.to_string(), 
            time: Duration::seconds(0), 
            created_on : Local::now(), 
            begin_dt : Local::now(), 
            end_dt : Local::now(), 
            state : State::Created 
        }
    }
    
    ///Returns the ellapsed time since
    ///the task has been started
    pub fn ellapsed_time(&self) -> Duration {
        match self.state {
            State::Started => self.time + (Local::now() - self.begin_dt),
            _ => self.time
        }
    }

    ///Starts the task
    pub fn start(&mut self) -> Result<(), &'static str> {
        match self.state {
            State::Created | State::Paused => 
                {
                    self.begin_dt = Local::now();
                    self.state = State::Started;
                    Ok(())
                },
            _ => Err("The task is already started"),
        }
    }

    ///Ends the task
    ///
    ///A ended task can't be resumed
    pub fn end(&mut self) -> Result<(), &'static str> {
        match self.state {
            State::Started =>
                {
                    self.end_dt = Local::now();
                    self.state = State::Ended;
                    self.time = self.time  + (self.end_dt - self.begin_dt);
                    Ok(())
                },
            _ => {
                Err("The task isn't in the right state")
            }
        }
    }

    ///Pause the given task
    pub fn pause(&mut self) -> Result<(), &'static str> {
        match self.state {
            State::Started => 
                {
                    self.end_dt = Local::now();
                    self.time = self.time + (self.end_dt - self.begin_dt);
                    self.state = State::Paused;
                    Ok(())
                },
            _ => Err("Not the right state")
        }
    }

    ///Returns the ellapsed time as a readable string for the user
    pub fn readable_ellapsed_time(&self) -> String {
        time_to_readable(self.ellapsed_time().num_seconds())
    }

}

///Returns the time as a readable string
pub fn time_to_readable(secs : i64) -> String {
    
    const MINUTE : i64 = 60;
    const HOUR : i64 = MINUTE * 60;
    const DAY : i64 = HOUR * 24;
    match secs {
        x if x < MINUTE => format!("{0}s", x),
        x if x < HOUR => format!("{0}m{1}s", x/MINUTE, x%MINUTE),
        x if x < DAY => format!("{0}h{1}m{2}s", x/HOUR, x/MINUTE%MINUTE, x%MINUTE),
        x => format!("{0}j{1}h{2}m{3}s", x/DAY, x/(HOUR)%24, x%HOUR/MINUTE, x%MINUTE)
    }
}

///Format the task for the stdout
impl std::fmt::Display for Task {

    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {

        let mut output = String::new(); 
        output.push_str(&format!("[{}] ", self.state.to_char()));
        output.push_str(&self.label);
        output.push_str(&format!(" ({})", time_to_readable(self.ellapsed_time().num_seconds())));
        output.push_str(&format!(" created on {}", self.begin_dt.to_rfc2822()));
        
        match self.state {
            State::Created => write!(f, "{}", output),
            State::Started => write!(f, "{}", style(output).yellow()), 
            State::Paused => write!(f, "{}", style(output).blue()), 
            State::Ended => {output.push_str(&format!(" and ended on {}", self.end_dt)); write!(f, "{}", style(output).yellow())}, 
        }
    }

}

impl SQLTask {
    
    ///Creates a sql task from a given task
    pub fn from_task(tsk : Task) -> SQLTask {
        SQLTask { 
            label : tsk.label, 
            time : (tsk.time.num_seconds() as i32).into(), 
            created_on : (tsk.created_on.timestamp() as i32).into(), 
            begin_dt : (tsk.begin_dt.timestamp() as i32).into(), 
            end_dt : (tsk.end_dt.timestamp() as i32).into(), 
            state : tsk.state.to_sql().into()
        }
    }

    ///Creates a task from a sql task
    pub fn as_task(&self) -> Task {
        let state : State = State::from_sql(self.state.as_str()).expect("Error in the database column named state");
        Task { 
            label : self.label.clone(), 
            time : Duration::seconds((self.time as i32).into()), 
            created_on :  (DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.created_on as i64, 0), Utc)).with_timezone(&Local),
            begin_dt : (DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.begin_dt as i64, 0), Utc)).with_timezone(&Local), 
            end_dt : (DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.end_dt as i64, 0), Utc)).with_timezone(&Local),
            state}  
    }
}
