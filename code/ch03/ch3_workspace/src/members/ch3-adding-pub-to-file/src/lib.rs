use std::fmt;

#[derive(Debug,PartialEq)]
pub enum FileState {
  Open,
  Closed,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct File {
  pub name: String,
  data: Vec<u8>,
  pub state: FileState,
}

impl Default for File {
  fn default() -> Self {
      Self {
          name: String::from(""),
          data: Vec::new(),
          state: FileState::Closed,
      }
  }
}

impl fmt::Display for File {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(
          f,
          "File: {}\nState: {:?}\nData Length: {} bytes",
          self.name,
          self.state,
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
    
}