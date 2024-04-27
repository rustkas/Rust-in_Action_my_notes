use std::io::prelude::*;
use regex::Regex; // <1>

pub fn process_lines<T: BufRead+Sized>(reader:T, re:Regex) {
    for line in reader.lines() {
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => ()
            
        }
    }
}