use std::collections::HashMap;

pub fn get_word_count(text: &str) -> HashMap<&str, u32> {
   
    let mut word_counts = HashMap::new();
    
    let pairs = text.split(" ")
                    .map(|x| { (x, 1) });
    
    for (word, count) in pairs {
        let tmp = word_counts.entry(word)
                             .or_insert(0);
        *tmp += count;
    }
    
    word_counts
}