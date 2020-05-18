/*
 * Convert strings to pig latin. The first consonant of each word is moved to the end of the word
 * and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
 * to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
 */

/// This function takes a string slice and it splits it out by words and by punctuation.
/// It returns a vector where each item in the vector is either a word or a non-word.  Each
/// word or non-word character (number or punctuation) is 1 item in the vector
fn tokenize_words_and_punctuation(text: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut token = String::new();
    for ch in text.chars() {
        if ch.is_alphabetic() {
            token.push(ch);
        } else {
            if !token.is_empty() {
                result.push(token);
            }
            token = String::new();
            result.push(ch.to_string());
        }
    }
    // Handle any sentence that ends with a word.
    if !token.is_empty() {
        result.push(token);
    }
    return result;
}

/// Convert one word to pig latin. The first consonant is moved to the end of the word and “ay” is
/// added, so "first" becomes "irst-fay."" If hte word starts with a vowel have "hay" added
/// to the end instead ("apple" becomes "apple-hay").
fn pig_latinize_word(word: &String) -> String {
    let is_upper = word.chars().next().unwrap().is_uppercase();
    let first_char = word.to_lowercase().chars().next().unwrap();
    let mod_word = match first_char {
        // Starts with Vowel
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        // Starts with consonant
        'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r' | 's'
        | 't' | 'v' | 'w' | 'x' | 'y' | 'z' => format!("{}-{}ay", &word[1..], first_char),
        // Any other character
        _ => format!("{}", first_char),
    };
    if is_upper {
        return capitalize(&mod_word);
    }
    return mod_word;
}

fn capitalize(word: &str) -> String {
    let first_char = word.chars().next().unwrap();
    return format!("{}{}", first_char.to_uppercase(), &word[1..]);
}

fn pig_latenize(text: &str) -> String {
    tokenize_words_and_punctuation(text)
        .iter()
        .map(pig_latinize_word)
        .collect::<Vec<String>>()
        .join("")
}

fn main() {
    println!("{}", pig_latenize("Hello again, out there wonderful world"));
    println!(
        "{}",
        pig_latenize("Today is a fantastic day, and I am so happy!")
    );
}
