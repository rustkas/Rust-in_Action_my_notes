fn main() {
    // assert!(0.1 + 0.2 == 0.3);
    assert_eq!(0.3 as f32, (0.1 + 0.2) as f32 ) ;
    assert_ne!(0.3 as f64, (0.1 + 0.2) as f64 ) ;
}
