fn main() {
    let mut letters = vec![
        "a", "b", "b"
    ];
    letters.push("c");
    let mut buf = String::new();
    for letter in &letters {
        let text = format!("{}, ", letter);
        buf.push_str(&text);
        // letters.push(letter.clone());
    }
    buf.pop();
    buf.pop();
    println!("the vector content: {buf}");
    

  }