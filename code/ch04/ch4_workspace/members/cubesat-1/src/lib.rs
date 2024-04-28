#[derive(Debug)]
pub struct Message{
pub to:u64,
pub content:String,
}

#[derive(Debug)]
pub struct Mailbox {
  pub messages: Vec<Message>,
}
#[derive(Debug)]
pub struct CubeSat {
  pub id: u64,
  pub mailbox: Mailbox,
}

#[derive(Debug)]
pub enum StatusMessage {
  Ok
}

pub struct GroundStation;

impl GroundStation {
  pub fn connect(&self, sat_id:u64) -> CubeSat {
    CubeSat {
      id: sat_id,
      mailbox: Mailbox { messages: vec![] },
    }
  }
  pub fn send(&self, to: &mut CubeSat, msg: Message) {
    to.mailbox.messages.push(msg);
  }
}

impl CubeSat {
  pub fn recv(&mut self) -> Option<Message> {
    self.mailbox.messages.pop()
  }
}

impl Mailbox {
    pub fn post(&mut self, msg: Message) {
      self.messages.push(msg);
    }

    pub fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
      for i in 0..self.messages.len() {
        if self.messages[i].to == recipient.id {
          let msg = self.messages.remove(i);
          return Some(msg);
        }
      }
      None
    }
}

pub fn send(to: &mut CubeSat, msg: Message) -> () {
  to.mailbox.messages.push(msg);
}