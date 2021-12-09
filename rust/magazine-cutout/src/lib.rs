use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // get number of occurrences of each word in the magazine
    let mut word_counts = HashMap::new();
    for word in magazine {
        word_counts
            .entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    // check each word in the note exists in the magazine
    for word in note {
        let cannot_find_spare_word = match word_counts.get(word) {
            None => true,
            Some(count) => {
                if *count > 0 {
                    false
                } else {
                    true
                }
            }
        };

        if cannot_find_spare_word {
            return false;
        } else {
            word_counts.entry(word).and_modify(|count| *count -= 1);
            continue;
        }
    }
    return true;
}
