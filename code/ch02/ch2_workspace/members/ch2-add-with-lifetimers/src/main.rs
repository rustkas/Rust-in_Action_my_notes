use std::time::Duration;

use ch2_add_with_lifetimers::{add, add_with_lifetimers, add_with_lifetimers2, add_with_lifetimes};

fn main() {

    let a = 10;
    let b = 20;

    let res = add_with_lifetimers(&a, &b);
    println!("{} = {}", stringify!(res), res);

    let res = add_with_lifetimers2(&a, &b);
    println!("{} = {}", stringify!(res), res);

    let res = add_with_lifetimes(&a, &b);
    println!("{} = {}", stringify!(res), res);


    let floats = add(1.2, 3.4);
    println!("{floats}");

    let ints = add(10, 20);
    println!("{ints}");

    let duration = add(Duration::new(5,0), Duration::new(10,0));
    println!("{duration:?}");
}
