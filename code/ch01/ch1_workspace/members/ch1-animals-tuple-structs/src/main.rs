#![allow(dead_code)]

use std::fmt;

#[derive(Debug)]
struct Animal {
    age: i32,
}

impl fmt::Display for Animal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Animal {{ age: {} }}", self.age)
    }
}
struct Cat(Animal);
struct Dog(Animal);
struct LoudDog(Animal);

trait Talk {
	fn talk(&self) -> ();
}

impl Talk for Cat {
	fn talk(&self) {
        
		println!("Meow, sad cat. {}",&self.0);
	}
}

impl Talk for Dog {
	fn talk(&self) {
		println!("Woof!, sad dog. {}",&self.0);
	}
}

impl Talk for LoudDog {
	fn talk(&self) {
		println!("WOOF!! sad loud dog. {}",&self.0);
	}
}

fn main() {
	let fluffy = Cat(Animal { age: 4 });
	let max = Dog(Animal { age: 2 });
	let neighbours_dog = LoudDog(Animal { age: 7 });

	fluffy.talk();
	max.talk();
	neighbours_dog.talk();
}
