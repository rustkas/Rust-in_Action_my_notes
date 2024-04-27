use std::fmt;

#[derive(Debug)] // <1>
pub struct File {
  pub name: String,
  pub data: Vec<u8>, // <2>
}

impl fmt::Display for File {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(
          f,
          "File: {}\nData Length: {} bytes",
          self.name,
          self.data.len()
      )
  }
}