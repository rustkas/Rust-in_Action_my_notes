use rand::Rng;

extern crate rand; // <> Make an external crate available to our code 
// use rand; // <> Bring `rand` into local scope

pub static mut ERROR: isize = 0;

pub struct File;


pub fn read(_f: &File, _save_to: Vec<u8>) -> usize {
    if rand::thread_rng().gen_ratio(1,2) {
        unsafe {
            ERROR = 1;
        }
    }

    0 // <> Always read() 0 bytes  
}