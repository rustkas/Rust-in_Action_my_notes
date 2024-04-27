use rand::random;

pub struct File;

pub static mut ERROR: isize = 0; 

pub fn read(_f: &File, _save_to: &mut Vec<u8>) -> usize {
  if random() && random() && random() { // <4>
      unsafe {
          ERROR = 1; // <5>
      }
  }

  0 // <6>
}