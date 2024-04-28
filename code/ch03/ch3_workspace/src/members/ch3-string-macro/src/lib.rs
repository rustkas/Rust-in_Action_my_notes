use std::fmt::Debug;

#[macro_export]
macro_rules! string {
    ($x:expr) => {
        String::from($x);
    };
}

pub fn report<T: Debug>(item: &T) -> () {
    println!("{:?}", item);
}

pub fn clear(text: &mut String) -> () {
    *text = String::from("");
}

pub fn dead_end() -> ! {
  panic!("That function returns nothing");
}

pub fn forever() -> ! {
  loop {

  }
}
