fn main() {
    let word = first_word_str(String::from("    hello      world    "));
    assert_eq!("hello", word);
}

fn first_word_str(word: String) -> String {
    let word_str = word.trim();
    let index = find_word_after_space(&word_str);
    return String::from(&word_str[0..index]);
}
fn find_word_after_space(word: &str) -> usize {
    let bytes = word.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return word.len();
}