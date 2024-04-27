use rand::Rng;

#[allow(dead_code)]
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    pub   fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> { // <5>
      let mut tmp = self.data.clone();
      let read_length = tmp.len();
      save_to.reserve(read_length);
      save_to.append(&mut tmp);
      Ok(read_length)  // <6>
    }
}

fn one_in() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_ratio(0, 1)
}

pub fn open(f: File) -> Result<File, String> {
    if one_in() {
        // <7>
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

pub fn close(f: File) -> Result<File, String> {
  if one_in() { // <8>
    let err_msg = String::from("Interrupted by signal!"); 
    return Err(err_msg);
  }
  Ok(f)
}
