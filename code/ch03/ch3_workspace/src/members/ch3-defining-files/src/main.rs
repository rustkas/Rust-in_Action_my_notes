use ch3_defining_files::File;

fn main() {
    let f1 = File {
        name: String::from("f1.txt"), // <3>
        data: Vec::new(),             // <4>
    };

    let f1_name = &f1.name; // <5>
    let f1_length = &f1.data.len(); // <5>

    println!("{f1}");
    println!("{f1_name} is {f1_length} bytes long");
}
