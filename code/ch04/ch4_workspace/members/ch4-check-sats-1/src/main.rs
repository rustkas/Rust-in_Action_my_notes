use ch4_check_sats_1::check_status;

fn main () {
    println!();
    let sat_a = 0;
    let sat_b = 1;
    let sat_c = 2;
  
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {a_status}, b: {b_status}, c: {c_status}");
  
    // "waiting" ...
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {a_status}, b: {b_status}, c: {c_status}");
    println!();
  }
  