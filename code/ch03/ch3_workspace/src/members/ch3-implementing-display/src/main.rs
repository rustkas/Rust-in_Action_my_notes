use ch3_implementing_display::File;

fn main() {
    let f5 = File::new("f5.txt");
    //...
    println!("{:?}", f5); // <1>
    println!("{}", f5); // <1>
  }