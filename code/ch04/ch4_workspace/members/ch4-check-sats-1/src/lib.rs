use std::fmt;

#[derive(Debug)]
pub enum StatusMessage {
  Ok,
}

impl fmt::Display for StatusMessage {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
          StatusMessage::Ok => write!(f, "OK"),
      }
  }
}


pub fn check_status(_sat_id: u8) -> StatusMessage {
  StatusMessage::Ok
}