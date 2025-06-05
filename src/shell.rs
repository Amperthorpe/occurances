use crate::occu_core::Event;
use chrono::{DateTime, Local};
use ordermap::OrderMap;
use std::{
    error::Error,
    io::{self, Write},
};
use uuid::Uuid;

#[derive(Debug)]
enum OccuError {
    Error(Box<dyn Error>),

    EventExists,

    InvalidUuidTimestamp,
    InvalidIndex,

    RequiresArgs(u8),
}

impl OccuError {
    fn handle(&self) {
        match self {
            Self::Error(e) => eprintln!("Wrapped error: {}", *e),
            Self::EventExists => eprintln!("Duplicate Event exists in memory."),
            Self::InvalidUuidTimestamp => eprintln!("Couldn't parse timestamp from UUID v7."),
            Self::InvalidIndex => eprintln!("Invalid index."),
            Self::RequiresArgs(count) => eprintln!("Command requires {} argument(s).", count),
        }
    }
}

type EventMap = OrderMap<Uuid, Event>;

pub fn run_shell() {
    let mut event_map: EventMap = OrderMap::new();

    'main: loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout.");
        let mut input = String::new();
        let _byte_count = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read_line.");

        // Split input and continue loop on empty
        let input_vec: Vec<&str> = input.split_whitespace().collect();
        if input_vec.is_empty() {
            continue 'main;
        }

        // Match commands
        let cmd_fn: CmdFunc = match input_vec[0] {
            "new" => cmd_new_event,
            "list" | "ls" => cmd_list_events,
            "remove" | "rm" => cmd_remove_event,

            "exit" | "quit" => std::process::exit(0),
            _ => continue 'main,
        };

        // Execute cmd function and handle errors.
        match cmd_fn(&input_vec[1..], &mut event_map) {
            Ok(()) => (),
            Err(oe) => oe.handle(),
        }
    }
}

fn add_event(
    title: String,
    description: String,
    event_map: &mut EventMap,
) -> Result<(), OccuError> {
    let (uuid, new_event) = Event::new_for_map(title, description);
    match event_map.insert(uuid, new_event) {
        Some(_) => Err(OccuError::EventExists),
        None => Ok(()),
    }
}

fn list_events(event_map: &EventMap) -> Result<(), OccuError> {
    println!("====== Events ======");
    if event_map.is_empty() {
        println!("    No events.");
        Ok(())
    } else {
        for (i, (k, event)) in event_map.iter().enumerate() {
            // Iterate through events, extracting and displaying the UUID v7 timestamp.
            let unix_timestamp = match k.get_timestamp() {
                Some(ts) => {
                    let (secs, nsecs) = ts.to_unix();
                    DateTime::from_timestamp(secs as i64, nsecs)
                }
                None => return Err(OccuError::InvalidUuidTimestamp),
            };
            let str_timestamp = unix_timestamp
                .ok_or(OccuError::InvalidUuidTimestamp)?
                .with_timezone(&Local)
                .format("%Y-%m-%d %I:%M %p");
            println!("{i}. {str_timestamp}\n {event:?}\n");
        }
        Ok(())
    }
}

fn remove_event(event_idx: usize, event_map: &mut EventMap) -> Result<(), OccuError> {
    let event = event_map.get_index(event_idx);

    if let Some((_, event)) = &event {
        println!(
            "Are you sure you want to delete event \"{}\"? y/N",
            event.title()
        );
        if get_yes_no() {
            event_map.remove_index(event_idx);
        }
    };
    Ok(())
}

// CmdFunc functions - must match this type signature.
type CmdFunc = fn(&[&str], &mut EventMap) -> Result<(), OccuError>;

fn cmd_new_event(args: &[&str], event_map: &mut EventMap) -> Result<(), OccuError> {
    if args.len() < 2 {
        return Err(OccuError::RequiresArgs(2));
    }
    let title = args[0].to_string();
    let description = args[1].to_string();
    add_event(title, description, event_map)
}

fn cmd_list_events(_args: &[&str], event_map: &mut EventMap) -> Result<(), OccuError> {
    list_events(event_map)
}

fn cmd_remove_event(args: &[&str], event_map: &mut EventMap) -> Result<(), OccuError> {
    if args.len() > 1 {
        return Err(OccuError::RequiresArgs(1));
    }
    // Parse index
    let idx = match args[0].parse::<usize>() {
        Ok(i) => i,
        Err(e) => return Err(OccuError::Error(Box::new(e))),
    };

    remove_event(idx, event_map)
}

// Misc
fn get_yes_no() -> bool {
    let mut input = String::new();
    let _byte_count = io::stdin().read_line(&mut input);
    if input.is_empty() {
        return false;
    }
    match input.to_lowercase().chars().next() {
        Some('y') => true,
        Some('n') => false,
        _ => false,
    }
}
