use ch1_word_counts::get_word_count;

fn main() {
    let text = "once upon a time ... once upon a time ...";
    let word_counts = get_word_count(text);
    println!("{:?}", word_counts);
}
