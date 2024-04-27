use clap::{Arg, Command};
use regex::Regex; // <1>


fn main() {
    let args = Command::new("grep-lite")
    .version("1.0")
    .about("searches for patterns")
    .arg((Arg::new("pattern")).required(true))
    .get_matches();

    let pattern = args.get_one::<String>("pattern").expect("requered");

    let re = Regex::new(pattern).unwrap();

    println!();
    // let re = Regex::new("picture").unwrap(); // <2>

    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        match re.find(line) {
            // <3>
            Some(_) => println!("{}", line), // <4>
            None => (),                      // <5>
        }
    }
    println!();
}
