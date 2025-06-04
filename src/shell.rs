use crate::occu_core::Event;
use chrono::DateTime;
use ordermap::OrderMap;
use std::io;
use uuid::Uuid;

#[derive(Debug)]
enum OccuError {
    InvalidUuidTimestamp,
    EventExists,
    RequiresArgs(u8),
}

impl OccuError {
    fn handle(&self) {
        match self {
            Self::InvalidUuidTimestamp => eprintln!("Couldn't parse timestamp from UUID v7."),
            Self::EventExists => eprintln!("Event already exists."),
            Self::RequiresArgs(count) => eprintln!("Command requires {} argument(s).", count),
        }
    }
}

type EventMap = OrderMap<Uuid, Event>;

pub fn run_shell() {
    let mut event_map: EventMap = OrderMap::new();

    'main: loop {
        let mut input = String::new();
        let _byte_count = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read_line.");

        // Split input
        let input_vec: Vec<&str> = input.split_whitespace().collect();

        // Match commands
        let cmd_fn: CmdFunc = match input_vec[0] {
            "new" => cmd_new_event,
            "list" => cmd_list_events,

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
    for (k, event) in event_map {
        // Iterate through events, extracting and displaying the UUID v7 timestamp.
        let unix_timestamp = match k.get_timestamp() {
            Some(ts) => {
                let (secs, nsecs) = ts.to_unix();
                DateTime::from_timestamp(secs as i64, nsecs)
            }
            None => return Err(OccuError::InvalidUuidTimestamp),
        };
        let str_timestamp = unix_timestamp.ok_or(OccuError::InvalidUuidTimestamp)?;
        println!("{str_timestamp}:\n {event:?}\n");
    }
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
