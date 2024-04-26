fn main() {
    let fruit = vec!['🥝', '🍌', '🍇'];
    if fruit.len() >= 4 {
        let buffer_overflow = fruit[4];   // <1>
        assert_eq!(buffer_overflow, '🍉') // <2>
    } else {
        let buffer_overflow = fruit[2];   
        assert_ne!(buffer_overflow, '🍉');
        for item in fruit {
            println!("{item}");
        } 
    }
    
  }