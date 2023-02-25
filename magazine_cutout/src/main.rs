use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let magazine = &["a", "b", "c", "d", "e"];
    let note = &["a", "e"];
    assert!(can_construct_note(magazine, note));
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut wordbag: HashMap<&str, i32> = HashMap::new();
    for n in note.iter() {
        wordbag.entry(&n).and_modify(|count| *count+= 1).or_insert(1);
    }
                    
    for w in magazine.iter() {
        wordbag.entry(&w).and_modify(|count| *count-= 1);
    }

    let results: i32 = wordbag.into_values().map(|x| max(0, x)).sum();
    if results == 0 { true } else { false }
}

