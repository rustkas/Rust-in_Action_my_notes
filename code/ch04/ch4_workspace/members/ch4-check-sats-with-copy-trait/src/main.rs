use ch4_check_sats_with_copy_trait::{check_status, CubeSat};

fn main() {
    let sat_a = CubeSat::new_with_data(0);
    let sat_b = CubeSat::new_with_data(1);
    let sat_c = CubeSat::new_with_data(2);

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {a_status}, b: {b_status}, c: {c_status}");
}
