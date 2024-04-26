use ch1_hashmap_hashset::get_character_count;

fn main() {
    let input_text = "Does this work?
I dont know.
How does Rust work?";
    let (character_counts, n_lines) = get_character_count(input_text);

    for (c, c_count) in &character_counts {
        if *c_count == n_lines {
            println!("{}", c);
        }
    }
}
