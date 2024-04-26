use ch2_expressions::is_even;

fn main() {
    let n = 123456;
    let description = if is_even(n) {
        "even"
    } else {
        "odd"
    };
    println!("{n} is {description}");

    let n = 654321;

    let description = match is_even(n) {
        true => "even",
        false => "odd"
    };
    println!("{n} is {description}");

    match n {
        0 => {},
        10..=20 => {},
        40 | 80 => {},
        _ => {println!("{n}")}
    }
   
}
