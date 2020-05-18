/*
 * Convert strings to pig latin. The first consonant of each word is moved to the end of the word
 * and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
 * to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
 */
fn pig_latenize(text: &str) -> String {
    let mut pig_lat = String::new();
    let chars = text.chars();

    for ch in chars {
        pig_lat.push(ch);
    }

    return pig_lat;
}

fn main() {
    println!("{}", pig_latenize("Hello world"));
}
