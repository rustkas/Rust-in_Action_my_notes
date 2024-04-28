use ch4_check_sats_clone_and_copy_traits::{check_status, CubeSat};

fn main() {
    let sat_a = CubeSat::new();

    let a_status = check_status(sat_a.clone());
    println!("a: {}", a_status.clone());
  
    let a_status = check_status(sat_a);
    println!("a: {:}", a_status);
}
