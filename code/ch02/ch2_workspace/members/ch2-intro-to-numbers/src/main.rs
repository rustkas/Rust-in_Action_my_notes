#[allow(unused_doc_comments)]
fn main() {
    let twenty = 20; // <1>
    let twenty_one: u32 = twenty + 1; // <2>
    let floats_okay = 21.00; // <3>
    let one_million = 1_000_000; // <4>

    println!(
        "{}; {}; {}; {}",
        twenty, twenty_one, floats_okay, one_million
    );

    let twenty_two = 22i32 as u32;

    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [42.0, 42f32, 42.0_f32];

    /// десь 0 означает заполнение значением 0, а 2 указывает на минимальную ширину поля, 
    /// которая равна двум символам. Если значение меньше двух символов, оно будет дополнено 
    /// нулями слева.
    println!("{:010}", forty_twos[0]);
}
