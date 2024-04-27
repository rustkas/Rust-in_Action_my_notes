use ch3_anystring_macro::string;
// mod lib;

fn main() {
    let s = string!(hello);
    println!("{}", s);

    let s = string!("hello");
    println!("{}", s);

    let s = string!(1);
    println!("{}", s);

    let s = string!(1.9);
    println!("{}", s);

    let s = string!([1]);
    println!("{}", s);

    let s = string!({1});
    println!("{}", s);

    let s = string!(&[1]);
    println!("{}", s);

    let s = string!("1".to_string());
    println!("{}", s);

    let s = string!(*"1");
    println!("{}", s);

    let s = string!(***************"1");
    println!("{}", s);

    let s = string!(&&&&&&&&&&&&&&&&"1");
    println!("{}", s);

    let s = string!(one_two);
    println!("{}", s);
}