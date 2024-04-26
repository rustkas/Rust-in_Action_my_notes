fn main() {
    let x = (-42.0_f32).sqrt();

    if !x.is_nan() {
        assert_eq!(x, x);
    }
    println!("{} is {}", stringify!(x), x);

    let x:f32 = 1.0 / 0.0;
    println!("{} is {}", stringify!(x), x);
    
    assert!(x.is_infinite());
    assert_eq!(f32::INFINITY, x);

}
