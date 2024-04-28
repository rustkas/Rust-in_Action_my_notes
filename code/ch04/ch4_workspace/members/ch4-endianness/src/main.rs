use ch4_endianness::transmute1;

fn main() {
    let [a, b] = transmute1();

    println!("{} vs {}", a, b);
}
