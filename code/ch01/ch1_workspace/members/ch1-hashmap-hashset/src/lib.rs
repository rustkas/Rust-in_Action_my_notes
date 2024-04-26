use std::collections::{HashMap, HashSet};

pub fn get_character_count(input_text: &str)->(HashMap<char,u32>,  u32)  {
  

  let mut character_counts = HashMap::new();

  let mut n_lines = 0u32;

  for l in input_text.lines() {
    n_lines = n_lines + 1;

    let mut chars_for_line = HashSet::new();

    for c in l.chars() {
      if chars_for_line.contains(&c) {
        continue
      }
      let c_count = character_counts.entry(c).or_insert(0u32);
      *c_count += 1;
      chars_for_line.insert(c);
    }
  }
  (character_counts, n_lines)
}