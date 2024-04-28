use std::fmt;

#[derive(Debug,Copy,Clone)]
pub struct CubeSat {
  id: u64,
}

#[derive(Debug,Copy,Clone)]
pub enum StatusMessage {
  Ok,
}

impl fmt::Display for CubeSat {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "CubeSat ID: {}", self.id)
  }
}
impl Default for CubeSat {
  fn default() -> Self {
      CubeSat { id: 1 }
  }
}
impl fmt::Display for StatusMessage {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
          StatusMessage::Ok => write!(f, "Status: Ok"),
      }
  }
}

impl CubeSat {
  pub fn new() -> Self {
      CubeSat{
        ..Default::default()
      }
  }

}


pub fn check_status(sat: CubeSat) -> StatusMessage {
  println!("{sat}");
    if sat.id >= 1 {
     return StatusMessage::Ok
    }
    panic!("Bad id")
}