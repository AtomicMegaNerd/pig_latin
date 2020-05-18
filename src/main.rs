/*
 * Convert strings to pig latin. The first consonant of each word is moved to the end of the word
 * and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
 * to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
 */
fn pig_latenize(text: &str) -> String {
    let words = text.split_whitespace();
    let mut pig_latin = String::new();
    let modified = words.map(add_hay);
    for word in modified {
        pig_latin = format!("{} {}", pig_latin, word);
    }
    return pig_latin;
}

fn add_hay(word: &str) -> String {
    let first_char = word.to_lowercase().chars().next().unwrap();
    match first_char {
        // Starts with Vowel
        'a' | 'e' | 'i' | 'o' | 'u' => return format!("{}-hay", word),
        // Starts with Consonant
        _ => return format!("{}-{}ay", &word[1..], first_char),
    }
}

fn main() {
    println!("{}", pig_latenize("Hello out there world"));
}
