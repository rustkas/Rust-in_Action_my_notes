use ch2_first_steps::add;

fn main() {
    let a = 10; // <1>
    let b: i32 = 20; // <2>

    let c = add(a, b);
    println!("a + b = {}", c);

    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("( а + Ь ) + ( с + d ) = { }", e);
}
