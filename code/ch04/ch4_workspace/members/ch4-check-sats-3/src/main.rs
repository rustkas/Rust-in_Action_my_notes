use ch4_check_sats_3::{check_status, CubeSat};

fn main() {
    let sat_a = CubeSat::new(0);
    let sat_b = CubeSat::new(1);
    let sat_c = CubeSat::new(2);

    {
        let sat_a = check_status(&sat_a);
        let sat_b = check_status(&sat_b);
        let sat_c = check_status(&sat_c);
        println!("a: {}, b: {}, c: {}", sat_a, sat_b, sat_c);

        
    }

    {
        // "waiting" ...
        let sat_a = check_status(&sat_a);
        let sat_b = check_status(&sat_b);
        let sat_c = check_status(&sat_c);
        println!("a: {}, b: {}, c: {}", sat_a, sat_b, sat_c);

    }
}
