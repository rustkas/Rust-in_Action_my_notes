
use std::error::Error;

use ch2_try_into::{check_value, types_as};


fn main() {
    types_as();


    let value01_u8:u8 = 1;
    let value02_u8:u8 = u8::MAX/2;

    let value_16:Result<u16,_> = (value01_u8 + value02_u8*2).try_into();
    if value_16.is_ok() {
        println!("ok - {}", value_16.ok().unwrap());
    }else {
        println!("error - {}", value_16.err().unwrap());
    }
    #[allow(unreachable_code)]
    let value_8_result:Result<u8, Box<dyn Error>> = (value01_u8 as u32 +9+ value02_u8  as u32 *2).try_into().map_err(|e| Box::new(e) as Box<dyn Error>);
    check_value(value_8_result);
    
    let value_8_result:Result<u8, Box<dyn Error>> = (u16::MAX).try_into().map_err(|e| Box::new(e) as Box<dyn Error>);
    check_value(value_8_result);

    let value_8_result:Result<u8, Box<dyn Error>> = (u32::MAX).try_into().map_err(|e| Box::new(e) as Box<dyn Error>);
    check_value(value_8_result);

    let value_8_result:Result<u8, Box<dyn Error>> = (u64::MAX).try_into().map_err(|e| Box::new(e) as Box<dyn Error>);
    check_value(value_8_result);
}
