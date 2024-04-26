use std::thread;

#[allow(non_upper_case_globals)]
fn main() {
    static mut data: u32 = 100;

    thread::spawn(|| unsafe {
        data = 500;
    });
    thread::spawn(|| unsafe {
        data = 1000;
    });

    unsafe {
        println!("{}", data);
    }
}
