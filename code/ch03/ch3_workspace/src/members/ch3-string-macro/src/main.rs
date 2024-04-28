use ch3_string_macro::{clear, dead_end, report, string};

fn main() {
    let mut s = string!("hello");
    println!("{s}");
    report(&s);
    clear(&mut s);
    println!("Now \'s\' is \"{s}\"");

    drop(s);
    let use_never_return_function = !true;

    #[allow(unused_variables, unreachable_code)]
    if use_never_return_function {
        let return_result = dead_end();
        println!("{return_result:?}");
    }
    
}