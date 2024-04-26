#![allow(dead_code)]

struct Animal {
    age: i32,
}

struct Cat {
    animal: Animal,
}

struct Dog {
    animal: Animal,
}

struct LoudDog {
    animal: Animal,
}

trait Talk {
    fn talk(&self);
}

impl Talk for Cat {
    fn talk(&self) {
        println!("Meow");
    }
}

impl Talk for Dog {
    fn talk(&self) {
        println!("Woof!");
    }
}

impl Talk for LoudDog {
    fn talk(&self) {
        println!("WOOF!!");
    }
}

fn main() {
    let fluffy = Cat { animal: Animal { age: 4 } };
    let max = Dog { animal: Animal { age: 2 } };
    let neighbours_dog = LoudDog { animal: Animal { age: 7 } };

    fluffy.talk();
    max.talk();
    neighbours_dog.talk();
}
