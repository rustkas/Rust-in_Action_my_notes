static B: [u8;10] = [99, 97,114,114,121,116,111,119,101,108];
static C: [u8;10] = [116, 104,97,110,107,115,102,105,115,104];
fn main() {
    let a = 42;
    let b = &B;
    let c = &C;
    println!("a={a}, b: {b:p}, c: {c:p}");
}