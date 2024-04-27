use std::fmt;

#[derive(Debug,PartialEq)]
pub enum FileState {
  Open,
  Closed,
}

#[derive(Debug)]
pub struct File {
  pub name: String,
  pub data: Vec<u8>,
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
  pub fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
    if self.state != FileState::Open {
      return Err(String::from("File must be open for reading"));
    }
    let mut tmp = self.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    Ok(read_length)  // <6>
  }
}

pub fn open(mut f: File) -> Result<File, String> {
  f.state = FileState::Open;
  Ok(f)
}

pub fn close(mut f: File) -> Result<File, String> {
  f.state = FileState::Closed;
  Ok(f)
}