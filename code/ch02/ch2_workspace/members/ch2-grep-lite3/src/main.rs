use std::fs::File;
use std::io::BufReader;
use ch2_grep_lite3::process_lines;
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

    let string = "-".to_string();
    let input = args.get_one::<String>("input").unwrap_or(&string);

    if input == "-" {
        let reader = {
            let stdin = std::io::stdin();
            let reader = stdin.lock();
            reader    
        };
        process_lines(reader, re);
    } else {
        let reader = {
            let file = File::open(input).unwrap();
            let reader = BufReader::new(file); 
            reader    
        };
        process_lines(reader, re);       
    }
    

    println!();
    
}
