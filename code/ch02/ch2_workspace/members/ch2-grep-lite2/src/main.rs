use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use clap::Command;
use regex::Regex; // <1>


fn main() {
    let args = Command::new("grep-lite")
    .version("1.0")
    .about("searches for patterns")
    // .arg((Arg::new("pattern"))
    .arg(clap::arg!(-p --pattern <PATTERN>)
    .help("The pattern to search for")
    .required(true))
    // .arg(Arg::new("input")
    .arg(clap::arg!(-i --input <FILENAME>)
    .help("File to search"))
    .get_matches();

    let re = {
        let pattern = args.get_one::<String>("pattern").expect("requered");

        let re = Regex::new(pattern).unwrap();
        re
    };
    let reader = {
        let input = args.get_one::<String>("input").expect("requered");
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);
        reader
    };
    
    for line in reader.lines() {
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => ()
            
        }
    }
    println!();
    
}
