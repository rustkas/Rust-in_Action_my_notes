use ch3_defining_files_neatly::File;

fn main() {
    let f3 = File::new("f3.txt");
    
    let f3_name = &f3.name; // <5> Fields are private by default, but can be accessed within the module that defines the struct. The module system is discussed further on in the chapter. 
   //let f3_length = f3.len();
   let f3_length = f3.data.len();
    
    println!("{f3}");
    println!("{f3_name} is {f3_length} bytes long");
  }