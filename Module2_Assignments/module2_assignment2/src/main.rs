fn most_frequent_word(text: &str) -> (String, usize) {

    let mut counts: Vec<(&str, usize)> = Vec::new();

    for w in text.split_whitespace(){
        let mut found = false;

        for(word, count) in counts.iter_mut() {
            if *word == w{
                *count += 1;

                found = true;
                break;
            }
        }

        if !found{
                counts.push((w, 1));
            }
    }
    
     let mut max_word: &str = "";
     let mut max_count: usize = 0;

     for(word, count) in counts.iter(){
        if *count > max_count{
            max_count = *count;
            max_word = *word;
        }
     }

    (max_word.to_string(), max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}