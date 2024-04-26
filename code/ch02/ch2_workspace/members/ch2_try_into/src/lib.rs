use std::error::Error;

pub fn types_as() {
    let value_u8 = 0_u8;
    let value_u16 = 0_u16;
    let value_u32 = 0_u32;
    let value_u64 = 0_u64;
    let value_u128 = 0_u128;

    let value_i8 = 0_i8;
    let value_i16 = 0_i16;
    let value_i32 = 0_i32;
    let value_i64 = 0_i64;
    let value_i128 = 0_i128;

    let value_f32 = 0_f32;
    let value_f64 = 0_f64;

    let value_usize = 0_usize;
    let value_isize = 0_isize;

    let value_true = true;
    let value_false = false;

    if value_u8 != value_i8 as u8 {
        println!(
            "{param1} != {param2}",
            param1 = stringify!(value_u8),
            param2 = stringify!(value_i8)
        )
    }

    assert_eq!(value_u8, value_u16 as u8);
    assert_eq!(value_u8, value_u32 as u8);
    assert_eq!(value_u8, value_u64 as u8);
    assert_eq!(value_u8, value_u128 as u8);

    assert_eq!(value_u8, value_i8 as u8);
    assert_eq!(value_u8, value_i16 as u8);
    assert_eq!(value_u8, value_i32 as u8);
    assert_eq!(value_u8, value_i64 as u8);
    assert_eq!(value_u8, value_i128 as u8);

    assert_eq!(value_u8, value_usize as u8);
    assert_eq!(value_u8, value_isize as u8);

    assert_eq!(value_u8, value_f32 as u8);
    assert_eq!(value_u8, value_f64 as u8);

    assert_ne!(value_u8, value_true as u8);
    assert_eq!(value_u8, value_false as u8);

    // assert_ne!(value_u8 as bool, value_true);
    // assert_eq!(value_u8 as bool, value_false);

    // if value_u8 as bool != value_true {
    //     println!("{param1} != {param2}", param1=stringify!(value_u8), param2=stringify!(value_i8))
    // }
    assert_eq!(value_u16, value_u8 as u16);
    assert_eq!(value_u32, value_u8 as u32);
    assert_eq!(value_u64, value_u8 as u64);
    assert_eq!(value_u128, value_u8 as u128);

    assert_eq!(value_i8, value_u8 as i8);
    assert_eq!(value_i16, value_u8 as i16);
    assert_eq!(value_i32, value_u8 as i32);
    assert_eq!(value_i64, value_u8 as i64);
    assert_eq!(value_i128, value_u8 as i128);

    assert_eq!(value_usize, value_u8 as usize);
    assert_eq!(value_isize, value_u8 as isize);

    assert_eq!(value_f32, value_u8 as f32);
    assert_eq!(value_f64, value_u8 as f64);

    // assert_ne!(value_true, value_u8 as bool);
    // assert_eq!(value_false, value_u8 as bool);
}

pub fn check_value(value_8_result: Result<u8, Box<dyn Error>>) {
    if value_8_result.is_ok() {
        println!("ok - {}", value_8_result.ok().unwrap());
    } else {
        println!("error - {}", value_8_result.err().unwrap());
    }
}
