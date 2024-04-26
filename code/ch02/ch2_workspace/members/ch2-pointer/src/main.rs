pub fn ex01() {
    let a = 42;
    let r = &a;
    let b = a + *r;

    println!("{} + {} = {}", stringify!(a), stringify!(*r), b);
}

pub fn ex02() {
    let needle = 0o204;
    println!("{} = {}", stringify!(needle), needle);
    let haystack = [1, 1, 2, 5, 15, 52,132, 203, 877, 4140, 21147];

    for item in &haystack {
        if *item == needle  {
            println!("{item}");
        }
    }
}
fn main() {
    ex02();
}
