use std::fmt;

#[derive(Debug,PartialEq)]
pub enum FileOpenMode {
    Read,
    Write,
    Append,
    Truncate,
}

#[derive(Debug)]
pub enum FileHandle {
    Handle(usize),
    None,
}

#[derive(Debug,PartialEq)]
pub enum FileState {
    PendingCreation,
    Created(FileOpenMode),
    Opened(FileOpenMode),
    Error(String),
    Closed,
    Deleted,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub data: Vec<u8>,
    pub state: FileState,
    pub handle: FileHandle,
}

impl Default for File {
    fn default() -> Self {
        Self {
            name: String::from(""),
            data: Vec::new(),
            state: FileState::PendingCreation,
            handle: FileHandle::None,
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File: {}\nState: {:?}\nHandle:{:?}\nData Length: {} bytes",
            self.name,
            self.state,
            self.handle,
            self.data.len()
        )
    }
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            ..Default::default()
        }
    }
    pub fn from_options(name: &str, state: FileState, handle: FileHandle) -> File {
        File {
            name: String::from(name),
            state: state,
            handle: handle,
            ..Default::default()
        }
    }
}
