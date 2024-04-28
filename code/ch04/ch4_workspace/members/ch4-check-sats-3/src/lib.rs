use std::fmt;

#[derive(Debug)]
pub struct CubeSat {
    pub id: u64,
}

impl CubeSat {
    pub fn new(id: u64) -> Self {
        CubeSat { id }
    }
}

impl fmt::Display for CubeSat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CubeSat ID: {}", self.id)
    }
}

#[derive(Debug)]
pub enum StatusMessage {
    Ok,
}

impl fmt::Display for StatusMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusMessage::Ok => write!(f, "Ok"),
        }
    }
}

pub fn check_status(sat_id: &CubeSat) -> StatusMessage {
    // let result = match self {
    //   Ok => Ok
    // }
    // println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    // result

    match sat_id.id {
        0..=11 => {
            println!("id: {}:, status: {}", sat_id, StatusMessage::Ok);
            StatusMessage::Ok
        }
        _ => panic!("Bad status"),
    }
}
