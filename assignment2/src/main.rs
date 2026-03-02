fn most_frequent_word(text: &str) -> (String, usize) {
    let mut words: Vec<(String, usize)> = Vec::new();

    for w in text.split_whitespace() {
        let mut found = false;

        for (word, count) in words.iter_mut() {
            if word == w {
                *count += 1;
                found = true;
                break;
            }
        }

        if !found {
            words.push((w.to_string(), 1));
        }
    }

    let mut max_word = String::new();
    let mut max_count = 0;

    for (word, count) in words.iter() {
        if *count > max_count {
            max_count = *count;
            max_word = word.clone();
        }
    }

    (max_word, max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";

    let (word, count) = most_frequent_word(text);

    println!("Most frequent word: \"{}\" ({} times)", word, count);
}