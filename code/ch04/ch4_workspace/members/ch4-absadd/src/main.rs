use ch4_absadd::add_abs;

fn main() {
    let x = 21;
    let y = -2;
    let z = add_abs(x, y);
    println!();
    println!(
        "{x} + {y} = {result}. 
{z} is {result}. It is absolute value",
        x = stringify!(x),
        y = stringify!(y),
        z = stringify!(z),
        result = z
    );
    println!();
}
