use std::fmt;
use std::fmt::Display;

#[derive(Debug,PartialEq)]
pub enum FileOpenMode {
    Read,
    Write,
    Append,
    Truncate,
}

impl Display for FileOpenMode {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use FileOpenMode::*;
    match *self {
        Read => write!(f, "READ"),     
        Write => write!(f, "WRITE"), 
        Append => write!(f, "APPEND"), 
        Truncate => write!(f, "TRUNCATE"), 
    }
  }
}


#[derive(Debug)]
pub enum FileHandle {
    Handle(usize),
    None,
}

impl Display for FileHandle {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use FileHandle::*;
    match *self {
      Handle(id) => write!(f, "HANDLE({id})"),     
      None => write!(f, "NONE"),
    }
  }
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

impl Display for FileState {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use FileState::*;
    match self {
      PendingCreation => write!(f, "PENDING_CREATION"),     
      Created(file_open_mode) => write!(f, "CREATED({file_open_mode})"),
      Opened(file_open_mode) => write!(f, "OPENED({file_open_mode})"),
      Error(error_info) => write!(f, "ERROR({error_info})"),
      Closed => write!(f, "CLOSED"),
      Deleted => write!(f, "DELETED"),
    }
  }
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
