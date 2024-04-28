use std::fmt;

#[derive(Debug)]
pub enum Event {
    Begin,
    Update,
    Delete,
    Unknown,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Event::*;
        match self {
            Begin => write!(f, "Begin"),
            Update => write!(f, "Update"),
            Delete => write!(f, "Delete"),
            Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
pub struct EventInfo(Event, String);

impl EventInfo {
    pub fn new(event: Event, details: String) -> Self {
        EventInfo(event, details)
    }

    pub fn event(&self) -> &Event {
        &self.0
    }

    pub fn details(&self) -> &String {
        &self.1
    }
}

impl fmt::Display for EventInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Event: {name}, Details: {info}", name = self.0, info=self.1)
    }
}

pub type Message = String; // <3> A convenient name for String for use in this crate's context

pub fn parse_log(line: &'static str) -> EventInfo {
    // <4> A function for parsing a line and converting it into semi-structured data
    let parts: Vec<&str> = line.splitn(2, ' ').collect(); // <5> `collect()` consumes an iterator (returned from `line.splitn()`) and returns `Vec<T>`
    if parts.len() == 1 {
        // <6> If `line.splitn()` didn't split `log` into two parts, return an error
        return EventInfo::new(Event::Unknown, String::from(line));
    }

    let event = parts[0]; // <7> Assign each part to a variable for ease of future use
    let rest = String::from(parts[1]); // <7>

    match event {
        "BEGIN" | "begin" => EventInfo(Event::Begin, rest),
        "UPDATE" | "update" => EventInfo(Event::Update, rest), // <8> When we match a known event, return structured data
        "DELETE" | "delete" => EventInfo(Event::Delete, rest), // <8>
        _ => EventInfo(Event::Unknown, rest), // <9> If we don't recognize the event type, return the whole line
    }
}
